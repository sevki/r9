//! A one-CPU simulated kernel, just enough to drive the c2rust translation of
//! `qlock.c`.
//!
//! The translated code calls out to the scheduler (`sched`, `ready`), the spin
//! lock (`lock`, `canlock`, `unlock`) and a few diagnostics. Here those are
//! recorders: `sched` notes that the current process went to sleep and `ready`
//! notes that one was woken. State is per test thread, so tests may run in
//! parallel.

use crate::c2rust::qlock as q;
use core::ffi::{c_char, c_int, CStr};
use std::cell::RefCell;

struct Sim {
    mach: Box<q::Mach>,
    scheds: u32,
    readied: Vec<c_int>,
    prints: Vec<String>,
}

thread_local! {
    static SIM: RefCell<Sim> = RefCell::new(Sim {
        // SAFETY: all-zero is a valid Mach (integers, raw pointers, options).
        mach: Box::new(unsafe { core::mem::zeroed() }),
        scheds: 0,
        readied: Vec::new(),
        prints: Vec::new(),
    });
}

#[no_mangle]
unsafe extern "C" fn machp() -> *mut q::Mach {
    SIM.with(|s| &mut *s.borrow_mut().mach as *mut q::Mach)
}

#[no_mangle]
unsafe extern "C" fn sched() {
    SIM.with(|s| s.borrow_mut().scheds += 1);
}

#[no_mangle]
unsafe extern "C" fn ready(p: *mut q::Proc) {
    SIM.with(|s| s.borrow_mut().readied.push((*p).pid));
}

#[no_mangle]
unsafe extern "C" fn canlock(l: *mut q::Lock) -> c_int {
    if (*l).key == 0 {
        (*l).key = 1;
        1
    } else {
        0
    }
}

#[no_mangle]
unsafe extern "C" fn lock(l: *mut q::Lock) -> c_int {
    // One CPU, and nothing holds `use` across a call: it can never be taken.
    assert_eq!((*l).key, 0, "spin lock already held");
    (*l).key = 1;
    0
}

#[no_mangle]
unsafe extern "C" fn unlock(l: *mut q::Lock) {
    (*l).key = 0;
}

#[no_mangle]
unsafe extern "C" fn print(fmt: *mut c_char, _args: ...) -> c_int {
    let s = CStr::from_ptr(fmt).to_string_lossy().into_owned();
    SIM.with(|sim| sim.borrow_mut().prints.push(s));
    0
}

#[no_mangle]
unsafe extern "C" fn panic(fmt: *mut c_char, _args: ...) {
    panic!("kernel panic: {}", CStr::from_ptr(fmt).to_string_lossy());
}

#[no_mangle]
unsafe extern "C" fn cycles(t: *mut u64) {
    *t = 0;
}

#[no_mangle]
unsafe extern "C" fn addwaitstat(_pc: usize, _t0: u64, _type: c_int) {}

#[no_mangle]
unsafe extern "C" fn islo() -> c_int {
    1
}

#[no_mangle]
unsafe extern "C" fn splhi() -> u64 {
    0
}

#[no_mangle]
unsafe extern "C" fn splx(_pl: u64) {}

#[no_mangle]
unsafe extern "C" fn stacksnippet() {}

/// What one call did, as seen from outside.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Step {
    /// The caller was queued and went to sleep in `sched`.
    pub blocked: bool,
    /// Processes passed to `ready`, in order.
    pub readied: Vec<usize>,
    /// Format strings passed to `print`.
    pub prints: Vec<String>,
}

/// Kind of a queued waiter on an `RWlock`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Waiter {
    Reader,
    Writer,
}

/// A set of processes sharing one `QLock` and one `RWlock`, driven through
/// the c2rust translation. Processes are numbered from 0.
pub struct World {
    procs: Vec<Box<q::Proc>>,
    ql: Box<q::QLock>,
    rw: Box<q::RWlock>,
}

