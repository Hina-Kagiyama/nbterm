use nbterm::notebook_util::Notebook;
use nbterm::tui::NotebookApp;

fn main() -> anyhow::Result<()> {
    let notebook = Notebook::from_file("example.ipynb")?;
    let mut app = NotebookApp::new_with_notebook(notebook);
    app.run()?;
    Ok(())
}
