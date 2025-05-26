use std::fmt::Display;

#[derive(Default)]
pub enum InputMode {
    #[default]
    Normal,
    Insert,
    Command,
    Visual,
    VisualLine,
    VisualBlock,
    Replace,

    // not part of the input mode, but used for navigation in the UI
    UICursor,
}

impl Display for InputMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputMode::Normal => write!(f, "[Nor]"),
            InputMode::Insert => write!(f, "[Ins]"),
            InputMode::Command => write!(f, "[Cmd]"),
            InputMode::Visual => write!(f, "[Vis]"),
            InputMode::VisualLine => write!(f, "[ViL]"),
            InputMode::VisualBlock => write!(f, "[ViB]"),
            InputMode::Replace => write!(f, "[Rep]"),
            InputMode::UICursor => write!(f, "[Cur]"),
        }
    }
}
