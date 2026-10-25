#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]
extern "C" {
    pub type Queue;
    pub type Mntrpc;
    pub type Mntcache;
    pub type DevConf;
    pub type Nixpctl;
    pub type Edf;
    pub type Timers;
    pub type Ureg;
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: u32_0,
    ) -> *mut ::core::ffi::c_void;
    fn seprint(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
        ...
    ) -> *mut ::core::ffi::c_char;
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
    pub tfn: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
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
    pub notify:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_char) -> ()>,
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
    pub stat: Option<unsafe extern "C" fn(*mut Chan, *mut ::core::ffi::c_uchar, i32_0) -> i32_0>,
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
    pub read:
        Option<unsafe extern "C" fn(*mut Chan, *mut ::core::ffi::c_void, i32_0, i64_0) -> i32_0>,
    pub bread: Option<unsafe extern "C" fn(*mut Chan, i32_0, i64_0) -> *mut Block>,
    pub write:
        Option<unsafe extern "C" fn(*mut Chan, *mut ::core::ffi::c_void, i32_0, i64_0) -> i32_0>,
    pub bwrite: Option<unsafe extern "C" fn(*mut Chan, *mut Block, i64_0) -> i32_0>,
    pub remove: Option<unsafe extern "C" fn(*mut Chan) -> ()>,
    pub wstat: Option<unsafe extern "C" fn(*mut Chan, *mut ::core::ffi::c_uchar, i32_0) -> i32_0>,
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
        unsafe extern "C" fn(*mut Chan, *mut Kzio, ::core::ffi::c_int, i64_0) -> ::core::ffi::c_int,
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
pub struct Note {
    pub msg: [::core::ffi::c_char; 128],
    pub flag: ::core::ffi::c_int,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Watermark {
    pub highwater: ::core::ffi::c_int,
    pub curr: ::core::ffi::c_int,
    pub max: ::core::ffi::c_int,
    pub hitmax: ::core::ffi::c_int,
    pub name: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub l: Lock,
    pub nonline: ::core::ffi::c_int,
    pub nbooting: ::core::ffi::c_int,
    pub exiting: ::core::ffi::c_int,
    pub ispanic: ::core::ffi::c_int,
    pub thunderbirdsarego: ::core::ffi::c_int,
}
pub static mut active: C2RustUnnamed_0 = C2RustUnnamed_0 {
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
pub static mut consdebug: Option<unsafe extern "C" fn() -> ()> = None;
pub static mut consputs: Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_char, ::core::ffi::c_int) -> (),
> = None;
pub static mut kproftimer: Option<unsafe extern "C" fn(usize_0) -> ()> = None;
pub static mut mfcinit: Option<unsafe extern "C" fn() -> ()> = None;
pub static mut mfcopen: Option<unsafe extern "C" fn(*mut Chan) -> ()> = None;
pub static mut mfcread: Option<
    unsafe extern "C" fn(
        *mut Chan,
        *mut ::core::ffi::c_uchar,
        ::core::ffi::c_int,
        i64_0,
    ) -> ::core::ffi::c_int,
> = None;
pub static mut mfcupdate: Option<
    unsafe extern "C" fn(*mut Chan, *mut ::core::ffi::c_uchar, ::core::ffi::c_int, i64_0) -> (),
> = None;
pub static mut mfcwrite: Option<
    unsafe extern "C" fn(*mut Chan, *mut ::core::ffi::c_uchar, ::core::ffi::c_int, i64_0) -> (),
> = None;
pub static mut mwait: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()> = None;
pub static mut proctrace: Option<unsafe extern "C" fn(*mut Proc, ::core::ffi::c_int, i64_0) -> ()> =
    None;
pub static mut coherence: Option<unsafe extern "C" fn() -> ()> = None;
pub unsafe extern "C" fn initmark(
    mut wp: *mut Watermark,
    mut max: ::core::ffi::c_int,
    mut name: *mut ::core::ffi::c_char,
) {
    memset(
        wp as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<Watermark>() as u32_0,
    );
    (*wp).max = max;
    (*wp).name = name;
}
pub unsafe extern "C" fn notemark(mut wp: *mut Watermark, mut val: ::core::ffi::c_int) {
    if val < 0 as ::core::ffi::c_int {
        val = 0 as ::core::ffi::c_int;
    } else if val > (*wp).max {
        val = (*wp).max;
    }
    if val > (*wp).highwater {
        (*wp).highwater = val;
        if val == (*wp).max && (*wp).curr < val {
            (*wp).hitmax += 1;
        }
    }
    (*wp).curr = val;
}
pub unsafe extern "C" fn seprintmark(
    mut buf: *mut ::core::ffi::c_char,
    mut ebuf: *mut ::core::ffi::c_char,
    mut wp: *mut Watermark,
) -> *mut ::core::ffi::c_char {
    return seprint(
        buf,
        ebuf,
        b"%s:\thighwater %d/%d curr %d hitmax %d\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        (*wp).name,
        (*wp).highwater,
        (*wp).max,
        (*wp).curr,
        (*wp).hitmax,
    );
}
