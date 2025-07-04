use crate::define::MenuImportButton;

use super::custom::*;
use bevy::prelude::*;
use tokio::sync::{broadcast, mpsc};

#[derive(Debug, Resource)]
pub struct ProcessState {
    pub progress_tx: mpsc::Sender<ProgressInfo>,
    pub progress_rx: mpsc::Receiver<ProgressInfo>,
    pub main_tx: broadcast::Sender<ProcessSignal>,
    pub layout: Option<Entity>,
    pub toast_message: Vec<String>,
    pub toast_tx: mpsc::Sender<String>,
    pub toast_rx: mpsc::Receiver<String>,
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
    pub toggle_setting: bool,
}

#[derive(Debug, Resource)]
pub struct FontHandle(pub Handle<Font>);
