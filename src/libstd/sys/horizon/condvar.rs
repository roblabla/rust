// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use cell::UnsafeCell;
use ptr;
use time::Duration;

use sys::mutex::{Mutex};

pub struct Condvar {
    lock: UnsafeCell<*mut i32>,
    seq: UnsafeCell<i32>
}

// TODO: Use sched_yield
impl Condvar {
    pub const fn new() -> Condvar {
        // TODO: Can't call unimplemented here
        Condvar {
            lock: UnsafeCell::new(ptr::null_mut()),
            seq: UnsafeCell::new(0)
        }
    }

    #[inline]
    pub unsafe fn init(&self) {
        *self.lock.get() = ptr::null_mut();
        *self.seq.get() = 0;
    }

    #[inline]
    pub fn notify_one(&self) {
        unimplemented!();
    }

    #[inline]
    pub fn notify_all(&self) {
        unimplemented!();
    }

    #[inline]
    pub fn wait(&self, _mutex: &Mutex) {
        unimplemented!();
    }

    #[inline]
    pub fn wait_timeout(&self, _mutex: &Mutex, _dur: Duration) -> bool {
        unimplemented!();
    }

    #[inline]
    pub unsafe fn destroy(&self) {
        *self.lock.get() = ptr::null_mut();
        *self.seq.get() = 0;
    }
}

unsafe impl Send for Condvar {}

unsafe impl Sync for Condvar {}
