//! Oracles for differential tests of the Rust port: Harvey's original C,
//! built with `cc` (`c`), and the unmodified c2rust translation of it
//! (`c2rust`). Both expose the same small API so a test can run the same
//! input through each.
#![feature(extern_types, raw_ref_op, c_variadic)]

pub mod c2rust;
pub mod qsim;

use core::ffi::{c_char, c_int, c_ulong, c_void};

/// Mirror of the C `Watermark` (same layout in the C and c2rust builds).
pub use c2rust::watermarks::Watermark;

/// Generates the shared API over one implementation's raw entry points.
macro_rules! api {
    ($mul64fract:path, $ipchksum:path, $latin1:path, $initmark:path, $notemark:path, $seprintmark:path) => {
        use crate::*;

        /// `mul64fract`, with the out parameter turned into a return value.
        pub fn mul64fract(a: u64, b: u64) -> u64 {
            let mut r = 0;
            // SAFETY: `r` is a valid, writable u64 for the duration of the call.
            unsafe { $mul64fract(&mut r, a, b) };
            r
        }

        /// `ipchksum` over `data`.
        pub fn ipchksum(data: &[u8]) -> u16 {
            // SAFETY: pointer and length describe `data`, which is only read.
            unsafe { $ipchksum(data.as_ptr().cast(), data.len() as c_ulong) }
        }

        /// `latin1` over `k`, returning its raw status code.
        pub fn latin1(k: &[u32]) -> i32 {
            // SAFETY: pointer and length describe `k`; at most `n` entries are read.
            unsafe { $latin1(k.as_ptr().cast_mut(), k.len() as c_int) }
        }

        /// `initmark`. `name` must outlive the returned value.
        pub fn initmark(max: i32, name: &'static core::ffi::CStr) -> Watermark {
            let mut wp =
                Watermark { highwater: 0, curr: 0, max: 0, hitmax: 0, name: core::ptr::null_mut() };
            // SAFETY: `wp` is writable; `name` is a valid, 'static C string.
            unsafe { $initmark(&mut wp, max, name.as_ptr().cast_mut()) };
            wp
        }

        /// `notemark`.
        pub fn notemark(wp: &mut Watermark, val: i32) {
            // SAFETY: `wp` was set up by `initmark`.
            unsafe { $notemark(wp, val) }
        }

        /// `seprintmark`, rendered into a fresh string.
        pub fn seprintmark(wp: &mut Watermark) -> String {
            let mut buf = [0u8; 256];
            let range = buf.as_mut_ptr_range();
            // SAFETY: `range` spans `buf`; the function writes a NUL-terminated
            // string inside it and returns a pointer within the range.
            let end = unsafe { $seprintmark(range.start.cast(), range.end.cast(), wp) };
            let len = end as usize - buf.as_ptr() as usize;
            String::from_utf8_lossy(&buf[..len]).into_owned()
        }
    };
}

/// Harvey's C, as built from `c/`.
pub mod c {
    mod raw {
        use crate::*;

        extern "C" {
            #[link_name = "c_mul64fract"]
            pub fn mul64fract(r: *mut u64, a: u64, b: u64);
            #[link_name = "c_ipchksum"]
            pub fn ipchksum(ptr: *const c_void, nbytes: c_ulong) -> u16;
            #[link_name = "c_latin1"]
            pub fn latin1(k: *mut u32, n: c_int) -> i32;
            #[link_name = "c_initmark"]
            pub fn initmark(wp: *mut Watermark, max: c_int, name: *mut c_char);
            #[link_name = "c_notemark"]
            pub fn notemark(wp: *mut Watermark, val: c_int);
            #[link_name = "c_seprintmark"]
            pub fn seprintmark(
                buf: *mut c_char,
                ebuf: *mut c_char,
                wp: *mut Watermark,
            ) -> *mut c_char;
        }
    }

    api!(
        raw::mul64fract,
        raw::ipchksum,
        raw::latin1,
        raw::initmark,
        raw::notemark,
        raw::seprintmark
    );
}

/// The unmodified c2rust translation of the same C.
pub mod translated {
    use crate::c2rust::{ipchecksum, latin1, mul64fract, watermarks};

    api!(
        mul64fract::mul64fract,
        ipchecksum::ipchksum,
        latin1::latin1,
        watermarks::initmark,
        watermarks::notemark,
        watermarks::seprintmark
    );
}

#[cfg(test)]
mod tests {
    use core::ffi::CStr;

    #[test]
    fn c_and_translated_agree() {
        assert_eq!(
            super::c::mul64fract(3 << 31, 5 << 32),
            super::translated::mul64fract(3 << 31, 5 << 32)
        );
        // Like the C, ipchksum reads u16s, so the buffer must be 2-aligned.
        let data = b"abc".to_vec();
        assert_eq!(super::c::ipchksum(&data), super::translated::ipchksum(&data));
        let k = [u32::from(b'\''), u32::from(b'e')];
        assert_eq!(super::c::latin1(&k), super::translated::latin1(&k));
        assert_eq!(super::c::latin1(&k), 'é' as i32);
        let name = CStr::from_bytes_with_nul(b"m\0").unwrap();
        let (mut a, mut b) = (super::c::initmark(4, name), super::translated::initmark(4, name));
        super::c::notemark(&mut a, 3);
        super::translated::notemark(&mut b, 3);
        assert_eq!(super::c::seprintmark(&mut a), super::translated::seprintmark(&mut b));
    }
}
