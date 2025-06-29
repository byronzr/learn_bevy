use std::any::Any;

use bevy::log::info;

pub trait MenuButtonNext: std::fmt::Display {
    fn next(&mut self) -> bool {
        false
    }
}

pub trait MenuButtonType: std::any::Any + Send + Sync + 'static {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: Any + Send + Sync> MenuButtonType for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

// import button
#[derive(Debug, Default, Clone, Copy)]
pub enum MenuImportButton {
    Lock,
    Once,
    #[default]
    Sequence,
}
impl std::fmt::Display for MenuImportButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MenuImportButton::Lock => write!(f, "Lock"),
            MenuImportButton::Once => write!(f, "Once"),
            MenuImportButton::Sequence => write!(f, "Sequence"),
        }
    }
}

impl MenuButtonNext for MenuImportButton {
    fn next(&mut self) -> bool {
        *self = match self {
            MenuImportButton::Lock => MenuImportButton::Once,
            MenuImportButton::Once => MenuImportButton::Sequence,
            MenuImportButton::Sequence => MenuImportButton::Lock,
        };
        if matches!(self, MenuImportButton::Lock) {
            true
        } else {
            false
        }
    }
}

// save button
#[derive(Debug, Default)]
pub struct MenuSaveButton;
impl std::fmt::Display for MenuSaveButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Save")
    }
}
impl MenuButtonNext for MenuSaveButton {}

// load button
#[derive(Debug, Default)]
pub struct MenuLoadButton;
impl std::fmt::Display for MenuLoadButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Load")
    }
}
impl MenuButtonNext for MenuLoadButton {}

// clear button
#[derive(Debug, Default)]
pub struct MenuClearButton;
impl std::fmt::Display for MenuClearButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Clear")
    }
}
impl MenuButtonNext for MenuClearButton {}

// hide button
#[derive(Debug, Default)]
pub struct MenuHideButton {
    pub checked: bool,
}
impl std::fmt::Display for MenuHideButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Hide")
    }
}
impl MenuButtonNext for MenuHideButton {
    fn next(&mut self) -> bool {
        self.checked = !self.checked;
        self.checked
    }
}

// setting button
#[derive(Debug, Default)]
pub struct MenuToggleSetting {
    pub checked: bool,
}

impl std::fmt::Display for MenuToggleSetting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Settings")
    }
}
impl MenuButtonNext for MenuToggleSetting {
    fn next(&mut self) -> bool {
        self.checked = !self.checked;
        self.checked
    }
}

// exit button
#[derive(Debug, Default)]
pub struct MenuExitButton;
impl std::fmt::Display for MenuExitButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Exit")
    }
}
impl MenuButtonNext for MenuExitButton {}
