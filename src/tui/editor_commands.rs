use regex::Regex;
use std::path::PathBuf;

#[derive(Debug)]
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
#[derive(Debug)]
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
