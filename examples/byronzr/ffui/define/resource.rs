use super::custom::*;
use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use tokio::sync::{broadcast, mpsc};

#[derive(Debug, Resource)]
pub struct ProcessState {
    pub progress_tx: mpsc::Sender<ProgressInfo>,
    pub progress_rx: mpsc::Receiver<ProgressInfo>,
    pub main_tx: broadcast::Sender<ProcessSignal>,
    pub progress: HashMap<usize, ProgressStatistics>,
}

#[derive(Debug, Resource, Default)]
pub struct PathDatas {
    pub state: FilesState,
    pub entities: Vec<Option<Entity>>,
    pub changed: bool,
}

#[derive(Debug, Resource)]
pub struct ProcessMenu {
    pub lock_import: bool,
    pub hide_done: bool,
}
