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
use io::{Error, ErrorKind, Result};
use net::{SocketAddr, Ipv4Addr, Ipv6Addr};
use time::Duration;

#[derive(Debug)]
pub struct UdpSocket((), UnsafeCell<Option<SocketAddr>>);

impl UdpSocket {
    pub fn bind(_addr: &SocketAddr) -> Result<UdpSocket> {
        Err(Error::new(ErrorKind::Other, "UdpSocket::bind not implemented"))
    }

    pub fn connect(&self, addr: &SocketAddr) -> Result<()> {
        unsafe { *self.1.get() = Some(*addr) };
        Ok(())
    }

    pub fn duplicate(&self) -> Result<UdpSocket> {
        Err(Error::new(ErrorKind::Other, "UdpSocket::duplicate not implemented"))
    }

    pub fn recv_from(&self, _buf: &mut [u8]) -> Result<(usize, SocketAddr)> {
        Err(Error::new(ErrorKind::Other, "UdpSocket::recv_from not implemented"))
    }

    pub fn recv(&self, _buf: &mut [u8]) -> Result<usize> {
        Err(Error::new(ErrorKind::Other, "UdpSocket::recv not implemented"))
    }

    pub fn send_to(&self, _buf: &[u8], _addr: &SocketAddr) -> Result<usize> {
        Err(Error::new(ErrorKind::Other, "UdpSocket::send_to not implemented"))
    }

    pub fn send(&self, _buf: &[u8]) -> Result<usize> {
        Err(Error::new(ErrorKind::Other, "UdpSocket::send not implemented"))
    }

    pub fn take_error(&self) -> Result<Option<Error>> {
        Err(Error::new(ErrorKind::Other, "UdpSocket::take_error not implemented"))
    }

    pub fn socket_addr(&self) -> Result<SocketAddr> {
        Err(Error::new(ErrorKind::Other, "UdpSocket::socket_addr not implemented"))
    }

    pub fn peek(&self, _buf: &mut [u8]) -> Result<usize> {
        Err(Error::new(ErrorKind::Other, "UdpSocket::peek not implemented"))
    }

    pub fn peek_from(&self, _buf: &mut [u8]) -> Result<(usize, SocketAddr)> {
        Err(Error::new(ErrorKind::Other, "UdpSocket::peek_from not implemented"))
    }

    pub fn broadcast(&self) -> Result<bool> {
        Err(Error::new(ErrorKind::Other, "UdpSocket::broadcast not implemented"))
    }

    pub fn multicast_loop_v4(&self) -> Result<bool> {
        Err(Error::new(ErrorKind::Other, "UdpSocket::multicast_loop_v4 not implemented"))
    }

    pub fn multicast_loop_v6(&self) -> Result<bool> {
        Err(Error::new(ErrorKind::Other, "UdpSocket::multicast_loop_v6 not implemented"))
    }

    pub fn multicast_ttl_v4(&self) -> Result<u32> {
        Err(Error::new(ErrorKind::Other, "UdpSocket::multicast_ttl_v4 not implemented"))
    }

    pub fn ttl(&self) -> Result<u32> {
        Err(Error::new(ErrorKind::Other, "UdpSocket::ttl not implemented"))
    }

    pub fn read_timeout(&self) -> Result<Option<Duration>> {
        Err(Error::new(ErrorKind::Other, "UdpSocket::read_timeout not implemented"))
    }

    pub fn write_timeout(&self) -> Result<Option<Duration>> {
        Err(Error::new(ErrorKind::Other, "UdpSocket::write_timeout not implemented"))
    }

    pub fn set_broadcast(&self, _broadcast: bool) -> Result<()> {
        Err(Error::new(ErrorKind::Other, "UdpSocket::set_broadcast not implemented"))
    }

    pub fn set_multicast_loop_v4(&self, _multicast_loop_v4: bool) -> Result<()> {
        Err(Error::new(ErrorKind::Other, "UdpSocket::set_multicast_loop_v4 not implemented"))
    }

    pub fn set_multicast_loop_v6(&self, _multicast_loop_v6: bool) -> Result<()> {
        Err(Error::new(ErrorKind::Other, "UdpSocket::set_multicast_loop_v6 not implemented"))
    }

    pub fn set_multicast_ttl_v4(&self, _multicast_ttl_v4: u32) -> Result<()> {
        Err(Error::new(ErrorKind::Other, "UdpSocket::set_multicast_ttl_v4 not implemented"))
    }

    pub fn set_nonblocking(&self, _nonblocking: bool) -> Result<()> {
        Err(Error::new(ErrorKind::Other, "UdpSocket::set_nonblocking not implemented"))
    }

    pub fn set_ttl(&self, _ttl: u32) -> Result<()> {
        Err(Error::new(ErrorKind::Other, "UdpSocket::set_ttl not implemented"))
    }

    pub fn set_read_timeout(&self, _duration_option: Option<Duration>) -> Result<()> {
        Err(Error::new(ErrorKind::Other, "UdpSocket::set_read_timeout not implemented"))
    }

    pub fn set_write_timeout(&self, _duration_option: Option<Duration>) -> Result<()> {
        Err(Error::new(ErrorKind::Other, "UdpSocket::set_write_timeout not implemented"))
    }

    pub fn join_multicast_v4(&self, _multiaddr: &Ipv4Addr, _interface: &Ipv4Addr) -> Result<()> {
        Err(Error::new(ErrorKind::Other, "UdpSocket::join_multicast_v4 not implemented"))
    }

    pub fn join_multicast_v6(&self, _multiaddr: &Ipv6Addr, _interface: u32) -> Result<()> {
        Err(Error::new(ErrorKind::Other, "UdpSocket::join_multicast_v6 not implemented"))
    }

    pub fn leave_multicast_v4(&self, _multiaddr: &Ipv4Addr, _interface: &Ipv4Addr) -> Result<()> {
        Err(Error::new(ErrorKind::Other, "UdpSocket::leave_multicast_v4 not implemented"))
    }

    pub fn leave_multicast_v6(&self, _multiaddr: &Ipv6Addr, _interface: u32) -> Result<()> {
        Err(Error::new(ErrorKind::Other, "UdpSocket::leave_multicast_v6 not implemented"))
    }
}
