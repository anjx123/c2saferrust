use std::ptr;

use ::libc;
extern "C" {
    fn copy_file_range(
        __infd: libc::c_int,
        __pinoff: *mut __off64_t,
        __outfd: libc::c_int,
        __poutoff: *mut __off64_t,
        __length: size_t,
        __flags: libc::c_uint,
    ) -> ssize_t;
    fn __errno_location() -> *mut libc::c_int;
    fn uname(__name: *mut utsname) -> libc::c_int;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
pub type off_t = __off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utsname {
    pub sysname: [libc::c_char; 65],
    pub nodename: [libc::c_char; 65],
    pub release: [libc::c_char; 65],
    pub version: [libc::c_char; 65],
    pub machine: [libc::c_char; 65],
    pub domainname: [libc::c_char; 65],
}
#[no_mangle]
pub fn rpl_copy_file_range(
    infd: libc::c_int,
    pinoff: &mut off_t,
    outfd: libc::c_int,
    poutoff: &mut off_t,
    length: size_t,
    flags: libc::c_uint,
) -> ssize_t {
    static mut ok: libc::c_schar = 0;

    unsafe {
        if ok == 0 {
            let mut name: utsname = utsname {
                sysname: [0; 65],
                nodename: [0; 65],
                release: [0; 65],
                version: [0; 65],
                machine: [0; 65],
                domainname: [0; 65],
            };
            uname(&mut name);
            let p = name.release.as_ptr();
            ok = if *p.offset(1) != '.' as i8
                || ('5' as i8) < *p
                || (*p == '5' as i8 && (*p.offset(3) != '.' as i8 || ('2' as i8) < *p.offset(2)))
            {
                1
            } else {
                -1
            } as libc::c_schar;
        }
        if ok > 0 {
            return copy_file_range(infd, pinoff, outfd, poutoff, length, flags);
        }
        *__errno_location() = 38;
    }
    return -1;
}

