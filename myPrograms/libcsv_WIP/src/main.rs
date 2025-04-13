#![feature(extern_types)]
#![feature(label_break_value)]
















use std::mem;

use std::io::{self, Write};

use std::ptr::null_mut;

use std::os::raw::c_void;

use std::slice;
use std::convert::TryInto;

use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct csv_parser {
    pub pstate: libc::c_int,
    pub quoted: libc::c_int,
    pub spaces: size_t,
    pub entry_buf: *mut libc::c_uchar,
    pub entry_pos: size_t,
    pub entry_size: size_t,
    pub status: libc::c_int,
    pub options: libc::c_uchar,
    pub quote_char: libc::c_uchar,
    pub delim_char: libc::c_uchar,
    pub is_space: Option::<unsafe extern "C" fn(libc::c_uchar) -> libc::c_int>,
    pub is_term: Option::<unsafe extern "C" fn(libc::c_uchar) -> libc::c_int>,
    pub blk_size: size_t,
    pub malloc_func: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    pub realloc_func: Option::<
        unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
    >,
    pub free_func: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
static mut csv_errors: [*const libc::c_char; 5] = [
    b"success\0" as *const u8 as *const libc::c_char,
    b"error parsing data while strict checking enabled\0" as *const u8
        as *const libc::c_char,
    b"memory exhausted while increasing buffer size\0" as *const u8
        as *const libc::c_char,
    b"data size too large\0" as *const u8 as *const libc::c_char,
    b"invalid status code\0" as *const u8 as *const libc::c_char,
];
#[no_mangle]
pub fn csv_error(p: &csv_parser) -> libc::c_int {
    p.status
}

#[no_mangle]
pub fn csv_strerror(status: i32) -> &'static str {
    let errors = [
        "success",
        "error parsing data while strict checking enabled",
        "memory exhausted while increasing buffer size",
        "data size too large",
        "invalid status code",
    ];
    if status >= 0 && status < 4 {
        errors[status as usize]
    } else {
        errors[4]
    }
}

#[no_mangle]
pub fn csv_get_opts(p: Option<&csv_parser>) -> i32 {
    match p {
        Some(parser) => parser.options as i32,
        None => -1,
    }
}

#[no_mangle]
pub fn csv_set_opts(parser: &mut csv_parser, options: u8) -> i32 {
    parser.options = options;
    0
}

#[no_mangle]
pub fn csv_init(p: &mut csv_parser, options: u8) -> i32 {
    p.entry_buf = std::ptr::null_mut();
    p.pstate = 0;
    p.quoted = 0;
    p.spaces = 0;
    p.entry_pos = 0;
    p.entry_size = 0;
    p.status = 0;
    p.options = options;
    p.quote_char = b'"';
    p.delim_char = b',';
    p.is_space = None;
    p.is_term = None;
    p.blk_size = 128;
    p.malloc_func = None;
    p.realloc_func = Some(realloc);
    p.free_func = Some(free);
    0
}

#[no_mangle]
pub fn csv_free(parser: Option<&mut csv_parser>) {
    if let Some(p) = parser {
        let buf = mem::replace(&mut p.entry_buf, std::ptr::null_mut());
        if !buf.is_null() {
            if let Some(free_func) = p.free_func.take() {
                unsafe {
                    free_func(buf as *mut c_void);
                }
            }
        }
        p.entry_size = 0;
    }
}

