use std::ffi::CStr;

use std::slice;

use ::libc;
extern "C" {
    fn setlocale_null_r_unlocked(
        category: libc::c_int,
        buf: *mut libc::c_char,
        bufsize: size_t,
    ) -> libc::c_int;
    fn setlocale_null_unlocked(category: libc::c_int) -> *const libc::c_char;
}
pub type size_t = libc::c_ulong;
#[no_mangle]
pub fn setlocale_null_r(
    category: libc::c_int,
    buf: &mut [libc::c_char],
) -> libc::c_int {
    let bufsize = buf.len() as size_t;
    unsafe {
        return setlocale_null_r_unlocked(category, buf.as_mut_ptr(), bufsize);
    }
}

#[no_mangle]
pub fn setlocale_null(category: libc::c_int) -> Option<String> {
    // Call the unsafe function within an unsafe block
    let locale_ptr = unsafe { setlocale_null_unlocked(category) };
    if locale_ptr.is_null() {
        None
    } else {
        // Convert the C string to a Rust String
        let c_str = unsafe { std::ffi::CStr::from_ptr(locale_ptr) };
        c_str.to_str().ok().map(|s| s.to_string())
    }
}

