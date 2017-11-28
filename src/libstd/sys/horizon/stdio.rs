// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use io;
use sys::cvt;
use libc;

pub struct Stdin;
pub struct Stdout;
pub struct Stderr;

impl Stdin {
    pub fn new() -> io::Result<Stdin> { Ok(Stdin) }

    pub fn read(&self, data: &mut [u8]) -> io::Result<usize> {
        unsafe {
            // TODO: Tuck the unsafe inside a "FileDesc" struct
            let ret = cvt(libc::read(libc::STDIN_FILENO, data.as_mut_ptr() as *mut _, data.len()))?;
            Ok(ret as usize)
        }
    }
}

impl Stdout {
    pub fn new() -> io::Result<Stdout> { Ok(Stdout) }

    pub fn write(&self, data: &[u8]) -> io::Result<usize> {
        unsafe {
            let ret = cvt(libc::write(libc::STDOUT_FILENO, data.as_ptr() as *const _, data.len()))?;
            Ok(ret as usize)
        }
    }

    pub fn flush(&self) -> io::Result<()> {
        Ok(())
    }
}

impl Stderr {
    pub fn new() -> io::Result<Stderr> { Ok(Stderr) }

    pub fn write(&self, data: &[u8]) -> io::Result<usize> {
        unsafe {
            let ret = cvt(libc::write(libc::STDERR_FILENO, data.as_ptr() as *const _, data.len()))?;
            Ok(ret as usize)
        }
    }

    pub fn flush(&self) -> io::Result<()> {
        Ok(())
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
