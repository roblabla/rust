// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub unsafe fn mutex_try_lock(_m: *mut i32) -> bool {
    unimplemented!()
}

pub unsafe fn mutex_lock(_m: *mut i32) {
    unimplemented!()
}

pub unsafe fn mutex_unlock(_m: *mut i32) {
    unimplemented!()
}

pub struct Mutex;

impl Mutex {
    /// Create a new mutex.
    pub const fn new() -> Self {
        // TODO: Sadly can't panic in constfn yet
        Mutex
    }

    #[inline]
    pub unsafe fn init(&self) {
        unimplemented!()
    }

    /// Try to lock the mutex
    #[inline]
    pub unsafe fn try_lock(&self) -> bool {
        unimplemented!()
    }

    /// Lock the mutex
    #[inline]
    pub unsafe fn lock(&self) {
        unimplemented!()
    }

    /// Unlock the mutex
    #[inline]
    pub unsafe fn unlock(&self) {
        unimplemented!()
    }

    #[inline]
    pub unsafe fn destroy(&self) {
        unimplemented!()
    }
}

pub struct ReentrantMutex;

impl ReentrantMutex {
    pub const fn uninitialized() -> Self {
        // TODO: Sadly can't panic in constfn yet
        ReentrantMutex
    }

    #[inline]
    pub unsafe fn init(&mut self) {
        unimplemented!()
    }

    /// Try to lock the mutex
    #[inline]
    pub unsafe fn try_lock(&self) -> bool {
        unimplemented!()
    }

    /// Lock the mutex
    #[inline]
    pub unsafe fn lock(&self) {
        unimplemented!()
    }

    /// Unlock the mutex
    #[inline]
    pub unsafe fn unlock(&self) {
        unimplemented!()
    }

    #[inline]
    pub unsafe fn destroy(&self) {
        unimplemented!()
    }
}
