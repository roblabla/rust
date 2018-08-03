// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi::OsStr;
use path::Prefix;

#[inline]
pub fn is_sep_byte(b: u8) -> bool {
    b == b'/'
}

#[inline]
pub fn is_verbatim_sep(b: u8) -> bool {
    b == b'/'
}

pub fn parse_prefix(path: &OsStr) -> Option<Prefix> {
    if let Some(path_str) = path.to_str() {
        path_str.split('/').next()
            .and_then(|s| s.bytes().position(|v| v == b':'))
            .map(|idx| PrefixComponent::from_os_str_kind(OsStr::new(&path_str[..idx + 1]), Prefix::Disk(0)))
    } else {
        None
    }
}

pub const MAIN_SEP_STR: &'static str = "/";
pub const MAIN_SEP: char = '/';
