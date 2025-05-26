use ratatui::widgets::{Block, Widget};

use crate::tui::title_padding;

#[derive(Default)]
pub struct FilePicker {
    current_path: String,
    selected_file: Option<String>,
}

impl Widget for &FilePicker {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        const TITLE: &str = "File Picker";
        // TODO: Implement the rendering logic for the file picker
        // placeholder implementation: just a plain block with right border
        let file_picker_widget = Block::default()
            .title(title_padding(area, TITLE))
            .borders(ratatui::widgets::Borders::RIGHT)
            .title_style(
                ratatui::style::Style::default()
                    .fg(ratatui::style::Color::Black)
                    .bg(ratatui::style::Color::DarkGray)
                    .add_modifier(ratatui::style::Modifier::BOLD),
            );

        file_picker_widget.render(area, buf);
    }
}
