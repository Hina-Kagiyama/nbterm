use super::types::*;
use std::slice::{Iter, IterMut};

impl Notebook {
    /// Returns an immutable iterator over all cells in the notebook.
    ///
    /// This allows iterating over cells in read-only mode:
    /// ```rust
    /// for cell in notebook.iter() {
    ///     // inspect cell
    /// }
    /// ```
    pub fn iter(&self) -> Iter<'_, Cell> {
        self.cells.iter()
    }

    /// Returns a mutable iterator over all cells in the notebook.
    ///
    /// Allows modifying each cell during iteration:
    /// ```rust
    /// for cell in notebook.iter_mut() {
    ///     // modify cell
    /// }
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<'_, Cell> {
        self.cells.iter_mut()
    }

    /// Returns the number of cells in the notebook.
    ///
    /// Equivalent to `notebook.iter().count()` but more efficient.
    pub fn len(&self) -> usize {
        self.cells.len()
    }

    /// Returns `true` if the notebook has no cells.
    ///
    /// This is a shortcut for `notebook.len() == 0`.
    pub fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }

    /// Returns an iterator over all code cells in the notebook.
    ///
    /// This filters and yields only cells of type `Cell::Code`:
    /// ```rust
    /// for code_cell in notebook.code_cells() {
    ///     println!("{:?}", code_cell.source);
    /// }
    /// ```
    pub fn code_cells(&self) -> impl Iterator<Item = &CodeCell> {
        self.cells.iter().filter_map(|cell| match cell {
            Cell::Code(code) => Some(code),
            _ => None,
        })
    }

    /// Returns an iterator over all markdown cells in the notebook.
    ///
    /// This filters and yields only cells of type `Cell::Markdown`.
    pub fn markdown_cells(&self) -> impl Iterator<Item = &MarkdownCell> {
        self.cells.iter().filter_map(|cell| match cell {
            Cell::Markdown(md) => Some(md),
            _ => None,
        })
    }
}

// IntoIterator implementations
impl IntoIterator for Notebook {
    type Item = Cell;
    type IntoIter = std::vec::IntoIter<Cell>;

    fn into_iter(self) -> Self::IntoIter {
        self.cells.into_iter()
    }
}

impl<'a> IntoIterator for &'a Notebook {
    type Item = &'a Cell;
    type IntoIter = std::slice::Iter<'a, Cell>;

    fn into_iter(self) -> Self::IntoIter {
        self.cells.iter()
    }
}

impl<'a> IntoIterator for &'a mut Notebook {
    type Item = &'a mut Cell;
    type IntoIter = std::slice::IterMut<'a, Cell>;

    fn into_iter(self) -> Self::IntoIter {
        self.cells.iter_mut()
    }
}

use serde_json::json;

impl Notebook {
    // === Internal cell handling ===

    /// Inserts a cell at the specified index.
    ///
    /// This is a private utility method used by public `insert_*_cell` methods.
    /// Returns `true` if the index was valid and insertion succeeded,
    /// or `false` if the index was out of bounds.
    ///
    /// # Arguments
    /// * `index` - The position at which to insert the cell.
    /// * `cell` - The `Cell` to insert.
    fn insert_cell(&mut self, index: usize, cell: Cell) -> bool {
        if index <= self.cells.len() {
            self.cells.insert(index, cell);
            true
        } else {
            false
        }
    }

    /// Pushes a cell to the end of the notebook.
    ///
    /// This is a private method used by public `push_*_cell` methods.
    ///
    /// # Arguments
    /// * `cell` - The `Cell` to append.
    fn push_cell(&mut self, cell: Cell) {
        self.cells.push(cell);
    }

    // === Public interfaces for structured cells ===

    /// Inserts a new Markdown cell at the specified index.
    ///
    /// Returns `true` if the index was valid and the insertion succeeded,
    /// or `false` if the index was out of bounds.
    ///
    /// # Arguments
    /// * `index` - The position to insert the cell.
    /// * `source` - A list of lines or strings that make up the markdown content.
    ///
    /// # Example
    /// ```
    /// notebook.insert_markdown_cell(0, vec!["# Heading", "Some description"]);
    /// ```
    pub fn insert_markdown_cell<S: Into<String>>(&mut self, index: usize, source: Vec<S>) -> bool {
        let cell = Cell::Markdown(MarkdownCell {
            source: source.into_iter().map(Into::into).collect(),
            metadata: json!({}),
        });
        self.insert_cell(index, cell)
    }

    /// Inserts a new Raw cell at the specified index.
    ///
    /// Returns `true` if the index was valid and the insertion succeeded,
    /// or `false` if the index was out of bounds.
    ///
    /// # Arguments
    /// * `index` - The position to insert the cell.
    /// * `source` - The content of the raw cell as lines.
    ///
    /// # Example
    /// ```
    /// notebook.insert_raw_cell(2, vec!["Raw content"]);
    /// ```
    pub fn insert_raw_cell<S: Into<String>>(&mut self, index: usize, source: Vec<S>) -> bool {
        let cell = Cell::Raw(RawCell {
            source: source.into_iter().map(Into::into).collect(),
            metadata: json!({}),
        });
        self.insert_cell(index, cell)
    }

