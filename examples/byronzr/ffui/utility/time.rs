use crate::define::*;
use bevy::prelude::*;
use std::time::Duration;

pub fn parse_duration(content: &str) -> Option<Duration> {
    let str_duration = content;
    let parts: Vec<&str> = str_duration.split(':').collect();
    if parts.len() != 3 {
        return None;
    }
    let hours: u64 = parts[0].parse().ok()?;
    let minutes: u64 = parts[1].parse().ok()?;
    let seconds: f64 = parts[2].parse().ok()?;
    Some(Duration::new(
        hours * 3600 + minutes * 60 + seconds as u64 + 1,
        0,
    ))
}
