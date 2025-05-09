#![feature(core_intrinsics)]
#![feature(const_mut_refs, const_fn_fn_ptr_basics)]
#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
// #![feature(const_transmute)]
#![feature(extern_types)]
#![feature(linkage)]
#![feature(ptr_offset_from)]
#![feature(register_tool)]
#![register_tool(c2rust)]
#![feature(c_variadic)]







pub mod blocksort;
pub mod bzip2;
pub mod bzip2recover;
pub mod bzlib;
pub mod compress;
pub mod crctable;
pub mod decompress;
pub mod huffman;
pub mod randtable;

