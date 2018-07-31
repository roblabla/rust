// Copyright 2014-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use error::Error;
use io;
use libc; 
use sys::backtrace::BacktraceContext;
use sys_common::backtrace::Frame;

use unwind as uw;

struct Context<'a> {
    idx: usize,
    frames: &'a mut [Frame],
}

#[derive(Debug)]
struct UnwindError(uw::_Unwind_Reason_Code);

impl Error for UnwindError {
    fn description(&self) -> &'static str {
        "unexpected return value while unwinding"
    }
}

impl ::fmt::Display for UnwindError {
    fn fmt(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
        write!(f, "{}: {:?}", self.description(), self.0)
    }
}

#[inline(never)] // if we know this is a function call, we can skip it when
                 // tracing
pub fn unwind_backtrace(frames: &mut [Frame])
    -> io::Result<(usize, BacktraceContext)>
{
    let mut cx = Context {
        idx: 0,
        frames,
    };
    let result_unwind = unsafe {
        uw::_Unwind_Backtrace(trace_fn,
                              &mut cx as *mut Context
                              as *mut libc::c_void)
    };
    // See libunwind:src/unwind/Backtrace.c for the return values.
    // No, there is no doc.
    match result_unwind {
        // These return codes seem to be benign and need to be ignored for backtraces
        // to show up properly on all tested platforms.
        uw::_URC_END_OF_STACK | uw::_URC_FATAL_PHASE1_ERROR | uw::_URC_FAILURE => {
            Ok((cx.idx, BacktraceContext))
        }
        _ => {
            Err(io::Error::new(io::ErrorKind::Other,
                               UnwindError(result_unwind)))
        }
    }
}

extern fn trace_fn(ctx: *mut uw::_Unwind_Context,
                   arg: *mut libc::c_void) -> uw::_Unwind_Reason_Code {
    let cx = unsafe { &mut *(arg as *mut Context) };
    let mut ip_before_insn = 0;
    let mut ip = unsafe {
        uw::_Unwind_GetIPInfo(ctx, &mut ip_before_insn) as *mut libc::c_void
    };
    if !ip.is_null() && ip_before_insn == 0 {
        // this is a non-signaling frame, so `ip` refers to the address
        // after the calling instruction. account for that.
        ip = (ip as usize - 1) as *mut _;
    }

    let symaddr = unsafe { uw::_Unwind_FindEnclosingFunction(ip) };

    if cx.idx < cx.frames.len() {
        cx.frames[cx.idx] = Frame {
            symbol_addr: symaddr as *mut u8,
            exact_position: ip as *mut u8,
            inline_context: 0,
        };
        cx.idx += 1;
    }

    uw::_URC_NO_REASON
}
