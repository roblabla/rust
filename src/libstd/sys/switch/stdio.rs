// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use io;
use sys::unsupported;

pub struct Stdin;
pub struct Stdout;
pub struct Stderr;

impl Stdin {
    pub fn new() -> io::Result<Stdin> {
        unsupported()
    }

    pub fn read(&self, _data: &mut [u8]) -> io::Result<usize> {
        unsupported()
    }
}

impl Stdout {
    pub fn new() -> io::Result<Stdout> {
        unsupported()
    }

    pub fn write(&self, _data: &[u8]) -> io::Result<usize> {
        unsupported()
    }

    pub fn flush(&self) -> io::Result<()> {
        unsupported()
    }
}

impl Stderr {
    pub fn new() -> io::Result<Stderr> {
        unsupported()
    }

    pub fn write(&self, _data: &[u8]) -> io::Result<usize> {
        unsupported()
    }

    pub fn flush(&self) -> io::Result<()> {
        unsupported()
    }
}

impl io::Write for Stderr {
    fn write(&mut self, _data: &[u8]) -> io::Result<usize> {
        unsupported()
    }
    fn flush(&mut self) -> io::Result<()> {
        unsupported()
    }
}

pub const STDIN_BUF_SIZE: usize = 0;

pub fn is_ebadf(_err: &io::Error) -> bool {
    true
}
