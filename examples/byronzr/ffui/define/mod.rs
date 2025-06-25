use bevy::prelude::*;
use crossbeam_channel::{Receiver, Sender, bounded};
use std::io::{BufRead, BufReader};
use std::process::{Child, Command, Stdio};
use std::time::Duration;

#[derive(Debug, Component)]
pub struct Container;

#[derive(Debug, Component)]
pub struct IndexOfline(pub usize);

#[derive(Debug, Resource)]
pub struct ProcessState {
    pub rx: Receiver<String>,
    pub tx: Sender<String>,
}

#[derive(Debug, Resource, Default)]
pub struct PathDatas {
    pub lines: Vec<String>,
    pub entities: Vec<Option<Entity>>,
    pub changed: bool,
}

pub struct ManagedProcess {
    child: Child,
}

impl ManagedProcess {
    pub fn new(
        command: &mut Command,
    ) -> std::io::Result<(
        Self,
        BufReader<std::process::ChildStdout>,
        BufReader<std::process::ChildStderr>,
    )> {
        let mut child = command.spawn()?;

        let stdout = BufReader::new(child.stdout.take().unwrap());
        let stderr = BufReader::new(child.stderr.take().unwrap());
        Ok((Self { child }, stdout, stderr))
    }
}

impl Drop for ManagedProcess {
    fn drop(&mut self) {
        let _ = self.child.kill(); // 尝试终止进程
        let _ = self.child.wait(); // 等待进程结束
    }
}
