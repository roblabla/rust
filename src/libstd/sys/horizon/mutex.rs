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
use intrinsics::atomic_cxchg;

pub unsafe fn mutex_try_lock(m: *mut i32) -> bool {
    atomic_cxchg(m, 0, 1).0 == 0
}

// TODO: Create stupid mutex (that just yields in a while loop)
// NOTE: We're in a cooperatively scheduled environment
pub unsafe fn mutex_lock(_m: *mut i32) {
}

pub unsafe fn mutex_unlock(_m: *mut i32) {
}

pub struct Mutex {
    pub lock: UnsafeCell<i32>,
}

impl Mutex {
    /// Create a new mutex.
    pub const fn new() -> Self {
        Mutex {
            lock: UnsafeCell::new(0),
        }
    }

    #[inline]
    pub unsafe fn init(&self) {
        *self.lock.get() = 0;
    }

    /// Try to lock the mutex
    #[inline]
    pub unsafe fn try_lock(&self) -> bool {
        mutex_try_lock(self.lock.get())
    }

    /// Lock the mutex
    #[inline]
    pub unsafe fn lock(&self) {
        mutex_lock(self.lock.get());
    }

    /// Unlock the mutex
    #[inline]
    pub unsafe fn unlock(&self) {
        mutex_unlock(self.lock.get());
    }

    #[inline]
    pub unsafe fn destroy(&self) {
        *self.lock.get() = 0;
    }
}

unsafe impl Send for Mutex {}

unsafe impl Sync for Mutex {}

pub struct ReentrantMutex {
    pub lock: UnsafeCell<i32>,
    pub owner: UnsafeCell<usize>,
    pub own_count: UnsafeCell<usize>,
}

impl ReentrantMutex {
    pub const fn uninitialized() -> Self {
        ReentrantMutex {
            lock: UnsafeCell::new(0),
            owner: UnsafeCell::new(0),
            own_count: UnsafeCell::new(0),
        }
    }

    #[inline]
    pub unsafe fn init(&mut self) {
        *self.lock.get() = 0;
        *self.owner.get() = 0;
        *self.own_count.get() = 0;
    }

    /// Try to lock the mutex
    #[inline]
    pub unsafe fn try_lock(&self) -> bool {
        // TODO: Implement this. Need raw syscall to get thread id, using
        // thread::id is not possible, it uses a mutex too :(
        mutex_try_lock(self.lock.get())
    }

    /// Lock the mutex
    #[inline]
    pub unsafe fn lock(&self) {
        mutex_lock(self.lock.get())
    }

    /// Unlock the mutex
    #[inline]
    pub unsafe fn unlock(&self) {
        mutex_unlock(self.lock.get())
    }

    #[inline]
    pub unsafe fn destroy(&self) {
        *self.lock.get() = 0;
        *self.owner.get() = 0;
        *self.own_count.get() = 0;
    }
}

unsafe impl Send for ReentrantMutex {}

unsafe impl Sync for ReentrantMutex {}
