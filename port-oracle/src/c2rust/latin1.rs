#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(raw_ref_op)]
pub type u32_0 = ::core::ffi::c_uint;
pub type i32_0 = ::core::ffi::c_int;
pub type Rune = u32_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cvlist {
    pub ld: *mut ::core::ffi::c_char,
    pub si: *mut ::core::ffi::c_char,
    pub so: *mut i32_0,
}
pub const nil: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub static mut latintab: [cvlist; 101] = unsafe {
    [
        cvlist {
            ld: b" \0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b" i\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_int; 3],
            >(*b"#$\0\x001\x01\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"!~\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"-=~\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 16],
                [::core::ffi::c_int; 4],
            >(*b"D\"\0\0G\"\0\0I\"\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"!\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"!<=>?bmp\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 36],
                [::core::ffi::c_int; 9],
            >(
                    *b"\xA1\0\0\0n\"\0\0`\"\0\0o\"\0\0= \0\0\x84\"\0\0\t\"\0\0\x85\"\0\0\0\0\0\0",
                )
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"\"*\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"IUiu\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 20],
                [::core::ffi::c_int; 5],
            >(*b"\xAA\x03\0\0\xAB\x03\0\0\xCA\x03\0\0\xCB\x03\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"\"\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"\"AEIOUYaeiouy\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 56],
                [::core::ffi::c_int; 14],
            >(
                    *b"\xA8\0\0\0\xC4\0\0\0\xCB\0\0\0\xCF\0\0\0\xD6\0\0\0\xDC\0\0\0x\x01\0\0\xE4\0\0\0\xEB\0\0\0\xEF\0\0\0\xF6\0\0\0\xFC\0\0\0\xFF\0\0\0\0\0\0\0",
                )
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"$*\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"fhk\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 16],
                [::core::ffi::c_int; 4],
            >(*b"\xD5\x03\0\0\xD1\x03\0\0\xF0\x03\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"$\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"BEFHILMRVaefglopv\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 72],
                [::core::ffi::c_int; 18],
            >(
                    *b",!\0\x000!\0\x001!\0\0\x0B!\0\0\x10!\0\0\x12!\0\x003!\0\0\x1B!\0\0\xB2\x01\0\0Q\x02\0\0/!\0\0\x92\x01\0\0\n!\0\0\x13!\0\x004!\0\0\x18!\0\0\x8B\x02\0\0\0\0\0\0",
                )
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"'\"\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"Uu\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_int; 3],
            >(*b"\xD7\x01\0\0\xD8\x01\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"'\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"'ACEILNORSUYZacegilnorsuyz\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 108],
                [::core::ffi::c_int; 27],
            >(
                    *b"\xB4\0\0\0\xC1\0\0\0\x06\x01\0\0\xC9\0\0\0\xCD\0\0\09\x01\0\0C\x01\0\0\xD3\0\0\0T\x01\0\0Z\x01\0\0\xDA\0\0\0\xDD\0\0\0y\x01\0\0\xE1\0\0\0\x07\x01\0\0\xE9\0\0\0#\x01\0\0\xED\0\0\0:\x01\0\0D\x01\0\0\xF3\0\0\0U\x01\0\0[\x01\0\0\xFA\0\0\0\xFD\0\0\0z\x01\0\0\0\0\0\0",
                )
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"*\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"*ABCDEFGHIKLMNOPQRSTUWXYZabcdefghiklmnopqrstuwxyz\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 200],
                [::core::ffi::c_int; 50],
            >(
                    *b"\x17\"\0\0\x91\x03\0\0\x92\x03\0\0\x9E\x03\0\0\x94\x03\0\0\x95\x03\0\0\xA6\x03\0\0\x93\x03\0\0\x98\x03\0\0\x99\x03\0\0\x9A\x03\0\0\x9B\x03\0\0\x9C\x03\0\0\x9D\x03\0\0\x9F\x03\0\0\xA0\x03\0\0\xA8\x03\0\0\xA1\x03\0\0\xA3\x03\0\0\xA4\x03\0\0\xA5\x03\0\0\xA9\x03\0\0\xA7\x03\0\0\x97\x03\0\0\x96\x03\0\0\xB1\x03\0\0\xB2\x03\0\0\xBE\x03\0\0\xB4\x03\0\0\xB5\x03\0\0\xC6\x03\0\0\xB3\x03\0\0\xB8\x03\0\0\xB9\x03\0\0\xBA\x03\0\0\xBB\x03\0\0\xBC\x03\0\0\xBD\x03\0\0\xBF\x03\0\0\xC0\x03\0\0\xC8\x03\0\0\xC1\x03\0\0\xC3\x03\0\0\xC4\x03\0\0\xC5\x03\0\0\xC9\x03\0\0\xC7\x03\0\0\xB7\x03\0\0\xB6\x03\0\0\0\0\0\0",
                )
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"+\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"-O\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_int; 3],
            >(*b"\xB1\0\0\0\x95\"\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b",\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b",ACEGIKLNORSTUacegiklnorstu\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 112],
                [::core::ffi::c_int; 28],
            >(
                    *b"\xB8\0\0\0\x04\x01\0\0\xC7\0\0\0\x18\x01\0\0\"\x01\0\0.\x01\0\x006\x01\0\0;\x01\0\0E\x01\0\0\xEA\x01\0\0V\x01\0\0^\x01\0\0b\x01\0\0r\x01\0\0\x05\x01\0\0\xE7\0\0\0\x19\x01\0\0#\x01\0\0/\x01\0\x007\x01\0\0<\x01\0\0F\x01\0\0\xEB\x01\0\0W\x01\0\0_\x01\0\0c\x01\0\0s\x01\0\0\0\0\0\0",
                )
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"-*\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"l\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_int; 2],
            >(*b"\x9B\x01\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"-\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"+-2:>DGHILOTZbdghiltuz~\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 96],
                [::core::ffi::c_int; 24],
            >(
                    *b"\x13\"\0\0\xAD\0\0\0\xBB\x01\0\0\xF7\0\0\0\x92!\0\0\xD0\0\0\0\xE4\x01\0\0&\x01\0\0\x97\x01\0\0A\x01\0\0\x96\"\0\0f\x01\0\0\xB5\x01\0\0\x80\x01\0\0\xF0\0\0\0\xE5\x01\0\0\x0F!\0\0h\x02\0\0B\x01\0\0g\x01\0\0\x89\x02\0\0\xB6\x01\0\0B\"\0\0\0\0\0\0",
                )
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b".\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b".CEGILOZceglz\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 56],
                [::core::ffi::c_int; 14],
            >(
                    *b"\xB7\0\0\0\n\x01\0\0\x16\x01\0\0 \x01\0\x000\x01\0\0?\x01\0\0\x99\"\0\0{\x01\0\0\x0B\x01\0\0\x17\x01\0\0!\x01\0\0@\x01\0\0|\x01\0\0\0\0\0\0",
                )
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"/\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"Oo\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_int; 3],
            >(*b"\xD8\0\0\0\xF8\0\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"1\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b".234568\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_int; 8],
            >(*b"$ \0\0\xBD\0\0\0S!\0\0\xBC\0\0\0U!\0\0Y!\0\0[!\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"2\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"-.35\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 20],
                [::core::ffi::c_int; 5],
            >(*b"\xBB\x01\0\0% \0\0T!\0\0V!\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"3\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b".458\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 20],
                [::core::ffi::c_int; 5],
            >(*b"& \0\0\xBE\0\0\0W!\0\0\\!\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"4\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"5\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_int; 2],
            >(*b"X!\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"5\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"68\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_int; 3],
            >(*b"Z!\0\0]!\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"7\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"8\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_int; 2],
            >(*b"^!\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b":\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"()-=\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 20],
                [::core::ffi::c_int; 5],
            >(*b"9&\0\0:&\0\0\xF7\0\0\0T\"\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"<!\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"=~\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_int; 3],
            >(*b"h\"\0\0\xE6\"\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"<\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"-<=>~\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 24],
                [::core::ffi::c_int; 6],
            >(*b"\x90!\0\0\xAB\0\0\0d\"\0\0v\"\0\0r\"\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"=\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b":<=>OV\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 28],
                [::core::ffi::c_int; 7],
            >(*b"U\"\0\0\xDC\"\0\0a\"\0\0\xDD\"\0\0\x9C\"\0\0\xD2!\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b">!\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"=~\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_int; 3],
            >(*b"i\"\0\0\xE7\"\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b">\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"<=>~\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 20],
                [::core::ffi::c_int; 5],
            >(*b"w\"\0\0e\"\0\0\xBB\0\0\0s\"\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"?\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"!?\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_int; 3],
            >(*b"= \0\0\xBF\0\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"@'\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"'\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_int; 2],
            >(*b"J\x04\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"@@\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"'EKSTYZekstyz\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 56],
                [::core::ffi::c_int; 14],
            >(
                    *b"L\x04\0\0\x15\x04\0\0\x1A\x04\0\0!\x04\0\0\"\x04\0\0+\x04\0\0\x17\x04\0\x005\x04\0\0:\x04\0\0A\x04\0\0B\x04\0\0K\x04\0\x007\x04\0\0\0\0\0\0",
                )
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"@C\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"Hh\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_int; 3],
            >(*b"'\x04\0\0'\x04\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"@E\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"Hh\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_int; 3],
            >(*b"-\x04\0\0-\x04\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"@K\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"Hh\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_int; 3],
            >(*b"%\x04\0\0%\x04\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"@S\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"CHch\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 20],
                [::core::ffi::c_int; 5],
            >(*b")\x04\0\0(\x04\0\0)\x04\0\0(\x04\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"@T\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"Ss\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_int; 3],
            >(*b"&\x04\0\0&\x04\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"@Y\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"AEOUaeou\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 36],
                [::core::ffi::c_int; 9],
            >(
                    *b"/\x04\0\0\x15\x04\0\0\x01\x04\0\0.\x04\0\0/\x04\0\0\x15\x04\0\0\x01\x04\0\0.\x04\0\0\0\0\0\0",
                )
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"@Z\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"Hh\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_int; 3],
            >(*b"\x16\x04\0\0\x16\x04\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"@c\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"h\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_int; 2],
            >(*b"G\x04\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"@e\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"h\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_int; 2],
            >(*b"M\x04\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"@k\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"h\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_int; 2],
            >(*b"E\x04\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"@s\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"ch\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_int; 3],
            >(*b"I\x04\0\0H\x04\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"@t\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"s\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_int; 2],
            >(*b"F\x04\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"@y\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"aeou\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 20],
                [::core::ffi::c_int; 5],
            >(*b"O\x04\0\x005\x04\0\0Q\x04\0\0N\x04\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"@z\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"h\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_int; 2],
            >(*b"6\x04\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"@\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"ABDFGIJLMNOPRUVXabdfgijlmnopruvx\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 132],
                [::core::ffi::c_int; 33],
            >(
                    *b"\x10\x04\0\0\x11\x04\0\0\x14\x04\0\0$\x04\0\0\x13\x04\0\0\x18\x04\0\0\x19\x04\0\0\x1B\x04\0\0\x1C\x04\0\0\x1D\x04\0\0\x1E\x04\0\0\x1F\x04\0\0 \x04\0\0#\x04\0\0\x12\x04\0\0%\x04\0\x000\x04\0\x001\x04\0\x004\x04\0\0D\x04\0\x003\x04\0\08\x04\0\09\x04\0\0;\x04\0\0<\x04\0\0=\x04\0\0>\x04\0\0?\x04\0\0@\x04\0\0C\x04\0\x002\x04\0\0E\x04\0\0\0\0\0\0",
                )
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"A\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"E\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_int; 2],
            >(*b"\xC6\0\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"C\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"ACU\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 16],
                [::core::ffi::c_int; 4],
            >(*b"\xC2\"\0\0\x02!\0\0\xC3\"\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"Dv\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"Zz\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_int; 3],
            >(*b"\xC4\x01\0\0\xC5\x01\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"D\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"-e\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_int; 3],
            >(*b"\xD0\0\0\0\x06\"\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"G\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"-\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_int; 2],
            >(*b"\xE4\x01\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"H\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"-H\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_int; 3],
            >(*b"&\x01\0\0\r!\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"I\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"-J\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_int; 3],
            >(*b"\x97\x01\0\x002\x01\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"L\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"&-Jj|\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 24],
                [::core::ffi::c_int; 6],
            >(*b"\xC0\"\0\0A\x01\0\0\xC7\x01\0\0\xC8\x01\0\0\xC1\"\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"M\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"#48bs\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 24],
                [::core::ffi::c_int; 6],
            >(*b"n&\0\0i&\0\0j&\0\0m&\0\0o&\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"N\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"JNj\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 16],
                [::core::ffi::c_int; 4],
            >(*b"\xCA\x01\0\0\x15!\0\0\xCB\x01\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"O\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"*+-./=EIcoprx\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 56],
                [::core::ffi::c_int; 14],
            >(
                    *b"\x9B\"\0\0\x95\"\0\0\x96\"\0\0\x99\"\0\0\x98\"\0\0\x9C\"\0\0R\x01\0\0\xA2\x01\0\0\xA9\0\0\0\x9A\"\0\0\x17!\0\0\xAE\0\0\0\x97\"\0\0\0\0\0\0",
                )
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"P\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"P\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_int; 2],
            >(*b"\x19!\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"Q\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"Q\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_int; 2],
            >(*b"\x1A!\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"R\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"R\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_int; 2],
            >(*b"\x1D!\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"S\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"123S\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 20],
                [::core::ffi::c_int; 5],
            >(*b"\xB9\0\0\0\xB2\0\0\0\xB3\0\0\0\xA7\0\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"T\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"-u\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_int; 3],
            >(*b"f\x01\0\0\xA8\"\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"V\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"=\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_int; 2],
            >(*b"\xD0!\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"Y\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"R\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_int; 2],
            >(*b"\xA6\x01\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"Z\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"-ACSZ\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 24],
                [::core::ffi::c_int; 6],
            >(*b"\xB5\x01\0\0\x15\xF0\0\0\x17\xF0\0\0\x16\xF0\0\0$!\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"^\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"ACEGHIJOSUWYaceghijosuwy\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 100],
                [::core::ffi::c_int; 25],
            >(
                    *b"\xC2\0\0\0\x08\x01\0\0\xCA\0\0\0\x1C\x01\0\0$\x01\0\0\xCE\0\0\x004\x01\0\0\xD4\0\0\0\\\x01\0\0\xDB\0\0\0t\x01\0\0v\x01\0\0\xE2\0\0\0\t\x01\0\0\xEA\0\0\0\x1D\x01\0\0%\x01\0\0\xEE\0\0\x005\x01\0\0\xF4\0\0\0]\x01\0\0\xFB\0\0\0u\x01\0\0w\x01\0\0\0\0\0\0",
                )
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"_\"\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"AUau\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 20],
                [::core::ffi::c_int; 5],
            >(*b"\xDE\x01\0\0\xD5\x01\0\0\xDF\x01\0\0\xD6\x01\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"_,\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"Oo\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_int; 3],
            >(*b"\xEC\x01\0\0\xED\x01\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"_.\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"Aa\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_int; 3],
            >(*b"\xE0\x01\0\0\xE1\x01\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"_\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"AEIOU_aeiou\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 48],
                [::core::ffi::c_int; 12],
            >(
                    *b"\0\x01\0\0\x12\x01\0\0*\x01\0\0L\x01\0\0j\x01\0\0\xAF\0\0\0\x01\x01\0\0\x13\x01\0\0+\x01\0\0M\x01\0\0k\x01\0\0\0\0\0\0",
                )
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"`\"\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"Uu\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_int; 3],
            >(*b"\xDB\x01\0\0\xDC\x01\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"`\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"AEIOUaeiou\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 44],
                [::core::ffi::c_int; 11],
            >(
                    *b"\xC0\0\0\0\xC8\0\0\0\xCC\0\0\0\xD2\0\0\0\xD9\0\0\0\xE0\0\0\0\xE8\0\0\0\xEC\0\0\0\xF2\0\0\0\xF9\0\0\0\0\0\0\0",
                )
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"a\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"ben\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 16],
                [::core::ffi::c_int; 4],
            >(*b"\x94!\0\0\xE6\0\0\0 \"\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"b\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"()+-0123456789=bknpqru\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 92],
                [::core::ffi::c_int; 23],
            >(
                    *b"\x8D \0\0\x8E \0\0\x8A \0\0\x8B \0\0\x80 \0\0\x81 \0\0\x82 \0\0\x83 \0\0\x84 \0\0\x85 \0\0\x86 \0\0\x87 \0\0\x88 \0\0\x89 \0\0\x8C \0\0]&\0\0Z&\0\0^&\0\0_&\0\0[&\0\0\\&\0\0\" \0\0\0\0\0\0",
                )
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"c\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"$Oagu\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 24],
                [::core::ffi::c_int; 6],
            >(*b"\xA2\0\0\0\xA9\0\0\0)\"\0\0E\"\0\0*\"\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"dv\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"z\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_int; 2],
            >(*b"\xC6\x01\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"d\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"-adegz\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 28],
                [::core::ffi::c_int; 7],
            >(*b"\xF0\0\0\0\x93!\0\0! \0\0\xB0\0\0\0  \0\0\xA3\x02\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"e\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"$lmns\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 24],
                [::core::ffi::c_int; 6],
            >(*b"\xAC \0\0\xEF\"\0\0\x14 \0\0\x13 \0\0\x05\"\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"f\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"a\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_int; 2],
            >(*b"\0\"\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"g\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"$-r\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 16],
                [::core::ffi::c_int; 4],
            >(*b"\xA4\0\0\0\xE5\x01\0\0\x07\"\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"h\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"-v\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_int; 3],
            >(*b"\x0F!\0\0\x95\x01\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"i\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"-bfjps\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 28],
                [::core::ffi::c_int; 7],
            >(*b"h\x02\0\0\x86\"\0\0\x1E\"\0\x003\x01\0\0\x87\"\0\0+\"\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"l\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"\"$&'-jz|\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 36],
                [::core::ffi::c_int; 9],
            >(
                    *b"\x1C \0\0\xA3\0\0\0'\"\0\0\x18 \0\0B\x01\0\0\xC9\x01\0\0\xC4\"\0\0(\"\0\0\0\0\0\0",
                )
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"m\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"iou\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 16],
                [::core::ffi::c_int; 4],
            >(*b"\xB5\0\0\0\x08\"\0\0\xD7\0\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"jo\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_int; 3],
            >(*b"\xCC\x01\0\0\xAC\0\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"o\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"AOUaeiu\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_int; 8],
            >(
                    *b"\xC5\0\0\0\x9A\"\0\0n\x01\0\0\xE5\0\0\0S\x01\0\0\xA3\x01\0\0o\x01\0\0\0\0\0\0",
                )
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"p\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"Odgrt\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 24],
                [::core::ffi::c_int; 6],
            >(*b"\x17!\0\0\x02\"\0\0\xB6\0\0\0\x0F\"\0\0\x1D\"\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"r\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"\"'O\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 16],
                [::core::ffi::c_int; 4],
            >(*b"\x1D \0\0\x19 \0\0\xAE\0\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"s\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"()+-0123456789=abnoprstu\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 100],
                [::core::ffi::c_int; 25],
            >(
                    *b"} \0\0~ \0\0z \0\0{ \0\0p \0\0q \0\0r \0\0s \0\0t \0\0u \0\0v \0\0w \0\0x \0\0y \0\0| \0\0\xAA\0\0\0\x82\"\0\0\x7F \0\0\xBA\0\0\0\x83\"\0\0\x1A\"\0\0\xDF\0\0\0\r\"\0\0\x11\"\0\0\0\0\0\0",
                )
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"t\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"-efmsu\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 28],
                [::core::ffi::c_int; 7],
            >(*b"g\x01\0\0\x03\"\0\x004\"\0\0\"!\0\0\xC2\x03\0\0\xA2\"\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"u\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"-AEGIOUaegiou\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 56],
                [::core::ffi::c_int; 14],
            >(
                    *b"\x89\x02\0\0\x02\x01\0\0\x14\x01\0\0\x1E\x01\0\0,\x01\0\0N\x01\0\0l\x01\0\0\x91!\0\0\x15\x01\0\0\x1F\x01\0\0-\x01\0\0O\x01\0\0m\x01\0\0\0\0\0\0",
                )
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"v\"\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"Uu\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_int; 3],
            >(*b"\xD9\x01\0\0\xDA\x01\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"v\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"ACDEGIKLNORSTUZacdegijklnorstuz\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 128],
                [::core::ffi::c_int; 32],
            >(
                    *b"\xCD\x01\0\0\x0C\x01\0\0\x0E\x01\0\0\x1A\x01\0\0\xE6\x01\0\0\xCF\x01\0\0\xE8\x01\0\0=\x01\0\0G\x01\0\0\xD1\x01\0\0X\x01\0\0`\x01\0\0d\x01\0\0\xD3\x01\0\0}\x01\0\0\xCE\x01\0\0\r\x01\0\0\x0F\x01\0\0\x1B\x01\0\0\xE7\x01\0\0\xD0\x01\0\0\xF0\x01\0\0\xE9\x01\0\0>\x01\0\0H\x01\0\0\xD2\x01\0\0Y\x01\0\0a\x01\0\0e\x01\0\0\xD4\x01\0\0~\x01\0\0\0\0\0\0",
                )
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"w\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"bknpqr\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 28],
                [::core::ffi::c_int; 7],
            >(*b"W&\0\0T&\0\0X&\0\0Y&\0\0U&\0\0V&\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"x\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"O\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_int; 2],
            >(*b"\x97\"\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"y\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"$\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_int; 2],
            >(*b"\xA5\0\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"z\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"-\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_int; 2],
            >(*b"\xB6\x01\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"|\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"Pp|\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 16],
                [::core::ffi::c_int; 4],
            >(*b"\xDE\0\0\0\xFE\0\0\0\xA6\0\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"~!\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"=\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_int; 2],
            >(*b"F\"\0\0\0\0\0\0")
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: b"~\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            si: b"-=AINOUainou~\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            so: ::core::mem::transmute::<
                [u8; 56],
                [::core::ffi::c_int; 14],
            >(
                    *b"C\"\0\0E\"\0\0\xC3\0\0\0(\x01\0\0\xD1\0\0\0\xD5\0\0\0h\x01\0\0\xE3\0\0\0)\x01\0\0\xF1\0\0\0\xF5\0\0\0i\x01\0\0H\"\0\0\0\0\0\0",
                )
                .as_ptr() as *mut i32_0,
        },
        cvlist {
            ld: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            si: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            so: ::core::ptr::null::<i32_0>() as *mut i32_0,
        },
    ]
};
#[no_mangle]
pub unsafe extern "C" fn unicode(mut k: *mut Rune) -> i32_0 {
    let mut i: i32_0 = 0;
    let mut c: i32_0 = 0;
    k = k.offset(1);
    c = 0 as ::core::ffi::c_int as i32_0;
    i = 0 as ::core::ffi::c_int as i32_0;
    while i < 4 as ::core::ffi::c_int {
        c <<= 4 as ::core::ffi::c_int;
        if '0' as i32 as Rune <= *k && *k <= '9' as i32 as Rune {
            c = (c as Rune).wrapping_add((*k).wrapping_sub('0' as i32 as Rune)) as i32_0 as i32_0;
        } else if 'a' as i32 as Rune <= *k && *k <= 'f' as i32 as Rune {
            c = (c as Rune).wrapping_add(
                (10 as Rune)
                    .wrapping_add(*k)
                    .wrapping_sub('a' as i32 as Rune),
            ) as i32_0 as i32_0;
        } else if 'A' as i32 as Rune <= *k && *k <= 'F' as i32 as Rune {
            c = (c as Rune).wrapping_add(
                (10 as Rune)
                    .wrapping_add(*k)
                    .wrapping_sub('A' as i32 as Rune),
            ) as i32_0 as i32_0;
        } else {
            return -(1 as i32_0);
        }
        i += 1;
        k = k.offset(1);
    }
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn latin1(mut k: *mut Rune, mut n: ::core::ffi::c_int) -> i32_0 {
    let mut l: *mut cvlist = ::core::ptr::null_mut::<cvlist>();
    let mut c: ::core::ffi::c_int = 0;
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if *k.offset(0 as ::core::ffi::c_int as isize) == 'X' as i32 as Rune {
        if n >= 5 as ::core::ffi::c_int {
            return unicode(k);
        } else {
            return -(5 as i32_0);
        }
    }
    let mut current_block_16: u64;
    l = &raw mut latintab as *mut cvlist as *mut cvlist;
    while !(*l).ld.is_null() {
        if *k.offset(0 as ::core::ffi::c_int as isize)
            == *(*l).ld.offset(0 as ::core::ffi::c_int as isize) as Rune
        {
            if n == 1 as ::core::ffi::c_int {
                return -(2 as i32_0);
            }
            if *(*l).ld.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                c = *k.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
                current_block_16 = 11050875288958768710;
            } else if *(*l).ld.offset(1 as ::core::ffi::c_int as isize) as Rune
                != *k.offset(1 as ::core::ffi::c_int as isize)
            {
                current_block_16 = 7095457783677275021;
            } else {
                if n == 2 as ::core::ffi::c_int {
                    return -(3 as i32_0);
                } else {
                    c = *k.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
                }
                current_block_16 = 11050875288958768710;
            }
            match current_block_16 {
                7095457783677275021 => {}
                _ => {
                    p = (*l).si;
                    while *p as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                        if *p as ::core::ffi::c_int == c {
                            return *(*l)
                                .so
                                .offset(p.offset_from((*l).si) as ::core::ffi::c_long as isize);
                        }
                        p = p.offset(1);
                    }
                    return -(1 as i32_0);
                }
            }
        }
        l = l.offset(1);
    }
    return -(1 as i32_0);
}
