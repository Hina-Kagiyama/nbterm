use crossterm::event::{
    Event::{self, Key},
    KeyCode, KeyEvent, KeyModifiers,
};
use std::collections::HashMap;

use super::input_mode::InputMode;
use crate::tui::editor_commands::EditorCommand;

pub struct EventTranslator {
    normal_mode_event_map: HashMap<(KeyCode, KeyModifiers), EditorCommand>,
    insert_mode_event_map: HashMap<(KeyCode, KeyModifiers), EditorCommand>,
    visual_mode_event_map: HashMap<(KeyCode, KeyModifiers), EditorCommand>,
    visual_line_mode_event_map: HashMap<(KeyCode, KeyModifiers), EditorCommand>,
    visual_block_mode_event_map: HashMap<(KeyCode, KeyModifiers), EditorCommand>,
    command_mode_event_map: HashMap<(KeyCode, KeyModifiers), EditorCommand>,
    ui_cursor_mode_event_map: HashMap<(KeyCode, KeyModifiers), EditorCommand>,
}

impl Default for EventTranslator {
    fn default() -> Self {
        use EditorCommand as C;
        use KeyCode as K;
        use KeyModifiers as M;

        let mut normal_mode_event_map: HashMap<(K, M), EditorCommand> = HashMap::new();
        let mut insert_mode_event_map: HashMap<(K, M), EditorCommand> = HashMap::new();
        let mut visual_mode_event_map: HashMap<(K, M), EditorCommand> = HashMap::new();
        let mut visual_line_mode_event_map: HashMap<(K, M), EditorCommand> = HashMap::new();
        let mut visual_block_mode_event_map: HashMap<(K, M), EditorCommand> = HashMap::new();
        let mut command_mode_event_map: HashMap<(K, M), EditorCommand> = HashMap::new();
        let mut ui_cursor_mode_event_map: HashMap<(K, M), EditorCommand> = HashMap::new();

        // Initialize the event maps with default commands for each mode
        normal_mode_event_map.insert((K::Char('q'), M::CONTROL), C::Quit);
        normal_mode_event_map.insert((K::Char('h'), M::CONTROL), C::ToLeftPane);
        normal_mode_event_map.insert((K::Char('l'), M::CONTROL), C::ToRightPane);
        normal_mode_event_map.insert((K::Char('j'), M::CONTROL), C::ToUpperPane);
        normal_mode_event_map.insert((K::Char('k'), M::CONTROL), C::ToLowerPane);
        normal_mode_event_map.insert((K::Char('i'), M::CONTROL), C::SwitchToInsertMode);
        normal_mode_event_map.insert((K::Char('v'), M::CONTROL), C::SwitchToVisualMode);
        normal_mode_event_map.insert((K::Char('V'), M::NONE), C::SwitchToVisualLineMode);
        normal_mode_event_map.insert((K::Char('v'), M::CONTROL), C::SwitchToVisualBlockMode);

        insert_mode_event_map.insert((K::Char('i'), M::CONTROL), C::Input("\t".to_string()));
        insert_mode_event_map.insert((K::Char('c'), M::CONTROL), C::SwitchToNormalMode);
        insert_mode_event_map.insert((K::Char('w'), M::CONTROL), C::DeletePreviousWord);
        insert_mode_event_map.insert((K::Esc, M::NONE), C::SwitchToNormalMode);

        // return
        Self {
            normal_mode_event_map,
            insert_mode_event_map,
            visual_mode_event_map,
            visual_line_mode_event_map,
            visual_block_mode_event_map,
            command_mode_event_map,
            ui_cursor_mode_event_map,
        }
    }
}

impl EventTranslator {
    pub fn translate_event(&self, event: Event, input_mode: InputMode) -> Option<EditorCommand> {
        match event {
            Key(key_event) => self.translate_key_event(key_event, input_mode),
            _ => None,
        }
    }

    fn translate_key_event(
        &self,
        key_event: KeyEvent,
        input_mode: InputMode,
    ) -> Option<EditorCommand> {
        let key = (key_event.code, key_event.modifiers);
        match input_mode {
            InputMode::Normal => self.normal_mode_event_map.get(&key).cloned(),
            InputMode::Insert => self.insert_mode_event_map.get(&key).cloned(),
            InputMode::Visual => self.visual_mode_event_map.get(&key).cloned(),
            InputMode::VisualLine => self.visual_line_mode_event_map.get(&key).cloned(),
            InputMode::VisualBlock => self.visual_block_mode_event_map.get(&key).cloned(),
            InputMode::Command => self.command_mode_event_map.get(&key).cloned(),
            InputMode::UICursor => self.ui_cursor_mode_event_map.get(&key).cloned(),
        }
    }
}
