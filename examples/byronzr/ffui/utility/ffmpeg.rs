use bevy::prelude::*;
//use std::io::{BufRead, BufReader};
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

pub fn create_ffmpeg_command(path: String) -> Command {
    let mut cmd = Command::new("ffmpeg");
    cmd.arg("-hwaccel")
        .arg("videotoolbox")
        .arg("-nostdin")
        .arg("-i")
        .arg(path) // 替换为你的命令
        .args(["-vf", "scale=-2:720,fps=30"])
        .arg("-c:v")
        .arg("hevc_videotoolbox")
        .args(["-quality", "high"])
        .args(["-b:v", "2000k"])
        .args(["-maxrate", "8000k"])
        .args(["-bufsize", "16000k"])
        .args(["-c:a", "copy"])
        .args(["-tag:v", "hvc1"])
        .args(["-loglevel", "info"])
        .args(["-progress", "pipe:1"])
        .arg("-y")
        .arg("out.mp4") // 覆盖输出文件
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    cmd
}