#[no_mangle]
pub unsafe extern "C" fn csv_fini(
    mut p: *mut csv_parser,
    mut cb1: Option::<
        unsafe extern "C" fn(*mut libc::c_void, size_t, *mut libc::c_void) -> (),
    >,
    mut cb2: Option::<unsafe extern "C" fn(libc::c_int, *mut libc::c_void) -> ()>,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    if p.is_null() {
        return -(1 as libc::c_int);
    }
    let mut quoted = (*p).quoted;
    let mut pstate = (*p).pstate;
    let mut spaces = (*p).spaces;
    let mut entry_pos = (*p).entry_pos;
    if pstate == 2 as libc::c_int && (*p).quoted != 0
        && (*p).options as libc::c_int & 1 as libc::c_int != 0
        && (*p).options as libc::c_int & 4 as libc::c_int != 0
    {
        (*p).status = 1 as libc::c_int;
        return -(1 as libc::c_int);
    }
    let mut current_block_26: u64;
    match pstate {
        3 => {
            (*p)
                .entry_pos = ((*p).entry_pos as libc::c_ulong)
                .wrapping_sub(
                    ((*p).spaces).wrapping_add(1 as libc::c_int as libc::c_ulong),
                ) as size_t as size_t;
            entry_pos = (*p).entry_pos;
            current_block_26 = 186392437941418340;
        }
        1 | 2 => {
            current_block_26 = 186392437941418340;
        }
        0 | _ => {
            current_block_26 = 15768484401365413375;
        }
    }
    match current_block_26 {
        186392437941418340 => {
            if quoted == 0 {
                entry_pos = (entry_pos as libc::c_ulong).wrapping_sub(spaces) as size_t
                    as size_t;
            }
            if (*p).options as libc::c_int & 8 as libc::c_int != 0 {
                *((*p).entry_buf)
                    .offset(entry_pos as isize) = '\0' as i32 as libc::c_uchar;
            }
            if cb1.is_some() && (*p).options as libc::c_int & 16 as libc::c_int != 0
                && quoted == 0 && entry_pos == 0 as libc::c_int as libc::c_ulong
            {
                cb1
                    .expect(
                        "non-null function pointer",
                    )(0 as *mut libc::c_void, entry_pos, data);
            } else if cb1.is_some() {
                cb1
                    .expect(
                        "non-null function pointer",
                    )((*p).entry_buf as *mut libc::c_void, entry_pos, data);
            }
            pstate = 1 as libc::c_int;
            spaces = 0 as libc::c_int as size_t;
            quoted = spaces as libc::c_int;
            entry_pos = quoted as size_t;
            if cb2.is_some() {
                cb2.expect("non-null function pointer")(-(1 as libc::c_int), data);
            }
            pstate = 0 as libc::c_int;
            spaces = 0 as libc::c_int as size_t;
            quoted = spaces as libc::c_int;
            entry_pos = quoted as size_t;
        }
        _ => {}
    }
    (*p).status = 0 as libc::c_int;
    (*p).entry_pos = (*p).status as size_t;
    (*p).quoted = (*p).entry_pos as libc::c_int;
    (*p).spaces = (*p).quoted as size_t;
    (*p).pstate = 0 as libc::c_int;
    return 0 as libc::c_int;
}
#[no_mangle]
pub fn csv_set_delim(parser: Option<&mut csv_parser>, c: u8) {
    if let Some(p) = parser {
        p.delim_char = c;
    }
}

#[no_mangle]
pub fn csv_set_quote(p: &mut csv_parser, c: u8) {
    p.quote_char = c;
}

#[no_mangle]
pub fn csv_get_delim(p: &csv_parser) -> libc::c_uchar {
    p.delim_char
}

#[no_mangle]
pub fn csv_get_quote(p: &csv_parser) -> libc::c_uchar {
    p.quote_char
}

#[no_mangle]
pub fn csv_set_space_func(parser: Option<&mut csv_parser>, f: Option<unsafe extern "C" fn(u8) -> i32>) {
    if let Some(p) = parser {
        p.is_space = f;
    }
}

#[no_mangle]
pub fn csv_set_term_func(parser: Option<&mut csv_parser>, term_fn: Option<unsafe extern "C" fn(u8) -> i32>) {
    if let Some(p) = parser {
        p.is_term = term_fn;
    }
}

