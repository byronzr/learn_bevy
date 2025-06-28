pub trait MenuButtonType {
    fn next(&mut self);
}

// import button
#[derive(Debug, Default)]
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

impl MenuButtonType for MenuImportButton {
    fn next(&mut self) {
        *self = match self {
            MenuImportButton::Lock => MenuImportButton::Once,
            MenuImportButton::Once => MenuImportButton::Sequence,
            MenuImportButton::Sequence => MenuImportButton::Lock,
        };
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
impl MenuButtonType for MenuSaveButton {
    fn next(&mut self) {}
}

// clear button
#[derive(Debug, Default)]
pub struct MenuClearButton {
    pub checked: bool,
}
impl std::fmt::Display for MenuClearButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Clear")
    }
}
impl MenuButtonType for MenuClearButton {
    fn next(&mut self) {
        self.checked != self.checked;
    }
}
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
impl MenuButtonType for MenuHideButton {
    fn next(&mut self) {
        self.checked != self.checked;
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
impl MenuButtonType for MenuExitButton {
    fn next(&mut self) {}
}
