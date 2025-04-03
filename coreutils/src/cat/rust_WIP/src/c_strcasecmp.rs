
use std::cmp::min;

use ::libc;
#[inline]
fn c_tolower(c: i32) -> i32 {
    match c {
        65..=90 => c - 'A' as i32 + 'a' as i32,
        _ => c,
    }
}

#[no_mangle]
pub fn c_strcasecmp(s1: &str, s2: &str) -> libc::c_int {
    let s1_bytes = s1.as_bytes();
    let s2_bytes = s2.as_bytes();
    let len = s1_bytes.len().min(s2_bytes.len());

    for i in 0..len {
        let c1 = c_tolower(s1_bytes[i] as libc::c_int) as libc::c_uchar;
        let c2 = c_tolower(s2_bytes[i] as libc::c_int) as libc::c_uchar;

        if c1 == b'\0' {
            return 0;
        }

        if c1 != c2 {
            return c1 as libc::c_int - c2 as libc::c_int;
        }
    }

    s1_bytes.len() as libc::c_int - s2_bytes.len() as libc::c_int
}

