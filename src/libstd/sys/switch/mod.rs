// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! System bindings for the Nintendo Switch platform
//!
//! This module contains the facade (aka platform-specific) implementations of
//! OS level functionality for the Nintendo Switch.
//!
//! Currently all functions here are basically stubs that immediately return
//! errors.

#![allow(dead_code, missing_docs, bad_style)]

use io::{self, ErrorKind};

pub mod args;
// TODO
#[cfg(feature = "backtrace")]
pub mod backtrace;
pub mod cmath;
pub mod condvar;
pub mod env;
pub mod fs;
pub mod memchr;
pub mod mutex;
pub mod net;
pub mod os;
pub mod os_str;
pub mod path;
pub mod pipe;
pub mod process;
pub mod rwlock;
pub mod stack_overflow;
pub mod stdio;
pub mod thread;
pub mod thread_local;
pub mod time;

#[cfg(not(test))]
pub fn init() {}

pub fn unsupported<T>() -> io::Result<T> {
    Err(unsupported_err())
}

pub fn unsupported_err() -> io::Error {
    io::Error::new(io::ErrorKind::Other,
                   "operation not supported on wasm yet")
}

pub fn decode_error_kind(_errno: i32) -> ErrorKind {
    io::ErrorKind::Other
}

/// TODO: Do a proper abort using the exit stuff.
pub unsafe fn abort_internal() -> ! {
    ::core::intrinsics::abort();
}

// This enum is used as the storage for a bunch of types which can't actually
// exist.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Void {}

pub unsafe fn strlen(mut s: *const i8) -> usize {
    let mut n = 0;
    while *s != 0 {
        n += 1;
        s = s.offset(1);
    }
    return n
}

// We don't have randomness yet, but I totally used a random number generator to
// generate these numbers.
//
// More seriously though this is just for DOS protection in hash maps. It's ok
// if we don't do that on wasm just yet.
pub fn hashmap_random_keys() -> (u64, u64) {
    (1, 2)
}
