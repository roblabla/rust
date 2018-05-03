// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use fmt;
use io::{self, Write};
use ffi::{CStr, CString};
use net::{SocketAddr, Shutdown, Ipv4Addr, Ipv6Addr};
use time::Duration;
use sys::{unsupported, Void};
use sys_common::{FromInner, IntoInner};
use self::netc as c;
use mem;
use slice;
use sync::Arc;
use megaton_hammer::kernel::Session;
use megaton_ipc::nn::socket::sf::IClient;
use megaton_ipc::nn::socket::resolver::IResolver;

pub struct TcpStream(Arc<IClient<Session>>, u32);

macro_rules! handle_err {
    ($x: expr) => {{
        let val = try_mth!($x);
        if val.0 == -1 {
            return Err(io::Error::from_raw_os_error(val.1 as i32));
        } else {
            val
        }
    }}
}

macro_rules! try_mth {
    ($x: expr) => {
        match $x {
            Ok(val) => val,
            Err(err) => {
                return Err(io::Error::new(io::ErrorKind::Other, Box::new(err)))
            }
        }
    }
}

fn sockname<F>(f: F) -> io::Result<SocketAddr>
    where F: FnOnce(*mut c::sockaddr, *mut c::socklen_t) -> io::Result<()>
{
    unsafe {
        let mut storage: c::sockaddr_storage = mem::zeroed();
        let mut len = mem::size_of_val(&storage) as c::socklen_t;
        f(&mut storage as *mut _ as *mut _, &mut len)?;
        sockaddr_to_addr(&storage, len as usize)
    }
}

pub fn sockaddr_to_addr(storage: &c::sockaddr_storage,
                    len: usize) -> io::Result<SocketAddr> {
    match storage.sa_family {
        c::AF_INET => {
            assert!(len as usize >= mem::size_of::<c::sockaddr_in>());
            Ok(SocketAddr::V4(FromInner::from_inner(unsafe {
                *(storage as *const _ as *const c::sockaddr_in)
            })))
        }
        c::AF_INET6 => {
            assert!(len as usize >= mem::size_of::<c::sockaddr_in6>());
            Ok(SocketAddr::V6(FromInner::from_inner(unsafe {
                *(storage as *const _ as *const c::sockaddr_in6)
            })))
        }
        _ => {
            Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid argument"))
        }
    }
}

impl TcpStream {
    pub fn connect(addr: &SocketAddr) -> io::Result<TcpStream> {
        let bsd = if let Ok(bsd) = IClient::new_bsd_u() {
            bsd
        } else {
            try_mth!(IClient::new_bsd_s())
        };

        if let &SocketAddr::V6(_) = addr {
            return unsupported()
        }

        let (socket, _) = handle_err!(bsd.socket(netc::AF_INET as u32, 1, 0)); // SOCK_STREAM

        let (addrp, _) = addr.into_inner();
        let _ = handle_err!(bsd.connect(socket as u32, unsafe { &*addrp }));

        Ok(TcpStream(bsd, socket as u32))
    }

    pub fn connect_timeout(_: &SocketAddr, _: Duration) -> io::Result<TcpStream> {
        // TODO: Use set_nonblocking, which itself does some magic...
        unsupported()
    }

