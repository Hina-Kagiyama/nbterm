use crate::notebook_util::Output;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};
use std::io;

use crate::notebook_util::{Cell, Notebook};

pub struct NotebookApp {
    pub notebook: Notebook,
    pub selected: usize,
}

impl NotebookApp {
    pub fn new_with_notebook(notebook: Notebook) -> Self {
        Self {
            notebook,
            selected: 0,
        }
    }

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
                let size = f.area();

                // First split: vertical -> A (left), right_pane (B+C)
                let main_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
                    .split(size);

                // Second split: right_pane -> B (top), C (bottom)
                let right_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Max(30), Constraint::Min(0)])
                    .split(main_chunks[1]);

                // === A: List of cells ===
                let items: Vec<ListItem> = self
                    .notebook
                    .iter()
                    .enumerate()
                    .map(|(i, cell)| {
                        let label = match cell {
                            Cell::Markdown(_) => format!("Markdown Cell {}", i),
                            Cell::Code(_) => format!("Code Cell {}", i),
                            Cell::Raw(_) => format!("Raw Cell {}", i),
                        };
                        ListItem::new(label)
                    })
                    .collect();

                let list = List::new(items)
                    .block(Block::default().borders(Borders::ALL).title("Cells"))
                    .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                    .highlight_symbol("> ");

                f.render_stateful_widget(list, main_chunks[0], &mut self.list_state());

                // Extract current cell
                let current_cell = self.notebook.iter().nth(self.selected);

                // === B: Cell Source ===
                let source_text = match current_cell {
                    Some(Cell::Markdown(cell)) => cell.source.join(""),
                    Some(Cell::Code(cell)) => cell.source.join(""),
                    Some(Cell::Raw(cell)) => cell.source.join(""),
                    None => String::new(),
                };

                let source = Paragraph::new(source_text)
                    .block(Block::default().borders(Borders::ALL).title("Source"));
                f.render_widget(source, right_chunks[0]);

                // === C: Cell Output ===
                let output_text = match current_cell {
                    Some(Cell::Code(cell)) => cell
                        .outputs
                        .iter()
                        .map(|output| match output {
                            Output::Stream { name, text } => {
                                format!("[{}]: {}", name, text.join(""))
                            }
                            Output::ExecuteResult { data, .. } => {
                                if let Some(txt) = data.get("text/plain").and_then(|v| v.as_str()) {
                                    txt.to_string()
                                } else {
                                    format!("{:?}", data)
                                }
                            }
                            Output::DisplayData { data, .. } => {
                                if let Some(b64) = data.get("image/png").and_then(|v| v.as_str()) {
                                    format!(
                                        "image/png: {}...",
                                        &b64.chars().take(80).collect::<String>()
                                    )
                                } else {
                                    format!("{:?}", data)
                                }
                            }
                            Output::Error {
                                ename,
                                evalue,
                                traceback,
                            } => {
                                format!("{}: {}\n{}", ename, evalue, traceback.join("\n"))
                            }
                        })
                        .collect::<Vec<String>>()
                        .join("\n\n"),
                    _ => String::new(),
                };

                let output = Paragraph::new(output_text)
                    .block(Block::default().borders(Borders::ALL).title("Output"));
                f.render_widget(output, right_chunks[1]);
            })?;

            // Handle input
            if event::poll(std::time::Duration::from_millis(250))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Down => {
                            if self.selected + 1 < self.notebook.len() {
                                self.selected += 1;
                            }
                        }
                        KeyCode::Up => {
                            if self.selected > 0 {
                                self.selected -= 1;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    fn list_state(&self) -> ratatui::widgets::ListState {
        let mut state = ratatui::widgets::ListState::default();
        if !self.notebook.is_empty() {
            state.select(Some(self.selected));
        }
        state
    }
}