#[no_mangle]
pub fn csv_set_realloc_func(
    parser: Option<&mut csv_parser>,
    f: Option<unsafe extern "C" fn(*mut c_void, u64) -> *mut c_void>,
) {
    if let (Some(parser), Some(func)) = (parser, f) {
        parser.realloc_func = Some(func);
    }
}

#[no_mangle]
pub fn csv_set_free_func(p: &mut csv_parser, f: Option<unsafe extern "C" fn(*mut c_void) -> ()>) {
    if let Some(func) = f {
        p.free_func = Some(func);
    }
}

#[no_mangle]
pub fn csv_set_blk_size(p: Option<&mut csv_parser>, size: usize) {
    if let Some(parser) = p {
        parser.blk_size = size as u64;
    }
}

#[no_mangle]
pub fn csv_get_buffer_size(parser: Option<&csv_parser>) -> usize {
    match parser {
        Some(parser) => parser.entry_size.try_into().unwrap(),
        None => 0,
    }
}

unsafe extern "C" fn csv_increase_buffer(mut p: *mut csv_parser) -> libc::c_int {
    if p.is_null() {
        return 0 as libc::c_int;
    }
    if ((*p).realloc_func).is_none() {
        return 0 as libc::c_int;
    }
    let mut to_add = (*p).blk_size;
    let mut vp = 0 as *mut libc::c_void;
    if (*p).entry_size >= (18446744073709551615 as libc::c_ulong).wrapping_sub(to_add) {
        to_add = (18446744073709551615 as libc::c_ulong).wrapping_sub((*p).entry_size);
    }
    if to_add == 0 {
        (*p).status = 3 as libc::c_int;
        return -(1 as libc::c_int);
    }
    loop {
        vp = ((*p).realloc_func)
            .expect(
                "non-null function pointer",
            )(
            (*p).entry_buf as *mut libc::c_void,
            ((*p).entry_size).wrapping_add(to_add),
        );
        if !vp.is_null() {
            break;
        }
        to_add = (to_add as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
        if to_add == 0 {
            (*p).status = 2 as libc::c_int;
            return -(1 as libc::c_int);
        }
    }
    (*p).entry_buf = vp as *mut libc::c_uchar;
    (*p)
        .entry_size = ((*p).entry_size as libc::c_ulong).wrapping_add(to_add) as size_t
        as size_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn csv_parse(
    mut p: *mut csv_parser,
    mut s: *const libc::c_void,
    mut len: size_t,
    mut cb1: Option::<
        unsafe extern "C" fn(*mut libc::c_void, size_t, *mut libc::c_void) -> (),
    >,
    mut cb2: Option::<unsafe extern "C" fn(libc::c_int, *mut libc::c_void) -> ()>,
    mut data: *mut libc::c_void,
) -> size_t {
    if !p.is_null()
        && !(b"received null csv_parser\0" as *const u8 as *const libc::c_char).is_null()
    {} else {
        __assert_fail(
            b"p && \"received null csv_parser\"\0" as *const u8 as *const libc::c_char,
            b"/workspace/programs/libcsv/libcsv.c\0" as *const u8 as *const libc::c_char,
            321 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 125],
                &[libc::c_char; 125],
            >(
                b"size_t csv_parse(struct csv_parser *, const void *, size_t, void (*)(void *, size_t, void *), void (*)(int, void *), void *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3814: {
        if !p.is_null()
            && !(b"received null csv_parser\0" as *const u8 as *const libc::c_char)
                .is_null()
        {} else {
            __assert_fail(
                b"p && \"received null csv_parser\"\0" as *const u8
                    as *const libc::c_char,
                b"/workspace/programs/libcsv/libcsv.c\0" as *const u8
                    as *const libc::c_char,
                321 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 125],
                    &[libc::c_char; 125],
                >(
                    b"size_t csv_parse(struct csv_parser *, const void *, size_t, void (*)(void *, size_t, void *), void (*)(int, void *), void *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if s.is_null() {
        return 0 as libc::c_int as size_t;
    }
    let mut us = s as *const libc::c_uchar;
    let mut c: libc::c_uchar = 0;
    let mut pos = 0 as libc::c_int as size_t;
    let mut delim = (*p).delim_char;
    let mut quote = (*p).quote_char;
    let mut is_space: Option::<unsafe extern "C" fn(libc::c_uchar) -> libc::c_int> = (*p)
        .is_space;
    let mut is_term: Option::<unsafe extern "C" fn(libc::c_uchar) -> libc::c_int> = (*p)
        .is_term;
    let mut quoted = (*p).quoted;
    let mut pstate = (*p).pstate;
    let mut spaces = (*p).spaces;
    let mut entry_pos = (*p).entry_pos;
    if ((*p).entry_buf).is_null() && pos < len {
        if csv_increase_buffer(p) != 0 as libc::c_int {
            (*p).quoted = quoted;
            (*p).pstate = pstate;
            (*p).spaces = spaces;
            (*p).entry_pos = entry_pos;
            return pos;
        }
    }
    while pos < len {
        if entry_pos
            == (if (*p).options as libc::c_int & 8 as libc::c_int != 0 {
                ((*p).entry_size).wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                (*p).entry_size
            })
        {
            if csv_increase_buffer(p) != 0 as libc::c_int {
                (*p).quoted = quoted;
                (*p).pstate = pstate;
                (*p).spaces = spaces;
                (*p).entry_pos = entry_pos;
                return pos;
            }
        }
        let fresh0 = pos;
        pos = pos.wrapping_add(1);
        c = *us.offset(fresh0 as isize);
        match pstate {
            0 | 1 => {
                if (if is_space.is_some() {
                    is_space.expect("non-null function pointer")(c)
                } else {
                    (c as libc::c_int == 0x20 as libc::c_int
                        || c as libc::c_int == 0x9 as libc::c_int) as libc::c_int
                }) != 0 && c as libc::c_int != delim as libc::c_int
                {
                    continue;
                }
                if if is_term.is_some() {
                    is_term.expect("non-null function pointer")(c)
                } else {
                    (c as libc::c_int == 0xd as libc::c_int
                        || c as libc::c_int == 0xa as libc::c_int) as libc::c_int
                } != 0
                {
                    if pstate == 1 as libc::c_int {
                        if quoted == 0 {
                            entry_pos = (entry_pos as libc::c_ulong).wrapping_sub(spaces)
                                as size_t as size_t;
                        }
                        if (*p).options as libc::c_int & 8 as libc::c_int != 0 {
                            *((*p).entry_buf)
                                .offset(entry_pos as isize) = '\0' as i32 as libc::c_uchar;
                        }
                        if cb1.is_some()
                            && (*p).options as libc::c_int & 16 as libc::c_int != 0
                            && quoted == 0
                            && entry_pos == 0 as libc::c_int as libc::c_ulong
                        {
                            cb1
                                .expect(
                                    "non-null function pointer",
                                )(0 as *mut libc::c_void, entry_pos, data);
                        } else if cb1.is_some() {
                            cb1
                                .expect(
                                    "non-null function pointer",
                                )((*p).entry_buf as *mut libc::c_void, entry_pos, data);
                        }
                        pstate = 1 as libc::c_int;
                        spaces = 0 as libc::c_int as size_t;
                        quoted = spaces as libc::c_int;
                        entry_pos = quoted as size_t;
                        if cb2.is_some() {
                            cb2
                                .expect(
                                    "non-null function pointer",
                                )(c as libc::c_int, data);
                        }
                        pstate = 0 as libc::c_int;
                        spaces = 0 as libc::c_int as size_t;
                        quoted = spaces as libc::c_int;
                        entry_pos = quoted as size_t;
                    } else if (*p).options as libc::c_int & 2 as libc::c_int != 0 {
                        if cb2.is_some() {
                            cb2
                                .expect(
                                    "non-null function pointer",
                                )(c as libc::c_int, data);
                        }
                        pstate = 0 as libc::c_int;
                        spaces = 0 as libc::c_int as size_t;
                        quoted = spaces as libc::c_int;
                        entry_pos = quoted as size_t;
                    }
                } else if c as libc::c_int == delim as libc::c_int {
                    if quoted == 0 {
                        entry_pos = (entry_pos as libc::c_ulong).wrapping_sub(spaces)
                            as size_t as size_t;
                    }
                    if (*p).options as libc::c_int & 8 as libc::c_int != 0 {
                        *((*p).entry_buf)
                            .offset(entry_pos as isize) = '\0' as i32 as libc::c_uchar;
                    }
                    if cb1.is_some()
                        && (*p).options as libc::c_int & 16 as libc::c_int != 0
                        && quoted == 0 && entry_pos == 0 as libc::c_int as libc::c_ulong
                    {
                        cb1
                            .expect(
                                "non-null function pointer",
                            )(0 as *mut libc::c_void, entry_pos, data);
                    } else if cb1.is_some() {
                        cb1
                            .expect(
                                "non-null function pointer",
                            )((*p).entry_buf as *mut libc::c_void, entry_pos, data);
                    }
                    pstate = 1 as libc::c_int;
                    spaces = 0 as libc::c_int as size_t;
                    quoted = spaces as libc::c_int;
                    entry_pos = quoted as size_t;
                } else if c as libc::c_int == quote as libc::c_int {
                    pstate = 2 as libc::c_int;
                    quoted = 1 as libc::c_int;
                } else {
                    pstate = 2 as libc::c_int;
                    quoted = 0 as libc::c_int;
                    let fresh1 = entry_pos;
                    entry_pos = entry_pos.wrapping_add(1);
                    *((*p).entry_buf).offset(fresh1 as isize) = c;
                }
            }
            2 => {
                if c as libc::c_int == quote as libc::c_int {
                    if quoted != 0 {
                        let fresh2 = entry_pos;
                        entry_pos = entry_pos.wrapping_add(1);
                        *((*p).entry_buf).offset(fresh2 as isize) = c;
                        pstate = 3 as libc::c_int;
                    } else {
                        if (*p).options as libc::c_int & 1 as libc::c_int != 0 {
                            (*p).status = 1 as libc::c_int;
                            (*p).quoted = quoted;
                            (*p).pstate = pstate;
                            (*p).spaces = spaces;
                            (*p).entry_pos = entry_pos;
                            return pos.wrapping_sub(1 as libc::c_int as libc::c_ulong);
                        }
                        let fresh3 = entry_pos;
                        entry_pos = entry_pos.wrapping_add(1);
                        *((*p).entry_buf).offset(fresh3 as isize) = c;
                        spaces = 0 as libc::c_int as size_t;
                    }
                } else if c as libc::c_int == delim as libc::c_int {
                    if quoted != 0 {
                        let fresh4 = entry_pos;
                        entry_pos = entry_pos.wrapping_add(1);
                        *((*p).entry_buf).offset(fresh4 as isize) = c;
                    } else {
                        if quoted == 0 {
                            entry_pos = (entry_pos as libc::c_ulong).wrapping_sub(spaces)
                                as size_t as size_t;
                        }
                        if (*p).options as libc::c_int & 8 as libc::c_int != 0 {
                            *((*p).entry_buf)
                                .offset(entry_pos as isize) = '\0' as i32 as libc::c_uchar;
                        }
                        if cb1.is_some()
                            && (*p).options as libc::c_int & 16 as libc::c_int != 0
                            && quoted == 0
                            && entry_pos == 0 as libc::c_int as libc::c_ulong
                        {
                            cb1
                                .expect(
                                    "non-null function pointer",
                                )(0 as *mut libc::c_void, entry_pos, data);
                        } else if cb1.is_some() {
                            cb1
                                .expect(
                                    "non-null function pointer",
                                )((*p).entry_buf as *mut libc::c_void, entry_pos, data);
                        }
                        pstate = 1 as libc::c_int;
                        spaces = 0 as libc::c_int as size_t;
                        quoted = spaces as libc::c_int;
                        entry_pos = quoted as size_t;
                    }
                } else if if is_term.is_some() {
                    is_term.expect("non-null function pointer")(c)
                } else {
                    (c as libc::c_int == 0xd as libc::c_int
                        || c as libc::c_int == 0xa as libc::c_int) as libc::c_int
                } != 0
                {
                    if quoted == 0 {
                        if quoted == 0 {
                            entry_pos = (entry_pos as libc::c_ulong).wrapping_sub(spaces)
                                as size_t as size_t;
                        }
                        if (*p).options as libc::c_int & 8 as libc::c_int != 0 {
                            *((*p).entry_buf)
                                .offset(entry_pos as isize) = '\0' as i32 as libc::c_uchar;
                        }
                        if cb1.is_some()
                            && (*p).options as libc::c_int & 16 as libc::c_int != 0
                            && quoted == 0
                            && entry_pos == 0 as libc::c_int as libc::c_ulong
                        {
                            cb1
                                .expect(
                                    "non-null function pointer",
                                )(0 as *mut libc::c_void, entry_pos, data);
                        } else if cb1.is_some() {
                            cb1
                                .expect(
                                    "non-null function pointer",
                                )((*p).entry_buf as *mut libc::c_void, entry_pos, data);
                        }
                        pstate = 1 as libc::c_int;
                        spaces = 0 as libc::c_int as size_t;
                        quoted = spaces as libc::c_int;
                        entry_pos = quoted as size_t;
                        if cb2.is_some() {
                            cb2
                                .expect(
                                    "non-null function pointer",
                                )(c as libc::c_int, data);
                        }
                        pstate = 0 as libc::c_int;
                        spaces = 0 as libc::c_int as size_t;
                        quoted = spaces as libc::c_int;
                        entry_pos = quoted as size_t;
                    } else {
                        let fresh5 = entry_pos;
                        entry_pos = entry_pos.wrapping_add(1);
                        *((*p).entry_buf).offset(fresh5 as isize) = c;
                    }
                } else if quoted == 0
                    && (if is_space.is_some() {
                        is_space.expect("non-null function pointer")(c)
                    } else {
                        (c as libc::c_int == 0x20 as libc::c_int
                            || c as libc::c_int == 0x9 as libc::c_int) as libc::c_int
                    }) != 0
                {
                    let fresh6 = entry_pos;
                    entry_pos = entry_pos.wrapping_add(1);
                    *((*p).entry_buf).offset(fresh6 as isize) = c;
                    spaces = spaces.wrapping_add(1);
                    spaces;
                } else {
                    let fresh7 = entry_pos;
                    entry_pos = entry_pos.wrapping_add(1);
                    *((*p).entry_buf).offset(fresh7 as isize) = c;
                    spaces = 0 as libc::c_int as size_t;
                }
            }
            3 => {
                if c as libc::c_int == delim as libc::c_int {
                    entry_pos = (entry_pos as libc::c_ulong)
                        .wrapping_sub(
                            spaces.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as size_t as size_t;
                    if quoted == 0 {
                        entry_pos = (entry_pos as libc::c_ulong).wrapping_sub(spaces)
                            as size_t as size_t;
                    }
                    if (*p).options as libc::c_int & 8 as libc::c_int != 0 {
                        *((*p).entry_buf)
                            .offset(entry_pos as isize) = '\0' as i32 as libc::c_uchar;
                    }
                    if cb1.is_some()
                        && (*p).options as libc::c_int & 16 as libc::c_int != 0
                        && quoted == 0 && entry_pos == 0 as libc::c_int as libc::c_ulong
                    {
                        cb1
                            .expect(
                                "non-null function pointer",
                            )(0 as *mut libc::c_void, entry_pos, data);
                    } else if cb1.is_some() {
                        cb1
                            .expect(
                                "non-null function pointer",
                            )((*p).entry_buf as *mut libc::c_void, entry_pos, data);
                    }
                    pstate = 1 as libc::c_int;
                    spaces = 0 as libc::c_int as size_t;
                    quoted = spaces as libc::c_int;
                    entry_pos = quoted as size_t;
                } else if if is_term.is_some() {
                    is_term.expect("non-null function pointer")(c)
                } else {
                    (c as libc::c_int == 0xd as libc::c_int
                        || c as libc::c_int == 0xa as libc::c_int) as libc::c_int
                } != 0
                {
                    entry_pos = (entry_pos as libc::c_ulong)
                        .wrapping_sub(
                            spaces.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as size_t as size_t;
                    if quoted == 0 {
                        entry_pos = (entry_pos as libc::c_ulong).wrapping_sub(spaces)
                            as size_t as size_t;
                    }
                    if (*p).options as libc::c_int & 8 as libc::c_int != 0 {
                        *((*p).entry_buf)
                            .offset(entry_pos as isize) = '\0' as i32 as libc::c_uchar;
                    }
                    if cb1.is_some()
                        && (*p).options as libc::c_int & 16 as libc::c_int != 0
                        && quoted == 0 && entry_pos == 0 as libc::c_int as libc::c_ulong
                    {
                        cb1
                            .expect(
                                "non-null function pointer",
                            )(0 as *mut libc::c_void, entry_pos, data);
                    } else if cb1.is_some() {
                        cb1
                            .expect(
                                "non-null function pointer",
                            )((*p).entry_buf as *mut libc::c_void, entry_pos, data);
                    }
                    pstate = 1 as libc::c_int;
                    spaces = 0 as libc::c_int as size_t;
                    quoted = spaces as libc::c_int;
                    entry_pos = quoted as size_t;
                    if cb2.is_some() {
                        cb2.expect("non-null function pointer")(c as libc::c_int, data);
                    }
                    pstate = 0 as libc::c_int;
                    spaces = 0 as libc::c_int as size_t;
                    quoted = spaces as libc::c_int;
                    entry_pos = quoted as size_t;
                } else if if is_space.is_some() {
                    is_space.expect("non-null function pointer")(c)
                } else {
                    (c as libc::c_int == 0x20 as libc::c_int
                        || c as libc::c_int == 0x9 as libc::c_int) as libc::c_int
                } != 0
                {
                    let fresh8 = entry_pos;
                    entry_pos = entry_pos.wrapping_add(1);
                    *((*p).entry_buf).offset(fresh8 as isize) = c;
                    spaces = spaces.wrapping_add(1);
                    spaces;
                } else if c as libc::c_int == quote as libc::c_int {
                    if spaces != 0 {
                        if (*p).options as libc::c_int & 1 as libc::c_int != 0 {
                            (*p).status = 1 as libc::c_int;
                            (*p).quoted = quoted;
                            (*p).pstate = pstate;
                            (*p).spaces = spaces;
                            (*p).entry_pos = entry_pos;
                            return pos.wrapping_sub(1 as libc::c_int as libc::c_ulong);
                        }
                        spaces = 0 as libc::c_int as size_t;
                        let fresh9 = entry_pos;
                        entry_pos = entry_pos.wrapping_add(1);
                        *((*p).entry_buf).offset(fresh9 as isize) = c;
                    } else {
                        pstate = 2 as libc::c_int;
                    }
                } else {
                    if (*p).options as libc::c_int & 1 as libc::c_int != 0 {
                        (*p).status = 1 as libc::c_int;
                        (*p).quoted = quoted;
                        (*p).pstate = pstate;
                        (*p).spaces = spaces;
                        (*p).entry_pos = entry_pos;
                        return pos.wrapping_sub(1 as libc::c_int as libc::c_ulong);
                    }
                    pstate = 2 as libc::c_int;
                    spaces = 0 as libc::c_int as size_t;
                    let fresh10 = entry_pos;
                    entry_pos = entry_pos.wrapping_add(1);
                    *((*p).entry_buf).offset(fresh10 as isize) = c;
                }
            }
            _ => {}
        }
    }
    (*p).quoted = quoted;
    (*p).pstate = pstate;
    (*p).spaces = spaces;
    (*p).entry_pos = entry_pos;
    return pos;
}
#[no_mangle]
pub fn csv_write(dest: &mut [u8], src: &[u8]) -> usize {
    // 0x22 is the ASCII code for the double quote character (")
    csv_write2(Some(dest), src, b'"').try_into().unwrap()
}

#[no_mangle]
pub fn csv_fwrite<W: Write>(fp: &mut W, src: &[u8]) -> std::io::Result<()> {
    // 0x22 is the ASCII code for the double quote, now given as 34u8.
    csv_fwrite2(fp, src, 34u8)
}

#[no_mangle]
pub fn csv_write2(mut dest: Option<&mut [u8]>, src: &[u8], quote: u8) -> u64 {
    let dest_len = dest.as_ref().map_or(0, |d| d.len());
    let mut count: usize = 0;

    // Write the leading quote.
    if dest_len > count {
        if let Some(buf) = dest.as_mut() {
            buf[count] = quote;
        }
    }
    count += 1;

    // Process each byte in src.
    for &byte in src {
        // If the byte equals the quote, output an extra quote.
        if byte == quote {
            if dest_len > count {
                if let Some(buf) = dest.as_mut() {
                    buf[count] = quote;
                }
            }
            count += 1;
        }
        if dest_len > count {
            if let Some(buf) = dest.as_mut() {
                buf[count] = byte;
            }
        }
        count += 1;
    }

    // Write the trailing quote.
    if dest_len > count {
        if let Some(buf) = dest.as_mut() {
            buf[count] = quote;
        }
    }
    count += 1;

    count
        .try_into()
        .expect("Conversion from usize to u64 failed")
}

#[no_mangle]
/// A safe Rust translation of csv_fwrite2.
/// This function writes a CSV cell by surrounding the contents with a quote and doubling
/// any occurrence of the quote inside the content.
/// It writes to a writer that implements std::io::Write and returns io::Result<()>.
pub fn csv_fwrite2(fp: &mut impl Write, src: &[u8], quote: u8) -> io::Result<()> {
    // Write the opening quote.
    fp.write_all(&[quote])?;
    // For each byte in src, write an extra quote if it equals the quote, then write the byte.
    for &byte in src {
        if byte == quote {
            fp.write_all(&[quote])?;
        }
        fp.write_all(&[byte])?;
    }
    // Write the closing quote.
    fp.write_all(&[quote])?;
    Ok(())
}

/// A wrapper around a C FILE pointer that implements std::io::Write.
/// This encapsulates the unsafe FFI calls to the C API (fputc) inside safe methods.
pub struct FileWrapper {
    pub fp: *mut libc::FILE,
}

impl Write for FileWrapper {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut written = 0;
        // Write each byte using fputc.
        for &byte in buf {
            // Safety: We assume that self.fp is a valid pointer to a C FILE.
            let ret = unsafe { libc::fputc(byte as libc::c_int, self.fp) };
            if ret == libc::EOF {
                return Err(io::Error::last_os_error());
            }
            written += 1;
        }
        Ok(written)
    }
    fn flush(&mut self) -> io::Result<()> {
        // Flushing is managed externally.
        Ok(())
    }
}


fn main() {
    // function body (currently empty)
}