    /// Appends a new Markdown cell to the end of the notebook.
    ///
    /// # Arguments
    /// * `source` - A list of lines or strings that make up the markdown content.
    ///
    /// # Example
    /// ```
    /// notebook.push_markdown_cell(vec!["## Section", "Details follow here..."]);
    /// ```
    pub fn push_markdown_cell<S: Into<String>>(&mut self, source: Vec<S>) {
        let cell = Cell::Markdown(MarkdownCell {
            source: source.into_iter().map(Into::into).collect(),
            metadata: json!({}),
        });
        self.push_cell(cell);
    }

    /// Appends a new Raw cell to the end of the notebook.
    ///
    /// # Arguments
    /// * `source` - A list of strings or lines for the raw cell content.
    ///
    /// # Example
    /// ```
    /// notebook.push_raw_cell(vec!["Unformatted text"]);
    /// ```
    pub fn push_raw_cell<S: Into<String>>(&mut self, source: Vec<S>) {
        let cell = Cell::Raw(RawCell {
            source: source.into_iter().map(Into::into).collect(),
            metadata: json!({}),
        });
        self.push_cell(cell);
    }
}

impl Notebook {
    /// Inserts a new Code cell at the specified index.
    ///
    /// Returns `true` if the index was valid and the insertion succeeded,
    /// or `false` if the index was out of bounds.
    ///
    /// # Arguments
    /// * `index` - The position to insert the cell.
    /// * `source` - A list of lines or strings that make up the code content.
    /// * `execution_count` - Optional execution count (e.g. `Some(1)` or `None`).
    /// * `outputs` - The outputs from the cell's execution.
    ///
    /// # Example
    /// ```
    /// notebook.insert_code_cell(0, vec!["print(\"Hello\")"], Some(1), vec![]);
    /// ```
    pub fn insert_code_cell<S: Into<String>>(
        &mut self,
        index: usize,
        source: Vec<S>,
        execution_count: Option<u32>,
        outputs: Vec<Output>,
    ) -> bool {
        let cell = Cell::Code(CodeCell {
            source: source.into_iter().map(Into::into).collect(),
            metadata: json!({}),
            execution_count,
            outputs,
        });
        self.insert_cell(index, cell)
    }

    /// Appends a new Code cell to the end of the notebook.
    ///
    /// # Arguments
    /// * `source` - Code lines to include in the cell.
    /// * `execution_count` - Optional execution count.
    /// * `outputs` - Cell outputs from execution.
    ///
    /// # Example
    /// ```
    /// notebook.push_code_cell(vec!["a = 1 + 2"], Some(1), vec![]);
    /// ```
    pub fn push_code_cell<S: Into<String>>(
        &mut self,
        source: Vec<S>,
        execution_count: Option<u32>,
        outputs: Vec<Output>,
    ) {
        let cell = Cell::Code(CodeCell {
            source: source.into_iter().map(Into::into).collect(),
            metadata: json!({}),
            execution_count,
            outputs,
        });
        self.push_cell(cell);
    }
}

impl Output {
    /// Creates a stream output for stdout.
    ///
    /// # Example
    /// ```
    /// let out = Output::stream_stdout("hello\n");
    /// ```
    pub fn stream_stdout<S: Into<String>>(text: S) -> Self {
        Output::Stream {
            name: "stdout".to_string(),
            text: vec![text.into()],
        }
    }

    /// Creates a stream output for stderr.
    pub fn stream_stderr<S: Into<String>>(text: S) -> Self {
        Output::Stream {
            name: "stderr".to_string(),
            text: vec![text.into()],
        }
    }

    /// Creates an execute_result output with plain text data.
    ///
    /// # Example
    /// ```
    /// let out = Output::execute_result(1, "3");
    /// ```
    pub fn execute_result<S: Into<String>>(execution_count: u32, result: S) -> Self {
        Output::ExecuteResult {
            execution_count,
            data: serde_json::json!({
                "text/plain": result.into()
            }),
            metadata: serde_json::json!({}),
        }
    }

    /// Creates an error output from a message and traceback.
    pub fn error<S: Into<String>>(ename: S, evalue: S, traceback: Vec<S>) -> Self {
        Output::Error {
            ename: ename.into(),
            evalue: evalue.into(),
            traceback: traceback.into_iter().map(Into::into).collect(),
        }
    }
}

use anyhow::{Context, Result}; // add `anyhow = "1"` to Cargo.toml
use std::fs;
use std::path::Path;

impl Notebook {
    /// Loads a notebook from a JSON string.
    ///
    /// Returns an error if the input is not valid JSON or doesn't match the notebook schema.
    pub fn from_str(s: &str) -> Result<Self> {
        let notebook: Self =
            serde_json::from_str(s).context("Failed to parse notebook from JSON string")?;
        Ok(notebook)
    }

    /// Loads a notebook from a `.ipynb` file.
    ///
    /// Returns an error if the file cannot be read or parsed.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read file: {}", path.as_ref().display()))?;
        Self::from_str(&content)
    }

    /// Serializes the notebook to a pretty JSON string.
    ///
    /// Returns an error if serialization fails.
    pub fn save_to_str(&self) -> Result<String> {
        let json =
            serde_json::to_string_pretty(self).context("Failed to serialize notebook to string")?;
        Ok(json)
    }

    /// Saves the notebook to a `.ipynb` file with pretty formatting.
    ///
    /// If the directory does not exist, it will be created.
    ///
    /// Returns an error if writing fails.
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path = path.as_ref();

        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!("Failed to create directories for: {}", parent.display())
            })?;
        }

        // Serialize and write the notebook
        let json = self.save_to_str()?;
        fs::write(path, json)
            .with_context(|| format!("Failed to write notebook to file: {}", path.display()))?;
        Ok(())
    }
}
