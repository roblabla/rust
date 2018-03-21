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
use slice;

use megaton_hammer::loader::{self, Logger, SocketKind};

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
        Ok(Stdout)
    }

    pub fn write(&self, data: &[u8]) -> io::Result<usize> {
        use megaton_ipc::nn::socket::sf::IClient;

        Logger.write(&data[..data.len()]);
        let msg_len = if let Some((kind, stdout)) = loader::get_stdout_socket() {
            let client = match kind {
                SocketKind::BsdU => IClient::new_bsd_u(),
                SocketKind::BsdS => IClient::new_bsd_s()
            };
            // Should be already initialized.
            match client.and_then(|client| client.write(stdout, unsafe { slice::from_raw_parts(data.as_ptr() as *const i8, data.len()) })) {
                Ok((ret, _bsd_errno)) if ret >= 0 => ret as usize,
                _ => data.len()
            }
        } else {
            data.len()
        };
        Ok(msg_len)
    }

    pub fn flush(&self) -> io::Result<()> {
        Ok(())
    }
}

impl Stderr {
    pub fn new() -> io::Result<Stderr> {
        Ok(Stderr)
    }

    pub fn write(&self, data: &[u8]) -> io::Result<usize> {
        use megaton_ipc::nn::socket::sf::IClient;

        let msg_len = if let Some((kind, stderr)) = loader::get_stderr_socket() {
            let client = match kind {
                SocketKind::BsdU => IClient::new_bsd_u(),
                SocketKind::BsdS => IClient::new_bsd_s()
            };
            // Should be already initialized.
            match client.and_then(|client| client.write(stderr, unsafe { slice::from_raw_parts(data.as_ptr() as *const i8, data.len()) } )) {
                Ok((ret, _bsd_errno)) if ret >= 0 => ret as usize,
                _ => data.len()
            }
        } else {
            data.len()
        };
        Logger.write(&data[..msg_len]);
        Ok(msg_len)
    }

    pub fn flush(&self) -> io::Result<()> {
        Ok(())
    }
}

impl io::Write for Stderr {
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        Stderr::write(self, data)
    }
    fn flush(&mut self) -> io::Result<()> {
        Stderr::flush(self)
    }
}

pub const STDIN_BUF_SIZE: usize = 0;

pub fn is_ebadf(_err: &io::Error) -> bool {
    true
}
