//! Queueing locks, from Harvey's `port/qlock.c`.
//!
//! [`QLock`] is a mutual-exclusion lock and [`RWLock`] a readers/writer lock.
//! Both hand the lock to waiters in FIFO order and put the caller to sleep
//! instead of spinning.
//!
//! The C versions reach into the current process, call the scheduler, and keep
//! their wait queue as a list threaded through `Proc`. Here a lock is just the
//! state machine: it is generic over a waiter handle `W` (a pid, say), and tells
//! the caller what to do. A `lock` call says whether the caller now holds the
//! lock or must go to sleep; an `unlock` call returns the waiters that now hold
//! the lock and must be made ready. Callers serialise access to the lock itself
//! (the C used a spin lock, `use`, for that), so methods take `&mut self`.
//!
//! Left to the caller, as it was never part of the locking logic: wait
//! statistics, tracing, and recording who took the lock.

extern crate alloc;

use alloc::collections::VecDeque;
use alloc::vec::Vec;

/// Outcome of a blocking lock request.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acquire {
    /// The caller holds the lock now.
    Held,
    /// The caller was queued; it holds the lock once it is handed back by
    /// an unlock, which returns it for the caller to make ready.
    Queued,
}

/// The lock was released while not held.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NotHeld;

/// A queueing mutual-exclusion lock.
#[derive(Debug)]
pub struct QLock<W> {
    locked: bool,
    waiters: VecDeque<W>,
}

impl<W> QLock<W> {
    pub const fn new() -> Self {
        Self { locked: false, waiters: VecDeque::new() }
    }

    pub fn is_locked(&self) -> bool {
        self.locked
    }

    /// Waiting callers, longest-waiting first.
    pub fn waiters(&self) -> impl Iterator<Item = &W> {
        self.waiters.iter()
    }

    /// Take the lock for `me`, or queue `me` behind the holder.
    pub fn lock(&mut self, me: W) -> Acquire {
        if self.locked {
            self.waiters.push_back(me);
            Acquire::Queued
        } else {
            self.locked = true;
            Acquire::Held
        }
    }

    /// Take the lock if it is free.
    pub fn try_lock(&mut self) -> bool {
        !core::mem::replace(&mut self.locked, true)
    }

    /// Release the lock. If someone is waiting the lock passes straight to
    /// the longest waiter, who is returned so it can be made ready.
    pub fn unlock(&mut self) -> Result<Option<W>, NotHeld> {
        if !self.locked {
            return Err(NotHeld);
        }
        let next = self.waiters.pop_front();
        self.locked = next.is_some();
        Ok(next)
    }
}

impl<W> Default for QLock<W> {
    fn default() -> Self {
        Self::new()
    }
}

/// Why a process is waiting on an [`RWLock`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Want {
    Read,
    Write,
}

/// A queueing readers/writer lock.
///
/// Any number of readers may hold the lock together, or a single writer. A
/// reader that arrives while anyone is queued waits too, so writers are not
/// starved.
#[derive(Debug)]
pub struct RWLock<W> {
    readers: u32,
    writer: bool,
    waiters: VecDeque<(W, Want)>,
}

impl<W> RWLock<W> {
    pub const fn new() -> Self {
        Self { readers: 0, writer: false, waiters: VecDeque::new() }
    }

    /// Number of readers holding the lock.
    pub fn readers(&self) -> u32 {
        self.readers
    }

    /// Whether a writer holds the lock.
    pub fn has_writer(&self) -> bool {
        self.writer
    }

    /// Waiting callers, longest-waiting first.
    pub fn waiters(&self) -> impl Iterator<Item = &(W, Want)> {
        self.waiters.iter()
    }

    /// Whether a new reader can go straight in: no writer, nobody queued.
    fn reader_may_enter(&self) -> bool {
        !self.writer && self.waiters.is_empty()
    }

    /// Take a read lock for `me`, or queue it.
    pub fn rlock(&mut self, me: W) -> Acquire {
        if self.reader_may_enter() {
            self.readers += 1;
            Acquire::Held
        } else {
            self.waiters.push_back((me, Want::Read));
            Acquire::Queued
        }
    }

    /// Take a read lock if no writer holds or waits for the lock.
    pub fn try_rlock(&mut self) -> bool {
        let ok = self.reader_may_enter();
        if ok {
            self.readers += 1;
        }
        ok
    }

