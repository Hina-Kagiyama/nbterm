use nbterm::tui::NotebookApp;

fn main() -> anyhow::Result<()> {
    let mut app = NotebookApp::default();
    app.run()?;
    Ok(())
}
