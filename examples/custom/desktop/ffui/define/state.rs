use bevy::prelude::*;

#[derive(Debug, States, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Initialize,
    Monitor,
    Setting,
}
