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
use net::{SocketAddr, Shutdown};
use time::Duration;

#[derive(Debug)]
pub struct TcpStream;

impl TcpStream {
    pub fn connect(_addr: &SocketAddr) -> Result<TcpStream> {
        Err(Error::new(ErrorKind::Other, "TcpStream::connect not implemented"))
    }

    pub fn connect_timeout(_addr: &SocketAddr, _timeout: Duration) -> Result<TcpStream> {
        Err(Error::new(ErrorKind::Other, "TcpStream::connect_timeout not implemented"))
    }

    pub fn duplicate(&self) -> Result<TcpStream> {
        Err(Error::new(ErrorKind::Other, "TcpStream::duplicate not implemented"))
    }

    pub fn read(&self, _buf: &mut [u8]) -> Result<usize> {
        Err(Error::new(ErrorKind::Other, "TcpStream::read not implemented"))
    }

    pub fn write(&self, _buf: &[u8]) -> Result<usize> {
        Err(Error::new(ErrorKind::Other, "TcpStream::write not implemented"))
    }

    pub fn take_error(&self) -> Result<Option<Error>> {
        Err(Error::new(ErrorKind::Other, "TcpStream::take_error not implemented"))
    }

    pub fn peer_addr(&self) -> Result<SocketAddr> {
        Err(Error::new(ErrorKind::Other, "TcpStream::peer_addr not implemented"))
    }

    pub fn socket_addr(&self) -> Result<SocketAddr> {
        Err(Error::new(ErrorKind::Other, "TcpStream::socket_addr not implemented"))
    }

    pub fn peek(&self, _buf: &mut [u8]) -> Result<usize> {
        Err(Error::new(ErrorKind::Other, "TcpStream::peek not implemented"))
    }

    pub fn shutdown(&self, _how: Shutdown) -> Result<()> {
        Err(Error::new(ErrorKind::Other, "TcpStream::shutdown not implemented"))
    }

    pub fn nodelay(&self) -> Result<bool> {
        Err(Error::new(ErrorKind::Other, "TcpStream::nodelay not implemented"))
    }

    pub fn ttl(&self) -> Result<u32> {
        Err(Error::new(ErrorKind::Other, "TcpStream::ttl not implemented"))
    }

    pub fn read_timeout(&self) -> Result<Option<Duration>> {
        Err(Error::new(ErrorKind::Other, "TcpStream::read_timeout not implemented"))
    }

    pub fn write_timeout(&self) -> Result<Option<Duration>> {
        Err(Error::new(ErrorKind::Other, "TcpStream::write_timeout not implemented"))
    }

    pub fn set_nodelay(&self, _nodelay: bool) -> Result<()> {
        Err(Error::new(ErrorKind::Other, "TcpStream::set_nodelay not implemented"))
    }

    pub fn set_nonblocking(&self, _nonblocking: bool) -> Result<()> {
        Err(Error::new(ErrorKind::Other, "TcpStream::set_nonblocking not implemented"))
    }

    pub fn set_ttl(&self, _ttl: u32) -> Result<()> {
        Err(Error::new(ErrorKind::Other, "TcpStream::set_ttl not implemented"))
    }

    pub fn set_read_timeout(&self, _duration_option: Option<Duration>) -> Result<()> {
        Err(Error::new(ErrorKind::Other, "TcpStream::set_read_timeout not implemented"))
    }

    pub fn set_write_timeout(&self, _duration_option: Option<Duration>) -> Result<()> {
        Err(Error::new(ErrorKind::Other, "TcpStream::set_write_timeout not implemented"))
    }
}

#[derive(Debug)]
pub struct TcpListener;

impl TcpListener {
    pub fn bind(_addr: &SocketAddr) -> Result<TcpListener> {
        Err(Error::new(ErrorKind::Other, "TcpListener::bind not implemented"))
    }

    pub fn accept(&self) -> Result<(TcpStream, SocketAddr)> {
        Err(Error::new(ErrorKind::Other, "TcpListener::accept not implemented"))
    }

    pub fn duplicate(&self) -> Result<TcpListener> {
        Err(Error::new(ErrorKind::Other, "TcpListener::duplicate not implemented"))
    }

    pub fn take_error(&self) -> Result<Option<Error>> {
        Err(Error::new(ErrorKind::Other, "TcpListener::take_error not implemented"))
    }

    pub fn socket_addr(&self) -> Result<SocketAddr> {
        Err(Error::new(ErrorKind::Other, "TcpListener::socket_addr not implemented"))
    }

    pub fn only_v6(&self) -> Result<bool> {
        Err(Error::new(ErrorKind::Other, "TcpListener::only_v6 not implemented"))
    }

    pub fn ttl(&self) -> Result<u32> {
        Err(Error::new(ErrorKind::Other, "TcpListener::ttl not implemented"))
    }

    pub fn set_nonblocking(&self, _nonblocking: bool) -> Result<()> {
        Err(Error::new(ErrorKind::Other, "TcpListener::set_nonblocking not implemented"))
    }

    pub fn set_only_v6(&self, _only_v6: bool) -> Result<()> {
        Err(Error::new(ErrorKind::Other, "TcpListener::set_only_v6 not implemented"))
    }

    pub fn set_ttl(&self, _ttl: u32) -> Result<()> {
        Err(Error::new(ErrorKind::Other, "TcpListener::set_ttl not implemented"))
    }
}
