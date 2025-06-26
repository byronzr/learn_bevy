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

#[derive(Debug, Clone)]
pub struct ProgressInfo {
    pub progress_type: ProgressType,
    pub progress_value: u64,
    pub progress_index: Option<usize>,
}

#[derive(Debug)]
pub struct ProgressStatistics {
    pub total: u64,
    pub current: u64,
    pub percent: f64,
}

impl ProgressInfo {
    pub fn total(value: u64, idx: usize) -> Self {
        Self {
            progress_type: ProgressType::Total,
            progress_value: value,
            progress_index: Some(idx),
        }
    }
    pub fn current(value: u64, idx: usize) -> Self {
        Self {
            progress_type: ProgressType::Current,
            progress_value: value,
            progress_index: Some(idx),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum TaskStatus {
    #[default]
    Waiting,
    Running,
    Done,
    Replaced,
}

#[derive(Debug, Default)]
pub struct FilesState {
    pub lines: Vec<String>,
    pub status: Vec<TaskStatus>,
}
