use regex::Regex;
use std::{path::PathBuf, slice::Iter};

use super::NotebookApp;

#[derive(Debug, Clone)]
pub enum EditorCommand {
    // Editor actions
    Quit,
    ToggleFilePicker,
    ToggleOutline,
    ToggleSettings,
    ToggleSymbols,
    ToggleSearch,
    ToggleLeftPane,
    ToggleRightPane,
    ToggleTabline,
    ToggleStatusBar,
    ToggleDiff,

    // Setting toggle actions
    ToggleLineNumbers,
    ToggleWordWrap,
    ToggleAutoIndent,
    ToggleSyntaxHighlighting,
    ToggleAutoComplete,
    ToggleAutoCloseBrackets,
    ToggleAutoCloseQuotes,

    // File actions
    OpenFile(PathBuf),
    SaveFile,
    SaveFileAs(PathBuf),
    CloseFile,
    NewFile,

    // Primitive actions
    Undo,
    Redo,
    Navigate(NavigationCommand),
    Repeat(Box<EditorCommand>, usize),
    Input(String), // input text directly, then move cursor to the end of the input.

    // Pane Navigation actions
    ToLeftPane,
    ToRightPane,
    ToUpperPane,
    ToLowerPane,

    // Tab Navigation actions
    ToNextTab,
    ToPreviousTab,
    ToTab(usize),

    // Search actions
    Search(Regex),   // set search term
    Replace(String), // replace selected text with this string

    // Text manipulation actions
    DeleteText,
    DeleteLine,
    DeleteWord,
    DeletePreviousWord,
    DeleteToEndOfLine,
    DeleteToStartOfLine,
    DeleteToEndOfFile,
    DeleteToStartOfFile,
    Copy,
    Cut,
    Paste,
    Concatenate,

    // Selection actions (for visual modes)
    Skip,     // move cursor without selecting, resulting in multiple selections
    Deselect, // remove selections

    // Mode switching
    SwitchToInsertMode,
    SwitchToNormalMode,
    SwitchToVisualMode,
    SwitchToVisualLineMode,
    SwitchToVisualBlockMode,
    SwitchToReplaceMode,
    SwitchToCommandMode,
}

// Some editor commands take a navigation command as an argument
#[derive(Debug, Clone)]
pub enum NavigationCommand {
    Up,
    Down,
    Left,
    Right,
    ToLine(usize),
    ToColumn(usize),
    ToNextWordStart,
    ToNextWordEnd,
    ToPreviousWordStart,
    ToLineStart,
    ToLineEnd,
    PageDown,
    PageUp,
    ToNextSearchResultStart,
    ToNextSearchResultEnd,
    ToPreviousSearchResultStart,
    ToPreviousSearchResultEnd,
    ToNextOutlineItemStart,
    ToNextOutlineItemEnd,
    ToPreviousOutlineItemStart,
    ToPreviousOutlineItemEnd,
    ToNextBookmark,
}

impl NotebookApp {
    pub fn execute_command(&mut self, command: EditorCommand) {
        match command {
            EditorCommand::Quit => {
                self.leaving = true;
            }
            // Handle other commands...
            _ => {}
        }
    }
}
