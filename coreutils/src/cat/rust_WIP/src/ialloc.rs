use std::option::Option;

use std::vec::Vec;

use std::vec;

use std::alloc;
use std::ptr;

use ::libc;
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn reallocarray(
        __ptr: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
    ) -> *mut libc::c_void;
}
pub type ptrdiff_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type idx_t = ptrdiff_t;
#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn ireallocarray(
    p: &mut Vec<u8>,
    n: idx_t,
    s: idx_t,
) -> *mut libc::c_void {
    if n as usize <= usize::MAX && s as usize <= usize::MAX {
        let mut nx: usize = n as usize;
        let mut sx: usize = s as usize;
        if n == 0 || s == 0 {
            sx = 1;
            nx = sx;
        }
        p.resize(nx * sx, 0);
        p.as_mut_ptr() as *mut libc::c_void
    } else {
        unsafe { _gl_alloc_nomem() }
    }
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn icalloc(n: usize, s: usize) -> Option<Vec<u8>> {
    if n > usize::MAX / s {
        return None; // Simulating _gl_alloc_nomem()
    }
    
    let total_size = n.checked_mul(s)?;
    let vec = vec![0u8; total_size];
    Some(vec)
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn irealloc(p: Option<Vec<u8>>, s: usize) -> Option<Vec<u8>> {
    if s <= usize::MAX {
        let mut vec = p.unwrap_or_else(Vec::new);
        vec.resize(s, 0);
        Some(vec)
    } else {
        return None; // Assuming _gl_alloc_nomem() returns None in this context
    }
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn imalloc(s: usize) -> Option<Box<[u8]>> {
    if s <= usize::MAX {
        let layout = std::alloc::Layout::from_size_align(s, 1).ok()?;
        let ptr = unsafe { std::alloc::alloc(layout) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { Box::from_raw(std::slice::from_raw_parts_mut(ptr, s)) })
        }
    } else {
        None // Return None instead of calling _gl_alloc_nomem()
    }
}

#[no_mangle]
#[cold]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn _gl_alloc_nomem() -> *mut libc::c_void {
    *__errno_location() = 12 as libc::c_int;
    return 0 as *mut libc::c_void;
}
