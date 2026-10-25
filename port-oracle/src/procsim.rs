//! Drives the c2rust translation of `proc.c`'s `sleep` and `wakeup` under the
//! same simulated one-CPU kernel as [`crate::qsim`].
//!
//! `sleep` ends in a context switch (`setlabel`/`gotolabel`) and, when a note
//! is pending, a non-local jump to an error handler. The simulation takes the
//! "going to sleep" side of `setlabel` and turns `gotolabel` into a recorded
//! event that returns, so a call to [`World::sleep`] reports what the process
//! did, but a sleeping process is never resumed. What happens on resume (check
//! for a pending note, maybe fail with `Eintr`) belongs to the scheduler, not
//! to the sleep/wakeup logic.
//!
//! `wakeup` runs the real `ready`, so a woken process lands in the
//! translation's own run queue.

use crate::c2rust::proc as p;
use crate::qsim::with_sim;
use core::ffi::{c_char, c_int, c_void, CStr};
use std::sync::{Mutex, MutexGuard, Once};

// Globals the translated code reaches directly. `run` is the translation's own
// scheduler state and `sys` points at a fake machine table, so worlds are
// serialised with `LOCK`.

#[no_mangle]
static mut sys: *mut p::Sys = core::ptr::null_mut();

#[no_mangle]
static mut Eintr: [c_char; 32] = [0; 32];

static LOCK: Mutex<()> = Mutex::new(());
static INIT: Once = Once::new();

fn init_globals() {
    INIT.call_once(|| {
        // SAFETY: all-zero is a valid Sys and Mach (integers, raw pointers,
        // options). Zero `load` makes `reprioritize` return the base priority.
        unsafe {
            let layout = std::alloc::Layout::new::<p::Sys>();
            let s = std::alloc::alloc_zeroed(layout) as *mut p::Sys;
            let m: *mut p::Mach = Box::into_raw(Box::new(core::mem::zeroed()));
            (*s).machptr[0] = m;
            (*s).nmach = 1;
            sys = s;
            core::ptr::copy_nonoverlapping(b"Eintr".as_ptr(), (&raw mut Eintr).cast::<u8>(), 5);
        }
    });
}

#[no_mangle]
unsafe extern "C" fn corecolor(_machno: c_int) -> c_int {
    0
}

#[no_mangle]
unsafe extern "C" fn edfready(_p: *mut p::Proc) -> c_int {
    0
}

#[no_mangle]
unsafe extern "C" fn slim_setlabel(_l: *mut p::Label) -> c_int {
    // The side that goes to sleep; resuming is not simulated.
    0
}

#[no_mangle]
unsafe extern "C" fn gotolabel(l: *mut p::Label) {
    with_sim(|s| {
        let sched = &raw mut (*(&mut *s.mach as *mut _ as *mut p::Mach)).sched;
        s.jumps.push(l == sched);
    });
}

#[no_mangle]
unsafe extern "C" fn procsave(_p: *mut p::Proc) {}

#[no_mangle]
unsafe extern "C" fn procrestore(_p: *mut p::Proc) {}

#[no_mangle]
unsafe extern "C" fn mmuflushtlb() {}

#[no_mangle]
unsafe extern "C" fn spllo() -> u64 {
    0
}

#[no_mangle]
unsafe extern "C" fn dumpstack() {}

#[no_mangle]
unsafe extern "C" fn forceclosefgrp() {}

#[no_mangle]
unsafe extern "C" fn kstrcpy(_dst: *mut c_char, _src: *mut c_char, _n: c_int) {}

#[no_mangle]
unsafe extern "C" fn _assert(msg: *mut c_char) {
    panic!("assertion failed: {}", CStr::from_ptr(msg).to_string_lossy());
}

/// The state of a process as `sleep` and `wakeup` see it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    Running,
    Wakeme,
    Ready,
}

/// What a call to `sleep` did.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Slept {
    /// The process committed to sleep and switched to the scheduler.
    pub switched: bool,
    /// A note was pending, so the process failed with `Eintr`.
    pub interrupted: bool,
    /// `sleep` reported that the rendezvous already had a sleeper.
    pub double_sleep: bool,
}

/// Processes and rendezvous points driven through the translation. Both are
/// numbered from 0.
pub struct World {
    procs: Vec<Box<p::Proc>>,
    rends: Vec<Box<p::Rendez>>,
    _serial: MutexGuard<'static, ()>,
}

unsafe extern "C" fn cond(arg: *mut c_void) -> c_int {
    *(arg as *const c_int)
}