    pub fn set_read_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        // TODO: setsockopt magic stuff
        unsupported()
    }

    pub fn set_write_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        // TODO: setsockopt magic stuff
        unsupported()
    }

    pub fn read_timeout(&self) -> io::Result<Option<Duration>> {
        // TODO: setsockopt magic stuff
        unsupported()
    }

    pub fn write_timeout(&self) -> io::Result<Option<Duration>> {
        // TODO: setsockopt magic stuff
        unsupported()
    }

    fn recv_with_flags(&self, buf: &mut [u8], flags: u32) -> io::Result<usize> {
        let (ret, _) = handle_err!(self.0.recv(self.1, flags, unsafe {
            slice::from_raw_parts_mut(buf as *mut [u8] as *mut u8 as *mut i8, buf.len())
        }));
        Ok(ret as usize)
    }

    pub fn peek(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.recv_with_flags(buf, 2) // MSG_PEEK, as defined by linux and freebsd.
    }

    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.recv_with_flags(buf, 0)
    }

    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        // TODO: let len = cmp::min(buf.len(), <wrlen_t>::max_value() as usize) as wrlen_t;
        // TODO: The standard implementation uses MSG_NOSIGNAL, which avoids
        // generating SIGPIPE on EOF. The switch has no notion of signals
        // however, so this seems to be a NOOP.
        let (ret, _) = handle_err!(self.0.send(self.1, 0, unsafe {
            slice::from_raw_parts(buf as *const [u8] as *const u8 as *const i8, buf.len())
        }));
        Ok(ret as usize)
    }

    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        sockname(|buf, _len| unsafe {
            handle_err!(self.0.get_peer_name(self.1, &mut *buf));
            Ok(())
        })
    }

    pub fn socket_addr(&self) -> io::Result<SocketAddr> {
        sockname(|buf, _len| unsafe {
            handle_err!(self.0.get_sock_name(self.1, &mut *buf));
            Ok(())
        })
    }

    pub fn shutdown(&self, how: Shutdown) -> io::Result<()> {
        // True in both freebsd and linux
        let how = match how {
            Shutdown::Write => 1, // SHUT_WR
            Shutdown::Read => 0,
            Shutdown::Both => 2,
        };
        handle_err!(self.0.shutdown(self.1, how));
        Ok(())
    }

    pub fn duplicate(&self) -> io::Result<TcpStream> {
        let (socket, _) = handle_err!(self.0.duplicate_socket(self.1, 0));
        Ok(TcpStream(self.0.clone(), socket as u32))
    }

    pub fn set_nodelay(&self, _: bool) -> io::Result<()> {
        unsupported()
    }

    pub fn nodelay(&self) -> io::Result<bool> {
        unsupported()
    }

    pub fn set_ttl(&self, _: u32) -> io::Result<()> {
        unsupported()
    }

    pub fn ttl(&self) -> io::Result<u32> {
        unsupported()
    }

    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        unsupported()
    }

    pub fn set_nonblocking(&self, _: bool) -> io::Result<()> {
        unsupported()
    }
}

impl fmt::Debug for TcpStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res = f.debug_struct("TcpStream");

        if let Ok(addr) = self.socket_addr() {
            res.field("addr", &addr);
        }

        if let Ok(peer) = self.peer_addr() {
            res.field("peer", &peer);
        }

        res.field("fd", &self.1)
            .finish()
    }
}

pub struct TcpListener(Void);

impl TcpListener {
    pub fn bind(_: &SocketAddr) -> io::Result<TcpListener> {
        unsupported()
    }

    pub fn socket_addr(&self) -> io::Result<SocketAddr> {
        match self.0 {}
    }

    pub fn accept(&self) -> io::Result<(TcpStream, SocketAddr)> {
        match self.0 {}
    }

    pub fn duplicate(&self) -> io::Result<TcpListener> {
        match self.0 {}
    }

    pub fn set_ttl(&self, _: u32) -> io::Result<()> {
        match self.0 {}
    }

    pub fn ttl(&self) -> io::Result<u32> {
        match self.0 {}
    }

    pub fn set_only_v6(&self, _: bool) -> io::Result<()> {
        match self.0 {}
    }

    pub fn only_v6(&self) -> io::Result<bool> {
        match self.0 {}
    }

    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        match self.0 {}
    }

    pub fn set_nonblocking(&self, _: bool) -> io::Result<()> {
        match self.0 {}
    }
}

impl fmt::Debug for TcpListener {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {}
    }
}

pub struct UdpSocket(Void);

impl UdpSocket {
    pub fn bind(_: &SocketAddr) -> io::Result<UdpSocket> {
        unsupported()
    }

    pub fn socket_addr(&self) -> io::Result<SocketAddr> {
        match self.0 {}
    }

    pub fn recv_from(&self, _: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        match self.0 {}
    }

    pub fn peek_from(&self, _: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        match self.0 {}
    }

    pub fn send_to(&self, _: &[u8], _: &SocketAddr) -> io::Result<usize> {
        match self.0 {}
    }

    pub fn duplicate(&self) -> io::Result<UdpSocket> {
        match self.0 {}
    }

