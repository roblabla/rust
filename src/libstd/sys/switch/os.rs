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

use error::Error as StdError;
use ffi::{OsString, OsStr};
use fmt;
use io;
use path::{self, PathBuf};
use sys::unsupported;

/// Returns the platform-specific value of errno
pub fn errno() -> i32 {
    // There's never an error...
    0
}

/// Gets a detailed string description for the given error number.
pub fn error_string(_errno: i32) -> String {
    "unknown error".to_string()
}

pub fn getcwd() -> io::Result<PathBuf> {
    unsupported()
}

pub fn chdir(_p: &path::Path) -> io::Result<()> {
    unsupported()
}

pub struct SplitPaths<'a>(&'a !);

pub fn split_paths(_unparsed: &OsStr) -> SplitPaths {
    unimplemented!()
}

impl<'a> Iterator for SplitPaths<'a> {
    type Item = PathBuf;
    fn next(&mut self) -> Option<PathBuf> { match self.0 {} }
    fn size_hint(&self) -> (usize, Option<usize>) { match self.0 {} }
}

#[derive(Debug)]
pub struct JoinPathsError;

pub fn join_paths<I, T>(_paths: I) -> Result<OsString, JoinPathsError>
    where I: Iterator<Item=T>, T: AsRef<OsStr>
{
    Err(JoinPathsError)
}

impl fmt::Display for JoinPathsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        "not supported on the switch yet".fmt(f)
    }
}

impl StdError for JoinPathsError {
    fn description(&self) -> &str { "not supported on the switch yet" }
}

pub fn current_exe() -> io::Result<PathBuf> {
    unsupported()
}

pub struct Env;

impl Iterator for Env {
    type Item = (OsString, OsString);
    fn next(&mut self) -> Option<(OsString, OsString)> { None }
    fn size_hint(&self) -> (usize, Option<usize>) { (0, None) }
}

/// Returns a vector of (variable, value) byte-vector pairs for all the
/// environment variables of the current process.
pub fn env() -> Env {
    unimplemented!()
}

pub fn getenv(_key: &OsStr) -> io::Result<Option<OsString>> {
    unsupported()
}

pub fn setenv(_key: &OsStr, _value: &OsStr) -> io::Result<()> {
    unsupported()
}

pub fn unsetenv(_key: &OsStr) -> io::Result<()> {
    unsupported()
}

pub fn temp_dir() -> PathBuf {
    unimplemented!()
}

pub fn home_dir() -> Option<PathBuf> {
    None
}

pub fn exit(code: i32) -> ! {
    ::megaton_hammer::loader::exit(code as u64);
}

pub fn getpid() -> u32 {
    unimplemented!()
}
