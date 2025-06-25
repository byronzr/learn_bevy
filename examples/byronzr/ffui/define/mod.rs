pub mod resource;
pub use resource::*;

pub mod custom;
pub use custom::*;

pub mod component;
pub use component::*;

use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Child, Command};

use crate::TOKIO_RT;

pub struct ManagedProcess {
    pub child: Child,
}

impl ManagedProcess {
    pub fn new(command: &mut Command) -> std::io::Result<Self> {
        let child = TOKIO_RT.block_on(async { command.spawn() })?;

        Ok(Self { child })
    }
    pub fn stdout(&mut self) -> BufReader<tokio::process::ChildStdout> {
        BufReader::new(self.child.stdout.take().unwrap())
    }

    pub fn stderr(&mut self) -> BufReader<tokio::process::ChildStderr> {
        BufReader::new(self.child.stderr.take().unwrap())
    }
}

impl Drop for ManagedProcess {
    fn drop(&mut self) {
        let _ = self.child.kill(); // 尝试终止进程
        let _ = self.child.wait(); // 等待进程结束
    }
}
