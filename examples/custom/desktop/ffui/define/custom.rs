use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub enum ProgressType {
    Total,
    Current,
}

#[derive(Debug, Clone, Copy)]
pub enum ProcessSignal {
    WindowClose,
    TaskInterrupt(usize),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProgressStatistics {
    pub total: u64,
    pub current: u64,
    pub percent: f64,
}

// Progress statistics for each file
// service Sender & Receiver
#[derive(Debug, Clone)]
pub struct ProgressInfo {
    pub progress_type: ProgressType,
    pub progress_value: u64,
    pub progress_index: Option<usize>,
}

impl ProgressInfo {
    // Create a new ProgressInfo instance with the Total type
    pub fn total(value: u64, idx: usize) -> Self {
        Self {
            progress_type: ProgressType::Total,
            progress_value: value,
            progress_index: Some(idx),
        }
    }
    // Create a new ProgressInfo instance with the Current type
    pub fn current(value: u64, idx: usize) -> Self {
        Self {
            progress_type: ProgressType::Current,
            progress_value: value,
            progress_index: Some(idx),
        }
    }
}

// Task status for each file
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    #[default]
    Waiting,
    Running,
    Done,
    Replaced,
}

// whole files information
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct FilesState {
    pub lines: Vec<String>,                           // each line is a file path
    pub status: Vec<TaskStatus>,                      // status of each file
    pub progress: HashMap<usize, ProgressStatistics>, // progress of each file
}
