use std::thread::{spawn, JoinHandle};

use crate::io::{create_io, IoTask};
use ag_utils::value::Value;
use crossbeam::channel::Sender;

pub fn thread_exec<'io>(frame: ThreadFrame) -> JoinHandle<Result<(), String>> {
    let io = create_io();
    let thread = spawn(move || ThreadVM::new(frame, io.1, io.0).run());
    thread
}

pub struct ThreadFrame {}

// SIZE_LIMIT:

/// A lighter version of the main VM, for threads
pub struct ThreadVM
//             ^^ so that the Thread VM
{
    frame: ThreadFrame,
    io_send: Sender<IoTask>,
    handle: JoinHandle<()>,
}
impl ThreadVM {
    pub fn new(frame: ThreadFrame, io_send: Sender<IoTask>, handle: JoinHandle<()>) -> ThreadVM {
        ThreadVM {
            frame,
            io_send,
            handle,
        }
    }
    pub fn run(mut self) -> Result<(), String> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::thread::{ThreadFrame, ThreadVM};

    #[test]
    fn vm_meets_size_limit() {
        assert!(size_of::<ThreadVM>() <= 1000)
    }
    #[test]
    fn frame_meets_size_limit() {
        assert!(size_of::<ThreadFrame>() <= 200)
    }
}