    /// Release a read lock. The last reader out starts the waiting writer,
    /// who is returned so it can be made ready.
    pub fn runlock(&mut self) -> Result<Option<W>, NotHeld> {
        self.readers = self.readers.checked_sub(1).ok_or(NotHeld)?;
        if self.readers > 0 {
            return Ok(None);
        }
        let Some((writer, want)) = self.waiters.pop_front() else {
            return Ok(None);
        };
        assert_eq!(want, Want::Write, "runlock: head waiter is not a writer");
        self.writer = true;
        Ok(Some(writer))
    }

    /// Take the write lock for `me`, or queue it.
    pub fn wlock(&mut self, me: W) -> Acquire {
        if self.readers == 0 && !self.writer {
            self.writer = true;
            Acquire::Held
        } else {
            self.waiters.push_back((me, Want::Write));
            Acquire::Queued
        }
    }

    /// Release the write lock and return the waiters that now hold the lock,
    /// to be made ready: the next writer alone, or else every reader at the
    /// head of the queue.
    pub fn wunlock(&mut self) -> Result<Vec<W>, NotHeld> {
        if !self.writer {
            return Err(NotHeld);
        }
        let mut woken = Vec::new();
        match self.waiters.front() {
            // The lock passes to the next writer, so `writer` stays set.
            Some((_, Want::Write)) => woken.extend(self.waiters.pop_front().map(|(w, _)| w)),
            _ => {
                self.writer = false;
                while let Some((_, Want::Read)) = self.waiters.front() {
                    woken.extend(self.waiters.pop_front().map(|(w, _)| w));
                    self.readers += 1;
                }
            }
        }
        Ok(woken)
    }
}

