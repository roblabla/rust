// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi::{OsString};
use fmt;
use io::{self, Error, ErrorKind, SeekFrom};
use path::{Path, PathBuf};
use sync::Arc;
use sys::time::SystemTime;
use sys_common::{FromInner};

pub struct File;

#[derive(Clone)]
pub struct FileAttr;

pub struct ReadDir {
    data: Vec<u8>,
    i: usize,
    root: Arc<PathBuf>,
}

pub struct DirEntry {
    name: Box<[u8]>
}

#[derive(Clone, Debug)]
pub struct OpenOptions {
    // generic
    read: bool,
    write: bool,
    append: bool,
    truncate: bool,
    create: bool,
    create_new: bool,
    // system-specific
    custom_flags: i32,
    mode: u16,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FilePermissions { mode: u16 }

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct FileType { mode: u16 }

#[derive(Debug)]
pub struct DirBuilder { mode: u16 }

impl FileAttr {
    pub fn size(&self) -> u64 { unimplemented!(); }
    pub fn perm(&self) -> FilePermissions { unimplemented!(); }
    pub fn file_type(&self) -> FileType {
        unimplemented!();
    }
}

impl FileAttr {
    pub fn modified(&self) -> io::Result<SystemTime> {
        Err(Error::new(ErrorKind::Other, "FileAttr::modified not implemented"))
    }

    pub fn accessed(&self) -> io::Result<SystemTime> {
        Err(Error::new(ErrorKind::Other, "FileAttr::accessed not implemented"))
    }

    pub fn created(&self) -> io::Result<SystemTime> {
        Err(Error::new(ErrorKind::Other, "FileAttr::created not implemented"))
    }
}

impl FilePermissions {
    pub fn readonly(&self) -> bool { self.mode & 0o222 == 0 }
    pub fn set_readonly(&mut self, readonly: bool) {
        if readonly {
            self.mode &= !0o222;
        } else {
            self.mode |= 0o222;
        }
    }
}

impl FileType {
    pub fn is_dir(&self) -> bool { unimplemented!(); }
    pub fn is_file(&self) -> bool { unimplemented!(); }
    pub fn is_symlink(&self) -> bool { unimplemented!(); }
}

impl FromInner<u32> for FilePermissions {
    fn from_inner(mode: u32) -> FilePermissions {
        FilePermissions { mode: mode as u16 }
    }
}

impl fmt::Debug for ReadDir {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // This will only be called from std::fs::ReadDir, which will add a "ReadDir()" frame.
        // Thus the result will be e g 'ReadDir("/home")'
        fmt::Debug::fmt(&*self.root, f)
    }
}

impl Iterator for ReadDir {
    type Item = io::Result<DirEntry>;

    fn next(&mut self) -> Option<io::Result<DirEntry>> {
        loop {
            let start = self.i;
            let mut i = self.i;
            while i < self.data.len() {
                self.i += 1;
                if self.data[i] == b'\n' {
                    break;
                }
                i += 1;
            }
            if start < self.i {
                let ret = DirEntry {
                    name: self.data[start .. i].to_owned().into_boxed_slice(),
                    //root: self.root.clone()
                };
                if ret.name_bytes() != b"." && ret.name_bytes() != b".." {
                    return Some(Ok(ret))
                }
            } else {
                return None;
            }
        }
    }
}

impl DirEntry {
    pub fn path(&self) -> PathBuf {
        unimplemented!();
    }

    pub fn file_name(&self) -> OsString {
        unimplemented!();
    }

    pub fn metadata(&self) -> io::Result<FileAttr> {
        lstat(&self.path())
    }

    pub fn file_type(&self) -> io::Result<FileType> {
        lstat(&self.path()).map(|m| m.file_type())
    }

    fn name_bytes(&self) -> &[u8] {
        &*self.name
    }
}

impl OpenOptions {
    pub fn new() -> OpenOptions {
        OpenOptions {
            // generic
            read: false,
            write: false,
            append: false,
            truncate: false,
            create: false,
            create_new: false,
            // system-specific
            custom_flags: 0,
            mode: 0o666,
        }
    }

    pub fn read(&mut self, read: bool) { self.read = read; }
    pub fn write(&mut self, write: bool) { self.write = write; }
    pub fn append(&mut self, append: bool) { self.append = append; }
    pub fn truncate(&mut self, truncate: bool) { self.truncate = truncate; }
    pub fn create(&mut self, create: bool) { self.create = create; }
    pub fn create_new(&mut self, create_new: bool) { self.create_new = create_new; }
}

impl File {
    pub fn open(_path: &Path, _opts: &OpenOptions) -> io::Result<File> {
        Err(Error::new(ErrorKind::Other, "File::open not implemented"))
    }

