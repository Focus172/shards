use crate::builtins;
use crate::helpers::{Fd, Shell};
use crate::parser::{Cmd, Simple};
use os_pipe::{pipe, PipeReader, PipeWriter};
use std::process::Command;
use std::rc::Rc;
use std::cell::RefCell;
use std::io::Read;

// This is useful to keep track of what each command does with its STDs
#[derive(Debug)]
struct CmdMeta {
    stdin: Option<PipeReader>,
    stdout: Option<PipeWriter>,
}

impl CmdMeta {
    fn inherit() -> CmdMeta {
        CmdMeta {
            stdin: None,
            stdout: None,
        }
    }

    fn pipe_out(writer: PipeWriter) -> CmdMeta {
        CmdMeta {
            stdin: None,
            stdout: Some(writer),
        }
    }

    fn new_in(self, reader: PipeReader) -> CmdMeta {
        CmdMeta {
            stdin: Some(reader),
            stdout: self.stdout,
        }
    }
}