impl World {
    pub fn new(nprocs: usize) -> Self {
        let procs = (0..nprocs)
            .map(|i| {
                // SAFETY: all-zero is a valid Proc (integers, raw pointers, options).
                let mut p: Box<q::Proc> = Box::new(unsafe { core::mem::zeroed() });
                p.pid = i as c_int + 1;
                p
            })
            .collect();
        // SAFETY: all-zero is the initial state of both locks.
        unsafe {
            Self {
                procs,
                ql: Box::new(core::mem::zeroed()),
                rw: Box::new(core::mem::zeroed()),
            }
        }
    }

    fn idx(p: *const q::Proc) -> usize {
        // SAFETY: only called on pointers into `self.procs`.
        unsafe { (*p).pid as usize - 1 }
    }

    /// Run `f` with process `p` as the current process.
    fn run(&mut self, p: usize, f: impl FnOnce(&mut Self) -> c_int) -> (c_int, Step) {
        let up: *mut q::Proc = &mut *self.procs[p];
        SIM.with(|s| {
            let mut s = s.borrow_mut();
            s.mach.externup = up;
            s.scheds = 0;
            s.readied.clear();
            s.prints.clear();
        });
        let ret = f(self);
        SIM.with(|s| {
            let s = s.borrow();
            let step = Step {
                blocked: s.scheds > 0,
                readied: s.readied.iter().map(|&pid| pid as usize - 1).collect(),
                prints: s.prints.clone(),
            };
            (ret, step)
        })
    }

    pub fn qlock(&mut self, p: usize) -> Step {
        // SAFETY: `ql` is a valid QLock and `p` the current process.
        self.run(p, |w| unsafe { q::qlock(&mut *w.ql) as c_int }).1
    }

    pub fn canqlock(&mut self, p: usize) -> (bool, Step) {
        // SAFETY: as above.
        let (r, s) = self.run(p, |w| unsafe { q::canqlock(&mut *w.ql) });
        (r != 0, s)
    }

    pub fn qunlock(&mut self, p: usize) -> Step {
        // SAFETY: as above.
        self.run(p, |w| unsafe { q::qunlock(&mut *w.ql) as c_int }).1
    }

    pub fn rlock(&mut self, p: usize) -> Step {
        // SAFETY: as above, for `rw`.
        self.run(p, |w| unsafe { q::rlock(&mut *w.rw) as c_int }).1
    }

    pub fn canrlock(&mut self, p: usize) -> (bool, Step) {
        // SAFETY: as above.
        let (r, s) = self.run(p, |w| unsafe { q::canrlock(&mut *w.rw) });
        (r != 0, s)
    }

    pub fn runlock(&mut self, p: usize) -> Step {
        // SAFETY: as above.
        self.run(p, |w| unsafe { q::runlock(&mut *w.rw) as c_int }).1
    }

    pub fn wlock(&mut self, p: usize) -> Step {
        // SAFETY: as above.
        self.run(p, |w| unsafe { q::wlock(&mut *w.rw) as c_int }).1
    }

    pub fn wunlock(&mut self, p: usize) -> Step {
        // SAFETY: as above.
        self.run(p, |w| unsafe { q::wunlock(&mut *w.rw) as c_int }).1
    }

    /// `(locked, waiting processes in queue order)` of the `QLock`.
    pub fn qstate(&self) -> (bool, Vec<usize>) {
        let mut queue = Vec::new();
        let mut p = self.ql.head;
        while !p.is_null() {
            queue.push(Self::idx(p));
            // SAFETY: queue links point into `self.procs`.
            p = unsafe { (*p).qnext };
        }
        (self.ql.locked != 0, queue)
    }

    /// `(readers, writer held, waiters in queue order)` of the `RWlock`.
    pub fn rwstate(&self) -> (u32, bool, Vec<(usize, Waiter)>) {
        let mut queue = Vec::new();
        let mut p = self.rw.head;
        while !p.is_null() {
            // SAFETY: queue links point into `self.procs`.
            let (state, next) = unsafe { ((*p).state, (*p).qnext) };
            let kind = if state == q::QueueingW as c_int { Waiter::Writer } else { Waiter::Reader };
            queue.push((Self::idx(p), kind));
            p = next;
        }
        (self.rw.readers as u32, self.rw.writer != 0, queue)
    }
}
