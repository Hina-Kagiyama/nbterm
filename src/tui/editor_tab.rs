use crate::notebook_util::Notebook;
use std::path::PathBuf;

pub struct EditorTab {
    pub name: String,
    pub path: PathBuf,
    pub content: Notebook,
    pub is_dirty: bool,
    pub is_read_only: bool,
}

impl Default for EditorTab {
    fn default() -> Self {
        Self {
            name: "[unnamed]".to_string(),
            // the system path for temporary files for this program, on unix it should be `/tmp/nbterm_tmp.ipynb`
            // should also consider using a more platform-independent way to get the temp directory
            path: std::env::temp_dir().join("nbterm_tmp.ipynb"),
            // a new notebook instance
            content: Notebook::default(),
            is_dirty: false,
            is_read_only: false,
        }
    }
}
