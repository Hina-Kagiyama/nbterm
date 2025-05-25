use ratatui::widgets::{Paragraph, Widget};

#[derive(Default)]
pub struct FilePicker {
    current_path: String,
    selected_file: Option<String>,
}

impl Widget for &FilePicker {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        // TODO: Implement the rendering logic for the file picker
        // placeholder implementation: just a plain block with right border
        Paragraph::new("File Picker")
            .block(ratatui::widgets::Block::default().borders(ratatui::widgets::Borders::RIGHT))
            .render(area, buf);
    }
}
