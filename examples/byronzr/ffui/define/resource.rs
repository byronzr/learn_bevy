use crate::define::MenuImportButton;

use super::custom::*;
use bevy::prelude::*;
use tokio::sync::{broadcast, mpsc};

#[derive(Debug, Resource)]
pub struct ProcessState {
    pub progress_tx: mpsc::Sender<ProgressInfo>,
    pub progress_rx: mpsc::Receiver<ProgressInfo>,
    pub main_tx: broadcast::Sender<ProcessSignal>,
}

#[derive(Debug, Resource, Default)]
pub struct PathDatas {
    pub state: FilesState,             // the information of each file
    pub entities: Vec<Option<Entity>>, // Store the entity of line lyaout conainter
    pub changed: bool,                 // Flag to indicate if the state has changed
}

#[derive(Debug, Resource)]
pub struct ProcessMenu {
    pub import_type: MenuImportButton,
    pub hide_done: bool,
}

#[derive(Debug, Resource)]
pub struct FontHandle(pub Handle<Font>);
