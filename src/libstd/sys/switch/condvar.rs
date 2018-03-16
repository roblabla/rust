// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use sys::mutex::Mutex;
use time::Duration;

pub struct Condvar;

impl Condvar {
    pub const fn new() -> Condvar {
        // TODO: Sadly can't panic in const fn yet.
        Condvar
    }

    #[inline]
    pub unsafe fn init(&self) {
        unimplemented!()
    }

    #[inline]
    pub fn notify_one(&self) {
        unimplemented!()
    }

    #[inline]
    pub fn notify_all(&self) {
        unimplemented!()
    }

    #[inline]
    pub fn wait(&self, _mutex: &Mutex) {
        unimplemented!()
    }

    #[inline]
    pub fn wait_timeout(&self, _mutex: &Mutex, _dur: Duration) -> bool {
        unimplemented!();
    }

    #[inline]
    pub unsafe fn destroy(&self) {
        unimplemented!()
    }
}