    pub fn file_attr(&self) -> io::Result<FileAttr> {
        Err(Error::new(ErrorKind::Other, "File::file_attr not implemented"))
    }

    pub fn fsync(&self) -> io::Result<()> {
        Err(Error::new(ErrorKind::Other, "File::fsync not implemented"))
    }

    pub fn datasync(&self) -> io::Result<()> {
        Err(Error::new(ErrorKind::Other, "File::datasync not implemented"))
    }

    pub fn truncate(&self, _size: u64) -> io::Result<()> {
        Err(Error::new(ErrorKind::Other, "File::truncate not implemented"))
    }

    pub fn read(&self, _buf: &mut [u8]) -> io::Result<usize> {
        Err(Error::new(ErrorKind::Other, "File::read not implemented"))
    }

    pub fn write(&self, _buf: &[u8]) -> io::Result<usize> {
        Err(Error::new(ErrorKind::Other, "File::write not implemented"))
    }

    pub fn flush(&self) -> io::Result<()> {
        Err(Error::new(ErrorKind::Other, "File::flush not implemented"))
    }

    pub fn seek(&self, _pos: SeekFrom) -> io::Result<u64> {
        Err(Error::new(ErrorKind::Other, "File::write not implemented"))
    }

    pub fn duplicate(&self) -> io::Result<File> {
        Err(Error::new(ErrorKind::Other, "File::duplicate not implemented"))
    }

    pub fn set_permissions(&self, perm: FilePermissions) -> io::Result<()> {
        set_perm(&self.path()?, perm)
    }

    pub fn path(&self) -> io::Result<PathBuf> {
        Err(Error::new(ErrorKind::Other, "File::path not implemented"))
    }
}

impl DirBuilder {
    pub fn new() -> DirBuilder {
        DirBuilder { mode: 0o777 }
    }

    pub fn mkdir(&self, _p: &Path) -> io::Result<()> {
        Err(Error::new(ErrorKind::Other, "DirBuilder::mkdir not implemented"))
    }
}

impl fmt::Debug for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut b = f.debug_struct("File");
        //b.field("fd", &self.0.raw());
        if let Ok(path) = self.path() {
            b.field("path", &path);
        }
        /*
        if let Some((read, write)) = get_mode(fd) {
            b.field("read", &read).field("write", &write);
        }
        */
        b.finish()
    }
}

pub fn readdir(_p: &Path) -> io::Result<ReadDir> {
    Err(Error::new(ErrorKind::Other, "readdir not implemented"))
}

pub fn unlink(_p: &Path) -> io::Result<()> {
    Err(Error::new(ErrorKind::Other, "unlink not implemented"))
}

pub fn rename(old: &Path, new: &Path) -> io::Result<()> {
    copy(old, new)?;
    unlink(old)?;
    Ok(())
}

pub fn set_perm(_p: &Path, _perm: FilePermissions) -> io::Result<()> {
    Err(Error::new(ErrorKind::Other, "set_perm not implemented"))
}

pub fn rmdir(_p: &Path) -> io::Result<()> {
    Err(Error::new(ErrorKind::Other, "rmdir not implemented"))
}

pub fn remove_dir_all(path: &Path) -> io::Result<()> {
    let filetype = lstat(path)?.file_type();
    if filetype.is_symlink() {
        unlink(path)
    } else {
        remove_dir_all_recursive(path)
    }
}

fn remove_dir_all_recursive(path: &Path) -> io::Result<()> {
    for child in readdir(path)? {
        let child = child?;
        if child.file_type()?.is_dir() {
            remove_dir_all_recursive(&child.path())?;
        } else {
            unlink(&child.path())?;
        }
    }
    rmdir(path)
}

pub fn readlink(_p: &Path) -> io::Result<PathBuf> {
    Err(Error::new(ErrorKind::Other, "readlink not implemented"))
}

pub fn symlink(_src: &Path, _dst: &Path) -> io::Result<()> {
    Err(Error::new(ErrorKind::Other, "symlink not implemented"))
}

pub fn link(_src: &Path, _dst: &Path) -> io::Result<()> {
    Err(Error::new(ErrorKind::Other, "link not implemented"))
}

pub fn stat(_p: &Path) -> io::Result<FileAttr> {
    Err(Error::new(ErrorKind::Other, "stat not implemented"))
}

pub fn lstat(_p: &Path) -> io::Result<FileAttr> {
    Err(Error::new(ErrorKind::Other, "lstat not implemented"))
}

pub fn canonicalize(_p: &Path) -> io::Result<PathBuf> {
    Err(Error::new(ErrorKind::Other, "canonicalize not implemented"))
}

pub fn copy(_from: &Path, _to: &Path) -> io::Result<u64> {
    Err(Error::new(ErrorKind::Other, "copy not implemented"))
}
