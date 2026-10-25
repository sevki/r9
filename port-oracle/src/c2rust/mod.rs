//! The c2rust translation of the C in `../../c`, one module per file.
//!
//! Generated code, so exempt from rustfmt. The only edits to it are the
//! removal of `#[no_mangle]` (so it can coexist with the C oracle and with
//! itself in one binary) and, in `latin1`, moving each transmuted rune array
//! into its own `static` (current rustc rejects the pointer-to-temporary
//! c2rust emitted).
#[rustfmt::skip]
pub mod ipchecksum;
#[rustfmt::skip]
pub mod latin1;
#[rustfmt::skip]
pub mod mul64fract;
#[rustfmt::skip]
pub mod watermarks;
#[rustfmt::skip]
pub mod qlock;
