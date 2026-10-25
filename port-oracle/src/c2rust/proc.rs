#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(asm, extern_types, raw_ref_op)]
use ::core::arch::asm;
extern "C" {
    pub type Queue;
    pub type Mntrpc;
    pub type Mntcache;
    pub type DevConf;
    pub type Nixpctl;
    pub type Timers;
    pub type Ureg;
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: u32_0,
    ) -> *mut ::core::ffi::c_void;
    fn memmove(
        _: *mut ::core::ffi::c_void,
        _: *mut ::core::ffi::c_void,
        _: u32_0,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strcpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn free(_: *mut ::core::ffi::c_void);
    fn print(_: *mut ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn snprint(
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn sprint(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    static mut eve: *mut ::core::ffi::c_char;
    static mut pga: Pgalloc;
    static sys: *mut Sys;
    fn _assert(_: *mut ::core::ffi::c_char);
    fn ainc(_: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn canlock(_: *mut Lock) -> ::core::ffi::c_int;
    fn canqlock(_: *mut QLock) -> ::core::ffi::c_int;
    fn cclose(_: *mut Chan);
    fn closeegrp(_: *mut Egrp);
    fn closefgrp(_: *mut Fgrp);
    fn closepgrp(_: *mut Pgrp);
    fn closergrp(_: *mut Rgrp);
    fn dbgpc(_: *mut Proc) -> usize_0;
    fn delay(_: ::core::ffi::c_int);
    fn dumpstack();
    fn edfready(_: *mut Proc) -> ::core::ffi::c_int;
    fn edfrecord(_: *mut Proc);
    fn edfrun(_: *mut Proc, _: ::core::ffi::c_int);
    fn edfstop(_: *mut Proc);
    fn forceclosefgrp();
    fn gotolabel(_: *mut Label);
    fn iprint(_: *mut ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn kprocchild(
        _: *mut Proc,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        _: *mut ::core::ffi::c_void,
    );
    fn kstrcpy(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    );
    fn kstrdup(_: *mut *mut ::core::ffi::c_char, _: *mut ::core::ffi::c_char);
    fn lock(_: *mut Lock) -> ::core::ffi::c_int;
    fn mfreeseg(_: *mut Segment, _: usize_0, _: ::core::ffi::c_int);
    fn mmuflush();
    fn mmuprocinit(_: *mut Proc);
    fn mmurelease(_: *mut Proc);
    fn mmuswitch(_: *mut Proc);
    fn newpgrp() -> *mut Pgrp;
    fn panic(_: *mut ::core::ffi::c_char, ...);
    fn perfticks() -> u64_0;
    fn procrestore(_: *mut Proc);
    fn procsave(_: *mut Proc);
    fn psdecref(_: *mut Proc);
    fn psincref(_: ::core::ffi::c_int) -> *mut Proc;
    fn putseg(_: *mut Segment);
    fn qlock(_: *mut QLock);
    fn qunlock(_: *mut QLock);
    fn slim_setlabel(_: *mut Label) -> ::core::ffi::c_int;
    fn smalloc(_: u32_0) -> *mut ::core::ffi::c_void;
    fn timeradd(_: *mut Timer);
    fn timerdel(_: *mut Timer);
    fn unlock(_: *mut Lock);
    fn intrac(_: *mut Proc);
    fn runacore();
    fn corecolor(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn decref(_: *mut Ref) -> ::core::ffi::c_int;
    fn hardhalt();
    fn idlehands();
    fn incref(_: *mut Ref) -> ::core::ffi::c_int;
    fn mmuflushtlb();
    fn splhi() -> u64_0;
    fn spllo() -> u64_0;
    fn splx(_: u64_0);
    fn stopac();
    fn infected_with_std() -> ::core::ffi::c_int;
    fn disinfect_std();
    fn DONE();
    fn machp() -> *mut Mach;
    static mut Eintr: [::core::ffi::c_char; 0];
    static mut Einuse: [::core::ffi::c_char; 0];
    static mut Enochild: [::core::ffi::c_char; 0];
    fn edflock(_: *mut Proc) -> *mut Edf;
    fn edfunlock();
    fn psalloc() -> *mut Proc;
    fn pshash(_: *mut Proc);
    fn psrelease(_: *mut Proc);
    fn psunhash(_: *mut Proc);
}
pub type u8_0 = ::core::ffi::c_uchar;
pub type u16_0 = ::core::ffi::c_ushort;
pub type u32_0 = ::core::ffi::c_uint;
pub type i32_0 = ::core::ffi::c_int;
pub type u64_0 = ::core::ffi::c_ulonglong;
pub type i64_0 = ::core::ffi::c_longlong;
pub type usize_0 = u64_0;
pub type isize_0 = i64_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Qid {
    pub path: u64_0,
    pub vers: u32_0,
    pub type_0: ::core::ffi::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Waitmsg {
    pub pid: ::core::ffi::c_int,
    pub time: [u32_0; 3],
    pub msg: [::core::ffi::c_char; 128],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Zio {
    pub data: *mut ::core::ffi::c_void,
    pub size: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fxsave {
    pub fcw: u16_0,
    pub fsw: u16_0,
    pub ftw: u8_0,
    pub zero: u8_0,
    pub fop: u16_0,
    pub rip: u64_0,
    pub rdp: u64_0,
    pub mxcsr: u32_0,
    pub mxcsrmask: u32_0,
    pub st: [::core::ffi::c_uchar; 128],
    pub xmm: [::core::ffi::c_uchar; 256],
    pub ign: [::core::ffi::c_uchar; 96],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ICC {
    pub fn_0: Option<unsafe extern "C" fn() -> ()>,
    pub flushtlb: ::core::ffi::c_int,
    pub rc: ::core::ffi::c_int,
    pub note: *mut ::core::ffi::c_char,
    pub data: [::core::ffi::c_uchar; 128],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Label {
    pub sp: usize_0,
    pub pc: usize_0,
    pub fp: usize_0,
    pub _pad: [usize_0; 13],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Lock {
    pub key: u32_0,
    pub isilock: ::core::ffi::c_int,
    pub pl: Mpl,
    pub _pc: usize_0,
    pub p: *mut Proc,
    pub m: *mut Mach,
    pub lockcycles: u64_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Mach {
    pub self_0: usize_0,
    pub splpc: u64_0,
    pub proc_0: *mut Proc,
    pub stack: usize_0,
    pub rathole: usize_0,
    pub externup: *mut Proc,
    pub machno: ::core::ffi::c_int,
    pub apicno: ::core::ffi::c_int,
    pub online: ::core::ffi::c_int,
    pub MMU: MMMU,
    pub vsvm: *mut ::core::ffi::c_uchar,
    pub gdt: *mut ::core::ffi::c_void,
    pub tss: *mut ::core::ffi::c_void,
    pub ticks: u64_0,
    pub sched: Label,
    pub alarmlock: Lock,
    pub alarm: *mut ::core::ffi::c_void,
    pub inclockintr: ::core::ffi::c_int,
    pub readied: *mut Proc,
    pub schedticks: u64_0,
    pub qstart: u64_0,
    pub qexpired: ::core::ffi::c_int,
    pub tlbfault: ::core::ffi::c_int,
    pub tlbpurge: ::core::ffi::c_int,
    pub pfault: ::core::ffi::c_int,
    pub cs: ::core::ffi::c_int,
    pub syscall: ::core::ffi::c_int,
    pub intr: ::core::ffi::c_int,
    pub mmuflush: ::core::ffi::c_int,
    pub ilockdepth: ::core::ffi::c_int,
    pub perf: Perf,
    pub inidle: ::core::ffi::c_int,
    pub lastintr: ::core::ffi::c_int,
    pub apictimerlock: Lock,
    pub cyclefreq: u64_0,
    pub cpuhz: i64_0,
    pub cpumhz: ::core::ffi::c_int,
    pub rdtsc: u64_0,
    pub pmclock: Lock,
    pub pmc: [PmcCtr; 4],
    pub FPU: MFPU,
    pub CPU: MCPU,
    pub NIX: NIX,
    pub sch: *mut Sched,
    pub load: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Sched {
    pub l: Lock,
    pub nrdy: ::core::ffi::c_int,
    pub delayedscheds: u32_0,
    pub skipscheds: i32_0,
    pub preempts: i32_0,
    pub schedgain: ::core::ffi::c_int,
    pub balancetime: u32_0,
    pub runq: [Schedq; 22],
    pub runvec: u32_0,
    pub nmach: ::core::ffi::c_int,
    pub nrun: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Schedq {
    pub head: *mut Proc,
    pub tail: *mut Proc,
    pub n: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Proc {
    pub sched: Label,
    pub kstack: *mut ::core::ffi::c_char,
    pub dbgreg: *mut ::core::ffi::c_void,
    pub tls: usize_0,
    pub mach: *mut Mach,
    pub text: *mut ::core::ffi::c_char,
    pub user: *mut ::core::ffi::c_char,
    pub args: *mut ::core::ffi::c_char,
    pub nargs: ::core::ffi::c_int,
    pub rnext: *mut Proc,
    pub qnext: *mut Proc,
    pub qlock: *mut QLock,
    pub state: ::core::ffi::c_int,
    pub psstate: *mut ::core::ffi::c_char,
    pub seg: [*mut Segment; 12],
    pub seglock: QLock,
    pub pid: ::core::ffi::c_int,
    pub index: ::core::ffi::c_int,
    pub ref_0: ::core::ffi::c_int,
    pub noteid: ::core::ffi::c_int,
    pub pidhash: *mut Proc,
    pub exl: Lock,
    pub waitq: *mut Waitq,
    pub nchild: ::core::ffi::c_int,
    pub nwait: ::core::ffi::c_int,
    pub qwaitr: QLock,
    pub waitr: Rendez,
    pub parent: *mut Proc,
    pub pgrp: *mut Pgrp,
    pub egrp: *mut Egrp,
    pub fgrp: *mut Fgrp,
    pub rgrp: *mut Rgrp,
    pub closingfgrp: *mut Fgrp,
    pub parentpid: ::core::ffi::c_int,
    pub time: [u64_0; 6],
    pub kentry: u64_0,
    pub pcycles: i64_0,
    pub insyscall: ::core::ffi::c_int,
    pub debug: QLock,
    pub pdbg: *mut Proc,
    pub procmode: u32_0,
    pub privatemem: u32_0,
    pub hang: ::core::ffi::c_int,
    pub procctl: ::core::ffi::c_int,
    pub pc: usize_0,
    pub rlock: Lock,
    pub r: *mut Rendez,
    pub sleep: Rendez,
    pub notepending: ::core::ffi::c_int,
    pub kp: ::core::ffi::c_int,
    pub palarm: *mut Proc,
    pub alarm: u32_0,
    pub newtlb: ::core::ffi::c_int,
    pub noswap: ::core::ffi::c_int,
    pub rendtag: usize_0,
    pub rendval: usize_0,
    pub rendhash: *mut Proc,
    pub Timer: Timer,
    pub trend: *mut Rendez,
    pub tfn: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
    >,
    pub kpfun: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub kparg: *mut ::core::ffi::c_void,
    pub scallnr: ::core::ffi::c_int,
    pub arg: [::core::ffi::c_uchar; 40],
    pub nerrlab: ::core::ffi::c_int,
    pub errlab: [Label; 64],
    pub syserrstr: *mut ::core::ffi::c_char,
    pub errstr: *mut ::core::ffi::c_char,
    pub errbuf0: [::core::ffi::c_char; 128],
    pub errbuf1: [::core::ffi::c_char; 128],
    pub genbuf: [::core::ffi::c_char; 128],
    pub slash: *mut Chan,
    pub dot: *mut Chan,
    pub note: [Note; 5],
    pub nnote: ::core::ffi::c_short,
    pub notified: ::core::ffi::c_short,
    pub lastnote: Note,
    pub notify: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_char) -> (),
    >,
    pub lockwait: *mut Lock,
    pub lastlock: *mut Lock,
    pub lastilock: *mut Lock,
    pub wired: *mut Mach,
    pub mp: *mut Mach,
    pub nlocks: ::core::ffi::c_int,
    pub delaysched: u32_0,
    pub priority: u32_0,
    pub basepri: u32_0,
    pub fixedpri: ::core::ffi::c_int,
    pub cpu: u64_0,
    pub lastupdate: u64_0,
    pub readytime: u64_0,
    pub movetime: u64_0,
    pub preempted: ::core::ffi::c_int,
    pub edf: *mut Edf,
    pub trace: ::core::ffi::c_int,
    pub qpc: usize_0,
    pub setargs: ::core::ffi::c_int,
    pub ureg: *mut ::core::ffi::c_void,
    pub color: ::core::ffi::c_int,
    pub fc: *mut Fastcall,
    pub fcount: ::core::ffi::c_int,
    pub syscalltrace: *mut ::core::ffi::c_char,
    pub ac: *mut Mach,
    pub acPageTableRoot: *mut Page,
    pub prepagemem: ::core::ffi::c_int,
    pub nixpctl: *mut Nixpctl,
    pub ntrap: u32_0,
    pub nintr: u32_0,
    pub nsyscall: u32_0,
    pub nactrap: u32_0,
    pub nacsyscall: u32_0,
    pub nicc: u32_0,
    pub actime1: u64_0,
    pub actime: u64_0,
    pub tctime: u64_0,
    pub nqtrap: ::core::ffi::c_int,
    pub nqsyscall: ::core::ffi::c_int,
    pub nfullq: ::core::ffi::c_int,
    pub FPU: PFPU,
    pub MMU: PMMU,
    pub NOTIFY: PNOTIFY,
    pub req: *mut Queue,
    pub resp: *mut Queue,
    pub strace: *mut Strace,
    pub strace_on: ::core::ffi::c_int,
    pub strace_inherit: ::core::ffi::c_int,
    pub plan9: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Strace {
    pub opened: ::core::ffi::c_int,
    pub tracing: ::core::ffi::c_int,
    pub inherit: ::core::ffi::c_int,
    pub procs: Ref,
    pub users: Ref,
    pub q: *mut Queue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Ref {
    pub l: Lock,
    pub ref_0: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PNOTIFY {
    pub emptiness: ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PMMU {
    pub root: *mut Page,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Page {
    pub l: Lock,
    pub pa: u64_0,
    pub va: usize_0,
    pub daddr: u32_0,
    pub ref_0: ::core::ffi::c_int,
    pub modref: ::core::ffi::c_uchar,
    pub color: ::core::ffi::c_int,
    pub cachectl: [::core::ffi::c_char; 32],
    pub image: *mut Image,
    pub next: *mut Page,
    pub prev: *mut Page,
    pub hash: *mut Page,
    pub pgszi: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Image {
    pub r: Ref,
    pub c: *mut Chan,
    pub qid: Qid,
    pub mqid: Qid,
    pub mchan: *mut Chan,
    pub dc: ::core::ffi::c_int,
    pub s: *mut Segment,
    pub hash: *mut Image,
    pub next: *mut Image,
    pub prev: *mut Image,
    pub notext: ::core::ffi::c_int,
    pub color: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Segment {
    pub r: Ref,
    pub lk: QLock,
    pub steal: u16_0,
    pub type_0: u16_0,
    pub pgszi: ::core::ffi::c_int,
    pub ptepertab: u32_0,
    pub color: ::core::ffi::c_int,
    pub base: usize_0,
    pub top: usize_0,
    pub size: usize_0,
    pub ldseg: Ldseg,
    pub flushme: ::core::ffi::c_int,
    pub image: *mut Image,
    pub pseg: *mut Physseg,
    pub profile: *mut u32_0,
    pub map: *mut *mut Pte,
    pub mapsize: ::core::ffi::c_int,
    pub ssegmap: [*mut Pte; 16],
    pub semalock: Lock,
    pub sema: Sema,
    pub zseg: Zseg,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Zseg {
    pub map: *mut ::core::ffi::c_void,
    pub addr: *mut usize_0,
    pub naddr: ::core::ffi::c_int,
    pub end: ::core::ffi::c_int,
    pub rr: Rendez,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Rendez {
    pub l: Lock,
    pub _p: *mut Proc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Sema {
    pub rend: Rendez,
    pub addr: *mut ::core::ffi::c_int,
    pub waiting: ::core::ffi::c_int,
    pub next: *mut Sema,
    pub prev: *mut Sema,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Pte {
    pub first: *mut *mut Page,
    pub last: *mut *mut Page,
    pub pages: [*mut Page; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Physseg {
    pub attr: u32_0,
    pub name: *mut ::core::ffi::c_char,
    pub pa: u64_0,
    pub size: usize_0,
    pub pgszi: ::core::ffi::c_int,
    pub pgalloc: Option<unsafe extern "C" fn(*mut Segment, usize_0) -> *mut Page>,
    pub pgfree: Option<unsafe extern "C" fn(*mut Page) -> ()>,
    pub gva: usize_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Ldseg {
    pub memsz: i64_0,
    pub filesz: i64_0,
    pub pg0fileoff: i64_0,
    pub pg0vaddr: usize_0,
    pub pg0off: u32_0,
    pub pgsz: u32_0,
    pub type_0: u16_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct QLock {
    pub use_0: Lock,
    pub head: *mut Proc,
    pub tail: *mut Proc,
    pub locked: ::core::ffi::c_int,
    pub pc: usize_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Chan {
    pub r: Ref,
    pub next: *mut Chan,
    pub link: *mut Chan,
    pub offset: i64_0,
    pub devoffset: i64_0,
    pub dev: *mut Dev,
    pub devno: u32_0,
    pub mode: u16_0,
    pub flag: u16_0,
    pub qid: Qid,
    pub fid: ::core::ffi::c_int,
    pub iounit: u32_0,
    pub umh: *mut Mhead,
    pub umc: *mut Chan,
    pub umqlock: QLock,
    pub uri: ::core::ffi::c_int,
    pub dri: ::core::ffi::c_int,
    pub dirrock: *mut ::core::ffi::c_uchar,
    pub nrock: ::core::ffi::c_int,
    pub mrock: ::core::ffi::c_int,
    pub rockqlock: QLock,
    pub ismtpt: ::core::ffi::c_int,
    pub mc: *mut Mntcache,
    pub mux: *mut Mnt,
    pub c2rust_unnamed: C2RustUnnamed,
    pub mchan: *mut Chan,
    pub mqid: Qid,
    pub path: *mut Path,
    pub writebuff: *mut ::core::ffi::c_uchar,
    pub buffend: ::core::ffi::c_int,
    pub writeoffset: ::core::ffi::c_int,
    pub buffsize: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Path {
    pub r: Ref,
    pub s: *mut ::core::ffi::c_char,
    pub mtpt: *mut *mut Chan,
    pub len: ::core::ffi::c_int,
    pub alen: ::core::ffi::c_int,
    pub mlen: ::core::ffi::c_int,
    pub malen: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub aux: *mut ::core::ffi::c_void,
    pub pgrpid: Qid,
    pub mid: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Mnt {
    pub l: Lock,
    pub c: *mut Chan,
    pub rip: *mut Proc,
    pub queue: *mut Mntrpc,
    pub id: u32_0,
    pub list: *mut Mnt,
    pub flags: ::core::ffi::c_int,
    pub msize: ::core::ffi::c_int,
    pub version: *mut ::core::ffi::c_char,
    pub q: *mut Queue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Mhead {
    pub r: Ref,
    pub lock: RWlock,
    pub from: *mut Chan,
    pub mount: *mut Mount,
    pub hash: *mut Mhead,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Mount {
    pub mountid: ::core::ffi::c_int,
    pub next: *mut Mount,
    pub head: *mut Mhead,
    pub copy: *mut Mount,
    pub order: *mut Mount,
    pub to: *mut Chan,
    pub mflag: ::core::ffi::c_int,
    pub spec: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RWlock {
    pub use_0: Lock,
    pub head: *mut Proc,
    pub tail: *mut Proc,
    pub wpc: usize_0,
    pub wproc: *mut Proc,
    pub readers: ::core::ffi::c_int,
    pub writer: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dev {
    pub dc: ::core::ffi::c_int,
    pub name: *mut ::core::ffi::c_char,
    pub reset: Option<unsafe extern "C" fn() -> ()>,
    pub init: Option<unsafe extern "C" fn() -> ()>,
    pub shutdown: Option<unsafe extern "C" fn() -> ()>,
    pub attach: Option<unsafe extern "C" fn(*mut ::core::ffi::c_char) -> *mut Chan>,
    pub walk: Option<
        unsafe extern "C" fn(
            *mut Chan,
            *mut Chan,
            *mut *mut ::core::ffi::c_char,
            ::core::ffi::c_int,
        ) -> *mut Walkqid,
    >,
    pub stat: Option<
        unsafe extern "C" fn(*mut Chan, *mut ::core::ffi::c_uchar, i32_0) -> i32_0,
    >,
    pub open: Option<unsafe extern "C" fn(*mut Chan, ::core::ffi::c_int) -> *mut Chan>,
    pub create: Option<
        unsafe extern "C" fn(
            *mut Chan,
            *mut ::core::ffi::c_char,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub close: Option<unsafe extern "C" fn(*mut Chan) -> ()>,
    pub read: Option<
        unsafe extern "C" fn(*mut Chan, *mut ::core::ffi::c_void, i32_0, i64_0) -> i32_0,
    >,
    pub bread: Option<unsafe extern "C" fn(*mut Chan, i32_0, i64_0) -> *mut Block>,
    pub write: Option<
        unsafe extern "C" fn(*mut Chan, *mut ::core::ffi::c_void, i32_0, i64_0) -> i32_0,
    >,
    pub bwrite: Option<unsafe extern "C" fn(*mut Chan, *mut Block, i64_0) -> i32_0>,
    pub remove: Option<unsafe extern "C" fn(*mut Chan) -> ()>,
    pub wstat: Option<
        unsafe extern "C" fn(*mut Chan, *mut ::core::ffi::c_uchar, i32_0) -> i32_0,
    >,
    pub power: Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>,
    pub config: Option<
        unsafe extern "C" fn(
            ::core::ffi::c_int,
            *mut ::core::ffi::c_char,
            *mut DevConf,
        ) -> ::core::ffi::c_int,
    >,
    pub zread: Option<
        unsafe extern "C" fn(
            *mut Chan,
            *mut Kzio,
            ::core::ffi::c_int,
            usize_0,
            i64_0,
        ) -> ::core::ffi::c_int,
    >,
    pub zwrite: Option<
        unsafe extern "C" fn(
            *mut Chan,
            *mut Kzio,
            ::core::ffi::c_int,
            i64_0,
        ) -> ::core::ffi::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Kzio {
    pub Zio: Zio,
    pub seg: *mut Segment,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Block {
    pub ref_0: i32_0,
    pub next: *mut Block,
    pub list: *mut Block,
    pub rp: *mut ::core::ffi::c_uchar,
    pub wp: *mut ::core::ffi::c_uchar,
    pub lim: *mut ::core::ffi::c_uchar,
    pub base: *mut ::core::ffi::c_uchar,
    pub free: Option<unsafe extern "C" fn(*mut Block) -> ()>,
    pub flag: u16_0,
    pub checksum: u16_0,
    pub magic: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Walkqid {
    pub clone: *mut Chan,
    pub nqid: ::core::ffi::c_int,
    pub qid: [Qid; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PFPU {
    pub fxsave: Fxsave,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fastcall {
    pub scnum: ::core::ffi::c_int,
    pub c: *mut Chan,
    pub fun: Option<unsafe extern "C" fn(*mut Ar0, *mut Fastcall) -> ()>,
    pub buf: *mut ::core::ffi::c_void,
    pub n: ::core::ffi::c_int,
    pub offset: i64_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union Ar0 {
    pub i: isize_0,
    pub l: i32_0,
    pub p: usize_0,
    pub u: usize_0,
    pub v: *mut ::core::ffi::c_void,
    pub vl: i64_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Edf {
    pub D: ::core::ffi::c_long,
    pub Delta: ::core::ffi::c_long,
    pub T: ::core::ffi::c_long,
    pub C: ::core::ffi::c_long,
    pub S: ::core::ffi::c_long,
    pub r: ::core::ffi::c_long,
    pub d: ::core::ffi::c_long,
    pub t: ::core::ffi::c_long,
    pub s: ::core::ffi::c_long,
    pub testDelta: ::core::ffi::c_long,
    pub testtype: ::core::ffi::c_int,
    pub testtime: ::core::ffi::c_long,
    pub testnext: *mut Proc,
    pub flags: u16_0,
    pub Timer: Timer,
    pub edfused: ::core::ffi::c_long,
    pub extraused: ::core::ffi::c_long,
    pub aged: ::core::ffi::c_long,
    pub periods: u32_0,
    pub missed: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Timer {
    pub tmode: ::core::ffi::c_int,
    pub tns: i64_0,
    pub tf: Option<unsafe extern "C" fn(*mut Ureg, *mut Timer) -> ()>,
    pub ta: *mut ::core::ffi::c_void,
    pub l: Lock,
    pub tt: *mut Timers,
    pub twhen: i64_0,
    pub tnext: *mut Timer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Note {
    pub msg: [::core::ffi::c_char; 128],
    pub flag: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fgrp {
    pub r: Ref,
    pub fd: *mut *mut Chan,
    pub nfd: ::core::ffi::c_int,
    pub maxfd: ::core::ffi::c_int,
    pub exceed: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Rgrp {
    pub r: Ref,
    pub rendhash: [*mut Proc; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Egrp {
    pub r: Ref,
    pub rwl: RWlock,
    pub ent: *mut *mut Evalue,
    pub nent: ::core::ffi::c_int,
    pub ment: ::core::ffi::c_int,
    pub path: u32_0,
    pub vers: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Evalue {
    pub name: *mut ::core::ffi::c_char,
    pub value: *mut ::core::ffi::c_char,
    pub len: ::core::ffi::c_int,
    pub link: *mut Evalue,
    pub qid: Qid,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Pgrp {
    pub r: Ref,
    pub noattach: ::core::ffi::c_int,
    pub pgrpid: u32_0,
    pub debug: QLock,
    pub ns: RWlock,
    pub mnthash: [*mut Mhead; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Waitq {
    pub w: Waitmsg,
    pub next: *mut Waitq,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NIX {
    pub icc: *mut ICC,
    pub nixtype: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MCPU {
    pub cpuinfo: [[u32_0; 4]; 3],
    pub ncpuinfos: ::core::ffi::c_int,
    pub ncpuinfoe: ::core::ffi::c_int,
    pub isintelcpu: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MFPU {
    pub fcw: u16_0,
    pub mxcsr: u32_0,
    pub mxcsrmask: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PmcCtr {
    pub stale: ::core::ffi::c_int,
    pub wq: *mut PmcWait,
    pub ctr: u64_0,
    pub ctrset: ::core::ffi::c_int,
    pub PmcCtl: PmcCtl,
    pub ctlset: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PmcCtl {
    pub r: Ref,
    pub _coreno: u32_0,
    pub enab: ::core::ffi::c_int,
    pub user: ::core::ffi::c_int,
    pub os: ::core::ffi::c_int,
    pub nodesc: ::core::ffi::c_int,
    pub descstr: [::core::ffi::c_char; 28],
    pub reset: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PmcWait {
    pub r: Ref,
    pub rend: Rendez,
    pub next: *mut PmcWait,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Perf {
    pub intrts: u64_0,
    pub inintr: u64_0,
    pub avg_inintr: u64_0,
    pub inidle: u64_0,
    pub avg_inidle: u64_0,
    pub last: u64_0,
    pub period: u64_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MMMU {
    pub cr2: usize_0,
    pub pml4: *mut Page,
    pub pml4kludge: Page,
}
pub type Mpl = u64_0;
pub type PTE = u64_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Sys {
    pub machstk: [::core::ffi::c_uchar; 32768],
    pub ipml4: [PTE; 512],
    pub epml4: [PTE; 512],
    pub epml3: [PTE; 512],
    pub epml2: [[PTE; 4]; 512],
    pub pml4: [PTE; 512],
    pub pml3: [PTE; 98304],
    pub vsvmpage: [::core::ffi::c_uchar; 4096],
    pub mach: Mach,
    pub machptr: [*mut Mach; 32],
    pub pmstart: u64_0,
    pub pmend: u64_0,
    pub epoch: u64_0,
    pub nc: [::core::ffi::c_int; 4],
    pub nmach: ::core::ffi::c_int,
    pub load: ::core::ffi::c_int,
    pub ticks: u64_0,
    pub cyclefreq: u64_0,
    pub pgszlg2: [u32_0; 4],
    pub pgszmask: [u32_0; 4],
    pub pgsz: [u32_0; 4],
    pub npgsz: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Pgalloc {
    pub l: Lock,
    pub userinit: ::core::ffi::c_int,
    pub pgsza: [Pgsza; 4],
    pub hash: [*mut Page; 512],
    pub hashlock: Lock,
    pub rend: Rendez,
    pub pwait: QLock,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Pgsza {
    pub freecount: u32_0,
    pub npages: Ref,
    pub head: *mut Page,
    pub tail: *mut Page,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Procalloc {
    pub l: Lock,
    pub nproc: ::core::ffi::c_int,
    pub ht: [*mut Proc; 128],
    pub arena: *mut Proc,
    pub free: *mut Proc,
}
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const NDebug: C2RustUnnamed_0 = 2;
pub const NExit: C2RustUnnamed_0 = 1;
pub const NUser: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const PGHSIZE: C2RustUnnamed_1 = 512;
pub const PGHLOG: C2RustUnnamed_1 = 9;
pub const NFD: C2RustUnnamed_1 = 100;
pub const MNTHASH: C2RustUnnamed_1 = 32;
pub const MNTLOG: C2RustUnnamed_1 = 5;
pub const RENDHASH: C2RustUnnamed_1 = 32;
pub const RENDLOG: C2RustUnnamed_1 = 5;
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const Tperiodic: C2RustUnnamed_2 = 1;
pub const Trelative: C2RustUnnamed_2 = 0;
pub type C2RustUnnamed_3 = ::core::ffi::c_uint;
pub const NSEG: C2RustUnnamed_3 = 12;
pub type C2RustUnnamed_4 = ::core::ffi::c_uint;
pub const PriRoot: C2RustUnnamed_4 = 13;
pub const PriKproc: C2RustUnnamed_4 = 13;
pub const PriExtra: C2RustUnnamed_4 = 19;
pub const PriNormal: C2RustUnnamed_4 = 10;
pub const PriEdf: C2RustUnnamed_4 = 21;
pub const PriRelease: C2RustUnnamed_4 = 20;
pub const Nrq: C2RustUnnamed_4 = 22;
pub const Npriq: C2RustUnnamed_4 = 20;
pub const NNOTE: C2RustUnnamed_4 = 5;
pub const NERR: C2RustUnnamed_4 = 64;
pub const TCReal: C2RustUnnamed_4 = 5;
pub const TCSys: C2RustUnnamed_4 = 4;
pub const TCUser: C2RustUnnamed_4 = 3;
pub const TReal: C2RustUnnamed_4 = 2;
pub const TSys: C2RustUnnamed_4 = 1;
pub const TUser: C2RustUnnamed_4 = 0;
pub const Proc_totc: C2RustUnnamed_4 = 7;
pub const Proc_toac: C2RustUnnamed_4 = 6;
pub const Proc_tracesyscall: C2RustUnnamed_4 = 5;
pub const Proc_exitbig: C2RustUnnamed_4 = 4;
pub const Proc_traceme: C2RustUnnamed_4 = 3;
pub const Proc_exitme: C2RustUnnamed_4 = 2;
pub const Proc_stopme: C2RustUnnamed_4 = 1;
pub const Semdown: C2RustUnnamed_4 = 14;
pub const Exotic: C2RustUnnamed_4 = 13;
pub const Waitrelease: C2RustUnnamed_4 = 12;
pub const Rendezvous: C2RustUnnamed_4 = 11;
pub const Stopped: C2RustUnnamed_4 = 10;
pub const Broken: C2RustUnnamed_4 = 9;
pub const Wakeme: C2RustUnnamed_4 = 8;
pub const QueueingW: C2RustUnnamed_4 = 7;
pub const QueueingR: C2RustUnnamed_4 = 6;
pub const Queueing: C2RustUnnamed_4 = 5;
pub const Running: C2RustUnnamed_4 = 4;
pub const Scheding: C2RustUnnamed_4 = 3;
pub const Ready: C2RustUnnamed_4 = 2;
pub const Moribund: C2RustUnnamed_4 = 1;
pub const Dead: C2RustUnnamed_4 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub l: Lock,
    pub nonline: ::core::ffi::c_int,
    pub nbooting: ::core::ffi::c_int,
    pub exiting: ::core::ffi::c_int,
    pub ispanic: ::core::ffi::c_int,
    pub thunderbirdsarego: ::core::ffi::c_int,
}
pub const Nsched: C2RustUnnamed_8 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub QLock: QLock,
    pub n: ::core::ffi::c_int,
    pub p: [*mut Proc; 4],
}
pub const SReady: Tevent = 4;
pub const Scaling: C2RustUnnamed_8 = 2;
pub const SRun: Tevent = 3;
pub const SDead: Tevent = 10;
pub const SSleep: Tevent = 5;
pub const Admitted: C2RustUnnamed_7 = 1;
pub type C2RustUnnamed_7 = ::core::ffi::c_ulong;
pub const Infinity: C2RustUnnamed_7 = 18446744073709551615;
pub const Extratime: C2RustUnnamed_7 = 64;
pub const Yield: C2RustUnnamed_7 = 32;
pub const Deadline: C2RustUnnamed_7 = 16;
pub const Sendnotes: C2RustUnnamed_7 = 8;
pub const Yieldonblock: C2RustUnnamed_7 = 4;
pub const Sporadic: C2RustUnnamed_7 = 2;
pub const Maxsteps: C2RustUnnamed_7 = 40000;
pub type Tevent = ::core::ffi::c_uint;
pub const Nevent: Tevent = 15;
pub const SLock: Tevent = 14;
pub const SUser: Tevent = 13;
pub const SInte: Tevent = 12;
pub const SInts: Tevent = 11;
pub const SExpel: Tevent = 9;
pub const SDeadline: Tevent = 8;
pub const SSlice: Tevent = 7;
pub const SYield: Tevent = 6;
pub const SEdf: Tevent = 2;
pub const SRelease: Tevent = 1;
pub const SAdmit: Tevent = 0;
pub type C2RustUnnamed_8 = ::core::ffi::c_uint;
pub const nil: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const ERRMAX: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const KiB: ::core::ffi::c_ulonglong = 1024 as ::core::ffi::c_ulonglong;
pub const KSTACK: ::core::ffi::c_int = 16 as ::core::ffi::c_int
    * 1024 as ::core::ffi::c_int;
pub const BIGPGSHFT: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const BIGPGSZ: ::core::ffi::c_ulonglong = (1 as ::core::ffi::c_ulonglong)
    << BIGPGSHFT;
pub const HZ: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
#[no_mangle]
pub static mut active: C2RustUnnamed_5 = C2RustUnnamed_5 {
    l: Lock {
        key: 0,
        isilock: 0,
        pl: 0,
        _pc: 0,
        p: ::core::ptr::null::<Proc>() as *mut Proc,
        m: ::core::ptr::null::<Mach>() as *mut Mach,
        lockcycles: 0,
    },
    nonline: 0,
    nbooting: 0,
    exiting: 0,
    ispanic: 0,
    thunderbirdsarego: 0,
};
#[no_mangle]
pub static mut consdebug: Option<unsafe extern "C" fn() -> ()> = None;
#[no_mangle]
pub static mut consputs: Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_char, ::core::ffi::c_int) -> (),
> = None;
#[no_mangle]
pub static mut kproftimer: Option<unsafe extern "C" fn(usize_0) -> ()> = None;
#[no_mangle]
pub static mut mfcinit: Option<unsafe extern "C" fn() -> ()> = None;
#[no_mangle]
pub static mut mfcopen: Option<unsafe extern "C" fn(*mut Chan) -> ()> = None;
#[no_mangle]
pub static mut mfcread: Option<
    unsafe extern "C" fn(
        *mut Chan,
        *mut ::core::ffi::c_uchar,
        ::core::ffi::c_int,
        i64_0,
    ) -> ::core::ffi::c_int,
> = None;
#[no_mangle]
pub static mut mfcupdate: Option<
    unsafe extern "C" fn(
        *mut Chan,
        *mut ::core::ffi::c_uchar,
        ::core::ffi::c_int,
        i64_0,
    ) -> (),
> = None;
#[no_mangle]
pub static mut mfcwrite: Option<
    unsafe extern "C" fn(
        *mut Chan,
        *mut ::core::ffi::c_uchar,
        ::core::ffi::c_int,
        i64_0,
    ) -> (),
> = None;
#[no_mangle]
pub static mut mwait: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()> = None;
#[no_mangle]
pub static mut proctrace: Option<
    unsafe extern "C" fn(*mut Proc, ::core::ffi::c_int, i64_0) -> (),
> = None;
#[no_mangle]
pub static mut coherence: Option<unsafe extern "C" fn() -> ()> = None;
#[inline]
unsafe extern "C" fn externup() -> *mut Proc {
    let mut pl: Mpl = splhi() as Mpl;
    let mut proc_0: *mut Proc = (*machp()).externup;
    splx(pl as u64_0);
    return proc_0;
}
#[inline]
unsafe extern "C" fn __clobber_callee_regs() {
    asm!(
        "\n", out("r12") _, out("r13") _, out("r14") _, out("r15") _,
        options(preserves_flags, att_syntax)
    );
}
#[no_mangle]
pub static mut noteidalloc: Ref = Ref {
    l: Lock {
        key: 0,
        isilock: 0,
        pl: 0,
        _pc: 0,
        p: ::core::ptr::null::<Proc>() as *mut Proc,
        m: ::core::ptr::null::<Mach>() as *mut Mach,
        lockcycles: 0,
    },
    ref_0: 0,
};
static mut pidalloc: Ref = Ref {
    l: Lock {
        key: 0,
        isilock: 0,
        pl: 0,
        _pc: 0,
        p: ::core::ptr::null::<Proc>() as *mut Proc,
        m: ::core::ptr::null::<Mach>() as *mut Mach,
        lockcycles: 0,
    },
    ref_0: 0,
};
#[no_mangle]
pub static mut run: [Sched; 1] = [Sched {
    l: Lock {
        key: 0,
        isilock: 0,
        pl: 0,
        _pc: 0,
        p: ::core::ptr::null::<Proc>() as *mut Proc,
        m: ::core::ptr::null::<Mach>() as *mut Mach,
        lockcycles: 0,
    },
    nrdy: 0,
    delayedscheds: 0,
    skipscheds: 0,
    preempts: 0,
    schedgain: 0,
    balancetime: 0,
    runq: [Schedq {
        head: ::core::ptr::null::<Proc>() as *mut Proc,
        tail: ::core::ptr::null::<Proc>() as *mut Proc,
        n: 0,
    }; 22],
    runvec: 0,
    nmach: 0,
    nrun: 0,
}; 1];
#[no_mangle]
pub static mut procalloc: Procalloc = Procalloc {
    l: Lock {
        key: 0,
        isilock: 0,
        pl: 0,
        _pc: 0,
        p: ::core::ptr::null::<Proc>() as *mut Proc,
        m: ::core::ptr::null::<Mach>() as *mut Mach,
        lockcycles: 0,
    },
    nproc: 0,
    ht: [::core::ptr::null::<Proc>() as *mut Proc; 128],
    arena: ::core::ptr::null::<Proc>() as *mut Proc,
    free: ::core::ptr::null::<Proc>() as *mut Proc,
};
#[no_mangle]
pub static mut schedsteals: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[no_mangle]
pub static mut scheddonates: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut statename: [*mut ::core::ffi::c_char; 15] = [
    b"Dead\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Moribund\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Ready\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Scheding\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Running\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Queueing\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"QueueingR\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"QueueingW\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Wakeme\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Broken\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Stopped\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Rendez\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Waitrelease\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Exotic\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Down\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn setmachsched(mut mp: *mut Mach) {
    let mut color: ::core::ffi::c_int = 0;
    color = corecolor((*mp).machno);
    if color < 0 as ::core::ffi::c_int {
        print(
            b"unknown color for cpu%d\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            (*mp).machno,
        );
        color = 0 as ::core::ffi::c_int;
    }
    (*mp).sch = (&raw mut run as *mut Sched)
        .offset((color % Nsched as ::core::ffi::c_int) as isize) as *mut Sched;
}
#[no_mangle]
pub unsafe extern "C" fn procsched(mut p: *mut Proc) -> *mut Sched {
    let mut pm: *mut Mach = ::core::ptr::null_mut::<Mach>();
    pm = (*p).mp;
    if pm.is_null() {
        pm = machp();
    }
    if (*pm).sch.is_null() {
        setmachsched(pm);
    }
    return (*pm).sch;
}
#[no_mangle]
pub unsafe extern "C" fn procinit0() {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < Nsched as ::core::ffi::c_int {
        run[i as usize].schedgain = 30 as ::core::ffi::c_int;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn schedinit() {
    let mut up: *mut Proc = ::core::ptr::null_mut::<Proc>();
    let mut e: *mut Edf = ::core::ptr::null_mut::<Edf>();
    (*machp()).inidle = 1 as ::core::ffi::c_int;
    if (*machp()).sch.is_null() {
        print(
            b"schedinit: no sch for cpu%d\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            (*machp()).machno,
        );
        setmachsched(machp());
    }
    ainc(&raw mut (*(*(machp as unsafe extern "C" fn() -> *mut Mach)()).sch).nmach);
    ({
        let mut err: ::core::ffi::c_int = 0;
        __clobber_callee_regs();
        err = slim_setlabel(
            &raw mut (*(machp as unsafe extern "C" fn() -> *mut Mach)()).sched,
        );
    });
    up = (*machp()).externup;
    if infected_with_std() != 0 {
        print(
            b"mach %d got an std from %s (pid %d)!\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            (*machp()).machno,
            if !up.is_null() {
                (*up).text as *const ::core::ffi::c_char
            } else {
                b"*notext\0" as *const u8 as *const ::core::ffi::c_char
            },
            if !up.is_null() { (*up).pid } else { -(1 as ::core::ffi::c_int) },
        );
        disinfect_std();
    }
    if !up.is_null() {
        e = (*up).edf as *mut Edf;
        if !e.is_null()
            && (*e).flags as ::core::ffi::c_int & Admitted as ::core::ffi::c_int != 0
        {
            edfrecord(up);
        }
        coherence.expect("non-null function pointer")();
        let ref mut fresh13 = (*machp()).proc_0;
        *fresh13 = ::core::ptr::null_mut::<Proc>();
        match (*up).state {
            4 => {
                ready(up);
            }
            1 => {
                (*up).state = Dead as ::core::ffi::c_int;
                stopac();
                edfstop(up);
                if !(*up).edf.is_null() {
                    free((*up).edf as *mut ::core::ffi::c_void);
                }
                (*up).edf = ::core::ptr::null_mut::<Edf>();
                mmurelease(up);
                unlock(&raw mut pga.l);
                psrelease(up);
                unlock(&raw mut procalloc.l);
            }
            _ => {}
        }
        (*up).mach = ::core::ptr::null_mut::<Mach>();
        updatecpu(up);
        let ref mut fresh14 = (*machp()).externup;
        *fresh14 = ::core::ptr::null_mut::<Proc>();
    }
    sched();
}
unsafe extern "C" fn stackok() {
    let mut up: *mut Proc = externup();
    let mut dummy: ::core::ffi::c_char = 0;
    if &raw mut dummy
        < (*up).kstack.offset((4 as ::core::ffi::c_ulonglong).wrapping_mul(KiB) as isize)
    {
        print(
            b"tc kernel stack overflow, cpu%d stopped\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            (*machp()).machno,
        );
        DONE();
    }
}
#[no_mangle]
pub unsafe extern "C" fn sched() {
    let mut p: *mut Proc = ::core::ptr::null_mut::<Proc>();
    let mut sch: *mut Sched = ::core::ptr::null_mut::<Sched>();
    let mut up: *mut Proc = externup();
    sch = (*machp()).sch;
    if (*machp()).ilockdepth != 0 {
        panic(
            b"cpu%d: ilockdepth %d, last lock %#p at %#p, sched called from %#p\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            (*machp()).machno,
            (*machp()).ilockdepth,
            if !up.is_null() {
                (*up).lastilock
            } else {
                ::core::ptr::null_mut::<Lock>()
            },
            if !up.is_null() && !(*up).lastilock.is_null() {
                0 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            },
            0 as ::core::ffi::c_int as usize_0,
        );
    }
    if !up.is_null() {
        if (*up).nlocks != 0 {
            if (*up).state != Moribund as ::core::ffi::c_int {
                if (*up).delaysched < 20 as u32_0 || pga.l.p == up || procalloc.l.p == up
                {
                    (*up).delaysched = (*up).delaysched.wrapping_add(1);
                    (*sch).delayedscheds = (*sch).delayedscheds.wrapping_add(1);
                    return;
                }
            }
        }
        (*up).delaysched = 0 as u32_0;
        splhi();
        if (*up).nqtrap == 0 as ::core::ffi::c_int
            && (*up).nqsyscall == 0 as ::core::ffi::c_int
        {
            (*up).nfullq += 1;
        }
        let ref mut fresh3 = (*machp()).cs;
        *fresh3 += 1;
        stackok();
        procsave(up);
        mmuflushtlb();
        if ({
            let mut err: ::core::ffi::c_int = 0;
            __clobber_callee_regs();
            err = slim_setlabel(&raw mut (*up).sched);
            err
        }) != 0
        {
            procrestore(up);
            spllo();
            return;
        }
        gotolabel(&raw mut (*(machp as unsafe extern "C" fn() -> *mut Mach)()).sched);
    }
    (*machp()).inidle = 1 as ::core::ffi::c_int;
    p = runproc();
    (*machp()).inidle = 0 as ::core::ffi::c_int;
    if (*p).edf.is_null() {
        updatecpu(p);
        (*p).priority = reprioritize(p) as u32_0;
    }
    if p != (*machp()).readied {
        (*machp()).schedticks = (*machp())
            .ticks
            .wrapping_add((HZ / 10 as ::core::ffi::c_int) as u64_0);
    }
    let ref mut fresh4 = (*machp()).readied;
    *fresh4 = ::core::ptr::null_mut::<Proc>();
    let ref mut fresh5 = (*machp()).externup;
    *fresh5 = p;
    if p.is_null() {
        _assert(
            b"p != nil\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
    }
    up = p;
    (*up).nqtrap = 0 as ::core::ffi::c_int;
    (*up).nqsyscall = 0 as ::core::ffi::c_int;
    (*up).state = Running as ::core::ffi::c_int;
    (*up).mach = (*sys).machptr[(*machp()).machno as usize];
    let ref mut fresh6 = (*machp()).proc_0;
    *fresh6 = up;
    mmuswitch(up);
    if !((*up).wired.is_null() || (*up).wired == machp()) {
        _assert(
            b"!up->wired || up->wired == machp()\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    gotolabel(&raw mut (*up).sched);
}
#[no_mangle]
pub unsafe extern "C" fn anyready() -> ::core::ffi::c_int {
    return (*(*machp()).sch).runvec as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn anyhigher() -> ::core::ffi::c_int {
    let mut up: *mut Proc = externup();
    return ((*(*machp()).sch).runvec
        & !(((1 as ::core::ffi::c_int) << (*up).priority.wrapping_add(1 as u32_0))
            - 1 as ::core::ffi::c_int) as u32_0) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn hzsched() {
    let mut up: *mut Proc = externup();
    if (*machp()).machno == 0 as ::core::ffi::c_int {
        rebalance();
    }
    if anyhigher() != 0
        || (*up).fixedpri == 0 && (*machp()).ticks > (*machp()).schedticks
            && anyready() != 0
    {
        let ref mut fresh1 = (*machp()).readied;
        *fresh1 = ::core::ptr::null_mut::<Proc>();
        (*up).delaysched = (*up).delaysched.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn preempted() -> ::core::ffi::c_int {
    let mut up: *mut Proc = externup();
    if !up.is_null() && (*up).state == Running as ::core::ffi::c_int {
        if (*up).preempted == 0 as ::core::ffi::c_int {
            if anyhigher() != 0 {
                if active.exiting == 0 {
                    let ref mut fresh10 = (*machp()).readied;
                    *fresh10 = ::core::ptr::null_mut::<Proc>();
                    (*up).preempted = 1 as ::core::ffi::c_int;
                    sched();
                    splhi();
                    (*up).preempted = 0 as ::core::ffi::c_int;
                    return 1 as ::core::ffi::c_int;
                }
            }
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn updatecpu(mut p: *mut Proc) {
    let mut up: *mut Proc = externup();
    let mut D: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut t: ::core::ffi::c_int = 0;
    let mut ocpu: ::core::ffi::c_int = 0;
    if !(*p).edf.is_null() {
        return;
    }
    t = (*(*sys).machptr[0 as ::core::ffi::c_int as usize])
        .ticks
        .wrapping_mul(Scaling as ::core::ffi::c_int as u64_0)
        .wrapping_add((Scaling as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as u64_0)
        as ::core::ffi::c_int;
    n = (t as u64_0).wrapping_sub((*p).lastupdate) as ::core::ffi::c_int;
    (*p).lastupdate = t as u64_0;
    if n == 0 as ::core::ffi::c_int {
        return;
    }
    if (*machp()).sch.is_null() {
        return;
    }
    D = (*(*machp()).sch).schedgain * HZ * Scaling as ::core::ffi::c_int;
    if n > D {
        n = D;
    }
    ocpu = (*p).cpu as ::core::ffi::c_int;
    if p != up {
        (*p).cpu = (ocpu * (D - n) / D) as u64_0;
    } else {
        t = 1000 as ::core::ffi::c_int - ocpu;
        t = t * (D - n) / D;
        (*p).cpu = (1000 as ::core::ffi::c_int - t) as u64_0;
    };
}
unsafe extern "C" fn reprioritize(mut p: *mut Proc) -> ::core::ffi::c_int {
    let mut fairshare: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut load: ::core::ffi::c_int = 0;
    let mut ratio: ::core::ffi::c_int = 0;
    load = (*(*sys).machptr[0 as ::core::ffi::c_int as usize]).load;
    if load == 0 as ::core::ffi::c_int {
        return (*p).basepri as ::core::ffi::c_int;
    }
    fairshare = (*sys).nmach * 1000 as ::core::ffi::c_int * 1000 as ::core::ffi::c_int
        / load;
    n = (*p).cpu as ::core::ffi::c_int;
    if n == 0 as ::core::ffi::c_int {
        n = 1 as ::core::ffi::c_int;
    }
    ratio = (fairshare + n / 2 as ::core::ffi::c_int) / n;
    if ratio as u32_0 > (*p).basepri {
        ratio = (*p).basepri as ::core::ffi::c_int;
    }
    if ratio < 0 as ::core::ffi::c_int {
        panic(
            b"reprioritize\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
    }
    return ratio;
}
unsafe extern "C" fn queueproc(
    mut sch: *mut Sched,
    mut rq: *mut Schedq,
    mut p: *mut Proc,
    mut locked: ::core::ffi::c_int,
) {
    let mut pri: ::core::ffi::c_int = 0;
    pri = rq.offset_from(&raw mut (*sch).runq as *mut Schedq) as ::core::ffi::c_long
        as ::core::ffi::c_int;
    if locked == 0 {
        lock(&raw mut (*sch).l);
    } else if canlock(&raw mut (*sch).l) != 0 {
        panic(
            b"queueproc: locked and can lock\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    (*p).priority = pri as u32_0;
    (*p).rnext = ::core::ptr::null_mut::<Proc>();
    if !(*rq).tail.is_null() {
        (*(*rq).tail).rnext = p;
    } else {
        (*rq).head = p;
    }
    (*rq).tail = p;
    (*rq).n += 1;
    (*sch).nrdy += 1;
    (*sch).runvec |= ((1 as ::core::ffi::c_int) << pri) as u32_0;
    if locked == 0 {
        unlock(&raw mut (*sch).l);
    }
}
#[no_mangle]
pub unsafe extern "C" fn dequeueproc(
    mut sch: *mut Sched,
    mut rq: *mut Schedq,
    mut tp: *mut Proc,
) -> *mut Proc {
    let mut l: *mut Proc = ::core::ptr::null_mut::<Proc>();
    let mut p: *mut Proc = ::core::ptr::null_mut::<Proc>();
    if canlock(&raw mut (*sch).l) == 0 {
        return ::core::ptr::null_mut::<Proc>();
    }
    l = ::core::ptr::null_mut::<Proc>();
    p = (*rq).head;
    while !p.is_null() {
        if p == tp {
            break;
        }
        l = p;
        p = (*p).rnext;
    }
    if p.is_null() || !(*p).mach.is_null() {
        unlock(&raw mut (*sch).l);
        return ::core::ptr::null_mut::<Proc>();
    }
    if (*p).rnext.is_null() {
        (*rq).tail = l;
    }
    if !l.is_null() {
        (*l).rnext = (*p).rnext;
    } else {
        (*rq).head = (*p).rnext;
    }
    if (*rq).head.is_null() {
        (*sch).runvec
            &= !((1 as ::core::ffi::c_int)
                << rq.offset_from(&raw mut (*sch).runq as *mut Schedq)
                    as ::core::ffi::c_long) as u32_0;
    }
    (*rq).n -= 1;
    (*sch).nrdy -= 1;
    if (*p).state != Ready as ::core::ffi::c_int {
        print(
            b"dequeueproc %s %d %s\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            (*p).text,
            (*p).pid,
            statename[(*p).state as usize],
        );
    }
    unlock(&raw mut (*sch).l);
    return p;
}
unsafe extern "C" fn schedready(
    mut sch: *mut Sched,
    mut p: *mut Proc,
    mut locked: ::core::ffi::c_int,
) {
    let mut pl: Mpl = 0;
    let mut pri: ::core::ffi::c_int = 0;
    let mut rq: *mut Schedq = ::core::ptr::null_mut::<Schedq>();
    let mut up: *mut Proc = ::core::ptr::null_mut::<Proc>();
    pl = splhi() as Mpl;
    if edfready(p) != 0 {
        splx(pl as u64_0);
        return;
    }
    up = externup();
    if up != p {
        let ref mut fresh0 = (*machp()).readied;
        *fresh0 = p;
    }
    updatecpu(p);
    pri = reprioritize(p);
    (*p).priority = pri as u32_0;
    rq = (&raw mut (*sch).runq as *mut Schedq).offset(pri as isize) as *mut Schedq;
    (*p).state = Ready as ::core::ffi::c_int;
    queueproc(sch, rq, p, locked);
    if (*p).trace != 0 {
        proctrace
            .expect(
                "non-null function pointer",
            )(p, SReady as ::core::ffi::c_int, 0 as i64_0);
    }
    splx(pl as u64_0);
}
#[no_mangle]
pub unsafe extern "C" fn ready(mut p: *mut Proc) {
    schedready(procsched(p), p, 0 as ::core::ffi::c_int);
}
#[export_name = "yield"]
pub unsafe extern "C" fn yield_0() {
    let mut up: *mut Proc = externup();
    if anyready() != 0 {
        (*up).lastupdate = (*up)
            .lastupdate
            .wrapping_sub(
                (Scaling as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as u64_0,
            );
        sched();
    }
}
unsafe extern "C" fn rebalance() {
    let mut pl: Mpl = 0;
    let mut pri: ::core::ffi::c_int = 0;
    let mut npri: ::core::ffi::c_int = 0;
    let mut t: i64_0 = 0;
    let mut sch: *mut Sched = ::core::ptr::null_mut::<Sched>();
    let mut rq: *mut Schedq = ::core::ptr::null_mut::<Schedq>();
    let mut p: *mut Proc = ::core::ptr::null_mut::<Proc>();
    sch = (*machp()).sch;
    t = (*machp()).ticks as i64_0;
    if (t - (*sch).balancetime as i64_0) < HZ as i64_0 {
        return;
    }
    (*sch).balancetime = t as u32_0;
    pri = 0 as ::core::ffi::c_int;
    rq = &raw mut (*sch).runq as *mut Schedq;
    while pri < Npriq as ::core::ffi::c_int {
        loop {
            p = (*rq).head;
            if p.is_null() {
                break;
            }
            if (*p).mp != (*sys).machptr[(*machp()).machno as usize] {
                break;
            }
            if pri as u32_0 == (*p).basepri {
                break;
            }
            updatecpu(p);
            npri = reprioritize(p);
            if !(npri != pri) {
                break;
            }
            pl = splhi() as Mpl;
            p = dequeueproc(sch, rq, p);
            if !p.is_null() {
                queueproc(
                    sch,
                    (&raw mut (*sch).runq as *mut Schedq).offset(npri as isize)
                        as *mut Schedq,
                    p,
                    0 as ::core::ffi::c_int,
                );
            }
            splx(pl as u64_0);
        }
        pri += 1;
        rq = rq.offset(1);
    }
}
unsafe extern "C" fn overloaded(mut sch: *mut Sched) -> ::core::ffi::c_int {
    return ((*sch).nmach != 0 as ::core::ffi::c_int && (*sch).nrdy > (*sch).nmach)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn steal() -> *mut Proc {
    static mut last: ::core::ffi::c_int = 0;
    let mut start: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut rq: *mut Schedq = ::core::ptr::null_mut::<Schedq>();
    let mut sch: *mut Sched = ::core::ptr::null_mut::<Sched>();
    let mut p: *mut Proc = ::core::ptr::null_mut::<Proc>();
    if schedsteals == 0 {
        return ::core::ptr::null_mut::<Proc>();
    }
    start = last;
    i = 0 as ::core::ffi::c_int;
    while i < Nsched as ::core::ffi::c_int {
        last = (start + i) % Nsched as ::core::ffi::c_int;
        sch = (&raw mut run as *mut Sched).offset(last as isize) as *mut Sched;
        if !(sch == (*machp()).sch || (*sch).nmach == 0 as ::core::ffi::c_int
            || overloaded(sch) == 0)
        {
            rq = (&raw mut (*sch).runq as *mut Schedq)
                .offset((Nrq as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize)
                as *mut Schedq;
            while rq >= &raw mut (*sch).runq as *mut Schedq {
                p = (*rq).head;
                while !p.is_null() {
                    if (*p).wired.is_null()
                        && (*p).priority < PriKproc as ::core::ffi::c_int as u32_0
                    {
                        break;
                    }
                    p = (*p).rnext;
                }
                if !p.is_null() && !dequeueproc(sch, rq, p).is_null() {
                    return p;
                }
                rq = rq.offset(-1);
            }
        }
        i += 1;
    }
    return ::core::ptr::null_mut::<Proc>();
}
#[no_mangle]
pub unsafe extern "C" fn runproc() -> *mut Proc {
    let mut current_block: u64;
    let mut rq: *mut Schedq = ::core::ptr::null_mut::<Schedq>();
    let mut sch: *mut Sched = ::core::ptr::null_mut::<Sched>();
    let mut p: *mut Proc = ::core::ptr::null_mut::<Proc>();
    let mut start: u64_0 = 0;
    let mut now: u64_0 = 0;
    let mut i: ::core::ffi::c_int = 0;
    start = perfticks();
    sch = (*machp()).sch;
    p = (*machp()).readied;
    if !p.is_null() && (*p).mach.is_null() && (*p).state == Ready as ::core::ffi::c_int
        && (*sch)
            .runq[(Nrq as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize]
            .head
            .is_null()
        && (*sch)
            .runq[(Nrq as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            .head
            .is_null() && ((*p).wired.is_null() || (*p).wired == machp())
    {
        (*sch).skipscheds += 1;
        rq = (&raw mut (*sch).runq as *mut Schedq).offset((*p).priority as isize)
            as *mut Schedq;
        current_block = 9710385404099836388;
    } else {
        (*sch).preempts += 1;
        current_block = 8766663631415631242;
    }
    '_found: loop {
        match current_block {
            9710385404099836388 => {
                splhi();
                p = dequeueproc(sch, rq, p);
                if p.is_null() {
                    current_block = 8766663631415631242;
                } else {
                    break;
                }
            }
            _ => {
                spllo();
                i = 0 as ::core::ffi::c_int;
                's_47: loop {
                    rq = (&raw mut (*sch).runq as *mut Schedq)
                        .offset(
                            (Nrq as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                as isize,
                        ) as *mut Schedq;
                    while rq >= &raw mut (*sch).runq as *mut Schedq {
                        p = (*rq).head;
                        while !p.is_null() {
                            if (*p).mp.is_null()
                                || (*p).mp == (*sys).machptr[(*machp()).machno as usize]
                                || (*p).wired.is_null() && i > 0 as ::core::ffi::c_int
                            {
                                current_block = 9710385404099836388;
                                break 's_47;
                            }
                            p = (*p).rnext;
                        }
                        rq = rq.offset(-1);
                    }
                    p = steal();
                    if !p.is_null() {
                        splhi();
                        break '_found;
                    } else {
                        idlehands();
                        now = perfticks();
                        let ref mut fresh7 = (*machp()).perf.inidle;
                        *fresh7 = (*fresh7).wrapping_add(now.wrapping_sub(start));
                        start = now;
                        i += 1;
                    }
                }
            }
        }
    }
    (*p).state = Scheding as ::core::ffi::c_int;
    (*p).mp = (*sys).machptr[(*machp()).machno as usize];
    if !edflock(p).is_null() {
        edfrun(
            p,
            (rq
                == (&raw mut (*sch).runq as *mut Schedq)
                    .offset(PriEdf as ::core::ffi::c_int as isize) as *mut Schedq)
                as ::core::ffi::c_int,
        );
        edfunlock();
    }
    if (*p).trace != 0 {
        proctrace
            .expect(
                "non-null function pointer",
            )(p, SRun as ::core::ffi::c_int, 0 as i64_0);
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn canpage(mut p: *mut Proc) -> ::core::ffi::c_int {
    let mut ok: ::core::ffi::c_int = 0;
    let mut sch: *mut Sched = ::core::ptr::null_mut::<Sched>();
    splhi();
    sch = procsched(p);
    lock(&raw mut (*sch).l);
    if (*p).mach.is_null() {
        (*p).newtlb = 1 as ::core::ffi::c_int;
        ok = 1 as ::core::ffi::c_int;
    } else {
        ok = 0 as ::core::ffi::c_int;
    }
    unlock(&raw mut (*sch).l);
    spllo();
    return ok;
}
#[no_mangle]
pub unsafe extern "C" fn newproc() -> *mut Proc {
    let mut up: *mut Proc = externup();
    let mut p: *mut Proc = ::core::ptr::null_mut::<Proc>();
    p = psalloc();
    (*p).state = Scheding as ::core::ffi::c_int;
    (*p).psstate = b"New\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char;
    (*p).mach = ::core::ptr::null_mut::<Mach>();
    (*p).qnext = ::core::ptr::null_mut::<Proc>();
    (*p).nchild = 0 as ::core::ffi::c_int;
    (*p).nwait = 0 as ::core::ffi::c_int;
    (*p).waitq = ::core::ptr::null_mut::<Waitq>();
    (*p).parent = ::core::ptr::null_mut::<Proc>();
    (*p).pgrp = ::core::ptr::null_mut::<Pgrp>();
    (*p).egrp = ::core::ptr::null_mut::<Egrp>();
    (*p).fgrp = ::core::ptr::null_mut::<Fgrp>();
    (*p).rgrp = ::core::ptr::null_mut::<Rgrp>();
    (*p).pdbg = ::core::ptr::null_mut::<Proc>();
    (*p).kp = 0 as ::core::ffi::c_int;
    if !up.is_null() && (*up).procctl == Proc_tracesyscall as ::core::ffi::c_int {
        (*p).procctl = Proc_tracesyscall as ::core::ffi::c_int;
    } else {
        (*p).procctl = 0 as ::core::ffi::c_int;
    }
    (*p).syscalltrace = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*p).notepending = 0 as ::core::ffi::c_int;
    (*p).ureg = ::core::ptr::null_mut::<::core::ffi::c_void>();
    (*p).privatemem = 0 as u32_0;
    (*p).noswap = 0 as ::core::ffi::c_int;
    (*p).errstr = &raw mut (*p).errbuf0 as *mut ::core::ffi::c_char;
    (*p).syserrstr = &raw mut (*p).errbuf1 as *mut ::core::ffi::c_char;
    (*p).errbuf0[0 as ::core::ffi::c_int as usize] = '\0' as i32 as ::core::ffi::c_char;
    (*p).errbuf1[0 as ::core::ffi::c_int as usize] = '\0' as i32 as ::core::ffi::c_char;
    (*p).nlocks = 0 as ::core::ffi::c_int;
    (*p).delaysched = 0 as u32_0;
    (*p).trace = 0 as ::core::ffi::c_int;
    kstrdup(
        &raw mut (*p).user,
        b"*nouser\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    );
    kstrdup(
        &raw mut (*p).text,
        b"*notext\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    );
    kstrdup(
        &raw mut (*p).args,
        b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    (*p).nargs = 0 as ::core::ffi::c_int;
    (*p).setargs = 0 as ::core::ffi::c_int;
    memset(
        &raw mut (*p).seg as *mut *mut Segment as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[*mut Segment; 12]>() as u32_0,
    );
    (*p).pid = incref(&raw mut pidalloc);
    pshash(p);
    (*p).noteid = incref(&raw mut noteidalloc);
    if (*p).pid <= 0 as ::core::ffi::c_int || (*p).noteid <= 0 as ::core::ffi::c_int {
        panic(
            b"pidalloc\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
    }
    if (*p).kstack.is_null() {
        (*p).kstack = smalloc(KSTACK as u32_0) as *mut ::core::ffi::c_char;
    }
    (*p).mp = ::core::ptr::null_mut::<Mach>();
    (*p).wired = ::core::ptr::null_mut::<Mach>();
    procpriority(p, PriNormal as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
    (*p).cpu = 0 as u64_0;
    (*p).lastupdate = (*(*sys).machptr[0 as ::core::ffi::c_int as usize])
        .ticks
        .wrapping_mul(Scaling as ::core::ffi::c_int as u64_0);
    (*p).edf = ::core::ptr::null_mut::<Edf>();
    (*p).ntrap = 0 as u32_0;
    (*p).nintr = 0 as u32_0;
    (*p).nsyscall = 0 as u32_0;
    (*p).nactrap = 0 as u32_0;
    (*p).nacsyscall = 0 as u32_0;
    (*p).nicc = 0 as u32_0;
    (*p).actime = 0 as ::core::ffi::c_ulonglong as u64_0;
    (*p).tctime = 0 as ::core::ffi::c_ulonglong as u64_0;
    (*p).ac = ::core::ptr::null_mut::<Mach>();
    (*p).nfullq = 0 as ::core::ffi::c_int;
    memset(
        &raw mut (*p).MMU as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<PMMU>() as u32_0,
    );
    mmuprocinit(p);
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn procwired(mut p: *mut Proc, mut bm: ::core::ffi::c_int) {
    let mut up: *mut Proc = externup();
    let mut pp: *mut Proc = ::core::ptr::null_mut::<Proc>();
    let mut i: ::core::ffi::c_int = 0;
    let mut nwired: [::core::ffi::c_char; 32] = [0; 32];
    let mut wm: *mut Mach = ::core::ptr::null_mut::<Mach>();
    if bm < 0 as ::core::ffi::c_int {
        memset(
            &raw mut nwired as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<[::core::ffi::c_char; 32]>() as u32_0,
        );
        (*p).wired = ::core::ptr::null_mut::<Mach>();
        i = 0 as ::core::ffi::c_int;
        loop {
            pp = psincref(i);
            if pp.is_null() {
                break;
            }
            wm = (*pp).wired;
            if !wm.is_null() && (*pp).pid != 0 {
                nwired[(*machp()).machno as usize] += 1;
            }
            psdecref(pp);
            i += 1;
        }
        bm = 0 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < (*sys).nmach {
            if (nwired[i as usize] as ::core::ffi::c_int)
                < nwired[bm as usize] as ::core::ffi::c_int
            {
                bm = i;
            }
            i += 1;
        }
    } else {
        bm = bm % (*sys).nmach;
    }
    (*p).wired = (*sys).machptr[bm as usize];
    (*p).mp = (*p).wired;
    if up.is_null() || p != up {
        return;
    }
    (*up).color = corecolor((*(*up).mp).machno);
    qlock(&raw mut (*up).seglock);
    i = 0 as ::core::ffi::c_int;
    while i < NSEG as ::core::ffi::c_int {
        if !(*up).seg[i as usize].is_null() {
            (*(*up).seg[i as usize]).color = (*up).color;
        }
        i += 1;
    }
    qunlock(&raw mut (*up).seglock);
}
#[no_mangle]
pub unsafe extern "C" fn procpriority(
    mut p: *mut Proc,
    mut pri: ::core::ffi::c_int,
    mut fixed: ::core::ffi::c_int,
) {
    if pri >= Npriq as ::core::ffi::c_int {
        pri = Npriq as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    } else if pri < 0 as ::core::ffi::c_int {
        pri = 0 as ::core::ffi::c_int;
    }
    (*p).basepri = pri as u32_0;
    (*p).priority = pri as u32_0;
    if fixed != 0 {
        (*p).fixedpri = 1 as ::core::ffi::c_int;
    } else {
        (*p).fixedpri = 0 as ::core::ffi::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn sleep(
    mut r: *mut Rendez,
    mut f: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
    mut arg: *mut ::core::ffi::c_void,
) {
    let mut up: *mut Proc = externup();
    let mut pl: Mpl = 0;
    pl = splhi() as Mpl;
    if (*up).nlocks != 0 {
        print(
            b"process %d sleeps with %d locks held, last lock %#p locked at pc %#p, sleep called from %#p\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            (*up).pid,
            (*up).nlocks,
            (*up).lastlock,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int as usize_0,
        );
    }
    lock(&raw mut (*r).l);
    lock(&raw mut (*up).rlock);
    if !(*r)._p.is_null() {
        print(
            b"double sleep called from %#p, %d %d\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            0 as ::core::ffi::c_int as usize_0,
            (*(*r)._p).pid,
            (*up).pid,
        );
        dumpstack();
    }
    (*r)._p = up;
    if Some(f.expect("non-null function pointer"))
        .expect("non-null function pointer")(arg) != 0 || (*up).notepending != 0
    {
        (*r)._p = ::core::ptr::null_mut::<Proc>();
        unlock(&raw mut (*up).rlock);
        unlock(&raw mut (*r).l);
    } else {
        if (*up).trace != 0 {
            proctrace
                .expect(
                    "non-null function pointer",
                )(up, SSleep as ::core::ffi::c_int, 0 as i64_0);
        }
        (*up).state = Wakeme as ::core::ffi::c_int;
        (*up).r = r;
        let ref mut fresh12 = (*machp()).cs;
        *fresh12 += 1;
        procsave(up);
        mmuflushtlb();
        if ({
            let mut err: ::core::ffi::c_int = 0;
            __clobber_callee_regs();
            err = slim_setlabel(&raw mut (*up).sched);
            err
        }) != 0
        {
            procrestore(up);
            spllo();
        } else {
            unlock(&raw mut (*up).rlock);
            unlock(&raw mut (*r).l);
            gotolabel(
                &raw mut (*(machp as unsafe extern "C" fn() -> *mut Mach)()).sched,
            );
        }
    }
    if (*up).notepending != 0 {
        (*up).notepending = 0 as ::core::ffi::c_int;
        splx(pl as u64_0);
        if (*up).procctl == Proc_exitme as ::core::ffi::c_int
            && !(*up).closingfgrp.is_null()
        {
            forceclosefgrp();
        }
        error(&raw mut Eintr as *mut ::core::ffi::c_char);
    }
    splx(pl as u64_0);
}
unsafe extern "C" fn tfn(mut arg: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let mut up: *mut Proc = externup();
    return ((*up).trend.is_null()
        || (*up).tfn.expect("non-null function pointer")(arg) != 0)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn twakeup(mut _: *mut Ureg, mut t: *mut Timer) {
    let mut p: *mut Proc = ::core::ptr::null_mut::<Proc>();
    let mut trend: *mut Rendez = ::core::ptr::null_mut::<Rendez>();
    p = (*t).ta as *mut Proc;
    trend = (*p).trend;
    (*p).trend = ::core::ptr::null_mut::<Rendez>();
    if !trend.is_null() {
        wakeup(trend);
    }
}
#[no_mangle]
pub unsafe extern "C" fn tsleep(
    mut r: *mut Rendez,
    mut fn_0: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
    >,
    mut arg: *mut ::core::ffi::c_void,
    mut ms: i64_0,
) {
    let mut up: *mut Proc = externup();
    if !(*up).Timer.tt.is_null() {
        print(
            b"tsleep: timer active: mode %d, tf %#p\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            (*up).Timer.tmode,
            (*up).Timer.tf,
        );
        timerdel(&raw mut (*up).Timer);
    }
    (*up).Timer.tns = (ms * 1000000 as ::core::ffi::c_longlong) as i64_0;
    (*up).Timer.tf = Some(twakeup as unsafe extern "C" fn(*mut Ureg, *mut Timer) -> ())
        as Option<unsafe extern "C" fn(*mut Ureg, *mut Timer) -> ()>;
    (*up).Timer.tmode = Trelative as ::core::ffi::c_int;
    (*up).Timer.ta = up as *mut ::core::ffi::c_void;
    (*up).trend = r;
    (*up).tfn = fn_0;
    timeradd(&raw mut (*up).Timer);
    if ({
        let mut err: ::core::ffi::c_int = 0;
        __clobber_callee_regs();
        let fresh15 = (*up).nerrlab;
        (*up).nerrlab = (*up).nerrlab + 1;
        err = slim_setlabel(
            (&raw mut (*up).errlab as *mut Label).offset(fresh15 as isize) as *mut Label,
        );
        err
    }) != 0
    {
        timerdel(&raw mut (*up).Timer);
        nexterror();
    }
    sleep(
        r,
        Some(
            tfn as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
        ),
        arg,
    );
    if !(*up).Timer.tt.is_null() {
        timerdel(&raw mut (*up).Timer);
    }
    (*up).Timer.twhen = 0 as i64_0;
    (*up).nerrlab -= 1;
}
#[no_mangle]
pub unsafe extern "C" fn wakeup(mut r: *mut Rendez) -> *mut Proc {
    let mut pl: Mpl = 0;
    let mut p: *mut Proc = ::core::ptr::null_mut::<Proc>();
    pl = splhi() as Mpl;
    lock(&raw mut (*r).l);
    p = (*r)._p;
    if !p.is_null() {
        lock(&raw mut (*p).rlock);
        if (*p).state != Wakeme as ::core::ffi::c_int || (*p).r != r {
            panic(
                b"wakeup: state\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
        }
        (*r)._p = ::core::ptr::null_mut::<Proc>();
        (*p).r = ::core::ptr::null_mut::<Rendez>();
        ready(p);
        unlock(&raw mut (*p).rlock);
    }
    unlock(&raw mut (*r).l);
    splx(pl as u64_0);
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn postnote(
    mut p: *mut Proc,
    mut dolock: ::core::ffi::c_int,
    mut n: *mut ::core::ffi::c_char,
    mut flag: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut pl: Mpl = 0;
    let mut ret: ::core::ffi::c_int = 0;
    let mut r: *mut Rendez = ::core::ptr::null_mut::<Rendez>();
    let mut d: *mut Proc = ::core::ptr::null_mut::<Proc>();
    let mut l: *mut *mut Proc = ::core::ptr::null_mut::<*mut Proc>();
    if dolock != 0 {
        qlock(&raw mut (*p).debug);
    }
    if flag != NUser as ::core::ffi::c_int
        && ((*p).notify.is_none() || (*p).notified as ::core::ffi::c_int != 0)
    {
        (*p).nnote = 0 as ::core::ffi::c_short;
    }
    ret = 0 as ::core::ffi::c_int;
    if ((*p).nnote as ::core::ffi::c_int) < NNOTE as ::core::ffi::c_int {
        strcpy(
            &raw mut (*(&raw mut (*p).note as *mut Note).offset((*p).nnote as isize)).msg
                as *mut ::core::ffi::c_char,
            n,
        );
        let fresh9 = (*p).nnote;
        (*p).nnote = (*p).nnote + 1;
        (*p).note[fresh9 as usize].flag = flag;
        ret = 1 as ::core::ffi::c_int;
    }
    (*p).notepending = 1 as ::core::ffi::c_int;
    if (*p).state == Exotic as ::core::ffi::c_int {
        intrac(p);
    }
    if dolock != 0 {
        qunlock(&raw mut (*p).debug);
    }
    loop {
        pl = splhi() as Mpl;
        lock(&raw mut (*p).rlock);
        r = (*p).r;
        if r.is_null() {
            break;
        }
        if canlock(&raw mut (*r).l) != 0 {
            if (*p).state != Wakeme as ::core::ffi::c_int || (*r)._p != p {
                panic(
                    b"postnote: state %d %d %d\0" as *const u8
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    ((*r)._p != p) as ::core::ffi::c_int,
                    ((*p).r != r) as ::core::ffi::c_int,
                    (*p).state,
                );
            }
            (*p).r = ::core::ptr::null_mut::<Rendez>();
            (*r)._p = ::core::ptr::null_mut::<Proc>();
            ready(p);
            unlock(&raw mut (*r).l);
            break;
        } else {
            unlock(&raw mut (*p).rlock);
            splx(pl as u64_0);
            sched();
        }
    }
    unlock(&raw mut (*p).rlock);
    splx(pl as u64_0);
    if (*p).state != Rendezvous as ::core::ffi::c_int {
        if (*p).state == Semdown as ::core::ffi::c_int {
            ready(p);
        }
        return ret;
    }
    lock(&raw mut (*(*p).rgrp).r.l);
    if (*p).state == Rendezvous as ::core::ffi::c_int {
        (*p).rendval = !(0 as ::core::ffi::c_int) as usize_0;
        l = (&raw mut (*(*p).rgrp).rendhash as *mut *mut Proc)
            .offset(
                ((*p).rendtag
                    & (((1 as ::core::ffi::c_int) << RENDLOG as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) as usize_0) as isize,
            ) as *mut *mut Proc;
        d = *l;
        while !d.is_null() {
            if d == p {
                *l = (*p).rendhash;
                break;
            } else {
                l = &raw mut (*d).rendhash;
                d = (*d).rendhash;
            }
        }
        ready(p);
    }
    unlock(&raw mut (*(*p).rgrp).r.l);
    return ret;
}
pub const NBROKEN: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
#[no_mangle]
pub static mut broken: C2RustUnnamed_6 = C2RustUnnamed_6 {
    QLock: QLock {
        use_0: Lock {
            key: 0,
            isilock: 0,
            pl: 0,
            _pc: 0,
            p: ::core::ptr::null::<Proc>() as *mut Proc,
            m: ::core::ptr::null::<Mach>() as *mut Mach,
            lockcycles: 0,
        },
        head: ::core::ptr::null::<Proc>() as *mut Proc,
        tail: ::core::ptr::null::<Proc>() as *mut Proc,
        locked: 0,
        pc: 0,
    },
    n: 0,
    p: [::core::ptr::null::<Proc>() as *mut Proc; 4],
};
#[no_mangle]
pub unsafe extern "C" fn addbroken(mut p: *mut Proc) {
    let mut up: *mut Proc = externup();
    qlock(&raw mut broken.QLock);
    if broken.n == NBROKEN {
        ready(broken.p[0 as ::core::ffi::c_int as usize]);
        memmove(
            (&raw mut broken.p as *mut *mut Proc)
                .offset(0 as ::core::ffi::c_int as isize) as *mut *mut Proc
                as *mut ::core::ffi::c_void,
            (&raw mut broken.p as *mut *mut Proc)
                .offset(1 as ::core::ffi::c_int as isize) as *mut *mut Proc
                as *mut ::core::ffi::c_void,
            (::core::mem::size_of::<*mut Proc>() as usize)
                .wrapping_mul((NBROKEN - 1 as ::core::ffi::c_int) as usize) as u32_0,
        );
        broken.n -= 1;
    }
    let fresh8 = broken.n;
    broken.n = broken.n + 1;
    broken.p[fresh8 as usize] = p;
    qunlock(&raw mut broken.QLock);
    stopac();
    edfstop(up);
    (*p).state = Broken as ::core::ffi::c_int;
    (*p).psstate = ::core::ptr::null_mut::<::core::ffi::c_char>();
    sched();
}
#[no_mangle]
pub unsafe extern "C" fn unbreak(mut p: *mut Proc) {
    let mut b: ::core::ffi::c_int = 0;
    qlock(&raw mut broken.QLock);
    b = 0 as ::core::ffi::c_int;
    while b < broken.n {
        if broken.p[b as usize] == p {
            broken.n -= 1;
            memmove(
                (&raw mut broken.p as *mut *mut Proc).offset(b as isize)
                    as *mut *mut Proc as *mut ::core::ffi::c_void,
                (&raw mut broken.p as *mut *mut Proc)
                    .offset((b + 1 as ::core::ffi::c_int) as isize) as *mut *mut Proc
                    as *mut ::core::ffi::c_void,
                (::core::mem::size_of::<*mut Proc>() as usize)
                    .wrapping_mul((NBROKEN - (b + 1 as ::core::ffi::c_int)) as usize)
                    as u32_0,
            );
            ready(p);
            break;
        } else {
            b += 1;
        }
    }
    qunlock(&raw mut broken.QLock);
}
#[no_mangle]
pub unsafe extern "C" fn freebroken() -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    qlock(&raw mut broken.QLock);
    n = broken.n;
    i = 0 as ::core::ffi::c_int;
    while i < n {
        ready(broken.p[i as usize]);
        broken.p[i as usize] = ::core::ptr::null_mut::<Proc>();
        i += 1;
    }
    broken.n = 0 as ::core::ffi::c_int;
    qunlock(&raw mut broken.QLock);
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn pexit(
    mut exitstr: *mut ::core::ffi::c_char,
    mut freemem: ::core::ffi::c_int,
) {
    let mut up: *mut Proc = externup();
    let mut p: *mut Proc = ::core::ptr::null_mut::<Proc>();
    let mut s: *mut *mut Segment = ::core::ptr::null_mut::<*mut Segment>();
    let mut es: *mut *mut Segment = ::core::ptr::null_mut::<*mut Segment>();
    let mut utime: u64_0 = 0;
    let mut stime: u64_0 = 0;
    let mut wq: *mut Waitq = ::core::ptr::null_mut::<Waitq>();
    let mut f: *mut Waitq = ::core::ptr::null_mut::<Waitq>();
    let mut next: *mut Waitq = ::core::ptr::null_mut::<Waitq>();
    let mut fgrp: *mut Fgrp = ::core::ptr::null_mut::<Fgrp>();
    let mut egrp: *mut Egrp = ::core::ptr::null_mut::<Egrp>();
    let mut rgrp: *mut Rgrp = ::core::ptr::null_mut::<Rgrp>();
    let mut pgrp: *mut Pgrp = ::core::ptr::null_mut::<Pgrp>();
    let mut dot: *mut Chan = ::core::ptr::null_mut::<Chan>();
    if 0 as ::core::ffi::c_int != 0 && (*up).nfullq > 0 as ::core::ffi::c_int {
        iprint(
            b" %s=%d\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            (*up).text,
            (*up).nfullq,
        );
    }
    if 0 as ::core::ffi::c_int != 0 && (*up).nicc > 0 as u32_0 {
        iprint(
            b" [%s nicc %u tctime %llu actime %llu]\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            (*up).text,
            (*up).nicc,
            (*up).tctime,
            (*up).actime,
        );
    }
    if !(*up).syscalltrace.is_null() {
        free((*up).syscalltrace as *mut ::core::ffi::c_void);
    }
    (*up).syscalltrace = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*up).alarm = 0 as u32_0;
    if !(*up).Timer.tt.is_null() {
        timerdel(&raw mut (*up).Timer);
    }
    if (*up).trace != 0 {
        proctrace
            .expect(
                "non-null function pointer",
            )(up, SDead as ::core::ffi::c_int, 0 as i64_0);
    }
    qlock(&raw mut (*up).debug);
    fgrp = (*up).fgrp;
    (*up).fgrp = ::core::ptr::null_mut::<Fgrp>();
    egrp = (*up).egrp;
    (*up).egrp = ::core::ptr::null_mut::<Egrp>();
    rgrp = (*up).rgrp;
    (*up).rgrp = ::core::ptr::null_mut::<Rgrp>();
    pgrp = (*up).pgrp;
    (*up).pgrp = ::core::ptr::null_mut::<Pgrp>();
    dot = (*up).dot;
    (*up).dot = ::core::ptr::null_mut::<Chan>();
    qunlock(&raw mut (*up).debug);
    if !fgrp.is_null() {
        closefgrp(fgrp);
    }
    if !egrp.is_null() {
        closeegrp(egrp);
    }
    if !rgrp.is_null() {
        closergrp(rgrp);
    }
    if !dot.is_null() {
        cclose(dot);
    }
    if !pgrp.is_null() {
        closepgrp(pgrp);
    }
    if (*up).kp == 0 as ::core::ffi::c_int {
        p = (*up).parent;
        if p.is_null() {
            if exitstr.is_null() {
                exitstr = b"unknown\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
            }
            panic(
                b"boot process died: %s\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                exitstr,
            );
        }
        while ({
            let mut err: ::core::ffi::c_int = 0;
            __clobber_callee_regs();
            let fresh2 = (*up).nerrlab;
            (*up).nerrlab = (*up).nerrlab + 1;
            err = slim_setlabel(
                (&raw mut (*up).errlab as *mut Label).offset(fresh2 as isize)
                    as *mut Label,
            );
            err
        }) != 0
        {}
        wq = smalloc(::core::mem::size_of::<Waitq>() as u32_0) as *mut Waitq;
        (*up).nerrlab -= 1;
        (*wq).w.pid = (*up).pid;
        utime = (*up)
            .time[TUser as ::core::ffi::c_int as usize]
            .wrapping_add((*up).time[TCUser as ::core::ffi::c_int as usize]);
        stime = (*up)
            .time[TSys as ::core::ffi::c_int as usize]
            .wrapping_add((*up).time[TCSys as ::core::ffi::c_int as usize]);
        (*wq).w.time[TUser as ::core::ffi::c_int as usize] = utime
            .wrapping_mul((1000 as ::core::ffi::c_int / HZ) as u64_0) as u32_0;
        (*wq).w.time[TSys as ::core::ffi::c_int as usize] = stime
            .wrapping_mul((1000 as ::core::ffi::c_int / HZ) as u64_0) as u32_0;
        (*wq).w.time[TReal as ::core::ffi::c_int as usize] = (*(*sys)
            .machptr[0 as ::core::ffi::c_int as usize])
            .ticks
            .wrapping_sub((*up).time[TReal as ::core::ffi::c_int as usize])
            .wrapping_mul((1000 as ::core::ffi::c_int / HZ) as u64_0) as u32_0;
        if !exitstr.is_null()
            && *exitstr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != 0
        {
            snprint(
                &raw mut (*wq).w.msg as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 128]>()
                    as ::core::ffi::c_int,
                b"%s %d: %s\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                (*up).text,
                (*up).pid,
                exitstr,
            );
        } else {
            (*wq).w.msg[0 as ::core::ffi::c_int as usize] = '\0' as i32
                as ::core::ffi::c_char;
        }
        lock(&raw mut (*p).exl);
        if (*p).pid == (*up).parentpid && (*p).state != Broken as ::core::ffi::c_int {
            (*p).nchild -= 1;
            (*p).time[TCUser as ::core::ffi::c_int as usize] = (*p)
                .time[TCUser as ::core::ffi::c_int as usize]
                .wrapping_add(utime);
            (*p).time[TCSys as ::core::ffi::c_int as usize] = (*p)
                .time[TCSys as ::core::ffi::c_int as usize]
                .wrapping_add(stime);
            if (*p).nwait < 128 as ::core::ffi::c_int {
                (*wq).next = (*p).waitq;
                (*p).waitq = wq;
                (*p).nwait += 1;
                wq = ::core::ptr::null_mut::<Waitq>();
                wakeup(&raw mut (*p).waitr);
            }
        }
        unlock(&raw mut (*p).exl);
        if !wq.is_null() {
            free(wq as *mut ::core::ffi::c_void);
        }
    }
    if freemem == 0 {
        addbroken(up);
    }
    qlock(&raw mut (*up).seglock);
    es = (&raw mut (*up).seg as *mut *mut Segment)
        .offset(NSEG as ::core::ffi::c_int as isize) as *mut *mut Segment;
    s = &raw mut (*up).seg as *mut *mut Segment;
    while s < es {
        if !(*s).is_null() {
            putseg(*s);
            *s = ::core::ptr::null_mut::<Segment>();
        }
        s = s.offset(1);
    }
    qunlock(&raw mut (*up).seglock);
    lock(&raw mut (*up).exl);
    psunhash(up);
    (*up).pid = 0 as ::core::ffi::c_int;
    wakeup(&raw mut (*up).waitr);
    unlock(&raw mut (*up).exl);
    f = (*up).waitq;
    while !f.is_null() {
        next = (*f).next;
        free(f as *mut ::core::ffi::c_void);
        f = next;
    }
    qlock(&raw mut (*up).debug);
    if !(*up).pdbg.is_null() {
        wakeup(&raw mut (*(*up).pdbg).sleep);
        (*up).pdbg = ::core::ptr::null_mut::<Proc>();
    }
    qunlock(&raw mut (*up).debug);
    lock(&raw mut procalloc.l);
    lock(&raw mut pga.l);
    stopac();
    edfstop(up);
    (*up).state = Moribund as ::core::ffi::c_int;
    sched();
    if !(*p).strace.is_null() {
        decref(&raw mut (*(*p).strace).procs);
        decref(&raw mut (*(*p).strace).users);
    }
    (*p).strace = ::core::ptr::null_mut::<Strace>();
    panic(
        b"pexit\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn haswaitq(
    mut x: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut p: *mut Proc = ::core::ptr::null_mut::<Proc>();
    p = x as *mut Proc;
    return ((*p).waitq != ::core::ptr::null_mut::<Waitq>()) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn pwait(mut w: *mut Waitmsg) -> ::core::ffi::c_int {
    let mut up: *mut Proc = externup();
    let mut cpid: ::core::ffi::c_int = 0;
    let mut wq: *mut Waitq = ::core::ptr::null_mut::<Waitq>();
    if canqlock(&raw mut (*up).qwaitr) == 0 {
        error(&raw mut Einuse as *mut ::core::ffi::c_char);
    }
    if ({
        let mut err: ::core::ffi::c_int = 0;
        __clobber_callee_regs();
        let fresh11 = (*up).nerrlab;
        (*up).nerrlab = (*up).nerrlab + 1;
        err = slim_setlabel(
            (&raw mut (*up).errlab as *mut Label).offset(fresh11 as isize) as *mut Label,
        );
        err
    }) != 0
    {
        qunlock(&raw mut (*up).qwaitr);
        nexterror();
    }
    lock(&raw mut (*up).exl);
    if (*up).nchild == 0 as ::core::ffi::c_int && (*up).waitq.is_null() {
        unlock(&raw mut (*up).exl);
        error(&raw mut Enochild as *mut ::core::ffi::c_char);
    }
    unlock(&raw mut (*up).exl);
    sleep(
        &raw mut (*up).waitr,
        Some(
            haswaitq
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
        ),
        up as *mut ::core::ffi::c_void,
    );
    lock(&raw mut (*up).exl);
    wq = (*up).waitq;
    (*up).waitq = (*wq).next;
    (*up).nwait -= 1;
    unlock(&raw mut (*up).exl);
    qunlock(&raw mut (*up).qwaitr);
    (*up).nerrlab -= 1;
    if !w.is_null() {
        memmove(
            w as *mut ::core::ffi::c_void,
            &raw mut (*wq).w as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<Waitmsg>() as u32_0,
        );
    }
    cpid = (*wq).w.pid;
    free(wq as *mut ::core::ffi::c_void);
    return cpid;
}
#[no_mangle]
pub unsafe extern "C" fn dumpaproc(mut p: *mut Proc) {
    let mut bss: usize_0 = 0;
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if p.is_null() {
        return;
    }
    bss = 0 as usize_0;
    print(
        b"dumpaproc: what are HSEG and BSEG\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    );
    s = (*p).psstate;
    if s.is_null() {
        s = statename[(*p).state as usize];
    }
    print(
        b"%3d:%10s pc %#p dbgpc %#p  %8s (%s) ut %ld st %ld bss %#p qpc %#p nl %d nd %lu lpc %#p pri %lu\n\0"
            as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        (*p).pid,
        (*p).text,
        (*p).pc,
        dbgpc(p),
        s,
        statename[(*p).state as usize],
        (*p).time[0 as ::core::ffi::c_int as usize],
        (*p).time[1 as ::core::ffi::c_int as usize],
        bss,
        (*p).qpc,
        (*p).nlocks,
        (*p).delaysched,
        if !(*p).lastlock.is_null() {
            0 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        },
        (*p).priority,
    );
}
#[no_mangle]
pub unsafe extern "C" fn procdump() {
    let mut up: *mut Proc = externup();
    let mut i: ::core::ffi::c_int = 0;
    let mut p: *mut Proc = ::core::ptr::null_mut::<Proc>();
    if !up.is_null() {
        print(
            b"up %d\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            (*up).pid,
        );
    } else {
        print(
            b"no current process\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
    }
    i = 0 as ::core::ffi::c_int;
    loop {
        p = psincref(i);
        if p.is_null() {
            break;
        }
        if (*p).state != Dead as ::core::ffi::c_int {
            dumpaproc(p);
        }
        psdecref(p);
        i += 1;
    };
}
#[no_mangle]
pub unsafe extern "C" fn procflushseg(mut s: *mut Segment) {
    let mut i: ::core::ffi::c_int = 0;
    let mut ns: ::core::ffi::c_int = 0;
    let mut nm: ::core::ffi::c_int = 0;
    let mut nwait: ::core::ffi::c_int = 0;
    let mut p: *mut Proc = ::core::ptr::null_mut::<Proc>();
    nwait = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    loop {
        p = psincref(i);
        if p.is_null() {
            break;
        }
        if (*p).state == Dead as ::core::ffi::c_int {
            psdecref(p);
        } else {
            ns = 0 as ::core::ffi::c_int;
            while ns < NSEG as ::core::ffi::c_int {
                if (*p).seg[ns as usize] == s {
                    (*p).newtlb = 1 as ::core::ffi::c_int;
                    nm = 0 as ::core::ffi::c_int;
                    while nm < (*sys).nmach {
                        if (*(*sys).machptr[nm as usize]).proc_0 == p {
                            (*(*sys).machptr[nm as usize]).mmuflush = 1
                                as ::core::ffi::c_int;
                            nwait += 1;
                        }
                        nm += 1;
                    }
                    break;
                } else {
                    ns += 1;
                }
            }
            psdecref(p);
        }
        i += 1;
    }
    if nwait == 0 as ::core::ffi::c_int {
        return;
    }
    nm = 0 as ::core::ffi::c_int;
    while nm < (*sys).nmach {
        if (*sys).machptr[nm as usize] != machp() {
            while (*(*sys).machptr[nm as usize]).mmuflush != 0 {
                sched();
            }
        }
        nm += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn scheddump() {
    let mut p: *mut Proc = ::core::ptr::null_mut::<Proc>();
    let mut sch: *mut Sched = ::core::ptr::null_mut::<Sched>();
    let mut rq: *mut Schedq = ::core::ptr::null_mut::<Schedq>();
    sch = &raw mut run as *mut Sched;
    while sch
        < (&raw mut run as *mut Sched).offset(Nsched as ::core::ffi::c_int as isize)
            as *mut Sched
    {
        rq = (&raw mut (*sch).runq as *mut Schedq)
            .offset((Nrq as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize)
            as *mut Schedq;
        while rq >= &raw mut (*sch).runq as *mut Schedq {
            if !(*rq).head.is_null() {
                print(
                    b"sch%ld rq%ld:\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    sch.offset_from(&raw mut run as *mut Sched) as ::core::ffi::c_long,
                    rq.offset_from(&raw mut (*sch).runq as *mut Schedq)
                        as ::core::ffi::c_long,
                );
                p = (*rq).head;
                while !p.is_null() {
                    print(
                        b" %d(%lu)\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        (*p).pid,
                        (*machp()).ticks.wrapping_sub((*p).readytime),
                    );
                    p = (*p).rnext;
                }
                print(
                    b"\n\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
                delay(150 as ::core::ffi::c_int);
            }
            rq = rq.offset(-1);
        }
        print(
            b"sch%ld: nrdy %d\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            sch.offset_from(&raw mut run as *mut Sched) as ::core::ffi::c_long,
            (*sch).nrdy,
        );
        sch = sch.offset(1);
    }
}
unsafe extern "C" fn inherit_strace(mut parent: *mut Proc, mut child: *mut Proc) {
    if !(*parent).strace.is_null() && (*parent).strace_inherit != 0 {
        incref(&raw mut (*(*parent).strace).users);
        incref(&raw mut (*(*parent).strace).procs);
        (*child).strace = (*parent).strace;
        (*child).strace_on = 1 as ::core::ffi::c_int;
        (*child).strace_inherit = 1 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn kproc(
    mut name: *mut ::core::ffi::c_char,
    mut func: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    mut arg: *mut ::core::ffi::c_void,
) {
    let mut up: *mut Proc = externup();
    let mut p: *mut Proc = ::core::ptr::null_mut::<Proc>();
    static mut kpgrp: *mut Pgrp = ::core::ptr::null::<Pgrp>() as *mut Pgrp;
    p = newproc();
    (*p).psstate = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*p).procmode = 0o640 as u32_0;
    (*p).kp = 1 as ::core::ffi::c_int;
    (*p).noswap = 1 as ::core::ffi::c_int;
    (*p).scallnr = (*up).scallnr;
    memmove(
        &raw mut (*p).arg as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
        &raw mut (*up).arg as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[::core::ffi::c_uchar; 40]>() as u32_0,
    );
    (*p).nerrlab = 0 as ::core::ffi::c_int;
    (*p).slash = (*up).slash;
    (*p).dot = (*up).dot;
    if !(*p).dot.is_null() {
        incref(&raw mut (*(*p).dot).r);
    }
    memmove(
        &raw mut (*p).note as *mut Note as *mut ::core::ffi::c_void,
        &raw mut (*up).note as *mut Note as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[Note; 5]>() as u32_0,
    );
    (*p).nnote = (*up).nnote;
    (*p).notified = 0 as ::core::ffi::c_short;
    (*p).lastnote = (*up).lastnote;
    (*p).notify = (*up).notify;
    (*p).ureg = ::core::ptr::null_mut::<::core::ffi::c_void>();
    (*p).dbgreg = ::core::ptr::null_mut::<::core::ffi::c_void>();
    inherit_strace(up, p);
    procpriority(p, PriKproc as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
    kprocchild(p, func, arg);
    kstrdup(&raw mut (*p).user, eve);
    kstrdup(&raw mut (*p).text, name);
    if kpgrp.is_null() {
        kpgrp = newpgrp();
    }
    (*p).pgrp = kpgrp;
    incref(&raw mut (*kpgrp).r);
    memset(
        &raw mut (*p).time as *mut u64_0 as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[u64_0; 6]>() as u32_0,
    );
    (*p).time[TReal as ::core::ffi::c_int as usize] = (*(*sys)
        .machptr[0 as ::core::ffi::c_int as usize])
        .ticks;
    ready(p);
    (*p).newtlb = 1 as ::core::ffi::c_int;
    mmuflush();
}
#[no_mangle]
pub unsafe extern "C" fn procctl(mut p: *mut Proc) {
    let mut up: *mut Proc = externup();
    let mut pl: Mpl = 0;
    let mut state: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut current_block_31: u64;
    match (*p).procctl {
        4 => {
            spllo();
            pexit(
                b"Killed: Insufficient physical memory\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
            );
            current_block_31 = 2816009856922431226;
        }
        2 => {
            current_block_31 = 2816009856922431226;
        }
        3 => {
            current_block_31 = 16927074241705595696;
        }
        1 => {
            current_block_31 = 15376878412459677469;
        }
        6 => {
            (*p).procctl = 0 as ::core::ffi::c_int;
            spllo();
            runacore();
            return;
        }
        7 => {
            (*p).procctl = 0 as ::core::ffi::c_int;
            if p != up {
                panic(
                    b"procctl: stopac: p != up\0" as *const u8
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                );
            }
            spllo();
            stopac();
            return;
        }
        _ => {
            return;
        }
    }
    match current_block_31 {
        2816009856922431226 => {
            spllo();
            pexit(
                b"Killed\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
            );
            current_block_31 = 16927074241705595696;
        }
        _ => {}
    }
    match current_block_31 {
        16927074241705595696 => {
            if (*p).nnote as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                return;
            }
        }
        _ => {}
    }
    (*p).procctl = 0 as ::core::ffi::c_int;
    state = (*p).psstate;
    (*p).psstate = b"Stopped\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char;
    pl = spllo() as Mpl;
    qlock(&raw mut (*p).debug);
    if !(*p).pdbg.is_null() {
        wakeup(&raw mut (*(*p).pdbg).sleep);
        (*p).pdbg = ::core::ptr::null_mut::<Proc>();
    }
    qunlock(&raw mut (*p).debug);
    splhi();
    (*p).state = Stopped as ::core::ffi::c_int;
    sched();
    (*p).psstate = state;
    splx(pl as u64_0);
    return;
}
#[no_mangle]
pub unsafe extern "C" fn error(mut err: *mut ::core::ffi::c_char) {
    let mut up: *mut Proc = externup();
    spllo();
    if !((*up).nerrlab < NERR as ::core::ffi::c_int) {
        _assert(
            b"up->nerrlab < NERR\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
    }
    kstrcpy((*up).errstr, err, ERRMAX);
    ({
        let mut err_0: ::core::ffi::c_int = 0;
        __clobber_callee_regs();
        err_0 = slim_setlabel(
            (&raw mut (*up).errlab as *mut Label)
                .offset((NERR as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize)
                as *mut Label,
        );
    });
    nexterror();
}
#[no_mangle]
pub unsafe extern "C" fn nexterror() {
    let mut up: *mut Proc = externup();
    (*up).nerrlab -= 1;
    gotolabel(
        (&raw mut (*up).errlab as *mut Label).offset((*up).nerrlab as isize)
            as *mut Label,
    );
}
#[no_mangle]
pub unsafe extern "C" fn exhausted(mut resource: *mut ::core::ffi::c_char) {
    let mut buf: [::core::ffi::c_char; 128] = [0; 128];
    sprint(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"no free %s\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        resource,
    );
    iprint(
        b"%s\n\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        &raw mut buf as *mut ::core::ffi::c_char,
    );
    error(&raw mut buf as *mut ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn killbig(mut why: *mut ::core::ffi::c_char) {
    let mut i: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut s: *mut Segment = ::core::ptr::null_mut::<Segment>();
    let mut l: u32_0 = 0;
    let mut max: u32_0 = 0;
    let mut p: *mut Proc = ::core::ptr::null_mut::<Proc>();
    let mut kp: *mut Proc = ::core::ptr::null_mut::<Proc>();
    max = 0 as u32_0;
    kp = ::core::ptr::null_mut::<Proc>();
    x = 0 as ::core::ffi::c_int;
    loop {
        p = psincref(x);
        if p.is_null() {
            break;
        }
        if (*p).state == Dead as ::core::ffi::c_int || (*p).kp != 0 {
            psdecref(p);
        } else {
            l = 0 as u32_0;
            i = 1 as ::core::ffi::c_int;
            while i < NSEG as ::core::ffi::c_int {
                s = (*p).seg[i as usize];
                if !s.is_null() {
                    l = (l as usize_0).wrapping_add((*s).top.wrapping_sub((*s).base))
                        as u32_0 as u32_0;
                }
                i += 1;
            }
            if l > max
                && ((*p).procmode & 0o222 as u32_0 != 0
                    || strcmp(eve, (*p).user) != 0 as ::core::ffi::c_int)
            {
                if !kp.is_null() {
                    psdecref(kp);
                }
                kp = p;
                max = l;
            } else {
                psdecref(p);
            }
        }
        x += 1;
    }
    if kp.is_null() {
        return;
    }
    print(
        b"%d: %s killed: %s\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        (*kp).pid,
        (*kp).text,
        why,
    );
    x = 0 as ::core::ffi::c_int;
    loop {
        p = psincref(x);
        if p.is_null() {
            break;
        }
        if (*p).state == Dead as ::core::ffi::c_int || (*p).kp != 0 {
            psdecref(p);
        } else {
            panic(
                b"killbig\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
            psdecref(p);
        }
        x += 1;
    }
    (*kp).procctl = Proc_exitbig as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < NSEG as ::core::ffi::c_int {
        s = (*kp).seg[i as usize];
        if !s.is_null() && canqlock(&raw mut (*s).lk) != 0 {
            mfreeseg(
                s,
                (*s).base,
                ((*s).top as ::core::ffi::c_ulonglong)
                    .wrapping_sub((*s).base as ::core::ffi::c_ulonglong)
                    .wrapping_div(BIGPGSZ) as ::core::ffi::c_int,
            );
            qunlock(&raw mut (*s).lk);
        }
        i += 1;
    }
    psdecref(kp);
}
#[no_mangle]
pub unsafe extern "C" fn renameuser(
    mut old: *mut ::core::ffi::c_char,
    mut new: *mut ::core::ffi::c_char,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut p: *mut Proc = ::core::ptr::null_mut::<Proc>();
    i = 0 as ::core::ffi::c_int;
    loop {
        p = psincref(i);
        if p.is_null() {
            break;
        }
        if !(*p).user.is_null() && strcmp(old, (*p).user) == 0 as ::core::ffi::c_int {
            kstrdup(&raw mut (*p).user, new);
        }
        psdecref(p);
        i += 1;
    };
}
#[no_mangle]
pub unsafe extern "C" fn accounttime() {
    let mut sch: *mut Sched = ::core::ptr::null_mut::<Sched>();
    let mut p: *mut Proc = ::core::ptr::null_mut::<Proc>();
    let mut n: u64_0 = 0;
    let mut per: u64_0 = 0;
    sch = (*machp()).sch;
    p = (*machp()).proc_0;
    if !p.is_null() {
        if (*machp()).machno == 0 as ::core::ffi::c_int {
            (*sch).nrun = (*sch).nrun.wrapping_add(1);
        }
        (*p).time[(*p).insyscall as usize] = (*p)
            .time[(*p).insyscall as usize]
            .wrapping_add(1);
    }
    n = perfticks();
    per = n.wrapping_sub((*machp()).perf.last);
    (*machp()).perf.last = n;
    per = (*machp())
        .perf
        .period
        .wrapping_mul((HZ - 1 as ::core::ffi::c_int) as u64_0)
        .wrapping_add(per)
        .wrapping_div(HZ as u64_0);
    if per != 0 as u64_0 {
        (*machp()).perf.period = per;
    }
    (*machp()).perf.avg_inidle = (*machp())
        .perf
        .avg_inidle
        .wrapping_mul((HZ - 1 as ::core::ffi::c_int) as u64_0)
        .wrapping_add((*machp()).perf.inidle)
        .wrapping_div(HZ as u64_0);
    (*machp()).perf.inidle = 0 as u64_0;
    (*machp()).perf.avg_inintr = (*machp())
        .perf
        .avg_inintr
        .wrapping_mul((HZ - 1 as ::core::ffi::c_int) as u64_0)
        .wrapping_add((*machp()).perf.inintr)
        .wrapping_div(HZ as u64_0);
    (*machp()).perf.inintr = 0 as u64_0;
    if (*machp()).machno != 0 as ::core::ffi::c_int {
        return;
    }
    n = (*sch).nrun as u64_0;
    (*sch).nrun = 0 as u32_0;
    n = ((*sch).nrdy as u64_0).wrapping_add(n).wrapping_mul(1000 as u64_0);
    (*machp()).load = (((*machp()).load * (HZ - 1 as ::core::ffi::c_int)) as u64_0)
        .wrapping_add(n)
        .wrapping_div(HZ as u64_0) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn halt() {
    if (*(*machp()).sch).nrdy != 0 as ::core::ffi::c_int {
        return;
    }
    hardhalt();
}
