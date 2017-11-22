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

////////////////////////////////////////////////////////////////////////////////
// Anonymous pipes
////////////////////////////////////////////////////////////////////////////////

pub struct AnonPipe;

/*pub fn anon_pipe() -> io::Result<(AnonPipe, AnonPipe)> {
    Err(Error::new(ErrorKind::Other, "anon_pipe not implemented"))
}*/

impl AnonPipe {
    pub fn read(&self, _buf: &mut [u8]) -> io::Result<usize> {
        Err(Error::new(ErrorKind::Other, "AnonPipe::read not implemented"))
    }

    pub fn write(&self, _buf: &[u8]) -> io::Result<usize> {
        Err(Error::new(ErrorKind::Other, "AnonPipe::write not implemented"))
    }
}

pub fn read2(_p1: AnonPipe,
             _v1: &mut Vec<u8>,
             _p2: AnonPipe,
             _v2: &mut Vec<u8>) -> io::Result<()> {
    Err(Error::new(ErrorKind::Other, "read2 not implemented"))
}
