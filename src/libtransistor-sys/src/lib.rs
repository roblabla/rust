#![feature(libc)]
#![no_std]
extern crate libc;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
//include!("bindings.rs");
