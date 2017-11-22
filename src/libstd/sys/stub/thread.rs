// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use alloc::boxed::FnBox;
use ffi::CStr;
use io;
use io::{Error, ErrorKind};
use time::Duration;

pub const DEFAULT_MIN_STACK_SIZE: usize = 2 * 1024 * 1024;

pub struct Thread;

// Some platforms may have pthread_t as a pointer in which case we still want
// a thread to be Send/Sync
unsafe impl Send for Thread {}
unsafe impl Sync for Thread {}

impl Thread {
    pub unsafe fn new<'a>(_stack: usize, _p: Box<FnBox() + 'a>) -> io::Result<Thread> {
        Err(Error::new(ErrorKind::Other, "Thread::new not implemented"))
    }

    pub fn yield_now() {
        unimplemented!();
    }

    pub fn set_name(_name: &CStr) {

    }

    pub fn sleep(_dur: Duration) {
        unimplemented!();
    }

    pub fn join(self) {
        unimplemented!();
    }
}

pub mod guard {
    pub unsafe fn current() -> Option<usize> { None }
    pub unsafe fn init() -> Option<usize> { None }
}
