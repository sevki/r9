//! Rendezvous points, from the `sleep` and `wakeup` of Harvey's `port/proc.c`.
//!
//! A [`Rendez`] is where a process waits for a condition that another process
//! will make true: the waiter calls [`Rendez::sleep`] with the condition, and
//! whoever changes the condition calls [`Rendez::wakeup`].
//!
//! As with [`crate::qlock`], this is the state machine only. It is generic
//! over a waiter handle `W`, takes `&mut self` (callers serialise access, as
//! the C did with the rendezvous and process spin locks), and tells the caller
//! what to do: whether the process must switch to the scheduler, and which
//! process to make ready. Saving and restoring the process, the context
//! switch itself, and what to do when the process is resumed (a note may have
//! arrived while it slept, which fails the call with `Eintr`) all belong to
//! the scheduler.
//!
//! A timed sleep (`tsleep`) is a `sleep` whose condition also holds once the
//! timer has fired; the timer's expiry (`twakeup`) is a `wakeup`.

/// What [`Rendez::sleep`] decided.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sleep {
    /// The condition already holds: carry on without sleeping.
    Satisfied,
    /// A note is pending: do not sleep, fail the call with `Eintr`.
    Interrupted,
    /// The process is now sleeping here. Switch to the scheduler; a
    /// [`Rendez::wakeup`] will return it to be made ready.
    Blocked,
}

/// The result of [`Rendez::sleep`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Slept<W> {
    pub outcome: Sleep,
    /// A process was already sleeping here ("double sleep", a kernel bug that
    /// the C reported by printing). It has been forgotten: it will never be
    /// returned by `wakeup`.
    pub displaced: Option<W>,
}

/// A rendezvous point with at most one sleeper.
#[derive(Debug)]
pub struct Rendez<W> {
    sleeper: Option<W>,
}

impl<W> Rendez<W> {
    pub const fn new() -> Self {
        Self { sleeper: None }
    }

    /// The process sleeping here, if any.
    pub fn sleeper(&self) -> Option<&W> {
        self.sleeper.as_ref()
    }

    /// Sleep `me` here unless `satisfied` holds or a note is pending.
    ///
    /// `satisfied` is always evaluated, once, after `me` has been registered
    /// as the sleeper. A pending note takes precedence over a satisfied
    /// condition.
    pub fn sleep(
        &mut self,
        me: W,
        satisfied: impl FnOnce() -> bool,
        note_pending: bool,
    ) -> Slept<W> {
        let displaced = self.sleeper.replace(me);
        let outcome = if satisfied() || note_pending {
            self.sleeper = None;
            if note_pending {
                Sleep::Interrupted
            } else {
                Sleep::Satisfied
            }
        } else {
            Sleep::Blocked
        };
        Slept { outcome, displaced }
    }

    /// Wake the sleeper, if there is one, and return it to be made ready.
    pub fn wakeup(&mut self) -> Option<W> {
        self.sleeper.take()
    }
}

