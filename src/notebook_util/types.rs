use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represents the top-level structure of a Jupyter notebook file.
#[derive(Debug, Serialize, Deserialize)]
pub struct Notebook {
    pub cells: Vec<Cell>,
    pub metadata: NotebookMetadata,
    pub nbformat: u8,
    pub nbformat_minor: u8,
}

/// Top-level metadata field (can contain various kernel or language info).
#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct NotebookMetadata {
    pub kernelspec: Option<Kernelspec>,
    pub language_info: Option<LanguageInfo>,
    #[serde(flatten)]
    pub other: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Kernelspec {
    pub name: String,
    pub display_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LanguageInfo {
    pub name: String,
    pub version: Option<String>,
    pub mimetype: Option<String>,
    pub file_extension: Option<String>,
    #[serde(flatten)]
    pub other: Value,
}

/// Enum for all supported Jupyter cell types.
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "cell_type")]
pub enum Cell {
    #[serde(rename = "code")]
    Code(CodeCell),

    #[serde(rename = "markdown")]
    Markdown(MarkdownCell),

    #[serde(rename = "raw")]
    Raw(RawCell),
}

/// A code cell with executable content and outputs.
#[derive(Debug, Serialize, Deserialize)]
pub struct CodeCell {
    pub source: Vec<String>,
    pub metadata: Value,
    pub execution_count: Option<u32>,
    pub outputs: Vec<Output>,
}

/// A markdown cell with formatted text.
#[derive(Debug, Serialize, Deserialize)]
pub struct MarkdownCell {
    pub source: Vec<String>,
    pub metadata: Value,
}

/// A raw cell with unformatted text.
#[derive(Debug, Serialize, Deserialize)]
pub struct RawCell {
    pub source: Vec<String>,
    pub metadata: Value,
}

/// Output objects for code cells.
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "output_type")]
pub enum Output {
    #[serde(rename = "stream")]
    Stream {
        name: String, // "stdout" or "stderr"
        text: Vec<String>,
    },
    #[serde(rename = "execute_result")]
    ExecuteResult {
        execution_count: u32,
        data: Value, // Typically contains "text/plain", "text/html", etc.
        metadata: Value,
    },
    #[serde(rename = "display_data")]
    DisplayData { data: Value, metadata: Value },
    #[serde(rename = "error")]
    Error {
        ename: String,
        evalue: String,
        traceback: Vec<String>,
    },
}

impl Default for Notebook {
    fn default() -> Self {
        Self {
            cells: vec![],
            metadata: NotebookMetadata::default(),
            nbformat: 4,
            nbformat_minor: 5,
        }
    }
}

impl Default for NotebookMetadata {
    fn default() -> Self {
        Self {
            kernelspec: None,
            language_info: None,
            other: Value::Null,
        }
    }
}
