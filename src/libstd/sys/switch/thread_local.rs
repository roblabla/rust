// Copyright 2014-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(dead_code)] // not used on all platforms

pub type Key = usize;

#[inline]
pub unsafe fn create(_dtor: Option<unsafe extern fn(*mut u8)>) -> Key {
    unimplemented!()
}

#[inline]
pub unsafe fn set(_key: Key, _value: *mut u8) {
    unimplemented!()
}

#[inline]
pub unsafe fn get(_key: Key) -> *mut u8 {
    unimplemented!()
}

#[inline]
pub unsafe fn destroy(_key: Key) {
    unimplemented!()
}

#[inline]
pub fn requires_synchronized_create() -> bool {
    false
}
