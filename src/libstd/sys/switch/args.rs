// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Global initialization and retrieval of command line arguments.
//!
//! On some platforms these are stored during runtime startup,
//! and on some they are retrieved from the system on demand.
//!
//! In our case, they're in the megaton_hammer Loader. But I don't need them
//! yet.

#![allow(dead_code)] // runtime init functions not used during testing

use ffi::OsString;

/// One-time global initialization.
pub unsafe fn init(_argc: isize, _argv: *const *const u8) { }

/// One-time global cleanup.
pub unsafe fn cleanup() { }

/// Returns the command line arguments
pub fn args() -> Args {
    Args { empty_array: [] }
}

pub struct Args {
    empty_array: [OsString; 0],
}

impl Args {
    pub fn inner_debug(&self) -> &[OsString] {
        &self.empty_array
    }
}

impl Iterator for Args {
    type Item = OsString;
    fn next(&mut self) -> Option<OsString> { None }
    fn size_hint(&self) -> (usize, Option<usize>) { (0, Some(0)) }
}

impl ExactSizeIterator for Args {
    fn len(&self) -> usize { 0 }
}

impl DoubleEndedIterator for Args {
    fn next_back(&mut self) -> Option<OsString> { None }
}
