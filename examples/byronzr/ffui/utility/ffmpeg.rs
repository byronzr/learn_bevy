use bevy::prelude::*;
//use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::Stdio;
use tokio::process::Command;

// -hwaccel
pub fn create_ffmpeg_command(path: String) -> Command {
    let Some(filename) = Path::new(&path)
        .file_stem()
        .and_then(|name| name.to_str())
        .map(|name_str| format!("{}.mp4", name_str))
    else {
        panic!("Invalid file path: {}", path);
    };

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
        .arg(filename) // 覆盖输出文件
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    cmd
}

pub fn create_ffmpeg_command_libx265(path: String) -> Command {
    let Some(filename) = Path::new(&path)
        .file_stem()
        .and_then(|name| name.to_str())
        .map(|name_str| format!("{}.mp4", name_str))
    else {
        panic!("Invalid file path: {}", path);
    };

    let mut cmd = Command::new("ffmpeg");
    cmd.arg("-nostdin")
        .arg("-i")
        .arg(path) // 替换为你的命令
        .args(["-vf", "scale=-2:720,fps=30"])
        .arg("-c:v")
        .arg("libx265")
        .args(["-quality", "high"])
        .args(["-b:v", "2000k"])
        .args(["-maxrate", "8000k"])
        .args(["-bufsize", "16000k"])
        .args(["-c:a", "copy"])
        .args(["-tag:v", "hvc1"])
        .args(["-loglevel", "info"])
        .args(["-progress", "pipe:1"])
        .arg("-y")
        .arg(filename) // 覆盖输出文件
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    cmd
}

pub fn snapshot_ffmpeg_command(path: String) -> Command {
    let Some(filename) = Path::new(&path)
        .file_stem()
        .and_then(|name| name.to_str())
        .map(|name_str| format!("{}.mp4", name_str))
    else {
        panic!("Invalid file path: {}", path);
    };

    let mut cmd = Command::new("ffmpeg");
    cmd.args([
        "-ss",
        "00:00:01.000", // 截图时间点
        "-i",
        &filename, // 输入文件路径
        "-frames:v",
        "1", // 只截取一帧
        "-vcodec",
        "png",
        "-q:v",
        "2", // 设置输出质量
        //"output.png", // 输出文件名
        "-f",
        "image2pipe",
        "-",
    ])
    .stdout(Stdio::piped())
    .stderr(Stdio::piped());
    cmd
}
