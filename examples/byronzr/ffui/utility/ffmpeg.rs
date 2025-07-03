use bevy::prelude::*;
//use std::io::{BufRead, BufReader};
use rand;
use std::path::Path;
use std::process::Stdio;
use tokio::process::Command;

use crate::define::ArgKeyValue;

// -hwaccel
// videotoolbox(MacOs)
pub fn create_ffmpeg_command(path: String, args: &Vec<ArgKeyValue>) -> Command {
    let Some(filename) = Path::new(&path)
        .file_stem()
        .and_then(|name| name.to_str())
        .map(|name_str| format!("{}.mp4", name_str))
    else {
        panic!("Invalid file path: {}", path);
    };

    let mut cmd = Command::new("ffmpeg");
    // cmd.arg("-hwaccel")
    //     .arg("videotoolbox")
    //     .arg("-nostdin")
    //     .arg("-i")
    //     .arg(path)
    //     .args(["-vf", "scale=-2:720,fps=30"])
    //     .arg("-c:v")
    //     .arg("hevc_videotoolbox")
    //     .args(["-quality", "high"])
    //     .args(["-b:v", "2000k"])
    //     .args(["-maxrate", "8000k"])
    //     .args(["-bufsize", "16000k"])
    //     .args(["-c:a", "copy"])
    //     .args(["-tag:v", "hvc1"])
    //     .args(["-loglevel", "info"])
    //     .args(["-progress", "pipe:1"])
    //     .arg("-y")
    //     .arg(filename)
    //     .stdout(Stdio::piped())
    //     .stderr(Stdio::piped());
    cmd.args(["-i", &path, "-y", &filename]);
    for arg in args {
        cmd.arg(&arg.key);
        if !arg.value.is_empty() {
            cmd.arg(&arg.value);
        }
    }
    cmd.stdout(Stdio::piped()).stderr(Stdio::piped());
    cmd
}

// software encoding
// libx265
pub fn create_ffmpeg_command_libx265(path: String, args: &Vec<ArgKeyValue>) -> Command {
    let Some(filename) = Path::new(&path)
        .file_stem()
        .and_then(|name| name.to_str())
        .map(|name_str| format!("{}.mp4", name_str))
    else {
        panic!("Invalid file path: {}", path);
    };

    let mut cmd = Command::new("ffmpeg");
    // cmd.arg("-nostdin")
    //     .arg("-i")
    //     .arg(path) // 替换为你的命令
    //     .args(["-vf", "scale=-2:720,fps=30"])
    //     .arg("-c:v")
    //     .arg("libx265")
    //     .args(["-quality", "high"])
    //     .args(["-b:v", "2000k"])
    //     .args(["-maxrate", "8000k"])
    //     .args(["-bufsize", "16000k"])
    //     .args(["-c:a", "copy"])
    //     .args(["-tag:v", "hvc1"])
    //     .args(["-loglevel", "info"])
    //     .args(["-progress", "pipe:1"])
    //     .arg("-y")
    //     .arg(filename) // 覆盖输出文件
    //     .stdout(Stdio::piped())
    //     .stderr(Stdio::piped());
    cmd.args(["-i", &path, "-y", &filename]);
    for arg in args {
        cmd.arg(&arg.key);
        if !arg.value.is_empty() {
            cmd.arg(&arg.value);
        }
    }
    cmd.stdout(Stdio::piped()).stderr(Stdio::piped());
    cmd
}

// snapshot
pub fn snapshot_ffmpeg_command(
    path: String,
    source: bool,
    total_secs: u64,
    args: &Vec<ArgKeyValue>,
) -> Command {
    let filename = if !source {
        let Some(filename) = Path::new(&path)
            .file_stem()
            .and_then(|name| name.to_str())
            .map(|name_str| format!("{}.mp4", name_str))
        else {
            panic!("Invalid file path: {}", path);
        };
        filename
    } else {
        path
    };
    // rand a second of total duration
    let second = rand::random_range(1..total_secs);
    // format second to hh:mm:ss
    let second_str = format!(
        "{:02}:{:02}:{:02}.000",
        second / 3600,
        (second % 3600) / 60,
        second % 60
    );

    let mut cmd = Command::new("ffmpeg");
    // the snapshot arguments order is important
    cmd.args(["-ss", &second_str, "-i", &filename]);
    for arg in args {
        cmd.arg(&arg.key);
        if !arg.value.is_empty() {
            cmd.arg(&arg.value);
        }
    }
    cmd.stdout(Stdio::piped()).stderr(Stdio::piped());
    cmd
}

// analyze
pub fn analyze_ffprobe_command(path: String, args: &Vec<ArgKeyValue>) -> Command {
    let mut cmd = Command::new("ffprobe");
    for arg in args {
        cmd.arg(&arg.key);
        if !arg.value.is_empty() {
            cmd.arg(&arg.value);
        }
    }
    // cmd.arg("-v")
    //     .arg("error")
    //     .arg("-show_entries")
    //     .arg("format=duration")
    //     .arg("-of")
    //     .arg("default=noprint_wrappers=1:nokey=1")
    //     .arg(path)
    //     .stdout(Stdio::piped())
    //     .stderr(Stdio::piped());
    cmd.arg(path).stdout(Stdio::piped()).stderr(Stdio::piped());
    cmd
}
