use crate::notebook_util::Notebook;

#[derive(Default)]
pub struct EditorTab {
    pub name: String,
    pub path: String,
    pub content: Notebook,
    pub is_dirty: bool,
    pub is_read_only: bool,
}
