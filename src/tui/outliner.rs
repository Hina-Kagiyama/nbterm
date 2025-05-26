use ratatui::widgets::Widget;

#[derive(Default)]
pub struct Outliner {
    pub items: Vec<OutlineItem>,
}

#[derive(Default)]
pub struct OutlineItem {
    pub content: String,
    pub relative_tab: usize,
    pub relative_pos: (usize, usize),
}

impl Widget for &Outliner {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        // TODO: Implement the rendering logic for the outliner
        // placeholder implementation: just a plain block with left border
        ratatui::widgets::Paragraph::new("Outliner")
            .block(ratatui::widgets::Block::default().borders(ratatui::widgets::Borders::LEFT))
            .render(area, buf);
    }
}
