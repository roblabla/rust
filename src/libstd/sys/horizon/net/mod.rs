// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use io::{Error, ErrorKind, Result};
use iter::Iterator;
use net::{SocketAddr};
use vec::IntoIter;

pub use self::tcp::{TcpStream, TcpListener};
pub use self::udp::UdpSocket;

pub mod netc;

mod dns;
mod tcp;
mod udp;

pub struct LookupHost(IntoIter<SocketAddr>);

impl Iterator for LookupHost {
    type Item = SocketAddr;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

pub fn lookup_host(_host: &str) -> Result<LookupHost> {
    Err(Error::new(ErrorKind::Other, "lookup_host not implemented"))
}