impl<W> Default for Rendez<W> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use port_oracle::procsim::{State, World};

    #[test]
    fn sleeps_until_woken() {
        let mut r = Rendez::new();
        let s = r.sleep(7, || false, false);
        assert_eq!(s, Slept { outcome: Sleep::Blocked, displaced: None });
        assert_eq!(r.sleeper(), Some(&7));
        assert_eq!(r.wakeup(), Some(7));
        assert_eq!(r.wakeup(), None);
    }

    #[test]
    fn does_not_sleep_when_satisfied_or_interrupted() {
        let mut r = Rendez::new();
        assert_eq!(r.sleep(1, || true, false).outcome, Sleep::Satisfied);
        assert_eq!(r.sleeper(), None);
        assert_eq!(r.sleep(1, || false, true).outcome, Sleep::Interrupted);
        assert_eq!(r.sleep(1, || true, true).outcome, Sleep::Interrupted);
        assert_eq!(r.sleeper(), None);
    }

    #[test]
    fn condition_sees_the_sleeper_registered() {
        let mut r = Rendez::new();
        let mut called = 0;
        r.sleep(
            1,
            || {
                called += 1;
                false
            },
            true,
        );
        assert_eq!(called, 1, "evaluated even when a note is pending");
    }

    #[test]
    fn a_second_sleeper_displaces_the_first() {
        let mut r = Rendez::new();
        r.sleep(1, || false, false);
        let s = r.sleep(2, || false, false);
        assert_eq!(s, Slept { outcome: Sleep::Blocked, displaced: Some(1) });
        assert_eq!(r.wakeup(), Some(2));

        // Even a second sleeper that doesn't block clears the rendezvous.
        r.sleep(1, || false, false);
        let s = r.sleep(2, || true, false);
        assert_eq!(s, Slept { outcome: Sleep::Satisfied, displaced: Some(1) });
        assert_eq!(r.wakeup(), None);
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

    /// Run random operation sequences through the c2rust translation of
    /// `sleep` and `wakeup` (with the real `ready`) and through this port in
    /// lock step, and require the same outcome and the same state after
    /// every operation.
    #[test]
    fn matches_c2rust_on_random_schedules() {
        const PROCS: usize = 5;
        const RENDS: usize = 3;
        for seed in 1..=300u64 {
            let mut rng = Rng(seed.wrapping_mul(0x9e37_79b9_7f4a_7c15) | 1);
            let mut oracle = World::new(PROCS, RENDS);
            let mut rends: Vec<Rendez<usize>> = (0..RENDS).map(|_| Rendez::new()).collect();
            let mut state = [State::Running; PROCS];

            let pick = |rng: &mut Rng, state: &[State; PROCS], want: State| {
                let candidates: Vec<usize> = (0..PROCS).filter(|&p| state[p] == want).collect();
                (!candidates.is_empty()).then(|| candidates[rng.below(candidates.len())])
            };

            for n in 0..200 {
                let ctx = |what: &str, state: &[State; PROCS]| {
                    format!("seed {seed} op {n} {what} {state:?}")
                };
                match rng.below(4) {
                    0 | 1 => {
                        let Some(p) = pick(&mut rng, &state, State::Running) else { continue };
                        let r = rng.below(RENDS);
                        let satisfied = rng.below(3) == 0;
                        let note = rng.below(5) == 0;
                        oracle.set_note_pending(p, note);
                        let want = oracle.sleep(p, r, satisfied);
                        let got = rends[r].sleep(p, || satisfied, note);
                        let what =
                            ctx(&format!("sleep p{p} r{r} cond {satisfied} note {note}"), &state);
                        assert_eq!(want.switched, got.outcome == Sleep::Blocked, "{what}");
                        assert_eq!(want.interrupted, got.outcome == Sleep::Interrupted, "{what}");
                        assert_eq!(want.double_sleep, got.displaced.is_some(), "{what}");
                        assert!(!oracle.note_pending(p), "the note is consumed: {what}");
                        if want.switched {
                            state[p] = State::Wakeme;
                        }
                    }
                    2 => {
                        let Some(who) = pick(&mut rng, &state, State::Running) else { continue };
                        let r = rng.below(RENDS);
                        let want = oracle.wakeup(who, r);
                        let got = rends[r].wakeup();
                        assert_eq!(want, got, "{}", ctx(&format!("wakeup r{r}"), &state));
                        if let Some(w) = got {
                            state[w] = State::Ready;
                        }
                    }
                    _ => {
                        let Some(p) = pick(&mut rng, &state, State::Ready) else { continue };
                        oracle.resume(p);
                        state[p] = State::Running;
                    }
                }

                for (r, rend) in rends.iter().enumerate() {
                    assert_eq!(
                        oracle.sleeper(r),
                        rend.sleeper().copied(),
                        "{}",
                        ctx(&format!("r{r}"), &state)
                    );
                }
                for (p, &st) in state.iter().enumerate() {
                    assert_eq!(oracle.state(p), st, "{}", ctx(&format!("p{p}"), &state));
                    if st != State::Wakeme {
                        // A process that isn't asleep isn't sleeping anywhere.
                        assert_eq!(
                            oracle.sleeping_on(p),
                            None,
                            "{}",
                            ctx(&format!("p{p}"), &state)
                        );
                    }
                }
            }
        }
    }
}