    pub fn set_read_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        match self.0 {}
    }

    pub fn set_write_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        match self.0 {}
    }

    pub fn read_timeout(&self) -> io::Result<Option<Duration>> {
        match self.0 {}
    }

    pub fn write_timeout(&self) -> io::Result<Option<Duration>> {
        match self.0 {}
    }

    pub fn set_broadcast(&self, _: bool) -> io::Result<()> {
        match self.0 {}
    }

    pub fn broadcast(&self) -> io::Result<bool> {
        match self.0 {}
    }

    pub fn set_multicast_loop_v4(&self, _: bool) -> io::Result<()> {
        match self.0 {}
    }

    pub fn multicast_loop_v4(&self) -> io::Result<bool> {
        match self.0 {}
    }

    pub fn set_multicast_ttl_v4(&self, _: u32) -> io::Result<()> {
        match self.0 {}
    }

    pub fn multicast_ttl_v4(&self) -> io::Result<u32> {
        match self.0 {}
    }

    pub fn set_multicast_loop_v6(&self, _: bool) -> io::Result<()> {
        match self.0 {}
    }

    pub fn multicast_loop_v6(&self) -> io::Result<bool> {
        match self.0 {}
    }

    pub fn join_multicast_v4(&self, _: &Ipv4Addr, _: &Ipv4Addr)
                         -> io::Result<()> {
        match self.0 {}
    }

    pub fn join_multicast_v6(&self, _: &Ipv6Addr, _: u32)
                         -> io::Result<()> {
        match self.0 {}
    }

    pub fn leave_multicast_v4(&self, _: &Ipv4Addr, _: &Ipv4Addr)
                          -> io::Result<()> {
        match self.0 {}
    }

    pub fn leave_multicast_v6(&self, _: &Ipv6Addr, _: u32)
                          -> io::Result<()> {
        match self.0 {}
    }

    pub fn set_ttl(&self, _: u32) -> io::Result<()> {
        match self.0 {}
    }

    pub fn ttl(&self) -> io::Result<u32> {
        match self.0 {}
    }

    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        match self.0 {}
    }

    pub fn set_nonblocking(&self, _: bool) -> io::Result<()> {
        match self.0 {}
    }

    pub fn recv(&self, _: &mut [u8]) -> io::Result<usize> {
        match self.0 {}
    }

    pub fn peek(&self, _: &mut [u8]) -> io::Result<usize> {
        match self.0 {}
    }

    pub fn send(&self, _: &[u8]) -> io::Result<usize> {
        match self.0 {}
    }

    pub fn connect(&self, _: &SocketAddr) -> io::Result<()> {
        match self.0 {}
    }
}

impl fmt::Debug for UdpSocket {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {}
    }
}

pub struct LookupHost {
    data: io::Cursor<Box<[u8]>>,
    done: bool
}

unsafe fn read_struct<T>(s: &mut T, f: &mut io::Read) -> io::Result<()> {
    let size = mem::size_of::<T>();
    let slice = slice::from_raw_parts_mut(
        s as *mut T as *mut u8,
        size
    );
    // `read_exact()` comes from `Read` impl for `&[u8]`
    f.read_exact(slice)
}

unsafe fn write_struct<T>(s: &T, f: &mut io::Write) -> io::Result<()> {
    let size = mem::size_of::<T>();
    let slice = slice::from_raw_parts(
        s as *const T as *const u8,
        size
    );
    // `read_exact()` comes from `Read` impl for `&[u8]`
    f.write_all(slice)
}


impl Iterator for LookupHost {
    type Item = SocketAddr;
    fn next(&mut self) -> Option<SocketAddr> {
        if self.done {
            return None
        }
        loop {
            let mut magic : u32 = 0;
            unsafe { read_struct(&mut magic, &mut self.data).ok()?; }

            if u32::from_be(magic) != 0xBEEFCAFE {
                self.done = true;
                return None;
            }

            let mut hdr : c::PackedAddrInfoHdr = unsafe { mem::zeroed() };
            unsafe { read_struct(&mut hdr, &mut self.data).expect("Can't fail"); }
            hdr.ai_flags = u32::from_be(hdr.ai_flags);
            hdr.family = u32::from_be(hdr.family);
            hdr.socktype = u32::from_be(hdr.socktype);
            hdr.protocol = u32::from_be(hdr.protocol);
            hdr.addrlen = u32::from_be(hdr.addrlen);

            let ret = if hdr.addrlen != 0 {
                let pos = self.data.position();
                let ret = if hdr.family == c::AF_INET as u32 {
                    if hdr.addrlen < mem::size_of::<c::sockaddr_in>() as u32 {
                        self.done = true;
                        panic!("Wrong addrlen: {}", hdr.addrlen);
                    }
                    let sockaddr = unsafe {
                        let mut sockaddr : c::sockaddr_in = mem::zeroed();
                        read_struct(&mut sockaddr, &mut self.data).expect("Can't fail");
                        let addr = &mut sockaddr as *mut c::sockaddr_in;
                        (*addr).sin_port = u16::from_be((*addr).sin_port);
                        (*addr).sin_addr.s_addr = u32::from_be((*addr).sin_addr.s_addr);
                        sockaddr
                    };
                    Some(SocketAddr::V4(FromInner::from_inner(sockaddr)))
                } else {
                    None
                };
                self.data.set_position(pos + hdr.addrlen as u64);
                ret
            } else {
                let pos = self.data.position();
                self.data.set_position(pos + 4);
                None
            };

            // skip over canonname.
            let eos_pos = self.data.get_ref()[self.data.position() as usize..].iter().position(|x| *x == b'\0').expect("Can't fail");
            let pos = self.data.position();
            self.data.set_position(pos + eos_pos as u64 + 1);

            if ret.is_some() {
                return ret;
            }
        }
    }
}