impl<W> Default for RWLock<W> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use port_oracle::qsim::{Step, Waiter, World};

    #[test]
    fn qlock_is_fifo_and_hands_off() {
        let mut q = QLock::new();
        assert_eq!(q.lock(0), Acquire::Held);
        assert_eq!(q.lock(1), Acquire::Queued);
        assert_eq!(q.lock(2), Acquire::Queued);
        assert!(!q.try_lock());
        assert_eq!(q.unlock(), Ok(Some(1)));
        assert!(q.is_locked(), "ownership passes to the waiter");
        assert_eq!(q.unlock(), Ok(Some(2)));
        assert_eq!(q.unlock(), Ok(None));
        assert!(!q.is_locked());
        assert_eq!(q.unlock(), Err(NotHeld));
        assert!(q.try_lock());
    }

    #[test]
    fn rwlock_readers_share_and_writers_exclude() {
        let mut l = RWLock::new();
        assert_eq!(l.rlock(0), Acquire::Held);
        assert_eq!(l.rlock(1), Acquire::Held);
        assert_eq!(l.wlock(2), Acquire::Queued);
        assert!(!l.try_rlock(), "a waiting writer keeps new readers out");
        assert_eq!(l.rlock(3), Acquire::Queued);

        assert_eq!(l.runlock(), Ok(None));
        assert_eq!(l.runlock(), Ok(Some(2)));
        assert!(l.has_writer());
        assert_eq!(l.wunlock(), Ok(vec![3]));
        assert_eq!((l.readers(), l.has_writer()), (1, false));
        assert_eq!(l.runlock(), Ok(None));
        assert_eq!(l.runlock(), Err(NotHeld));
        assert_eq!(l.wunlock(), Err(NotHeld));
    }

    #[test]
    fn wunlock_wakes_a_run_of_readers_then_stops_at_a_writer() {
        let mut l = RWLock::new();
        assert_eq!(l.wlock(0), Acquire::Held);
        for r in 1..=3 {
            assert_eq!(l.rlock(r), Acquire::Queued);
        }
        assert_eq!(l.wlock(4), Acquire::Queued);
        assert_eq!(l.rlock(5), Acquire::Queued);

        assert_eq!(l.wunlock(), Ok(vec![1, 2, 3]));
        assert_eq!((l.readers(), l.has_writer()), (3, false));
        let rest: Vec<_> = l.waiters().copied().collect();
        assert_eq!(rest, [(4, Want::Write), (5, Want::Read)]);
    }

    /// Deterministic xorshift64* generator.
    struct Rng(u64);

    impl Rng {
        fn below(&mut self, n: usize) -> usize {
            self.0 ^= self.0 >> 12;
            self.0 ^= self.0 << 25;
            self.0 ^= self.0 >> 27;
            (self.0.wrapping_mul(0x2545_f491_4f6c_dd1d) >> 33) as usize % n
        }
    }

    /// What a process is doing, as the test driver sees it.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum Proc {
        Idle,
        WaitingQ,
        WaitingR,
        WaitingW,
        HoldsQ,
        HoldsR,
        HoldsW,
    }

    fn step(blocked: bool, readied: &[usize]) -> Step {
        Step { blocked, readied: readied.to_vec(), prints: vec![] }
    }

    /// Run random but legal operation sequences through the c2rust
    /// translation of `qlock.c` and through this port in lock step, and
    /// require the same outcome and the same queues after every operation.
    #[test]
    fn matches_c2rust_on_random_schedules() {
        const PROCS: usize = 5;
        for seed in 1..=300u64 {
            let mut rng = Rng(seed.wrapping_mul(0x9e37_79b9_7f4a_7c15) | 1);
            let mut oracle = World::new(PROCS);
            let mut q = QLock::<usize>::new();
            let mut rw = RWLock::<usize>::new();
            let mut state = [Proc::Idle; PROCS];

            for n in 0..200 {
                let p = rng.below(PROCS);
                let ctx = format!("seed {seed} op {n} proc {p} {state:?}");
                let (expect, got) = match state[p] {
                    Proc::WaitingQ | Proc::WaitingR | Proc::WaitingW => continue,
                    Proc::Idle => match rng.below(5) {
                        0 => {
                            let s = oracle.qlock(p);
                            state[p] = if s.blocked { Proc::WaitingQ } else { Proc::HoldsQ };
                            (s.clone(), step(q.lock(p) == Acquire::Queued, &[]))
                        }
                        1 => {
                            let (ok, s) = oracle.canqlock(p);
                            if ok {
                                state[p] = Proc::HoldsQ;
                            }
                            // `blocked` is false and nothing is readied for a try.
                            assert_eq!(q.try_lock(), ok, "try_lock: {ctx}");
                            (s, step(false, &[]))
                        }
                        2 => {
                            let s = oracle.rlock(p);
                            state[p] = if s.blocked { Proc::WaitingR } else { Proc::HoldsR };
                            (s.clone(), step(rw.rlock(p) == Acquire::Queued, &[]))
                        }
                        3 => {
                            let (ok, s) = oracle.canrlock(p);
                            if ok {
                                state[p] = Proc::HoldsR;
                            }
                            assert_eq!(rw.try_rlock(), ok, "try_rlock: {ctx}");
                            (s, step(false, &[]))
                        }
                        _ => {
                            let s = oracle.wlock(p);
                            state[p] = if s.blocked { Proc::WaitingW } else { Proc::HoldsW };
                            (s.clone(), step(rw.wlock(p) == Acquire::Queued, &[]))
                        }
                    },
                    Proc::HoldsQ => {
                        let s = oracle.qunlock(p);
                        state[p] = Proc::Idle;
                        for &w in &s.readied {
                            state[w] = Proc::HoldsQ;
                        }
                        (s, step(false, &q.unlock().unwrap().into_iter().collect::<Vec<_>>()))
                    }
                    Proc::HoldsR => {
                        let s = oracle.runlock(p);
                        state[p] = Proc::Idle;
                        for &w in &s.readied {
                            state[w] = Proc::HoldsW;
                        }
                        (s, step(false, &rw.runlock().unwrap().into_iter().collect::<Vec<_>>()))
                    }
                    Proc::HoldsW => {
                        let s = oracle.wunlock(p);
                        state[p] = Proc::Idle;
                        let woken = rw.wunlock().unwrap();
                        // Readers are woken as a run; a lone waiter is a writer.
                        let readers = rw.readers() as usize;
                        for &w in &s.readied {
                            state[w] = if readers > 0 { Proc::HoldsR } else { Proc::HoldsW };
                        }
                        (s, step(false, &woken))
                    }
                };
                assert_eq!(expect, got, "outcome: {ctx}");

                let (locked, queue) = oracle.qstate();
                assert_eq!(q.is_locked(), locked, "locked: {ctx}");
                assert!(q.waiters().copied().eq(queue), "qlock queue: {ctx}");

                let (readers, writer, queue) = oracle.rwstate();
                assert_eq!((rw.readers(), rw.has_writer()), (readers, writer), "rw: {ctx}");
                let want = |w| match w {
                    Waiter::Reader => Want::Read,
                    Waiter::Writer => Want::Write,
                };
                assert!(
                    rw.waiters().copied().eq(queue.into_iter().map(|(p, w)| (p, want(w)))),
                    "rwlock queue: {ctx}"
                );
            }
        }
    }
}
