#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type u64_0 = ::core::ffi::c_ulonglong;
#[no_mangle]
pub unsafe extern "C" fn mul64fract(mut r: *mut u64_0, mut a: u64_0, mut b: u64_0) {
    let mut bh: u64_0 = 0;
    let mut bl: u64_0 = 0;
    let mut ah: u64_0 = 0;
    let mut al: u64_0 = 0;
    let mut res: u64_0 = 0;
    bl = (b as ::core::ffi::c_ulonglong & 0xffffffff as ::core::ffi::c_ulonglong) as u64_0;
    bh = b >> 32 as ::core::ffi::c_int;
    al = (a as ::core::ffi::c_ulonglong & 0xffffffff as ::core::ffi::c_ulonglong) as u64_0;
    ah = a >> 32 as ::core::ffi::c_int;
    res = al.wrapping_mul(bl) >> 32 as ::core::ffi::c_int;
    res = res.wrapping_add(al.wrapping_mul(bh));
    res = res.wrapping_add(ah.wrapping_mul(bl));
    res = res.wrapping_add(ah.wrapping_mul(bh) << 32 as ::core::ffi::c_int);
    *r = res;
}