fn pack_ai(ai: &c::addrinfo, buf: &mut [u8]) -> io::Result<()> {
    let mut cursor = io::Cursor::new(buf);
    let mut cur_ai = ai as *const c::addrinfo;
    unsafe {
        while !cur_ai.is_null() {
            write_struct(&0xBEEFCAFEu32.to_be(), &mut cursor)?;
            write_struct(&c::PackedAddrInfoHdr {
                ai_flags: (*cur_ai).ai_flags.to_be(),
                family: (*cur_ai).ai_family.to_be(),
                socktype: (*cur_ai).ai_socktype.to_be(),
                protocol: (*cur_ai).ai_protocol.to_be(),
                addrlen: (*cur_ai).ai_addrlen.to_be(),
            }, &mut cursor)?;
            if (*cur_ai).ai_addrlen == 0 {
                write_struct(&0u32, &mut cursor)?;
            } else if (*cur_ai).ai_family == c::AF_INET as u32 {
                // Write sockaddr
                let addr = (*cur_ai).ai_addr as *const c::sockaddr_in;
                write_struct(&0u32, &mut cursor)?; // len
                write_struct(&((*addr).sin_family as u32), &mut cursor)?; // family
                write_struct(&((*addr).sin_port as u32).to_be(), &mut cursor)?; // port
                write_struct(&((*addr).sin_addr.s_addr).to_be(), &mut cursor)?; // addr
            } else {
                let slice = slice::from_raw_parts(
                    (*cur_ai).ai_addr as *const u8,
                    (*cur_ai).ai_addrlen as usize
                );
                cursor.write_all(slice)?;
            }
            if (*cur_ai).ai_canonname.is_null() {
                write_struct(&0u8, &mut cursor)?;
            } else {
                cursor.write_all(CStr::from_ptr((*cur_ai).ai_canonname as _).to_bytes_with_nul())?;
            }
            cur_ai = (*cur_ai).ai_next;
        }
        write_struct(&0u32, &mut cursor)?;
    }
    Ok(())
}

pub fn lookup_host(host: &str) -> io::Result<LookupHost> {
    let sfdnsres = try_mth!(IResolver::new());

    let c_host = CString::new(host)?;
    // TODO
    let mut hints: c::addrinfo = unsafe { mem::zeroed() };
    hints.ai_socktype = c::SOCK_STREAM;

    let mut hints_packed = [0; 0x400];

    pack_ai(&hints, &mut hints_packed[..])?;

    // Box the result as it's a bit big (stack size can be as small as 0x4000
    // on the switch.
    let mut res = Box::new([0; 0x1000]);
    //TODO: Use gethostbyname instead.
    let (ret, _errno, _size) = match sfdnsres.get_addr_info(1, 0, 0, c_host.as_bytes_with_nul(), &b"0\0"[..], &hints_packed, &mut *res) {
        Ok(x) => x,
        Err(err) => return Err(io::Error::new(io::ErrorKind::Other, Box::new(err)))
    };

    if ret == 0 {
        Ok(LookupHost {
            data: io::Cursor::new(res),
            // TODO: size: size,
            done: false
        })
    } else {
        // TODO: ret or errno?
        Err(io::Error::from_raw_os_error(ret as i32))
    }
}


#[allow(bad_style)]
pub mod netc {
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PackedAddrInfoHdr {
        pub ai_flags: u32,
        pub family: u32,
        pub socktype: u32,
        pub protocol: u32,
        pub addrlen: u32,
    }

    pub struct addrinfo {
        pub ai_flags: u32,
        pub ai_family: u32,
        pub ai_socktype: u32,
        pub ai_protocol: u32,
        pub ai_addrlen: u32,
        pub ai_addr: *const sockaddr,
        pub ai_canonname: *const u8,
        pub ai_next: *const addrinfo,
    }

    pub const AF_INET: u8 = 2;
    pub const AF_INET6: u8 = 10;
    pub type sa_family_t = u8;

    pub const SOCK_STREAM: u32 = 1;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct in_addr {
        pub s_addr: u32,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct sockaddr_in {
        pub sin_len: u8,
        pub sin_family: sa_family_t,
        pub sin_port: u16,
        pub sin_addr: in_addr,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct in6_addr {
        pub s6_addr: [u8; 16],
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct sockaddr_in6 {
        pub sin6_len: u8,
        pub sin6_family: sa_family_t,
        pub sin6_port: u16,
        pub sin6_addr: in6_addr,
        pub sin6_flowinfo: u32,
        pub sin6_scope_id: u32,
    }

    pub type sockaddr = ::megaton_ipc::nn::socket::Sockaddr;

    pub type sockaddr_storage = sockaddr;

    pub type socklen_t = usize;
}
