use bevy::prelude::*;

pub mod setup;
pub use setup::*;

pub mod interaction;
pub use interaction::*;

#[derive(Debug, Component)]
pub enum PanelMenuButton {
    ShowLog,
    EnemyAppear,
    DebugRender,
    VirtualTurret,
    DetectTest,
    MeshMode,
    EngineFlame,
    LockPlayer,
    GameSpeed,
}

#[derive(Component, Debug)]
pub struct UILayoutMain;
