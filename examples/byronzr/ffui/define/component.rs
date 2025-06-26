use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct Container;

#[derive(Debug, Component)]
pub struct ProgressBar;

#[derive(Debug, Component)]
pub struct IndexOfline(pub usize);

#[derive(Debug, Component)]
pub struct TaskButton;

#[derive(Debug, Component)]
pub struct MenuButton;

#[derive(Debug, Component)]
pub struct FileLineBar;

#[derive(Debug, Component)]
pub struct ReplaceButton;

#[derive(Debug, Component)]
pub struct OpenButton;
