// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Implementation of `std::os` functionality for unix systems

#![allow(unused_imports)] // lots of cfg code here

//use os::unix::prelude::*;

use error::Error as StdError;
use ffi::{OsString, OsStr};
use fmt;
use io::{self, Error, ErrorKind, Read, Write};
use iter;
use marker::PhantomData;
use mem;
use memchr;
use path::{self, PathBuf};
use ptr;
use slice;
use str;
use sys_common::mutex::Mutex;
//use sys::{cvt, fd, syscall};
use vec;

//const TMPBUF_SZ: usize = 128;
//static ENV_LOCK: Mutex = Mutex::new();

/*extern {
    #[link_name = "__errno_location"]
    fn errno_location() -> *mut i32;
}*/

/// Returns the platform-specific value of errno
pub fn errno() -> i32 {
    // TODO: This is probably wrong :D Get it from libc.
    0
}

/// Gets a detailed string description for the given error number.
pub fn error_string(_errno: i32) -> String {
    // TODO
    "unknown error".to_string()
}

pub fn getcwd() -> io::Result<PathBuf> {
    Err(Error::new(ErrorKind::Other, "getcwd not implemented"))
}

pub fn chdir(_p: &path::Path) -> io::Result<()> {
    Err(Error::new(ErrorKind::Other, "chdir not implemented"))
}

// TODO: Implement split_paths
pub struct SplitPaths<'a> {
    data: PhantomData<&'a ()>
}

pub fn split_paths(_unparsed: &OsStr) -> SplitPaths {
    SplitPaths { data: PhantomData }
}

impl<'a> Iterator for SplitPaths<'a> {
    type Item = PathBuf;
    fn next(&mut self) -> Option<PathBuf> { None }
    fn size_hint(&self) -> (usize, Option<usize>) { (0, Some(0)) }
}

// TODO: Implement join_paths
#[derive(Debug)]
pub struct JoinPathsError;

pub fn join_paths<I, T>(_paths: I) -> Result<OsString, JoinPathsError>
    where I: Iterator<Item=T>, T: AsRef<OsStr>
{
    Err(JoinPathsError)
}

impl fmt::Display for JoinPathsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        "join_paths not implemented".fmt(f)
    }
}

impl StdError for JoinPathsError {
    fn description(&self) -> &str { "join_paths not implemented" }
}

pub fn current_exe() -> io::Result<PathBuf> {
    Err(Error::new(ErrorKind::Other, "current_exe not implemented"))
}

// TODO: Implement env
pub struct Env;

impl Iterator for Env {
    type Item = (OsString, OsString);
    fn next(&mut self) -> Option<(OsString, OsString)> { None }
    fn size_hint(&self) -> (usize, Option<usize>) { (0, Some(0)) }
}

/// Returns a vector of (variable, value) byte-vector pairs for all the
/// environment variables of the current process.
pub fn env() -> Env {
    Env
}

pub fn getenv(_key: &OsStr) -> io::Result<Option<OsString>> {
    Err(Error::new(ErrorKind::Other, "get_env not implemented"))
}

pub fn setenv(_key: &OsStr, _value: &OsStr) -> io::Result<()> {
    Err(Error::new(ErrorKind::Other, "set_env not implemented"))
}

pub fn unsetenv(_key: &OsStr) -> io::Result<()> {
    Err(Error::new(ErrorKind::Other, "unset_env not implemented"))
}

pub fn temp_dir() -> PathBuf {
    unimplemented!();
}

pub fn home_dir() -> Option<PathBuf> {
    None
}

pub fn exit(_code: i32) -> ! {
    // TODO: libc::exit
    unimplemented!();
}

pub fn getpid() -> u32 {
    // TODO: Syscall to get thread id
    unimplemented!();
}
