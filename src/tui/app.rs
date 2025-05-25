use super::{
    editor_tab::EditorTab, file_picker::FilePicker, outliner::Outliner, settings::Settings,
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

use ratatui::{
    Terminal,
    layout::{Constraint, Direction, Layout},
    prelude::{CrosstermBackend, StatefulWidget, Widget},
};

use std::io;

pub struct NotebookApp {
    left_pane_mode: Option<LeftPaneMode>,
    right_pane_mode: Option<RightPaneMode>,
    file_path: Option<String>,
    file_picker: FilePicker,
    outliner: Outliner,
    settings: Settings,
    tabs: Vec<EditorTab>,
}

impl Default for NotebookApp {
    fn default() -> Self {
        Self {
            left_pane_mode: Some(LeftPaneMode::FilePicker),
            right_pane_mode: None,
            file_path: None,
            file_picker: FilePicker::default(),
            outliner: Outliner::default(),
            settings: Settings::default(),
            tabs: vec![],
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

                // Divide the terminal into 3 sections vertically
                // - the tabline at the top, 1 line
                // - the editor and the left and right panes
                // - the status bar at the bottom, 1 line

                let terminal_layout = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(1),
                        Constraint::Min(0),
                        Constraint::Length(1),
                    ])
                    .split(area);

                let tabline_area = terminal_layout[0];
                let editor_area = terminal_layout[1];
                let status_bar_area = terminal_layout[2];

                // Draw the tabline
                f.render_widget(
                    ratatui::widgets::Block::default().title("Tabline"),
                    tabline_area,
                );

                // Draw the editor area
                // split the editor area into:
                // - the left pane, 1/5 of the width, but at least 20 characters, if it is open.
                // - the editor area, 3/5 of the width
                // - the right pane, 1/5 of the width, but at least 20 characters, if it is open.
                let editor_layout = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(match (&self.left_pane_mode, &self.right_pane_mode) {
                        (Some(_), Some(_)) => {
                            vec![
                                Constraint::Length(20),
                                Constraint::Min(0),
                                Constraint::Length(20),
                            ]
                        }
                        (Some(_), None) => {
                            vec![Constraint::Length(20), Constraint::Min(0)]
                        }
                        (None, Some(_)) => {
                            vec![Constraint::Min(0), Constraint::Length(20)]
                        }
                        _ => vec![Constraint::Min(0)],
                    })
                    .split(editor_area);

                // render the left pane on need
                if let Some(left_pane_mode) = &self.left_pane_mode {
                    match left_pane_mode {
                        LeftPaneMode::FilePicker => {
                            self.file_picker.render(editor_layout[0], f.buffer_mut());
                        }
                        LeftPaneMode::Outline => {
                            self.outliner.render(editor_layout[0], f.buffer_mut());
                        }
                    }
                }

                // Draw the status bar
                f.render_widget(
                    ratatui::widgets::Block::default().title("Status Bar"),
                    status_bar_area,
                );
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
