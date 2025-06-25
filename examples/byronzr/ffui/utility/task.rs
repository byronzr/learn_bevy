use crate::define::*;
use super::ffmpeg::{create_ffmpeg_command};
use super::time::parse_duration;
use tokio::io::{AsyncBufReadExt, BufReader};
use crate::TOKIO_RT;

pub fn task(index:usize,process_state: &ProcessState, path: String) {
    // preparse variations and move them into the background thread
    let tx = process_state.progress_tx.clone();
    let mut main_rx = process_state.main_tx.subscribe();

    // start a background thread to run ffmpeg
    std::thread::spawn(move || {
        println!("start ffmpeg process");
        let mut cmd = create_ffmpeg_command(path);

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
                    println!("FFmpeg process stopped by main thread");
                    return;
                }
            }
            
            // return if stderr and stdout are both EOF
            if stdoff == 0b11 {
                println!("FFmpeg process completed");
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
                                        println!("无法解析时长: {}", lin);
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
                        match line.unwrap() {
                            Some(lin)=>{
                                if lin.contains("Duration") {
                                    let vec_content: Vec<&str> = lin.split(',').collect();
                                    let (str_duration, str_start, str_bitrate) =
                                        (vec_content[0], vec_content[1], vec_content[2]);
                                    let Some(duration) = parse_duration(
                                        str_duration.trim().trim_start_matches("Duration: ").trim(),
                                    ) else {
                                        println!("无法解析时长: {}", str_duration);
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