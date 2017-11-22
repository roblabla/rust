// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use collections::hash_map::HashMap;
use ffi::OsStr;
use fmt;
use io::{self, Error, ErrorKind};
use sys::fs::{File};
use sys::pipe::{AnonPipe};

////////////////////////////////////////////////////////////////////////////////
// Command
////////////////////////////////////////////////////////////////////////////////

pub struct Command {
    // Currently we try hard to ensure that the call to `.exec()` doesn't
    // actually allocate any memory. While many platforms try to ensure that
    // memory allocation works after a fork in a multithreaded process, it's
    // been observed to be buggy and somewhat unreliable, so we do our best to
    // just not do it at all!
    //
    // Along those lines, the `argv` and `envp` raw pointers here are exactly
    // what's gonna get passed to `execvp`. The `argv` array starts with the
    // `program` and ends with a NULL, and the `envp` pointer, if present, is
    // also null-terminated.
    //
    // Right now we don't support removing arguments, so there's no much fancy
    // support there, but we support adding and removing environment variables,
    // so a side table is used to track where in the `envp` array each key is
    // located. Whenever we add a key we update it in place if it's already
    // present, and whenever we remove a key we update the locations of all
    // other keys.
    program: String,
    args: Vec<String>,
    env: HashMap<String, String>,

    cwd: Option<String>,
    //uid: Option<u32>,
    //gid: Option<u32>,
    //saw_nul: bool,
    //closures: Vec<Box<FnMut() -> io::Result<()> + Send + Sync>>,
    stdin: Option<Stdio>,
    stdout: Option<Stdio>,
    stderr: Option<Stdio>,
}

// passed back to std::process with the pipes connected to the child, if any
// were requested
pub struct StdioPipes {
    pub stdin: Option<AnonPipe>,
    pub stdout: Option<AnonPipe>,
    pub stderr: Option<AnonPipe>,
}

pub enum Stdio {
    Inherit,
    Null,
    MakePipe,
}

impl Command {
    pub fn new(program: &OsStr) -> Command {
        Command {
            program: program.to_str().unwrap().to_owned(),
            args: Vec::new(),
            env: HashMap::new(),
            cwd: None,
            //uid: None,
            //gid: None,
            //saw_nul: false,
            //closures: Vec::new(),
            stdin: None,
            stdout: None,
            stderr: None,
        }
    }

    pub fn arg(&mut self, arg: &OsStr) {
        self.args.push(arg.to_str().unwrap().to_owned());
    }

    pub fn env(&mut self, key: &OsStr, val: &OsStr) {
        self.env.insert(key.to_str().unwrap().to_owned(), val.to_str().unwrap().to_owned());
    }

    pub fn env_remove(&mut self, key: &OsStr) {
        self.env.remove(key.to_str().unwrap());
    }

    pub fn env_clear(&mut self) {
        self.env.clear();
    }

    pub fn cwd(&mut self, dir: &OsStr) {
        self.cwd = Some(dir.to_str().unwrap().to_owned());
    }

    pub fn stdin(&mut self, stdin: Stdio) {
        self.stdin = Some(stdin);
    }
    pub fn stdout(&mut self, stdout: Stdio) {
        self.stdout = Some(stdout);
    }
    pub fn stderr(&mut self, stderr: Stdio) {
        self.stderr = Some(stderr);
    }

    pub fn spawn(&mut self, _default: Stdio, _needs_stdin: bool)
                 -> io::Result<(Process, StdioPipes)> {
        Err(Error::new(ErrorKind::Other, "Command::spawn not implemented"))
    }
}

impl From<AnonPipe> for Stdio {
    fn from(_pipe: AnonPipe) -> Stdio {
        unimplemented!();
        //Stdio::Fd(pipe.into_fd())
    }
}

impl From<File> for Stdio {
    fn from(_file: File) -> Stdio {
        unimplemented!();
        //Stdio::Fd(file.into_fd())
    }
}

impl fmt::Debug for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.program)?;
        for arg in &self.args {
            write!(f, " {:?}", arg)?;
        }
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////
// Processes
////////////////////////////////////////////////////////////////////////////////

/// Unix exit statuses
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct ExitStatus(i32);

impl ExitStatus {
    fn exited(&self) -> bool {
        self.0 & 0x7F == 0
    }

    pub fn success(&self) -> bool {
        self.code() == Some(0)
    }

    pub fn code(&self) -> Option<i32> {
        if self.exited() {
            Some((self.0 >> 8) & 0xFF)
        } else {
            None
        }
    }

    pub fn signal(&self) -> Option<i32> {
        if !self.exited() {
            Some(self.0 & 0x7F)
        } else {
            None
        }
    }
}

impl From<i32> for ExitStatus {
    fn from(a: i32) -> ExitStatus {
        ExitStatus(a)
    }
}

impl fmt::Display for ExitStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(code) = self.code() {
            write!(f, "exit code: {}", code)
        } else {
            let signal = self.signal().unwrap();
            write!(f, "signal: {}", signal)
        }
    }
}

/// The unique id of the process (this should never be negative).
pub struct Process {
    pid: usize,
    //status: Option<ExitStatus>,
}

impl Process {
    pub fn id(&self) -> u32 {
        self.pid as u32
    }

    pub fn kill(&mut self) -> io::Result<()> {
        unimplemented!();
        /*// If we've already waited on this process then the pid can be recycled
        // and used for another process, and we probably shouldn't be killing
        // random processes, so just return an error.
        if self.status.is_some() {
            Err(Error::new(ErrorKind::InvalidInput,
                           "invalid argument: can't kill an exited process"))
        } else {
            cvt(syscall::kill(self.pid, syscall::SIGKILL))?;
            Ok(())
        }*/
    }

    pub fn wait(&mut self) -> io::Result<ExitStatus> {
        unimplemented!();
        /*if let Some(status) = self.status {
            return Ok(status)
        }
        let mut status = 0;
        cvt(syscall::waitpid(self.pid, &mut status, 0))?;
        self.status = Some(ExitStatus(status as i32));
        Ok(ExitStatus(status as i32))*/
    }

    pub fn try_wait(&mut self) -> io::Result<Option<ExitStatus>> {
        unimplemented!();
        /*if let Some(status) = self.status {
            return Ok(Some(status))
        }
        let mut status = 0;
        let pid = cvt(syscall::waitpid(self.pid, &mut status, syscall::WNOHANG))?;
        if pid == 0 {
            Ok(None)
        } else {
            self.status = Some(ExitStatus(status as i32));
            Ok(Some(ExitStatus(status as i32)))
        }*/
    }
}
