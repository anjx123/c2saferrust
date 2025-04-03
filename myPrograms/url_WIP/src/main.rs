#![feature(extern_types)]

















use std::borrow::Cow;
use std::os::raw::c_char;

use std::ptr;

use ::libc;
use std::ffi::{CStr, CString};
extern "C" {
    #![feature(extern_types)]
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub struct url_key_value {
    pub key: *const libc::c_char,
    pub value: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct url_data {
    pub whole_url: *mut libc::c_char,
    pub protocol: *const libc::c_char,
    pub userinfo: *const libc::c_char,
    pub host: *const libc::c_char,
    pub port: *const libc::c_char,
    pub path: *const libc::c_char,
    pub query: *const url_key_value,
    pub fragment: *const libc::c_char,
}
pub type url_data_t = url_data;
pub type Category = libc::c_uint;
pub const IPv6Char: Category = 256;
pub const Userinfo: Category = 128;
pub const Fragment: Category = 64;
pub const Query: Category = 64;
pub const HexDigit: Category = 32;
pub const PCharSlash: Category = 16;
pub const SubDelim: Category = 8;
pub const GenDelim: Category = 4;
pub const Unreserved: Category = 2;
pub const Scheme: Category = 1;
#[inline]
pub fn url_get_protocol(url: &str) -> String {
    url_get_scheme(url).unwrap_or_default()
}

#[inline]
pub fn url_get_hash(url: &str) -> String {
    url_get_fragment(url).unwrap_or_default()
}

static mut char_cat: [libc::c_ushort; 256] = [
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0xd8 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0x4 as libc::c_int as libc::c_ushort,
    0xd8 as libc::c_int as libc::c_ushort,
    0xd0 as libc::c_int as libc::c_ushort,
    0xd8 as libc::c_int as libc::c_ushort,
    0xd8 as libc::c_int as libc::c_ushort,
    0xd8 as libc::c_int as libc::c_ushort,
    0xd8 as libc::c_int as libc::c_ushort,
    0xd8 as libc::c_int as libc::c_ushort,
    0xd9 as libc::c_int as libc::c_ushort,
    0xd8 as libc::c_int as libc::c_ushort,
    0xd3 as libc::c_int as libc::c_ushort,
    0xd3 as libc::c_int as libc::c_ushort,
    0x54 as libc::c_int as libc::c_ushort,
    0x1ff as libc::c_int as libc::c_ushort,
    0x1ff as libc::c_int as libc::c_ushort,
    0x1ff as libc::c_int as libc::c_ushort,
    0x1ff as libc::c_int as libc::c_ushort,
    0x1ff as libc::c_int as libc::c_ushort,
    0x1ff as libc::c_int as libc::c_ushort,
    0x1ff as libc::c_int as libc::c_ushort,
    0x1ff as libc::c_int as libc::c_ushort,
    0x1ff as libc::c_int as libc::c_ushort,
    0x1ff as libc::c_int as libc::c_ushort,
    0x1d4 as libc::c_int as libc::c_ushort,
    0xd8 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0xd8 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0x44 as libc::c_int as libc::c_ushort,
    0x54 as libc::c_int as libc::c_ushort,
    0x1ff as libc::c_int as libc::c_ushort,
    0x1ff as libc::c_int as libc::c_ushort,
    0x1ff as libc::c_int as libc::c_ushort,
    0x1ff as libc::c_int as libc::c_ushort,
    0x1ff as libc::c_int as libc::c_ushort,
    0x1ff as libc::c_int as libc::c_ushort,
    0xdf as libc::c_int as libc::c_ushort,
    0xdf as libc::c_int as libc::c_ushort,
    0xdf as libc::c_int as libc::c_ushort,
    0xdf as libc::c_int as libc::c_ushort,
    0xdf as libc::c_int as libc::c_ushort,
    0xdf as libc::c_int as libc::c_ushort,
    0xdf as libc::c_int as libc::c_ushort,
    0xdf as libc::c_int as libc::c_ushort,
    0xdf as libc::c_int as libc::c_ushort,
    0xdf as libc::c_int as libc::c_ushort,
    0xdf as libc::c_int as libc::c_ushort,
    0xdf as libc::c_int as libc::c_ushort,
    0xdf as libc::c_int as libc::c_ushort,
    0xdf as libc::c_int as libc::c_ushort,
    0xdf as libc::c_int as libc::c_ushort,
    0xdf as libc::c_int as libc::c_ushort,
    0xdf as libc::c_int as libc::c_ushort,
    0xdf as libc::c_int as libc::c_ushort,
    0xdf as libc::c_int as libc::c_ushort,
    0xdf as libc::c_int as libc::c_ushort,
    0x4 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0x4 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0xd2 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0x1ff as libc::c_int as libc::c_ushort,
    0x1ff as libc::c_int as libc::c_ushort,
    0x1ff as libc::c_int as libc::c_ushort,
    0x1ff as libc::c_int as libc::c_ushort,
    0x1ff as libc::c_int as libc::c_ushort,
    0x1ff as libc::c_int as libc::c_ushort,
    0xdf as libc::c_int as libc::c_ushort,
    0xdf as libc::c_int as libc::c_ushort,
    0xdf as libc::c_int as libc::c_ushort,
    0xdf as libc::c_int as libc::c_ushort,
    0xdf as libc::c_int as libc::c_ushort,
    0xdf as libc::c_int as libc::c_ushort,
    0xdf as libc::c_int as libc::c_ushort,
    0xdf as libc::c_int as libc::c_ushort,
    0xdf as libc::c_int as libc::c_ushort,
    0xdf as libc::c_int as libc::c_ushort,
    0xdf as libc::c_int as libc::c_ushort,
    0xdf as libc::c_int as libc::c_ushort,
    0xdf as libc::c_int as libc::c_ushort,
    0xdf as libc::c_int as libc::c_ushort,
    0xdf as libc::c_int as libc::c_ushort,
    0xdf as libc::c_int as libc::c_ushort,
    0xdf as libc::c_int as libc::c_ushort,
    0xdf as libc::c_int as libc::c_ushort,
    0xdf as libc::c_int as libc::c_ushort,
    0xdf as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0xd2 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
];
static mut URL_SCHEMES: [*const libc::c_char; 177] = [
    b"aaa\0" as *const u8 as *const libc::c_char,
    b"aaas\0" as *const u8 as *const libc::c_char,
    b"about\0" as *const u8 as *const libc::c_char,
    b"acap\0" as *const u8 as *const libc::c_char,
    b"acct\0" as *const u8 as *const libc::c_char,
    b"adiumxtra\0" as *const u8 as *const libc::c_char,
    b"afp\0" as *const u8 as *const libc::c_char,
    b"afs\0" as *const u8 as *const libc::c_char,
    b"aim\0" as *const u8 as *const libc::c_char,
    b"apt\0" as *const u8 as *const libc::c_char,
    b"attachment\0" as *const u8 as *const libc::c_char,
    b"aw\0" as *const u8 as *const libc::c_char,
    b"beshare\0" as *const u8 as *const libc::c_char,
    b"bitcoin\0" as *const u8 as *const libc::c_char,
    b"bolo\0" as *const u8 as *const libc::c_char,
    b"callto\0" as *const u8 as *const libc::c_char,
    b"cap\0" as *const u8 as *const libc::c_char,
    b"chrome\0" as *const u8 as *const libc::c_char,
    b"crome-extension\0" as *const u8 as *const libc::c_char,
    b"com-evenbrite-attendee\0" as *const u8 as *const libc::c_char,
    b"cid\0" as *const u8 as *const libc::c_char,
    b"coap\0" as *const u8 as *const libc::c_char,
    b"coaps\0" as *const u8 as *const libc::c_char,
    b"content\0" as *const u8 as *const libc::c_char,
    b"crid\0" as *const u8 as *const libc::c_char,
    b"cvs\0" as *const u8 as *const libc::c_char,
    b"data\0" as *const u8 as *const libc::c_char,
    b"dav\0" as *const u8 as *const libc::c_char,
    b"dict\0" as *const u8 as *const libc::c_char,
    b"lna-playsingle\0" as *const u8 as *const libc::c_char,
    b"dln-playcontainer\0" as *const u8 as *const libc::c_char,
    b"dns\0" as *const u8 as *const libc::c_char,
    b"dtn\0" as *const u8 as *const libc::c_char,
    b"dvb\0" as *const u8 as *const libc::c_char,
    b"ed2k\0" as *const u8 as *const libc::c_char,
    b"facetime\0" as *const u8 as *const libc::c_char,
    b"fax\0" as *const u8 as *const libc::c_char,
    b"feed\0" as *const u8 as *const libc::c_char,
    b"file\0" as *const u8 as *const libc::c_char,
    b"finger\0" as *const u8 as *const libc::c_char,
    b"fish\0" as *const u8 as *const libc::c_char,
    b"ftp\0" as *const u8 as *const libc::c_char,
    b"geo\0" as *const u8 as *const libc::c_char,
    b"gg\0" as *const u8 as *const libc::c_char,
    b"git\0" as *const u8 as *const libc::c_char,
    b"gizmoproject\0" as *const u8 as *const libc::c_char,
    b"go\0" as *const u8 as *const libc::c_char,
    b"gopher\0" as *const u8 as *const libc::c_char,
    b"gtalk\0" as *const u8 as *const libc::c_char,
    b"h323\0" as *const u8 as *const libc::c_char,
    b"hcp\0" as *const u8 as *const libc::c_char,
    b"http\0" as *const u8 as *const libc::c_char,
    b"https\0" as *const u8 as *const libc::c_char,
    b"iax\0" as *const u8 as *const libc::c_char,
    b"icap\0" as *const u8 as *const libc::c_char,
    b"icon\0" as *const u8 as *const libc::c_char,
    b"im\0" as *const u8 as *const libc::c_char,
    b"imap\0" as *const u8 as *const libc::c_char,
    b"info\0" as *const u8 as *const libc::c_char,
    b"ipn\0" as *const u8 as *const libc::c_char,
    b"ipp\0" as *const u8 as *const libc::c_char,
    b"irc\0" as *const u8 as *const libc::c_char,
    b"irc6\0" as *const u8 as *const libc::c_char,
    b"ircs\0" as *const u8 as *const libc::c_char,
    b"iris\0" as *const u8 as *const libc::c_char,
    b"iris.beep\0" as *const u8 as *const libc::c_char,
    b"iris.xpc\0" as *const u8 as *const libc::c_char,
    b"iris.xpcs\0" as *const u8 as *const libc::c_char,
    b"iris.lws\0" as *const u8 as *const libc::c_char,
    b"itms\0" as *const u8 as *const libc::c_char,
    b"jabber\0" as *const u8 as *const libc::c_char,
    b"jar\0" as *const u8 as *const libc::c_char,
    b"jms\0" as *const u8 as *const libc::c_char,
    b"keyparc\0" as *const u8 as *const libc::c_char,
    b"lastfm\0" as *const u8 as *const libc::c_char,
    b"ldap\0" as *const u8 as *const libc::c_char,
    b"ldaps\0" as *const u8 as *const libc::c_char,
    b"magnet\0" as *const u8 as *const libc::c_char,
    b"mailserver\0" as *const u8 as *const libc::c_char,
    b"mailto\0" as *const u8 as *const libc::c_char,
    b"maps\0" as *const u8 as *const libc::c_char,
    b"market\0" as *const u8 as *const libc::c_char,
    b"message\0" as *const u8 as *const libc::c_char,
    b"mid\0" as *const u8 as *const libc::c_char,
    b"mms\0" as *const u8 as *const libc::c_char,
    b"modem\0" as *const u8 as *const libc::c_char,
    b"ms-help\0" as *const u8 as *const libc::c_char,
    b"mssettings-power\0" as *const u8 as *const libc::c_char,
    b"msnim\0" as *const u8 as *const libc::c_char,
    b"msrp\0" as *const u8 as *const libc::c_char,
    b"msrps\0" as *const u8 as *const libc::c_char,
    b"mtqp\0" as *const u8 as *const libc::c_char,
    b"mumble\0" as *const u8 as *const libc::c_char,
    b"mupdate\0" as *const u8 as *const libc::c_char,
    b"mvn\0" as *const u8 as *const libc::c_char,
    b"news\0" as *const u8 as *const libc::c_char,
    b"nfs\0" as *const u8 as *const libc::c_char,
    b"ni\0" as *const u8 as *const libc::c_char,
    b"nih\0" as *const u8 as *const libc::c_char,
    b"nntp\0" as *const u8 as *const libc::c_char,
    b"notes\0" as *const u8 as *const libc::c_char,
    b"oid\0" as *const u8 as *const libc::c_char,
    b"paquelocktoken\0" as *const u8 as *const libc::c_char,
    b"pack\0" as *const u8 as *const libc::c_char,
    b"palm\0" as *const u8 as *const libc::c_char,
    b"paparazzi\0" as *const u8 as *const libc::c_char,
    b"pkcs11\0" as *const u8 as *const libc::c_char,
    b"platform\0" as *const u8 as *const libc::c_char,
    b"pop\0" as *const u8 as *const libc::c_char,
    b"pres\0" as *const u8 as *const libc::c_char,
    b"prospero\0" as *const u8 as *const libc::c_char,
    b"proxy\0" as *const u8 as *const libc::c_char,
    b"psyc\0" as *const u8 as *const libc::c_char,
    b"query\0" as *const u8 as *const libc::c_char,
    b"reload\0" as *const u8 as *const libc::c_char,
    b"res\0" as *const u8 as *const libc::c_char,
    b"resource\0" as *const u8 as *const libc::c_char,
    b"rmi\0" as *const u8 as *const libc::c_char,
    b"rsync\0" as *const u8 as *const libc::c_char,
    b"rtmp\0" as *const u8 as *const libc::c_char,
    b"rtsp\0" as *const u8 as *const libc::c_char,
    b"secondlife\0" as *const u8 as *const libc::c_char,
    b"service\0" as *const u8 as *const libc::c_char,
    b"session\0" as *const u8 as *const libc::c_char,
    b"sftp\0" as *const u8 as *const libc::c_char,
    b"sgn\0" as *const u8 as *const libc::c_char,
    b"shttp\0" as *const u8 as *const libc::c_char,
    b"sieve\0" as *const u8 as *const libc::c_char,
    b"sip\0" as *const u8 as *const libc::c_char,
    b"sips\0" as *const u8 as *const libc::c_char,
    b"skype\0" as *const u8 as *const libc::c_char,
    b"smb\0" as *const u8 as *const libc::c_char,
    b"sms\0" as *const u8 as *const libc::c_char,
    b"snews\0" as *const u8 as *const libc::c_char,
    b"snmp\0" as *const u8 as *const libc::c_char,
    b"soap.beep\0" as *const u8 as *const libc::c_char,
    b"soap.beeps\0" as *const u8 as *const libc::c_char,
    b"soldat\0" as *const u8 as *const libc::c_char,
    b"spotify\0" as *const u8 as *const libc::c_char,
    b"ssh\0" as *const u8 as *const libc::c_char,
    b"steam\0" as *const u8 as *const libc::c_char,
    b"svn\0" as *const u8 as *const libc::c_char,
    b"tag\0" as *const u8 as *const libc::c_char,
    b"teamspeak\0" as *const u8 as *const libc::c_char,
    b"tel\0" as *const u8 as *const libc::c_char,
    b"telnet\0" as *const u8 as *const libc::c_char,
    b"tftp\0" as *const u8 as *const libc::c_char,
    b"things\0" as *const u8 as *const libc::c_char,
    b"thismessage\0" as *const u8 as *const libc::c_char,
    b"tn3270\0" as *const u8 as *const libc::c_char,
    b"tip\0" as *const u8 as *const libc::c_char,
    b"tv\0" as *const u8 as *const libc::c_char,
    b"udp\0" as *const u8 as *const libc::c_char,
    b"unreal\0" as *const u8 as *const libc::c_char,
    b"urn\0" as *const u8 as *const libc::c_char,
    b"ut2004\0" as *const u8 as *const libc::c_char,
    b"vemmi\0" as *const u8 as *const libc::c_char,
    b"ventrilo\0" as *const u8 as *const libc::c_char,
    b"videotex\0" as *const u8 as *const libc::c_char,
    b"view-source\0" as *const u8 as *const libc::c_char,
    b"wais\0" as *const u8 as *const libc::c_char,
    b"webcal\0" as *const u8 as *const libc::c_char,
    b"ws\0" as *const u8 as *const libc::c_char,
    b"wss\0" as *const u8 as *const libc::c_char,
    b"wtai\0" as *const u8 as *const libc::c_char,
    b"wyciwyg\0" as *const u8 as *const libc::c_char,
    b"xcon\0" as *const u8 as *const libc::c_char,
    b"xcon-userid\0" as *const u8 as *const libc::c_char,
    b"xfire\0" as *const u8 as *const libc::c_char,
    b"xmlrpc.beep\0" as *const u8 as *const libc::c_char,
    b"xmlrpc.beeps\0" as *const u8 as *const libc::c_char,
    b"xmpp\0" as *const u8 as *const libc::c_char,
    b"xri\0" as *const u8 as *const libc::c_char,
    b"ymsgr\0" as *const u8 as *const libc::c_char,
    b"javascript\0" as *const u8 as *const libc::c_char,
    b"jdbc\0" as *const u8 as *const libc::c_char,
    b"doi\0" as *const u8 as *const libc::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn strdup(mut str: *const libc::c_char) -> *mut libc::c_char {
    let n = (strlen(str)).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    let mut dup = malloc(n as libc::c_ulong) as *mut libc::c_char;
    if !dup.is_null() {
        strcpy(dup, str);
    }
    return dup;
}
pub fn unhex(s: *const libc::c_char) -> i32 {
    if s.is_null() {
        return -1;
    }
    // SAFETY: s is assumed to be a valid, null‐terminated C string.
    let c_str = unsafe { CStr::from_ptr(s) };
    let s_str = match c_str.to_str() {
        Ok(s) if !s.is_empty() => s,
        _ => return -1,
    };
    let ch = s_str.chars().next().unwrap();
    match ch {
        '0'..='9' => ch as i32 - '0' as i32,
        'A'..='F' => ch as i32 - 'A' as i32 + 10,
        'a'..='f' => ch as i32 - 'a' as i32 + 10,
        _ => -1,
    }
}

pub fn decode_percent(s: &str) -> Option<CString> {
    let mut bytes: Vec<u8> = Vec::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '%' {
            // Expect two hex digits after '%'
            let high_char = chars.next()?;
            let low_char = chars.next()?;
            let high_c = high_char as libc::c_char;
            let low_c = low_char as libc::c_char;
            let high = unsafe { unhex(&high_c as *const libc::c_char) };
            if high < 0 {
                return None;
            }
            let low = unsafe { unhex(&low_c as *const libc::c_char) };
            if low < 0 {
                return None;
            }
            let byte = (high * 16 + low) as u8;
            bytes.push(byte);
        } else {
            bytes.push(c as u8);
        }
    }
    CString::new(bytes).ok()
}

unsafe extern "C" fn scan_part(
    mut start: *mut libc::c_char,
    mut category: Category,
    mut delimiter1: libc::c_char,
    mut delimiter2: libc::c_char,
) -> *mut libc::c_char {
    let mut p = start;
    loop {
        if *p as libc::c_int == '\0' as i32
            || *p as libc::c_int == delimiter1 as libc::c_int
            || *p as libc::c_int == delimiter2 as libc::c_int
        {
            return p;
        }
        if char_cat[*p as libc::c_uchar as usize] as libc::c_uint
            & category as libc::c_uint != 0
        {
            p = p.offset(1);
            p;
        } else {
            return 0 as *mut libc::c_char
        }
    };
}
unsafe extern "C" fn scan_decimal_number(
    mut start: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut p = start;
    while *p as libc::c_int >= '0' as i32 && *p as libc::c_int <= '9' as i32 {
        p = p.offset(1);
        p;
    }
    return if p != start { p } else { 0 as *mut libc::c_char };
}
unsafe extern "C" fn parse_query_string(
    mut begin: *mut libc::c_char,
    mut end: *mut libc::c_char,
) -> *mut url_key_value {
    let mut current_block: u64;
    let mut elements = 1 as libc::c_int as libc::c_uint;
    let mut p: *const libc::c_char = begin;
    while p != end as *const libc::c_char {
        if *p as libc::c_int == '&' as i32 || *p as libc::c_int == ';' as i32 {
            elements = elements.wrapping_add(1);
            elements;
        }
        p = p.offset(1);
        p;
    }
    let mut kv = calloc(
        elements.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong,
        ::core::mem::size_of::<url_key_value>() as libc::c_ulong,
    ) as *mut url_key_value;
    if kv.is_null() {
        return 0 as *mut url_key_value;
    }
    let mut p_0 = begin;
    let mut element = 0 as libc::c_int as libc::c_uint;
    loop {
        if !(element <= elements && p_0 < end) {
            current_block = 11584701595673473500;
            break;
        }
        let mut key = p_0;
        let mut kv_end = scan_part(
            p_0,
            Query,
            '&' as i32 as libc::c_char,
            ';' as i32 as libc::c_char,
        );
        if kv_end.is_null() {
            fprintf(
                stderr,
                b"ERROR %s Line %u! p=\xC2\xAB%s\xC2\xBB\n\0" as *const u8
                    as *const libc::c_char,
                b"/workspace/programs/url/url.c\0" as *const u8 as *const libc::c_char,
                153 as libc::c_int,
                p_0,
            );
            current_block = 7272095228866324518;
            break;
        } else {
            *kv_end = '\0' as i32 as libc::c_char;
            let mut key_end = scan_part(
                p_0,
                Query,
                '=' as i32 as libc::c_char,
                '\0' as i32 as libc::c_char,
            );
            let has_value = *key_end as libc::c_int == '=' as i32;
            *key_end = '\0' as i32 as libc::c_char;
            let ref mut fresh2 = (*kv.offset(element as isize)).key;
            {
    let key_str = unsafe { CStr::from_ptr(key) }
        .to_str()
        .expect("Invalid UTF-8 in key");
    let decoded_cstring = decode_percent(key_str)
        .expect("Invalid percent encoding in key");
    *fresh2 = decoded_cstring.into_raw();
}
            if has_value {
                let mut value = key_end.offset(1 as libc::c_int as isize);
                let ref mut fresh3 = (*kv.offset(element as isize)).value;
                {
    let value_str = unsafe { CStr::from_ptr(value) }
        .to_str()
        .expect("Invalid UTF-8 in value");
    let decoded_cstring = decode_percent(value_str)
        .expect("Invalid percent encoding in value");
    *fresh3 = decoded_cstring.into_raw();
}
            } else {
                let ref mut fresh4 = (*kv.offset(element as isize)).value;
                *fresh4 = key_end;
            }
            p_0 = kv_end.offset(1 as libc::c_int as isize);
            element = element.wrapping_add(1);
            element;
        }
    }
    match current_block {
        11584701595673473500 => return kv,
        _ => {
            free(kv as *mut libc::c_void);
            return 0 as *mut url_key_value;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn url_parse(mut url: *const libc::c_char) -> *mut url_data_t {
    let mut p_end: *const libc::c_char = 0 as *const libc::c_char;
    let mut protocol_end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut is_ssh: bool = false;
    let mut second_slash: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut userinfo_end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut hostname_end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut path_end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut has_query: bool = false;
    let mut has_fragment: bool = false;
    let mut current_block: u64;
    let mut data = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<url_data_t>() as libc::c_ulong,
    ) as *mut url_data_t;
    if data.is_null() {
        return 0 as *mut url_data_t;
    }
    let mut p = strdup(url);
    if p.is_null() {
        fprintf(
            stderr,
            b"ERROR %s Line %u! p=\xC2\xAB%s\xC2\xBB\n\0" as *const u8
                as *const libc::c_char,
            b"/workspace/programs/url/url.c\0" as *const u8 as *const libc::c_char,
            190 as libc::c_int,
            p,
        );
    } else {
        (*data).whole_url = p;
        p_end = p.offset(strlen(p) as isize);
        protocol_end = scan_part(
            p,
            Scheme,
            ':' as i32 as libc::c_char,
            '\0' as i32 as libc::c_char,
        );
        if protocol_end.is_null() || *protocol_end as libc::c_int == '\0' as i32 {
            fprintf(
                stderr,
                b"ERROR %s Line %u! p=\xC2\xAB%s\xC2\xBB\n\0" as *const u8
                    as *const libc::c_char,
                b"/workspace/programs/url/url.c\0" as *const u8 as *const libc::c_char,
                197 as libc::c_int,
                p,
            );
        } else {
            *protocol_end = '\0' as i32 as libc::c_char;
            (*data).protocol = p;
            let protocol_str = unsafe {
    CStr::from_ptr((*data).protocol)
}
.to_str()
.expect("protocol is not valid UTF-8");
let is_ssh = url_is_ssh(protocol_str);
            p = protocol_end.offset(1 as libc::c_int as isize);
            if p >= p_end as *mut libc::c_char || *p as libc::c_int != '/' as i32 {
                fprintf(
                    stderr,
                    b"ERROR %s Line %u! p=\xC2\xAB%s\xC2\xBB\n\0" as *const u8
                        as *const libc::c_char,
                    b"/workspace/programs/url/url.c\0" as *const u8
                        as *const libc::c_char,
                    205 as libc::c_int,
                    p,
                );
            } else {
                p = p.offset(1);
                p;
                if p >= p_end as *mut libc::c_char || *p as libc::c_int != '/' as i32 {
                    fprintf(
                        stderr,
                        b"ERROR %s Line %u! p=\xC2\xAB%s\xC2\xBB\n\0" as *const u8
                            as *const libc::c_char,
                        b"/workspace/programs/url/url.c\0" as *const u8
                            as *const libc::c_char,
                        209 as libc::c_int,
                        p,
                    );
                } else {
                    /*
The variables live at this point are:
(mut url: *const i8, mut p_end: *const i8, mut is_ssh: bool, mut second_slash: *mut i8, mut userinfo_end: *mut i8, mut hostname_end: *mut i8, mut path_end: *mut i8, mut has_query: bool, mut has_fragment: bool, mut current_block: u64, mut data: *mut url_data, mut p: *mut i8, is_ssh: bool)
*/
{
    // We create a safe sub-slice from p up to p_end.
    let len = (p_end as usize).wrapping_sub(p as usize);
    let safe_slice = unsafe { std::slice::from_raw_parts(p as *const u8, len) };
    let safe_str = std::str::from_utf8(safe_slice).expect("expected valid UTF-8");

    // Record the current position in p.
    second_slash = p;

    // Advance one character from safe_str.
    let mut chars_iter = safe_str.chars();
    if let Some(_first_char) = chars_iter.next() {
        let remaining_str = chars_iter.as_str();
        // Update p to the new position by converting the remaining string back to a pointer.
        p = remaining_str.as_ptr() as *mut i8;
         userinfo_end = scan_part(
                            p,
                            Userinfo,
                            '@' as i32 as libc::c_char,
                            '\0' as i32 as libc::c_char,
                        );
                        if !userinfo_end.is_null()
                            && *userinfo_end as libc::c_int == '@' as i32
                        {
                            *userinfo_end = '\0' as i32 as libc::c_char;
                            (*data).userinfo = p;
                            p = userinfo_end.offset(1 as libc::c_int as isize);
                        }
                        if p >= p_end as *mut libc::c_char {
                            fprintf(
                                stderr,
                                b"ERROR %s Line %u! p=\xC2\xAB%s\xC2\xBB\n\0" as *const u8
                                    as *const libc::c_char,
                                b"/workspace/programs/url/url.c\0" as *const u8
                                    as *const libc::c_char,
                                225 as libc::c_int,
                                p,
                            );
                        } else {
                            hostname_end = 0 as *mut libc::c_char;
                            if *p as libc::c_int == '[' as i32 {
                                p = p.offset(1);
                                p;
                                hostname_end = scan_part(
                                    p,
                                    IPv6Char,
                                    ']' as i32 as libc::c_char,
                                    '\0' as i32 as libc::c_char,
                                );
                                if hostname_end.is_null() {
                                    fprintf(
                                        stderr,
                                        b"ERROR %s Line %u! p=\xC2\xAB%s\xC2\xBB\n\0" as *const u8
                                            as *const libc::c_char,
                                        b"/workspace/programs/url/url.c\0" as *const u8
                                            as *const libc::c_char,
                                        234 as libc::c_int,
                                        p,
                                    );
                                    current_block = 17635118975278168935;
                                } else {
                                    *hostname_end = '\0' as i32 as libc::c_char;
                                    (*data).host = p;
                                    hostname_end = hostname_end.offset(1);
                                    hostname_end;
                                    if hostname_end < p_end as *mut libc::c_char && !is_ssh
                                        && *hostname_end as libc::c_int == ':' as i32
                                    {
                                        let mut port_end = scan_decimal_number(
                                            hostname_end.offset(1 as libc::c_int as isize),
                                        );
                                        if !port_end.is_null() {
                                            (*data)
                                                .port = hostname_end.offset(1 as libc::c_int as isize);
                                            p = port_end;
                                            current_block = 7252614138838059896;
                                        } else {
                                            fprintf(
                                                stderr,
                                                b"ERROR %s Line %u! p=\xC2\xAB%s\xC2\xBB\n\0" as *const u8
                                                    as *const libc::c_char,
                                                b"/workspace/programs/url/url.c\0" as *const u8
                                                    as *const libc::c_char,
                                                247 as libc::c_int,
                                                p,
                                            );
                                            current_block = 17635118975278168935;
                                        }
                                    } else {
                                        p = hostname_end;
                                        current_block = 7252614138838059896;
                                    }
                                }
                            } else {
                                hostname_end = scan_part(
                                    p,
                                    (Unreserved as libc::c_int | SubDelim as libc::c_int)
                                        as Category,
                                    ':' as i32 as libc::c_char,
                                    '/' as i32 as libc::c_char,
                                );
                                if hostname_end.is_null() {
                                    fprintf(
                                        stderr,
                                        b"ERROR %s Line %u! p=\xC2\xAB%s\xC2\xBB\n\0" as *const u8
                                            as *const libc::c_char,
                                        b"/workspace/programs/url/url.c\0" as *const u8
                                            as *const libc::c_char,
                                        255 as libc::c_int,
                                        p,
                                    );
                                    current_block = 17635118975278168935;
                                } else {
                                    (*data).host = p;
                                    if !is_ssh && *hostname_end as libc::c_int == ':' as i32 {
                                        *hostname_end = '\0' as i32 as libc::c_char;
                                        let mut port_end_0 = scan_decimal_number(
                                            hostname_end.offset(1 as libc::c_int as isize),
                                        );
                                        if !port_end_0.is_null() {
                                            (*data)
                                                .port = hostname_end.offset(1 as libc::c_int as isize);
                                            p = port_end_0;
                                            current_block = 7252614138838059896;
                                        } else {
                                            fprintf(
                                                stderr,
                                                b"ERROR %s Line %u! p=\xC2\xAB%s\xC2\xBB\n\0" as *const u8
                                                    as *const libc::c_char,
                                                b"/workspace/programs/url/url.c\0" as *const u8
                                                    as *const libc::c_char,
                                                267 as libc::c_int,
                                                p,
                                            );
                                            current_block = 17635118975278168935;
                                        }
                                    } else {
                                        p = hostname_end;
                                        current_block = 7252614138838059896;
                                    }
                                }
                            }
                            match current_block {
                                17635118975278168935 => {}
                                _ => {
                                    memmove(
                                        second_slash as *mut libc::c_void,
                                        second_slash.offset(1 as libc::c_int as isize)
                                            as *const libc::c_void,
                                        p.offset_from(second_slash) as libc::c_long as libc::c_ulong,
                                    );
                                    if !((*data).userinfo).is_null() {
                                        (*data).userinfo = ((*data).userinfo).offset(-1);
                                        (*data).userinfo;
                                    }
                                    (*data).host = ((*data).host).offset(-1);
                                    (*data).host;
                                    if !((*data).port).is_null() {
                                        (*data).port = ((*data).port).offset(-1);
                                        (*data).port;
                                    }
                                    *p
                                        .offset(
                                            -(1 as libc::c_int) as isize,
                                        ) = '\0' as i32 as libc::c_char;
                                    if is_ssh as libc::c_int != 0
                                        && *p as libc::c_int == ':' as i32
                                    {
                                        p = p.offset(1);
                                        p;
                                    }
                                    path_end = scan_part(
                                        p,
                                        PCharSlash,
                                        '?' as i32 as libc::c_char,
                                        '#' as i32 as libc::c_char,
                                    );
                                    if path_end.is_null() {
                                        fprintf(
                                            stderr,
                                            b"ERROR %s Line %u! p=\xC2\xAB%s\xC2\xBB\n\0" as *const u8
                                                as *const libc::c_char,
                                            b"/workspace/programs/url/url.c\0" as *const u8
                                                as *const libc::c_char,
                                            295 as libc::c_int,
                                            p,
                                        );
                                    } else {
                                        has_query = *path_end as libc::c_int == '?' as i32;
                                        has_fragment = *path_end as libc::c_int == '#' as i32;
                                        *path_end = '\0' as i32 as libc::c_char;
                                        {
    let p_str = unsafe { CStr::from_ptr(p) }
        .to_str()
        .expect("Invalid UTF-8 in p");
    let decoded_cstring = decode_percent(p_str)
        .expect("Invalid percent encoding in path");
    (*data).path = decoded_cstring.into_raw();
}
                                        p = path_end.offset(1 as libc::c_int as isize);
                                        if has_query {
                                            let mut query_end = scan_part(
                                                p,
                                                Query,
                                                '#' as i32 as libc::c_char,
                                                '\0' as i32 as libc::c_char,
                                            );
                                            if !query_end.is_null() {
                                                let has_fragment_0 = *query_end as libc::c_int
                                                    == '#' as i32;
                                                *query_end = '\0' as i32 as libc::c_char;
                                                (*data).query = parse_query_string(p, query_end);
                                                if has_fragment_0 {
                                                    let mut fragment_end = scan_part(
                                                        query_end.offset(1 as libc::c_int as isize),
                                                        Fragment,
                                                        '\0' as i32 as libc::c_char,
                                                        '\0' as i32 as libc::c_char,
                                                    );
                                                    if !fragment_end.is_null() {
                                                        {
    let query_ptr = unsafe { CStr::from_ptr(query_end.offset(1)) }
        .to_str()
        .expect("Invalid UTF-8 in query fragment");
    let decoded_cstring = decode_percent(query_ptr)
        .expect("Invalid percent encoding in fragment");
    (*data).fragment = decoded_cstring.into_raw();
}
                                                        current_block = 6406431739208918833;
                                                    } else {
                                                        fprintf(
                                                            stderr,
                                                            b"ERROR %s Line %u! p=\xC2\xAB%s\xC2\xBB\n\0" as *const u8
                                                                as *const libc::c_char,
                                                            b"/workspace/programs/url/url.c\0" as *const u8
                                                                as *const libc::c_char,
                                                            319 as libc::c_int,
                                                            p,
                                                        );
                                                        current_block = 17635118975278168935;
                                                    }
                                                } else {
                                                    current_block = 6406431739208918833;
                                                }
                                            } else {
                                                fprintf(
                                                    stderr,
                                                    b"ERROR %s Line %u! p=\xC2\xAB%s\xC2\xBB\n\0" as *const u8
                                                        as *const libc::c_char,
                                                    b"/workspace/programs/url/url.c\0" as *const u8
                                                        as *const libc::c_char,
                                                    323 as libc::c_int,
                                                    p,
                                                );
                                                current_block = 17635118975278168935;
                                            }
                                        } else if has_fragment {
                                            let mut fragment_end_0 = scan_part(
                                                p,
                                                Fragment,
                                                '\0' as i32 as libc::c_char,
                                                '\0' as i32 as libc::c_char,
                                            );
                                            if !fragment_end_0.is_null() {
                                                {
    let p_str = unsafe { CStr::from_ptr(p) }
        .to_str()
        .expect("Invalid UTF-8 in p");
    let decoded_cstring = decode_percent(p_str)
        .expect("Invalid percent encoding in fragment");
    (*data).fragment = decoded_cstring.into_raw();
}
                                                current_block = 6406431739208918833;
                                            } else {
                                                fprintf(
                                                    stderr,
                                                    b"ERROR %s Line %u! p=\xC2\xAB%s\xC2\xBB\n\0" as *const u8
                                                        as *const libc::c_char,
                                                    b"/workspace/programs/url/url.c\0" as *const u8
                                                        as *const libc::c_char,
                                                    332 as libc::c_int,
                                                    p,
                                                );
                                                current_block = 17635118975278168935;
                                            }
                                        } else {
                                            current_block = 6406431739208918833;
                                        }
                                        match current_block {
                                            17635118975278168935 => {}
                                            _ => return data,
                                        }
                                    }
                                }
                            }
                        }

    } else {
        // If there is no character to advance, then p has reached or exceeded p_end.
        eprintln!(
            "ERROR {} Line {}! p={:?}",
            "/workspace/programs/url/url.c",
            215,
            p
        );
    }
}
/*
The variables live at this point are:
(mut url: *const i8, mut p_end: *const i8, mut is_ssh: bool, mut second_slash: *mut i8, mut userinfo_end: *mut i8, mut hostname_end: *mut i8, mut path_end: *mut i8, mut has_query: bool, mut has_fragment: bool, mut current_block: u64, mut data: *mut url_data, mut p: *mut i8, is_ssh: bool)
*/

                }
            }
        }
    }
    if data.is_null() {
    url_free(None);
} else {
    url_free(Some(unsafe { Box::from_raw(data) }));
}
    return 0 as *mut url_data_t;
}
#[no_mangle]
pub unsafe extern "C" fn url_is_protocol(mut str: *const libc::c_char) -> bool {
    let count = (::core::mem::size_of::<[*const libc::c_char; 177]>() as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
        as libc::c_uint;
    let mut i = 0 as libc::c_int as libc::c_uint;
    while i < count {
        if 0 as libc::c_int == strcmp(URL_SCHEMES[i as usize], str) {
            return 1 as libc::c_int != 0;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int != 0;
}
#[no_mangle]
pub fn url_is_ssh(input: &str) -> bool {
    input == "ssh" || input == "git"
}

#[no_mangle]
pub fn url_get_scheme(url: &str) -> Option<String> {
    // Extract the protocol substring: all characters until ':' or '/'
    let end_index = url.find(|c| c == ':' || c == '/').unwrap_or(url.len());
    let protocol = &url[..end_index];

    // Convert the protocol into a CString for compatibility with the external function.
    let c_protocol = CString::new(protocol).ok()?;
    
    // Call the external function (defined elsewhere) within an unsafe block.
    unsafe {
        if url_is_protocol(c_protocol.as_ptr()) {
            Some(protocol.to_string())
        } else {
            None
        }
    }
}

#[no_mangle]
pub fn url_get_userinfo(url: &str) -> Option<String> {
    // Convert the Rust string slice to a CString.
    let c_url = CString::new(url).ok()?;
    // Call the C API to parse the URL.
    let data_ptr = unsafe { url_parse(c_url.as_ptr()) };
    if data_ptr.is_null() {
        return None;
    }
    // Take ownership of the returned data by converting it into a Box.
    let data_box = unsafe { Box::from_raw(data_ptr) };
    // Extract userinfo if available, duplicating its contents into a Rust-owned String.
    let result = unsafe {
        if data_box.userinfo.is_null() {
            None
        } else {
            CStr::from_ptr(data_box.userinfo)
                .to_str()
                .ok()
                .map(String::from)
        }
    };
    // Free the parsed URL data using the provided API.
    url_free(Some(data_box));
    result
}

#[no_mangle]
pub fn url_get_hostname(url: &str) -> Option<String> {
    // Convert the Rust string into a CString.
    let c_url = CString::new(url).ok()?;
    
    // Call url_parse using our C API.
    let data = unsafe { url_parse(c_url.as_ptr()) };
    
    // If data is null, free it (by passing None) and return None.
    if data.is_null() {
        unsafe { url_free(None) };
        return None;
    }
    
    // Data is non-null at this point.
    let host = unsafe {
        if (*data).host.is_null() {
            None
        } else {
            Some(CStr::from_ptr((*data).host).to_string_lossy().into_owned())
        }
    };
    
    // Convert the raw pointer into a Box and free it.
    unsafe { url_free(Some(Box::from_raw(data))) };
    
    host
}

#[no_mangle]
pub fn url_get_host(url: &str) -> Option<String> {
    // Convert the input string to a CString.
    let c_url = CString::new(url).ok()?;
    unsafe {
        // Call the C API function.
        let data_ptr = url_parse(c_url.as_ptr());
        if data_ptr.is_null() {
            return None;
        }
        // Reconstruct ownership of the parsed data.
        let data_box = Box::from_raw(data_ptr);
        // Extract the host field if it exists.
        let host = if !data_box.host.is_null() {
            Some(CStr::from_ptr(data_box.host).to_string_lossy().into_owned())
        } else {
            None
        };
        // Pass ownership to url_free so it can free the memory.
        url_free(Some(data_box));
        host
    }
}

#[no_mangle]
pub fn url_get_pathname(url: &str) -> Option<String> {
    // Convert the Rust string to a C-style CString.
    let c_url = CString::new(url).ok()?;
    // Call the external C function.
    let data_ptr = unsafe { url_parse(c_url.as_ptr()) };
    if data_ptr.is_null() {
        return None;
    }
    // Extract the pathname.
    let pathname = unsafe {
        if !(*data_ptr).path.is_null() {
            Some(CStr::from_ptr((*data_ptr).path)
                .to_string_lossy()
                .into_owned())
        } else {
            None
        }
    };
    // Convert the raw pointer into a Box and free it using the provided function.
    unsafe { url_free(Some(Box::from_raw(data_ptr))) };
    pathname
}

#[no_mangle]
pub fn url_get_path(url: &str) -> Option<String> {
    // Convert the Rust string to a CString; return None if it contains an interior nul.
    let c_url = CString::new(url).ok()?;
    // Call the external url_parse function.
    let data_ptr = unsafe { url_parse(c_url.as_ptr()) };
    if data_ptr.is_null() {
        return None;
    }
    let result = unsafe {
        // Borrow the data to extract the path before freeing.
        let data_ref = &*data_ptr;
        let path = if data_ref.path.is_null() {
            None
        } else {
            Some(CStr::from_ptr(data_ref.path).to_string_lossy().into_owned())
        };
        // Free the allocated data by converting the raw pointer into a Box.
        url_free(Some(Box::from_raw(data_ptr)));
        path
    };
    result
}

#[no_mangle]
pub fn url_get_query_value<'a>(url: &'a url_data_t, key: &str) -> Option<&'a str> {
    // Obtain the query pointer; if it's null, return None.
    let mut kv = url.query;
    if kv.is_null() {
        return None;
    }
    unsafe {
        while !(*kv).key.is_null() {
            // Convert the C-string key to a Rust &str; if conversion fails, skip this entry.
            if let Ok(current_key) = CStr::from_ptr((*kv).key).to_str() {
                if current_key == key {
                    // Convert the C-string value and return it if valid UTF-8.
                    return CStr::from_ptr((*kv).value).to_str().ok();
                }
            }
            kv = kv.add(1);
        }
    }
    None
}

#[no_mangle]
pub fn url_get_fragment(url: &str) -> Option<String> {
    // Convert the Rust &str url to a C-compatible CString.
    let c_url = CString::new(url).ok()?;
    // Call the external function url_parse which expects a *const c_char.
    let data_ptr = unsafe { url_parse(c_url.as_ptr()) };
    if data_ptr.is_null() {
        return None;
    }
    // Extract the fragment from the parsed data.
    let fragment = unsafe {
        if (*data_ptr).fragment.is_null() {
            None
        } else {
            Some(CStr::from_ptr((*data_ptr).fragment)
                .to_string_lossy()
                .into_owned())
        }
    };
    // Convert the raw pointer into a Box and free it using url_free.
    let data_box = unsafe { Box::from_raw(data_ptr) };
    url_free(Some(data_box));
    fragment
}

#[no_mangle]
pub fn url_get_port(url: &str) -> Option<String> {
    let c_url = CString::new(url).ok()?;
    let parsed_ptr = unsafe { url_parse(c_url.as_ptr()) };
    if parsed_ptr.is_null() {
        return None;
    }
    let parsed_box = unsafe { Box::from_raw(parsed_ptr) };
    let port = if !parsed_box.port.is_null() {
        Some(unsafe { CStr::from_ptr(parsed_box.port).to_string_lossy().into_owned() })
    } else {
        None
    };
    url_free(Some(parsed_box));
    port
}

#[no_mangle]
pub fn url_inspect(url: &str) {
    let c_url = std::ffi::CString::new(url).expect("Invalid url: contains null byte");
    
    let data_ptr = unsafe { url_parse(c_url.as_ptr()) };
    let data_ref = unsafe {
        assert!(!data_ptr.is_null(), "url_parse returned null");
        &*data_ptr
    };
    
    url_data_inspect(data_ref);
}

#[no_mangle]
pub fn url_data_inspect(data: &url_data_t) {
    // Helper to convert a C string pointer to an Option<Cow<str>>.
    fn safe_str<'a>(ptr: *const c_char) -> Option<Cow<'a, str>> {
        if ptr.is_null() {
            None
        } else {
            // SAFETY: the pointer is assumed to be a valid null-terminated C string.
            Some(unsafe { std::ffi::CStr::from_ptr(ptr).to_string_lossy() })
        }
    }

    println!("#url =>");

    println!(
        "    .protocol: {}",
        safe_str(data.protocol)
            .map(|s| format!("\"{}\"", s))
            .unwrap_or_else(|| "(NULL)".to_string())
    );

    println!(
        "    .host: {}",
        safe_str(data.host)
            .map(|s| format!("\"{}\"", s))
            .unwrap_or_else(|| "(NULL)".to_string())
    );

    println!(
        "    .userinfo: {}",
        safe_str(data.userinfo)
            .map(|s| format!("\"{}\"", s))
            .unwrap_or_else(|| "(NULL)".to_string())
    );

    // Note: the original code prints .host twice.
    println!(
        "    .host: {}",
        safe_str(data.host)
            .map(|s| format!("\"{}\"", s))
            .unwrap_or_else(|| "(NULL)".to_string())
    );

    println!(
        "    .port: {}",
        safe_str(data.port)
            .map(|s| format!("\"{}\"", s))
            .unwrap_or_else(|| "(NULL)".to_string())
    );

    println!(
        "    .path: {}",
        safe_str(data.path)
            .map(|s| format!("\"{}\"", s))
            .unwrap_or_else(|| "(NULL)".to_string())
    );

    if !data.query.is_null() {
        let mut nr = 0;
        loop {
            // SAFETY: we assume data.query is a pointer to an array of url_key_value ending with a key == NULL.
            let pair = unsafe { &*data.query.add(nr) };
            if pair.key.is_null() {
                break;
            }
            let key = safe_str(pair.key).unwrap_or(Cow::Borrowed("(NULL)"));
            print!("    .query[{}]: \"{}\" -> ", nr, key);
            if pair.value.is_null() {
                println!("(NULL)");
            } else {
                let value = safe_str(pair.value).unwrap_or(Cow::Borrowed("(NULL)"));
                println!("\"{}\"", value);
            }
            nr += 1;
        }
    }

    println!(
        "    .fragment: {}",
        safe_str(data.fragment)
            .map(|s| format!("\"{}\"", s))
            .unwrap_or_else(|| "(NULL)".to_string())
    );
}

#[no_mangle]
pub fn url_free(data: Option<Box<url_data_t>>) {
    // Box automatically releases its owned resources when dropped.
    // If url_data_t owns its resources (e.g. whole_url, query) via proper Rust types with Drop,
    // they will be freed automatically.
    drop(data);
}


fn safe_url_parse(url: &str) -> Option<&'static url_data_t> {
    // Convert the Rust string into a C-compatible string.
    let c_url = CString::new(url).ok()?;
    // Call the external url_parse function safely.
    let result = unsafe { url_parse(c_url.as_ptr()) };
    if result.is_null() {
        None
    } else {
        Some(unsafe { &*result })
    }
}

fn safe_url_inspect(url_data: &url_data_t) {
    // Call the external url_data_inspect function safely.
    unsafe {
        url_data_inspect(url_data);
    }
}

fn main() {
    let first_url = "git://git@github.com:jwerle/url.h.git";
    if let Some(url_data) = safe_url_parse(first_url) {
        safe_url_inspect(url_data);
    } else {
        eprintln!("Error: url_parse returned null.");
    }
    
    let second_url = "http://user:pass@subdomain.host.com:8080/p/%C3%A5/t/h?qu%C3%ABry=strin%C4%9F&foo=bar=yuk&key%23%26%3D=%25&lol#h%C3%A6sh";
    if let Some(url_data) = safe_url_parse(second_url) {
        safe_url_inspect(url_data);
    }
}
