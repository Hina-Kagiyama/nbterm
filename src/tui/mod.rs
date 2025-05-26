pub mod app;
pub use app::NotebookApp;
pub mod editor_commands;
pub mod editor_tab;
pub mod file_picker;
pub mod input_mode;
pub mod outliner;
pub mod settings;
pub mod variables_viewer;

pub(crate) fn title_padding(area: ratatui::layout::Rect, title: &str) -> String {
    let padding = area.width.saturating_sub(title.len() as u16);
    format!("{}{}", title, " ".repeat(padding as usize))
}
