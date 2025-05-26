use super::{
    editor_tab::EditorTab, file_picker::FilePicker, input_mode::InputMode, outliner::Outliner,
    settings::Settings, variables_viewer::VariablesViewer,
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

use ratatui::{
    Terminal,
    layout::{Constraint, Direction, Layout},
    prelude::{CrosstermBackend, Widget},
};

use std::io;

pub struct NotebookApp {
    left_pane_mode: Option<LeftPaneMode>,
    right_pane_mode: Option<RightPaneMode>,
    file_path: Option<String>,
    file_picker: FilePicker,
    outliner: Outliner,
    variables: VariablesViewer,
    settings: Settings,
    tabs: Vec<EditorTab>,
    tab_selected: usize,
    input_mode: InputMode,
}

impl Default for NotebookApp {
    fn default() -> Self {
        Self {
            left_pane_mode: None,
            right_pane_mode: None,
            file_path: None,
            file_picker: FilePicker::default(),
            outliner: Outliner::default(),
            variables: VariablesViewer::default(),
            settings: Settings::default(),
            tabs: vec![EditorTab::default()],
            tab_selected: 0,
            input_mode: InputMode::default(),
        }
    }
}

#[derive(Default)]
pub enum LeftPaneMode {
    #[default]
    FilePicker,
    Outline,
}

#[derive(Default)]
pub enum RightPaneMode {
    #[default]
    Symbols,
    Variables,
}

impl NotebookApp {
    pub fn run(&mut self) -> io::Result<()> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let res = self.ui_loop(&mut terminal);

        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        res
    }

    fn ui_loop<B: ratatui::backend::Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
    ) -> io::Result<()> {
        loop {
            terminal.draw(|f| {
                let area = f.area();

                // split the terminal into two vertical sections
                // - upper section for the editor
                // - lower section for the status bar, 1 line high
                let terminal_layout = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Min(0), Constraint::Length(1)].as_ref())
                    .split(area);

                // then split the upper section into horizontal sections on need
                let main_section = terminal_layout[0];
                let main_section_layout = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(
                        match (
                            self.left_pane_mode.is_some(),
                            self.right_pane_mode.is_some(),
                        ) {
                            (true, true) => vec![
                                Constraint::Length(30), // Left pane
                                Constraint::Min(0),     // Main content
                                Constraint::Length(30), // Right pane
                            ],
                            (true, false) => vec![
                                Constraint::Length(30), // Left pane
                                Constraint::Min(0),     // Main content
                            ],
                            (false, true) => vec![
                                Constraint::Min(0),     // Main content
                                Constraint::Length(30), // Right pane
                            ],
                            (false, false) => vec![Constraint::Min(0)], // Only main content
                        },
                    )
                    .split(main_section);

                // Draw the left pane if it is enabled
                if let Some(left_mode) = &self.left_pane_mode {
                    match left_mode {
                        LeftPaneMode::FilePicker => {
                            self.file_picker
                                .render(main_section_layout[0], f.buffer_mut());
                        }
                        LeftPaneMode::Outline => {
                            self.outliner.render(main_section_layout[0], f.buffer_mut());
                        }
                    }
                }

                // get the main content area
                let main_content_area = if self.left_pane_mode.is_some() {
                    main_section_layout[1]
                } else {
                    main_section_layout[0]
                };

                // divide the main content area into:
                // - upper section for the editor tabs, 1 line high, if there are more than one tab
                // - lower section for the editor content
                let main_content_layout = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(if self.tabs.len() > 1 {
                        vec![Constraint::Length(1), Constraint::Min(0)]
                    } else {
                        vec![Constraint::Min(0)]
                    })
                    .split(main_content_area);
                // Draw the editor tabs if there are more than one tab
                if self.tabs.len() > 1 {
                    let tab_area = main_content_layout[0];
                    // Here you would render the tabs, for now we just draw a placeholder
                    let tab_widget = ratatui::widgets::Tabs::default()
                        .titles(
                            self.tabs
                                .iter()
                                .map(|tab| tab.name.clone() + (if tab.is_dirty { "*" } else { "" }))
                                .collect::<Vec<_>>(),
                        )
                        .style(
                            ratatui::style::Style::default()
                                .fg(ratatui::style::Color::White)
                                .bg(ratatui::style::Color::DarkGray),
                        )
                        .highlight_style(
                            ratatui::style::Style::default()
                                .fg(ratatui::style::Color::Black)
                                .bg(ratatui::style::Color::White)
                                .add_modifier(ratatui::style::Modifier::BOLD),
                        )
                        .select(self.tab_selected);
                    tab_widget.render(tab_area, f.buffer_mut());
                }

                // Draw the editor content in the lower section
                // TODO: Implement the actual editor rendering logic
                // For now, we just draw a placeholder
                let editor_area = if self.tabs.len() > 1 {
                    main_content_layout[1]
                } else {
                    main_content_layout[0]
                };
                let editor_widget = ratatui::widgets::Paragraph::new("Editor Content Placeholder")
                    .block(
                        ratatui::widgets::Block::default()
                            .borders(ratatui::widgets::Borders::ALL)
                            .title("Editor"),
                    );
                editor_widget.render(editor_area, f.buffer_mut());

                // Draw the right pane if it is enabled
                if let Some(right_mode) = &self.right_pane_mode {
                    match right_mode {
                        RightPaneMode::Symbols => {
                            // Placeholder for symbols outliner pane
                            self.outliner
                                .render(*main_section_layout.last().unwrap(), f.buffer_mut());
                        }
                        RightPaneMode::Variables => {
                            // Placeholder for variables pane
                            self.variables
                                .render(*main_section_layout.last().unwrap(), f.buffer_mut());
                        }
                    }
                }

                // Draw the status bar at the bottom
                // This is a simple status bar showing the current input mode
                let status_bar_area = terminal_layout[1];
                let status_bar_widget = ratatui::widgets::Paragraph::new(format!(
                    "Input Mode: {} | Press 'q' to quit",
                    self.input_mode
                ))
                .style(
                    ratatui::style::Style::default()
                        .fg(ratatui::style::Color::White)
                        .bg(ratatui::style::Color::DarkGray),
                );
                status_bar_widget.render(status_bar_area, f.buffer_mut());
            })?;

            // Handle input
            if event::poll(std::time::Duration::from_millis(250))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        event::KeyCode::Char('q') => break Ok(()),
                        _ => {}
                    }
                }
            }
        }
    }
}