impl World {
    pub fn new(nprocs: usize, nrends: usize) -> Self {
        let serial = LOCK.lock().unwrap_or_else(|e| e.into_inner());
        init_globals();
        // SAFETY: all-zero is a valid Sched; no other world is live.
        unsafe {
            core::ptr::write_bytes(&raw mut p::run, 0, 1);
            p::procinit0();
        }
        with_sim(|s| s.mach.sch = core::ptr::null_mut());
        let procs = (0..nprocs)
            .map(|i| {
                // SAFETY: all-zero is a valid Proc.
                let mut q: Box<p::Proc> = Box::new(unsafe { core::mem::zeroed() });
                q.pid = i as c_int + 1;
                q.state = p::Running as c_int;
                q.nerrlab = 1;
                q
            })
            .collect();
        let rends = (0..nrends)
            // SAFETY: all-zero is an idle Rendez.
            .map(|_| Box::new(unsafe { core::mem::zeroed() }))
            .collect();
        Self { procs, rends, _serial: serial }
    }

    fn idx(q: *const p::Proc) -> usize {
        // SAFETY: only called on pointers into `self.procs`.
        unsafe { (*q).pid as usize - 1 }
    }

    /// Run `f` with process `who` current, returning what the simulator saw.
    fn run<R>(
        &mut self,
        who: usize,
        f: impl FnOnce(&mut Self) -> R,
    ) -> (R, Vec<bool>, Vec<String>) {
        let up: *mut p::Proc = &mut *self.procs[who];
        with_sim(|s| {
            s.mach.externup = up.cast();
            s.jumps.clear();
            s.prints.clear();
        });
        let r = f(self);
        with_sim(|s| (r, s.jumps.clone(), s.prints.clone()))
    }

    /// Set whether a note is pending for process `who`.
    pub fn set_note_pending(&mut self, who: usize, pending: bool) {
        self.procs[who].notepending = pending as c_int;
    }

    pub fn note_pending(&self, who: usize) -> bool {
        self.procs[who].notepending != 0
    }

    /// Let a woken process run again.
    pub fn resume(&mut self, who: usize) {
        self.procs[who].state = p::Running as c_int;
    }

    /// `sleep(r, f, arg)` by `who`, where the condition `f` reports `satisfied`.
    pub fn sleep(&mut self, who: usize, r: usize, satisfied: bool) -> Slept {
        let mut flag: c_int = satisfied as c_int;
        let rend: *mut p::Rendez = &mut *self.rends[r];
        // SAFETY: `rend` is a valid Rendez, `cond` reads `flag`, which outlives the call.
        let (_, jumps, prints) = self.run(who, |_| unsafe {
            p::sleep(rend, Some(cond), (&mut flag as *mut c_int).cast());
        });
        Slept {
            switched: jumps.contains(&true),
            interrupted: jumps.contains(&false),
            double_sleep: prints.iter().any(|s| s.starts_with("double sleep")),
        }
    }

    /// `wakeup(r)`, called by process `who`; returns the process it woke.
    pub fn wakeup(&mut self, who: usize, r: usize) -> Option<usize> {
        let rend: *mut p::Rendez = &mut *self.rends[r];
        // SAFETY: `rend` is a valid Rendez; any sleeper is in `self.procs`.
        let (woken, _, _) = self.run(who, |_| unsafe { p::wakeup(rend) });
        (!woken.is_null()).then(|| Self::idx(woken))
    }

    /// The process sleeping on `r`.
    pub fn sleeper(&self, r: usize) -> Option<usize> {
        let q = self.rends[r]._p;
        (!q.is_null()).then(|| Self::idx(q))
    }

    /// The rendezvous point `who` is sleeping on.
    pub fn sleeping_on(&self, who: usize) -> Option<usize> {
        let r = self.procs[who].r;
        self.rends.iter().position(|b| core::ptr::eq(&**b, r))
    }

    pub fn state(&self, who: usize) -> State {
        match self.procs[who].state {
            s if s == p::Wakeme as c_int => State::Wakeme,
            s if s == p::Ready as c_int => State::Ready,
            s if s == p::Running as c_int => State::Running,
            s => panic!("unexpected state {s}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sleep_blocks_until_woken() {
        let mut w = World::new(2, 1);
        let s = w.sleep(0, 0, false);
        assert_eq!(s, Slept { switched: true, interrupted: false, double_sleep: false });
        assert_eq!((w.sleeper(0), w.sleeping_on(0), w.state(0)), (Some(0), Some(0), State::Wakeme));

        assert_eq!(w.wakeup(1, 0), Some(0));
        assert_eq!((w.sleeper(0), w.sleeping_on(0), w.state(0)), (None, None, State::Ready));
        assert_eq!(w.wakeup(1, 0), None);
    }

    #[test]
    fn satisfied_condition_and_pending_note_do_not_sleep() {
        let mut w = World::new(2, 1);
        assert_eq!(
            w.sleep(0, 0, true),
            Slept { switched: false, interrupted: false, double_sleep: false }
        );
        assert_eq!((w.sleeper(0), w.state(0)), (None, State::Running));

        w.set_note_pending(1, true);
        let s = w.sleep(1, 0, false);
        assert_eq!(s, Slept { switched: false, interrupted: true, double_sleep: false });
        assert!(!w.note_pending(1));
        assert_eq!((w.sleeper(0), w.state(1)), (None, State::Running));
    }
}
