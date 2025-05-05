#![feature(extern_types)]


















// No new imports needed.

use std::slice;
use std::str;

use std::ptr;

use std::ffi::{CStr, CString};
use std::string::String;

use ::libc;
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
pub fn url_get_protocol(url: &str) -> Option<String> {
    url_get_scheme(url)
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
    // SAFETY: Caller must ensure that s is a valid, null-terminated C string.
    let s_str = unsafe { CStr::from_ptr(s) }
        .to_str()
        .unwrap_or("");
    
    let c = match s_str.chars().next() {
        Some(ch) => ch,
        None => return -1,
    };

    match c {
        '0'..='9' => c as i32 - '0' as i32,
        'A'..='F' => c as i32 - 'A' as i32 + 10,
        'a'..='f' => c as i32 - 'a' as i32 + 10,
        _ => -1,
    }
}

pub fn decode_percent(s: &str) -> Option<String> {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(ch) = chars.next() {
        if ch == '%' {
            // Get two hexadecimal digits following '%'
            let high = chars.next()?;
            let low = chars.next()?;

            // Convert high digit: create a temporary libc::c_char from the char value.
            let high_c: libc::c_char = high as u8 as libc::c_char;
            let high_val = unsafe { unhex(&high_c as *const libc::c_char) };
            if high_val < 0 {
                return None;
            }

            // Convert low digit.
            let low_c: libc::c_char = low as u8 as libc::c_char;
            let low_val = unsafe { unhex(&low_c as *const libc::c_char) };
            if low_val < 0 {
                return None;
            }

            let decoded_byte = (high_val as u8).wrapping_mul(16).wrapping_add(low_val as u8);
            // Assuming that the decoded byte is valid ASCII/UTF-8.
            out.push(decoded_byte as char);
        } else {
            out.push(ch);
        }
    }
    Some(out)
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
pub fn scan_decimal_number(start: &str) -> Option<&str> {
    // Find the index of the first character that is not an ASCII digit.
    let pos = start.find(|c: char| !c.is_ascii_digit()).unwrap_or(start.len());
    // If at least one digit was found, return the remainder of the string.
    if pos > 0 {
        Some(&start[pos..])
    } else {
        None
    }
}

pub fn parse_query_string(query: &str) -> *mut url_key_value {
    // Split the query string by '&' and ';' and filter out empty segments.
    let segments: Vec<&str> = query.split(&['&', ';'][..])
        .filter(|seg| !seg.is_empty())
        .collect();
    let elements = segments.len();

    // Create a vector to hold the url_key_value entries.
    let mut entries = Vec::with_capacity(elements + 1);

    for segment in segments {
        // Split at the first '=' character.
        let (key_part, value_part) = match segment.find('=') {
            Some(pos) => (&segment[..pos], &segment[pos + 1..]),
            None => (segment, ""),
        };

        // Assume decode_percent returns Option<String>.
        // If decoding fails, fallback to the original substring.
        let decoded_key = decode_percent(key_part).unwrap_or_else(|| key_part.to_owned());
        let decoded_value = if !value_part.is_empty() {
            decode_percent(value_part).unwrap_or_else(|| value_part.to_owned())
        } else {
            String::new()
        };

        // Convert the decoded strings into CStrings.
        // Note: Handling of interior NULs is assumed to be dealt with appropriately.
        let c_key = CString::new(decoded_key).expect("CString::new failed for key");
        let c_value = CString::new(decoded_value).expect("CString::new failed for value");

        // Convert the CStrings into raw pointers.
        let key_ptr = c_key.into_raw();
        let value_ptr = c_value.into_raw();

        entries.push(url_key_value {
            key: key_ptr,
            value: value_ptr,
        });
    }

    // Add a terminating element with null pointers.
    entries.push(url_key_value {
        key: std::ptr::null(),
        value: std::ptr::null(),
    });

    // Convert the vector into a boxed slice and then into a raw pointer.
    let boxed_slice = entries.into_boxed_slice();
    Box::into_raw(boxed_slice) as *mut url_key_value
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
        {
    // Convert the raw C string pointer p into a Rust &str for error reporting.
    let p_str = unsafe {
        CStr::from_ptr(p)
            .to_str()
            .expect("p is not valid UTF-8")
    };

    // Assign whole_url using the pointer p.
    unsafe {
        (*data).whole_url = p;
        // Compute p_end as the end of the C string pointed to by p.
        p_end = p.add(p_str.len());
        // Call scan_part with proper delimiter characters.
        protocol_end = scan_part(p, Scheme, b':' as i8, b'\0' as i8);
        if protocol_end.is_null() || *protocol_end == 0 {
            eprintln!(
                "ERROR {} Line {}! p=«{}»",
                "/workspace/programs/url/url.c",
                197,
                p_str
            );
        } else {
             *protocol_end = '\0' as i32 as libc::c_char;
            (*data).protocol = p;
            let protocol = unsafe {
    CStr::from_ptr((*data).protocol)
        .to_str()
        .expect("Invalid UTF-8 in protocol")
};
let is_ssh = url_is_ssh(protocol);
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
                // Helper functions to work with raw pointers safely.
// These helper functions encapsulate the minimal unsafe code required for pointer arithmetic and conversion.
// They assume that the pointers `p` and `p_end` define a valid contiguous region of UTF-8 encoded data.
fn advance_ptr_by_one(p: *mut i8) -> *mut i8 {
    // Advances the pointer by one byte.
    unsafe { p.add(1) }
}

fn ptr_to_str<'a>(p: *mut i8, p_end: *const i8) -> &'a str {
    let start = p as *const u8;
    let start_addr = p as usize;
    let end_addr = p_end as usize;
    let len = end_addr.saturating_sub(start_addr);
    // Uses the already imported std::slice and std::str from the file's global imports.
    let slice = unsafe { std::slice::from_raw_parts(start, len) };
    std::str::from_utf8(slice).unwrap()
}

// The live variables at this point are:
// (mut url: *const i8, mut p_end: *const i8, mut is_ssh: bool, mut second_slash: *mut i8, mut userinfo_end: *mut i8, mut hostname_end: *mut i8, mut path_end: *mut i8, mut has_query: bool, mut has_fragment: bool, mut current_block: u64, mut data: *mut url_data, mut p: *mut i8, is_ssh: bool)

// Advance `p` by one character.
p = advance_ptr_by_one(p);

// Create a temporary safe string slice from the pointer range [p, p_end).
let current_str = ptr_to_str(p, p_end);

// Check if we've reached the end of the input or if the current character is not '/'
if current_str.is_empty() || !current_str.starts_with('/') {
    // Use eprintln! for error output. The safe string slice `current_str` displays properly.
    eprintln!("ERROR /workspace/programs/url/url.c Line {}! p=«{}»", 209, current_str);
} else {
     second_slash = p;
                    p = p.offset(1);
                    p;
                    if p >= p_end as *mut libc::c_char {
                        fprintf(
                            stderr,
                            b"ERROR %s Line %u! p=\xC2\xAB%s\xC2\xBB\n\0" as *const u8
                                as *const libc::c_char,
                            b"/workspace/programs/url/url.c\0" as *const u8
                                as *const libc::c_char,
                            215 as libc::c_int,
                            p,
                        );
                    } else {
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
                                        // Translation for call site 2
// Replace the original line:
// let mut port_end = scan_decimal_number(hostname_end.offset(1 as libc::c_int as isize));
//
// Convert the raw C-string pointer 'hostname_end' to a Rust string slice.
let hostname_str = unsafe { std::ffi::CStr::from_ptr(hostname_end) }
    .to_str()
    .expect("Invalid UTF-8 in hostname_end");
let input = &hostname_str[1..];
// Call the idiomatic function.
let port_end = match scan_decimal_number(input) {
    Some(rest) => {
        let digits_consumed = input.len() - rest.len();
        unsafe { hostname_end.offset(1 + digits_consumed as isize) }
    }
    None => ptr::null_mut(),
};
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
                                        // Translation for call site 1
// Replace the original line:
// let mut port_end_0 = scan_decimal_number(hostname_end.offset(1 as libc::c_int as isize));
//
// We assume that 'hostname_end' is a *mut libc::c_char pointing to a valid NUL-terminated UTF-8 string.
let hostname_str = unsafe { std::ffi::CStr::from_ptr(hostname_end) }
    .to_str()
    .expect("Invalid UTF-8 in hostname_end");
// Create a slice starting from the character after the first one.
let input = &hostname_str[1..];
// Call the idiomatic function.
let port_end_0 = match scan_decimal_number(input) {
    Some(rest) => {
        let digits_consumed = input.len() - rest.len();
        // Compute the new pointer: move hostname_end by 1 plus the number of digits consumed.
        unsafe { hostname_end.offset(1 + digits_consumed as isize) }
    }
    None => ptr::null_mut(),
};
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
                                        // Call site 5:
{
    // Convert the raw pointer 'p' to a &str.
    let p_str = unsafe {
        CStr::from_ptr(p as *const libc::c_char)
            .to_str()
            .expect("Invalid UTF-8 in path string (call site 5)")
    };
    // Decode.
    let decoded = decode_percent(p_str)
        .expect("Invalid percent encoding in path (call site 5)");
    // Convert to CString then to raw pointer.
    let cstring = CString::new(decoded).expect("CString conversion failed (call site 5)");
    (*data).path = cstring.into_raw();
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
                                                {
    // Convert the raw C pointer range [p, query_end) to a Rust &str.
    // Safety: It is assumed that the memory from p to query_end is valid and UTF-8 encoded.
    let query_len = unsafe { query_end.offset_from(p) as usize };
    let query_bytes = unsafe { slice::from_raw_parts(p as *const u8, query_len) };
    let query_str = str::from_utf8(query_bytes).expect("Invalid UTF-8 in query string");

    // Call the idiomatic Rust version of parse_query_string.
    // Note: The type of data.query should be updated accordingly to a raw pointer
    // of type *mut url_key_value.
    (*data).query = parse_query_string(query_str);
}
                                                if has_fragment_0 {
                                                    let mut fragment_end = scan_part(
                                                        query_end.offset(1 as libc::c_int as isize),
                                                        Fragment,
                                                        '\0' as i32 as libc::c_char,
                                                        '\0' as i32 as libc::c_char,
                                                    );
                                                    if !fragment_end.is_null() {
                                                        // Call site 4:
{
    // Convert the raw pointer 'query_end' to a &str.
    let query_str = unsafe {
        CStr::from_ptr(query_end as *const libc::c_char)
            .to_str()
            .expect("Invalid UTF-8 in query string (call site 4)")
    };
    // Skip the first character.
    let query_substr = query_str.get(1..)
        .expect("Query string too short when skipping first character (call site 4)");
    // Decode.
    let decoded = decode_percent(query_substr)
        .expect("Invalid percent encoding in query (call site 4)");
    // Convert decoded string to CString then to raw pointer.
    let cstring = CString::new(decoded).expect("CString conversion failed (call site 4)");
    (*data).fragment = cstring.into_raw();
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
                                                // Call site 1:
{
    // Convert the raw pointer 'p' to a &str.
    let p_str = unsafe {
        CStr::from_ptr(p as *const libc::c_char)
            .to_str()
            .expect("Invalid UTF-8 in fragment string (call site 1)")
    };
    // Decode percent-encoded string.
    let decoded = decode_percent(p_str)
        .expect("Invalid percent encoding in fragment (call site 1)");
    // Convert the decoded string into a CString and obtain a raw pointer.
    let cstring = CString::new(decoded).expect("CString conversion failed (call site 1)");
    (*data).fragment = cstring.into_raw();
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
                    }

}

// The live variables remain unchanged:
// (mut url: *const i8, mut p_end: *const i8, mut is_ssh: bool, mut second_slash: *mut i8, mut userinfo_end: *mut i8, mut hostname_end: *mut i8, mut path_end: *mut i8, mut has_query: bool, mut has_fragment: bool, mut current_block: u64, mut data: *mut url_data, mut p: *mut i8, is_ssh: bool)

            }

        }
    }
}

    }
    // Convert the raw pointer to an Option<Box<url_data_t>>.
// If data is null, pass None; otherwise, wrap it in a Box.
// Note: Box::from_raw is unsafe.
let data_box = if data.is_null() { None } else { Some(unsafe { Box::from_raw(data) }) };
url_free(data_box);
    return 0 as *mut url_data_t;
}
#[no_mangle]
pub fn url_is_protocol(input: *const libc::c_char) -> bool {
    // If the input pointer is null, return false.
    if input.is_null() {
        return false;
    }
    // Convert the raw C string pointer to a Rust &str.
    let input_str = unsafe {
        CStr::from_ptr(input)
            .to_str()
            .unwrap_or("")
    };
    // Accessing the static URL_SCHEMES requires an unsafe block.
    let schemes = unsafe { &URL_SCHEMES };
    schemes.iter().any(|&scheme_ptr| {
        // Skip null scheme pointers.
        if scheme_ptr.is_null() {
            return false;
        }
        unsafe {
            CStr::from_ptr(scheme_ptr)
                .to_str()
                .map(|scheme_str| scheme_str == input_str)
                .unwrap_or(false)
        }
    })
}

#[no_mangle]
pub fn url_is_ssh(s: &str) -> bool {
    s == "ssh" || s == "git"
}

#[no_mangle]
pub fn url_get_scheme(url: &str) -> Option<String> {
    // Extract characters from the start until a ':' or '/' is encountered.
    let protocol: String = url.chars().take_while(|&c| c != ':' && c != '/').collect();
    // Convert the protocol string into a CString for the foreign function.
    let c_protocol = CString::new(protocol.clone()).ok()?;
    unsafe {
        if url_is_protocol(c_protocol.as_ptr()) {
            Some(protocol)
        } else {
            None
        }
    }
}

#[no_mangle]
pub fn url_get_userinfo(url: &str) -> Option<String> {
    // Convert the Rust string to a C-compatible CString.
    let c_url = CString::new(url).ok()?;
    unsafe {
        let raw_data = url_parse(c_url.as_ptr());
        if raw_data.is_null() {
            return None;
        }
        // Transfer ownership of the raw pointer into a Box.
        let boxed_data = Box::from_raw(raw_data);
        // Extract the userinfo if available.
        let result = if !boxed_data.userinfo.is_null() {
            Some(CStr::from_ptr(boxed_data.userinfo).to_string_lossy().into_owned())
        } else {
            None
        };
        // Pass ownership of the boxed data to url_free for proper deallocation.
        url_free(Some(boxed_data));
        result
    }
}

#[no_mangle]
pub fn url_get_hostname(url: &str) -> Option<String> {
    let c_url = CString::new(url).ok()?;
    let data_ptr = unsafe { url_parse(c_url.as_ptr()) };
    if data_ptr.is_null() {
        return None;
    }
    
    let hostname = unsafe {
        if (*data_ptr).host.is_null() {
            None
        } else {
            Some(CStr::from_ptr((*data_ptr).host)
                .to_string_lossy()
                .into_owned())
        }
    };
    
    unsafe {
        // Convert the raw pointer into an Option<Box<url_data_t>> as expected by url_free.
        url_free(Some(Box::from_raw(data_ptr)))
    }
    
    hostname
}

#[no_mangle]
pub fn url_get_host(url: &str) -> Option<String> {
    // Convert the Rust string to a C-compatible string.
    let c_url = CString::new(url).ok()?;

    // Call the foreign url_parse function in an unsafe block.
    let data_ptr = unsafe { url_parse(c_url.as_ptr()) };

    if data_ptr.is_null() {
        return None;
    }

    // Extract the host from the parsed data.
    let host = unsafe {
        if (*data_ptr).host.is_null() {
            None
        } else {
            Some(CStr::from_ptr((*data_ptr).host).to_string_lossy().into_owned())
        }
    };

    // Free the allocated data by wrapping it in the expected Option.
    unsafe { url_free(Some(Box::from_raw(data_ptr))) };

    host
}

#[no_mangle]
pub fn url_get_pathname(url: &str) -> Option<String> {
    let c_url = CString::new(url).ok()?;
    let data_ptr = unsafe { url_parse(c_url.as_ptr()) };
    if data_ptr.is_null() {
        return None;
    }
    // Convert the raw pointer into a Box to take ownership.
    let data_box = unsafe { Box::from_raw(data_ptr) };
    let result = unsafe {
        if (*data_box).path.is_null() {
            None
        } else {
            CStr::from_ptr((*data_box).path)
                .to_str()
                .ok()
                .map(|s| s.to_owned())
        }
    };
    // Free the allocated data using the provided url_free function.
    url_free(Some(data_box));
    result
}

#[no_mangle]
pub fn url_get_path(url: &str) -> Option<String> {
    // Convert the Rust string to a C-compatible CString.
    let c_url = CString::new(url).ok()?;
    // Call the external C function to parse the URL.
    let data_ptr = unsafe { url_parse(c_url.as_ptr()) };
    if data_ptr.is_null() {
        return None;
    }
    // Take ownership of the returned pointer.
    let data_box = unsafe { Box::from_raw(data_ptr) };
    // Extract the path if it exists.
    let result = if !data_box.path.is_null() {
        Some(unsafe { CStr::from_ptr(data_box.path) }
            .to_string_lossy()
            .into_owned())
    } else {
        None
    };
    // Free the URL data using the safe url_free function.
    url_free(Some(data_box));
    result
}

#[no_mangle]
pub unsafe extern "C" fn url_get_query_value(
    mut url: *const url_data_t,
    mut key: *const libc::c_char,
) -> *const libc::c_char {
    if ((*url).query).is_null() {
        return 0 as *const libc::c_char;
    }
    let mut kv = (*url).query;
    while !((*kv).key).is_null() {
        if strcmp((*kv).key, key) == 0 as libc::c_int {
            return (*kv).value;
        }
        kv = kv.offset(1);
        kv;
    }
    return 0 as *const libc::c_char;
}
#[no_mangle]
pub fn url_get_fragment(url: &str) -> Option<String> {
    // Convert the input &str into a CString to pass to the FFI function.
    let c_url = CString::new(url).expect("CString conversion failed");
    // Call the external url_parse function.
    let data_ptr = unsafe { url_parse(c_url.as_ptr()) };
    // Extract the fragment string if available.
    let fragment = if data_ptr.is_null() {
        None
    } else {
        unsafe {
            let data_ref = &*data_ptr;
            if data_ref.fragment.is_null() {
                None
            } else {
                Some(CStr::from_ptr(data_ref.fragment)
                    .to_string_lossy()
                    .into_owned())
            }
        }
    };
    // Wrap the raw pointer in an Option<Box<url_data_t>> as expected by url_free.
    let boxed_data = if data_ptr.is_null() {
        None
    } else {
        Some(unsafe { Box::from_raw(data_ptr) })
    };
    // Free the parsed data.
    url_free(boxed_data);
    fragment
}

#[no_mangle]
pub fn url_get_port(url: &str) -> Option<String> {
    // Convert the input string to a C-style string.
    let c_url = CString::new(url).ok()?;
    // Call the C API function `url_parse` using the inner pointer.
    let parsed_ptr = unsafe { url_parse(c_url.as_ptr()) };
    if parsed_ptr.is_null() {
        return None;
    }
    // Wrap the raw pointer into a Box for proper memory management.
    let parsed = unsafe { Box::from_raw(parsed_ptr) };
    // Extract the port if it exists.
    let result = if parsed.port.is_null() {
        None
    } else {
        Some(unsafe { CStr::from_ptr(parsed.port).to_string_lossy().into_owned() })
    };
    // Free the parsed URL data using the C API.
    unsafe { url_free(Some(parsed)) };
    result
}

#[no_mangle]
pub fn url_inspect(url: &str) {
    let c_url = CString::new(url).expect("CString conversion failed");
    unsafe {
        let parsed_ptr = url_parse(c_url.as_ptr());
        url_data_inspect(&*parsed_ptr);
    }
}

#[no_mangle]
pub fn url_data_inspect(data: &url_data_t) {
    println!("#url =>");
    unsafe {
        if !data.protocol.is_null() {
            let s = std::ffi::CStr::from_ptr(data.protocol).to_string_lossy();
            println!("    .protocol: \"{}\"", s);
        } else {
            println!("    .protocol: (NULL)");
        }
        if !data.host.is_null() {
            let s = std::ffi::CStr::from_ptr(data.host).to_string_lossy();
            println!("    .host: \"{}\"", s);
        } else {
            println!("    .host: (NULL)");
        }
        if !data.userinfo.is_null() {
            let s = std::ffi::CStr::from_ptr(data.userinfo).to_string_lossy();
            println!("    .userinfo: \"{}\"", s);
        } else {
            println!("    .userinfo: (NULL)");
        }
        // Print host a second time, as in the original function.
        if !data.host.is_null() {
            let s = std::ffi::CStr::from_ptr(data.host).to_string_lossy();
            println!("    .host: \"{}\"", s);
        } else {
            println!("    .host: (NULL)");
        }
        if !data.port.is_null() {
            let s = std::ffi::CStr::from_ptr(data.port).to_string_lossy();
            println!("    .port: \"{}\"", s);
        } else {
            println!("    .port: (NULL)");
        }
        if !data.path.is_null() {
            let s = std::ffi::CStr::from_ptr(data.path).to_string_lossy();
            println!("    .path: \"{}\"", s);
        } else {
            println!("    .path: (NULL)");
        }
        if !data.query.is_null() {
            let mut nr: isize = 0;
            loop {
                let pair = data.query.offset(nr);
                if (*pair).key.is_null() {
                    break;
                }
                let key = std::ffi::CStr::from_ptr((*pair).key).to_string_lossy();
                print!("    .query[{}]: \"{}\" -> ", nr, key);
                if !(*pair).value.is_null() {
                    let value = std::ffi::CStr::from_ptr((*pair).value).to_string_lossy();
                    println!("\"{}\"", value);
                } else {
                    println!("(NULL)");
                }
                nr += 1;
            }
        }
        if !data.fragment.is_null() {
            let s = std::ffi::CStr::from_ptr(data.fragment).to_string_lossy();
            println!("    .fragment: \"{}\"", s);
        } else {
            println!("    .fragment: (NULL)");
        }
    }
}

#[no_mangle]
pub fn url_free(data: Option<Box<url_data_t>>) {
    // When data is Some, dropping the Box will automatically free its fields.
    // No further action is required.
    drop(data);
}


pub fn main() {
    // Function body remains empty as there is no functionality to convert.
}
