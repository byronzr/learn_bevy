use crate::define::*;
use crate::utility::parse_duration;
use bevy::prelude::*;
use std::io::{BufRead, BufReader};
use std::process::{Child, Command, Stdio};

pub fn button_interaction(
    mut interaction_query: Query<
        (Entity, &Interaction, &IndexOfline, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    data: Res<PathDatas>,
    process_state: Res<ProcessState>,
) -> Result {
    for (entity, interaction, idx, mut bg) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Hovered => {
                *bg = BackgroundColor(Color::srgb_u8(0, 84, 0));
            }
            Interaction::Pressed => {
                *bg = BackgroundColor(Color::srgb_u8(84, 84, 84));
                let Some(path) = data.lines.get(idx.0).cloned() else {
                    return Ok(());
                };
                println!("convert path: {}", path);
                // ffmpeg -hwaccel videotoolbox -nostdin \
                // -i "$1" \
                // -vf 'scale=-2:720,fps=30' \
                // -c:v hevc_videotoolbox \
                // -quality high -b:v 2000k -maxrate 8000k -bufsize 16000k \
                // -c:a copy \
                // -tag:v hvc1 \
                // -stats \
                // "$bn"
                let tx1 = process_state.tx.clone();
                let tx2 = process_state.tx.clone();
                std::thread::spawn(move || {
                    println!("start ffmpeg process");
                    let mut cmd = Command::new("ffmpeg");
                    cmd.arg("-hwaccel")
                        .arg("videotoolbox")
                        //.arg("-nostdin")
                        .arg("-i")
                        .arg(path) // 替换为你的命令
                        .args(["-vf", "scale=-2:720,fps=30"])
                        .arg("-c:v")
                        .arg("hevc_videotoolbox")
                        .args(["-quality", " high"])
                        .args(["-b:v", "2000k"])
                        .args(["-maxrate", "8000k"])
                        .args(["-bufsize", "16000k"])
                        .args(["-c:a", "copy"])
                        .args(["-tag:v", "hvc1"])
                        .arg("-loglevel")
                        .arg("info")
                        .arg("-progress")
                        .arg("pipe:1")
                        //.arg("-stats")
                        .arg("out.mp4") // 覆盖输出文件
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped());

                    let (_process, stdout, stderr) = ManagedProcess::new(&mut cmd).unwrap();

                    // let stdout = BufReader::new(process.child.stdout.take().unwrap());
                    // let stderr = BufReader::new(process.child.stderr.take().unwrap());

                    // 标准输出
                    let stdout_handle = std::thread::spawn(move || {
                        for line in stdout.lines() {
                            let Ok(lin) = line else {
                                continue;
                            };
                            if lin.contains("out_time=") {
                                let Some(duration) = parse_duration(
                                    lin.trim().trim_start_matches("out_time=").trim(),
                                ) else {
                                    println!("无法解析时长: {}", lin);
                                    continue;
                                };
                                let content = format!("FFmpeg stout: {}", duration.as_secs());
                                tx1.send(content).unwrap();
                            }
                        }
                        println!("FFmpeg stdout stream closed");
                    });

                    // 标准错误
                    let stderr_handle = std::thread::spawn(move || {
                        for line in stderr.lines() {
                            let Ok(lin) = line else {
                                continue;
                            };

                            if lin.contains("Duration") {
                                let vec_content: Vec<&str> = lin.split(',').collect();
                                let (str_duration, str_start, str_bitrate) =
                                    (vec_content[0], vec_content[1], vec_content[2]);
                                let Some(duration) = parse_duration(
                                    str_duration.trim().trim_start_matches("Duration: ").trim(),
                                ) else {
                                    println!("无法解析时长: {}", str_duration);
                                    continue;
                                };
                                let content = format!("FFmpeg stderr: {}", duration.as_secs());
                                tx2.send(content).unwrap();
                            }
                        }
                        println!("FFmpeg stderr stream closed");
                    });

                    // 等待进程结束
                    std::thread::JoinHandle::join(stdout_handle).unwrap();
                    std::thread::JoinHandle::join(stderr_handle).unwrap();
                });
            }
            Interaction::None => {
                *bg = BackgroundColor(Color::srgb_u8(0, 0, 0));
            }
        }
    }
    Ok(())
}
