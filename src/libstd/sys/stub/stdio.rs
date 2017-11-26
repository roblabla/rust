// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use io::{self, Error, ErrorKind};

pub struct Stdin(());
pub struct Stdout(());
pub struct Stderr(());

impl Stdin {
    pub fn new() -> io::Result<Stdin> { Ok(Stdin(())) }

    pub fn read(&self, _data: &mut [u8]) -> io::Result<usize> {
        Err(Error::new(ErrorKind::Other, "Stdin::read not implemented"))
    }
}

impl Stdout {
    pub fn new() -> io::Result<Stdout> { Ok(Stdout(())) }

    pub fn write(&self, _data: &[u8]) -> io::Result<usize> {
        Err(Error::new(ErrorKind::Other, "Stdout::write not implemented"))
    }

    pub fn flush(&self) -> io::Result<()> {
        Err(Error::new(ErrorKind::Other, "Stdout::flush not implemented"))
    }
}

impl Stderr {
    pub fn new() -> io::Result<Stderr> { Ok(Stderr(())) }

    pub fn write(&self, _data: &[u8]) -> io::Result<usize> {
        Err(Error::new(ErrorKind::Other, "Stderr::write not implemented"))
    }

    pub fn flush(&self) -> io::Result<()> {
        Err(Error::new(ErrorKind::Other, "Stderr::flush not implemented"))
    }
}

// FIXME: right now this raw stderr handle is used in a few places because
//        std::io::stderr_raw isn't exposed, but once that's exposed this impl
//        should go away
impl io::Write for Stderr {
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        Stderr::write(self, data)
    }

    fn flush(&mut self) -> io::Result<()> {
        Stderr::flush(self)
    }
}

pub fn is_ebadf(_err: &io::Error) -> bool {
    false
}

pub const STDIN_BUF_SIZE: usize = ::sys_common::io::DEFAULT_BUF_SIZE;
