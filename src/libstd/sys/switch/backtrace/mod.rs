/// See sys/unix/backtrace/mod.rs for an explanation of the method used here.

pub use self::tracing::unwind_backtrace;
pub use self::printing::{foreach_symbol_fileline, resolve_symname};

// tracing impls:
mod tracing;
// symbol resolvers:
mod printing;

pub mod gnu {
    use io;
    use fs;
    use libc::c_char;
    use vec::Vec;

    pub fn get_executable_filename() -> io::Result<(Vec<c_char>, fs::File)> {
        Err(io::Error::new(io::ErrorKind::Other, "Unimplemented"))
    }
}

pub struct BacktraceContext;
