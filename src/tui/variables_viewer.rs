use ratatui::widgets::Widget;

use crate::tui::title_padding;

#[derive(Default)]
pub struct VariablesViewer {
    variables: Vec<Variable>,
}

pub struct Variable {
    name: String,
    value: String,
}

impl Widget for &VariablesViewer {
    fn render(self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer) {
        const TITLE: &str = "Variables";
        // a list of variables
        let variables_widget =
            ratatui::widgets::List::new(self.variables.iter().map(|var| {
                ratatui::widgets::ListItem::new(format!("{}: {}", var.name, var.value))
            }))
            .block(
                ratatui::widgets::Block::default()
                    .title(title_padding(area, TITLE))
                    .title_alignment(ratatui::layout::Alignment::Left)
                    .title_style(
                        ratatui::style::Style::default()
                            .fg(ratatui::style::Color::Black)
                            .bg(ratatui::style::Color::DarkGray)
                            .add_modifier(ratatui::style::Modifier::BOLD),
                    )
                    .borders(ratatui::widgets::Borders::LEFT),
            )
            .highlight_style(ratatui::style::Style::default().fg(ratatui::style::Color::Yellow));

        variables_widget.render(area, buf);
    }
}
