use std::ffi::CStr;

use ::libc;
extern "C" {
    fn c_strcasecmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
    fn gettext(__msgid: *const libc::c_char) -> *mut libc::c_char;
    fn locale_charset() -> *const libc::c_char;
}
#[no_mangle]
pub fn proper_name_lite<'a>(
    name_ascii: &'a str,
    name_utf8: &'a str,
) -> String {
    let translation = unsafe { CStr::from_ptr(gettext(name_ascii.as_ptr() as *const libc::c_char)) };
    if translation.as_ptr() != name_ascii.as_ptr() as *const libc::c_char {
        return translation.to_string_lossy().into_owned();
    } else if unsafe { c_strcasecmp(locale_charset(), b"UTF-8\0".as_ptr() as *const libc::c_char) } == 0 {
        return name_utf8.to_string();
    } else {
        return name_ascii.to_string();
    }
}

