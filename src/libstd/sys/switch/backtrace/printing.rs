extern crate addr2line;
extern crate gimli;

use self::addr2line::Context;
use self::gimli::{DebugAbbrev, DebugInfo, DebugLine, DebugRanges, DebugRngLists, DebugStr, LittleEndian};

use io;
use slice;
use sys::backtrace::BacktraceContext;
use sys_common::backtrace::Frame;

pub fn resolve_symname<F>(frame: Frame,
                          callback: F,
                          _ctx: &BacktraceContext) -> io::Result<()>
where
    F: FnOnce(Option<&str>) -> io::Result<()>
{
    let mut cb: Option<F> = Some(callback);
    // TODO: Use .dynamic.
    from_debuginfo(frame, |name, file, line| cb.take().map_or(Ok(()), |cb| cb(name)))
}

pub fn foreach_symbol_fileline<F>(frame: Frame, mut f: F, _ctx: &BacktraceContext) -> io::Result<bool>
where
    F: FnMut(&[u8], u32) -> io::Result<()>
{
    from_debuginfo(frame, |name, file, line| f(file, line))?;
    Ok(false)
}

fn from_debuginfo<F>(frame: Frame, mut cb: F) -> io::Result<()>
where
    F: FnMut(Option<&str>, &[u8], u32) -> io::Result<()>
{
    extern {
        static __debug_abbrev_start: u8;
        static __debug_abbrev_end: u8;
        static __debug_info_start: u8;
        static __debug_info_end: u8;
        static __debug_line_start: u8;
        static __debug_line_end: u8;
        static __debug_ranges_start: u8;
        static __debug_ranges_end: u8;
        static __debug_rnglists_start: u8;
        static __debug_rnglists_end: u8;
        static __debug_str_start: u8;
        static __debug_str_end: u8;
    }

    unsafe fn from_ptrs(start: &u8, end: &u8) -> &'static [u8] {
        slice::from_raw_parts(start, end as *const u8 as usize - start as *const u8 as usize)
    }

    let ctx = unsafe {
        // SAFETY: The addresses are set by the linker script, and should be correct.
        let abbrev = DebugAbbrev::new(from_ptrs(&__debug_abbrev_start, &__debug_abbrev_end), LittleEndian);
        let info = DebugInfo::new(from_ptrs(&__debug_info_start, &__debug_info_end), LittleEndian);
        let line = DebugLine::new(from_ptrs(&__debug_line_start, &__debug_line_end), LittleEndian);
        let ranges = DebugRanges::new(from_ptrs(&__debug_ranges_start, &__debug_ranges_end), LittleEndian);
        let rnglists = DebugRngLists::new(from_ptrs(&__debug_rnglists_start, &__debug_rnglists_end), LittleEndian);
        let s = DebugStr::new(from_ptrs(&__debug_str_start, &__debug_str_end), LittleEndian);

        match Context::from_sections(abbrev, info, line, ranges, rnglists, s) {
            Ok(k) => k,
            Err(err) => panic!("LOL"), // TODO: Propagate error
        }
    };

    if let Ok(mut frames) = ctx.find_frames(frame.symbol_addr as u64) {
        while let Ok(Some(mut frame)) = frames.next() {
            let (file, line) = frame
                .location
                .map(|l| (l.file, l.line))
                .unwrap_or((None, None));
            let name = frame
                .function
                .as_ref()
                .and_then(|f| f.raw_name().ok());
            if let (Some(f), Some(l)) = (file, line) {
                cb(name.as_ref().map(|v| v.as_ref()), f.as_bytes(), l as u32)?;
            }
        }
    }
    Ok(())
}
