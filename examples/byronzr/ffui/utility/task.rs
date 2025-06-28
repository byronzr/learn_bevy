use std::io::Write;

use crate::define::*;
use crate::utility::{create_ffmpeg_command_libx265, snapshot_ffmpeg_command};
use super::ffmpeg::{create_ffmpeg_command};
use super::time::parse_duration;
use tokio::io::{AsyncBufReadExt, AsyncReadExt};
use log::info;
use crate::TOKIO_RT;

pub fn task(index:usize,process_state: &ProcessState, path: String,soft:bool) {
    // preparse variations and move them into the background thread
    let tx = process_state.progress_tx.clone();
    let mut main_rx = process_state.main_tx.subscribe();

    // start a background thread to run ffmpeg
    std::thread::spawn(move || {
        info!("start ffmpeg process");
        let mut cmd = if soft {
            create_ffmpeg_command_libx265(path)
        }else{
            create_ffmpeg_command(path)
        };

        let mut process = ManagedProcess::new(&mut cmd).unwrap();
        let (stdout, stderr) = (process.stdout(), process.stderr());

        // 标准输出
        let mut stdout_lines = stdout.lines();
        let mut stderr_lines = stderr.lines();
        let mut stdoff = 0b00;
        
        loop {
            // wait for the ffmpeg process to finish until the main thread signals
            if let Ok(signal) = main_rx.try_recv(){
                if matches!(signal,ProcessSignal::WindowClose) {
                    info!("task stopped by main thread");
                    return;
                }
            }
            
            // return if stderr and stdout are both EOF
            if stdoff == 0b11 {
                info!("task completed");
                return;
            }

            TOKIO_RT.block_on(async{
                // read stdout and stderr lines (non-blocking)
                tokio::select! {
                    line = stdout_lines.next_line()=>{
                        match line.unwrap() {
                            Some(lin)=>{
                                if lin.contains("out_time=") {
                                    let Some(duration) = parse_duration(
                                    lin.trim().trim_start_matches("out_time=").trim(),
                                    ) else {
                                        info!("parse failed: {}", lin);
                                        return;
                                    };
                                    tx.send(ProgressInfo::current(duration.as_secs(), index)).await.unwrap();
                                } 
                            }
                            None=>{
                                // complete(EOF)
                                stdoff |= 0b01;
                                return;
                            }
                            
                        }
                    }
                    line = stderr_lines.next_line()=>{
                        match line.unwrap_or_else(|_| None) {
                            Some(lin)=>{
                                if lin.contains("Duration") {
                                    let vec_content: Vec<&str> = lin.split(',').collect();
                                    let (str_duration, _str_start, _str_bitrate) =
                                        (vec_content[0], vec_content[1], vec_content[2]);
                                    let Some(duration) = parse_duration(
                                        str_duration.trim().trim_start_matches("Duration: ").trim(),
                                    ) else {
                                        info!("parse failed: {}", str_duration);
                                        return;
                                    };
                                    tx.send(ProgressInfo::total(duration.as_secs(), index)).await.unwrap();
                                }
                            }
                            None=>{
                                // complete(EOF)
                                stdoff |= 0b10;
                                return;
                            }
                        }
                    }
                }
            });
        }
    });
}

pub fn replace(index:usize,path:String,data: &mut PathDatas) {
    let filename = std::path::Path::new(&path)
        .file_stem()
        .and_then(|name| name.to_str())
        .map(|name_str| format!("{}.mp4", name_str))
        .expect("Invalid file path");
    let dir =  std::path::Path::new(&path)
        .parent()
        .and_then(|name| name.to_str())
        .expect("Failed to get parent directory");

    let ext = std::path::Path::new(&path)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("unknown");

    let  target = format!("{}/{}", dir, filename);
    info!("local: {}", filename);
    info!("remote: {}", path);
    info!("target: {}", target);

    // To avoid "Cross-device link" error, use copy and remove instead of rename
    // remove remote file
    std::fs::remove_file(&path)
        .expect("Failed to remove file");
    // copy local file to remote
    std::fs::copy(&filename, &target)
        .expect("Failed to copy file");
    // remove local file
    std::fs::remove_file(&filename)
        .expect("Failed to remove file");

    // replace metadata.json file 
    let metadata_path = format!("{}/metadata.json", dir);
    info!("metadata path: {}", metadata_path);
    let content = std::fs::read_to_string(&metadata_path).unwrap();
    let from_str = format!("\"ext\":\"{}\"",ext);
    let to_str = format!("\"ext\":\"mp4\"");
    info!("replace {} with {}", from_str, to_str);
    let new_content = content.replace(&from_str, &to_str);
    std::fs::write(metadata_path, new_content).unwrap();

    data.state.status[index] = TaskStatus::Replaced;
}

pub fn snapshot(path:String)->Vec<u8>{

        info!("snapshot ffmpeg process");
        let mut cmd = snapshot_ffmpeg_command(path);

        let mut process = ManagedProcess::new(&mut cmd).unwrap();
        let buf = TOKIO_RT.block_on(async move {
            let mut png_bytes = Vec::new();
            process.child.stdout.as_mut().unwrap().read_to_end(&mut png_bytes).await.unwrap();
            println!("snapshot bytes: {}", png_bytes.len());
            //let mut file = std::fs::File::create("output.png").unwrap();
            // file.write_all(&png_bytes).unwrap();
            // info!("snapshot saved to output.png");
            process.child.wait().await.unwrap();
            png_bytes
        });
        info!("snapshot completed");
        buf
}

pub fn open_dir(path:String){
    let dir =  std::path::Path::new(&path)
        .parent()
        .and_then(|name| name.to_str())
        .expect("Failed to get parent directory");
    std::process::Command::new("open").arg(dir).spawn().unwrap();
}