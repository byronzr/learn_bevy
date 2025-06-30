use crate::define::*;
use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct LinesContainer;

#[derive(Debug, Component)]
pub struct SettingContainer;

#[derive(Debug, Component)]
pub struct ProgressBar;

#[derive(Debug, Component)]
pub struct IndexOfline(pub usize);

#[derive(Debug, Component)]
pub struct TaskButton;

#[derive(Debug, Component)]
pub struct TaskButtonType(pub bool);

// Define a new trait that combines MenuButtonType and MenuButtonNext
pub trait MenuButtonTrait: MenuButtonType + MenuButtonNext + std::fmt::Debug {}

impl<T: MenuButtonType + MenuButtonNext + std::fmt::Debug> MenuButtonTrait for T {}

#[derive(Component)]
pub struct MenuButton {
    pub button_type: Box<dyn MenuButtonTrait + Send + Sync + 'static>,
}

#[derive(Debug, Component)]
pub struct FileLineBar;

#[derive(Debug, Component)]
pub struct ReplaceButton;

#[derive(Debug, Component)]
pub struct SnapshotButton(pub bool);

#[derive(Debug, Component)]
pub struct OpenButton;
#[derive(Debug, Component)]
pub struct PreviewWindow;
