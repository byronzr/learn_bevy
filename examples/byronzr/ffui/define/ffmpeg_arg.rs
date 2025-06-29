use bevy_ecs::resource::Resource;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArgKeyValue {
    pub key: String,
    pub value: String,
}
impl ArgKeyValue {
    pub fn new(key: &str, value: &str) -> Self {
        Self {
            key: key.to_string(),
            value: value.to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Resource)]
pub struct FfmpegArg {
    pub sf_convert: Vec<ArgKeyValue>,
    pub hw_convert: Vec<ArgKeyValue>,
    pub snapshot: Vec<ArgKeyValue>,
    pub analyze: Vec<ArgKeyValue>,
}

impl Default for FfmpegArg {
    fn default() -> Self {
        let mut hw_convert = vec![];
        {
            hw_convert.push(ArgKeyValue::new("-hwaccel", "videotoolbox"));
            hw_convert.push(ArgKeyValue::new("-nostdin", ""));
            hw_convert.push(ArgKeyValue::new("-vf", "scale=-2:720,fps=30"));
            hw_convert.push(ArgKeyValue::new("-c:v", "hevc_videotoolbox"));
            hw_convert.push(ArgKeyValue::new("-quality", "high"));
            hw_convert.push(ArgKeyValue::new("-b:v", "2000k"));
            hw_convert.push(ArgKeyValue::new("-maxrate", "8000k"));
            hw_convert.push(ArgKeyValue::new("-bufsize", "16000k"));
            hw_convert.push(ArgKeyValue::new("-c:a", "copy"));
            hw_convert.push(ArgKeyValue::new("-tag:v", "hvc1"));
            hw_convert.push(ArgKeyValue::new("-loglevel", "info"));
            hw_convert.push(ArgKeyValue::new("-progress", "pipe:1"));
        }
        let mut sf_convert = vec![];
        {
            sf_convert.push(ArgKeyValue::new("-nostdin", ""));
            sf_convert.push(ArgKeyValue::new("-vf", "scale=-2:720,fps=30"));
            sf_convert.push(ArgKeyValue::new("-c:v", "libx264"));
            sf_convert.push(ArgKeyValue::new("-quality", "high"));
            sf_convert.push(ArgKeyValue::new("-b:v", "2000k"));
            sf_convert.push(ArgKeyValue::new("-maxrate", "8000k"));
            sf_convert.push(ArgKeyValue::new("-bufsize", "16000k"));
            sf_convert.push(ArgKeyValue::new("-c:a", "copy"));
            sf_convert.push(ArgKeyValue::new("-tag:v", "hvc1"));
            sf_convert.push(ArgKeyValue::new("-loglevel", "info"));
            sf_convert.push(ArgKeyValue::new("-progress", "pipe:1"));
            sf_convert.push(ArgKeyValue::new("-pix_fmt", "yuv420p"));
        }

        let mut snapshot = vec![];
        {
            snapshot.push(ArgKeyValue::new("-frames:v", "1"));
            snapshot.push(ArgKeyValue::new("-vcodec", "png"));
            snapshot.push(ArgKeyValue::new("-vf", "scale=-2:180"));
            snapshot.push(ArgKeyValue::new("-q:v", "2"));
            snapshot.push(ArgKeyValue::new("-f", "image2pipe"));
            snapshot.push(ArgKeyValue::new("-", ""));
        }
        let mut analyze = vec![];
        {
            analyze.push(ArgKeyValue::new("-v", "error"));
            analyze.push(ArgKeyValue::new("-show_entries", "format=duration"));
            analyze.push(ArgKeyValue::new(
                "-of",
                "default=noprint_wrappers=1:nokey=1",
            ));
        }

        Self {
            sf_convert,
            hw_convert,
            snapshot,
            analyze,
        }
    }
}
