#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
#![feature(alloc_error_handler)]
#![feature(allocator_api)]
#![feature(array_chunks)]
#![feature(asm_const)]
#![feature(c_variadic)]
#![feature(core_intrinsics)]
#![feature(lang_items)]
#![feature(linkage)]
#![feature(stmt_expr_attributes)]
#![feature(str_internals)]
#![feature(thread_local)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::cast_ptr_alignment)]
#![allow(clippy::derive_hash_xor_eq)]
#![allow(clippy::eval_order_dependence)]
#![allow(clippy::mut_from_ref)]
// TODO: fix these
// #![warn(unaligned_references)]

#[macro_use]
extern crate alloc;
extern crate cbitset;
extern crate core_io;
extern crate goblin;
#[macro_use]
extern crate lazy_static;
extern crate memchr;
#[macro_use]
extern crate memoffset;
extern crate posix_regex;
extern crate rand;

#[macro_use]
pub mod macros;
pub mod c_str;
pub mod c_vec;
pub mod cxa;
pub mod db;
pub mod fs;
pub mod header;
pub mod io;
pub mod platform;
