use crate::controler::events::AppEvent;
use crate::input::keybindings::KeyBindings;
use crate::input::reader::{InputReader, TimedInputReader};
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::io::Result;
use std::time::Duration;

pub struct KeyboardInputReader {
    keybindings: KeyBindings,
    timeout_ms: u64,
}

impl KeyboardInputReader {
    pub fn new(keybindings: KeyBindings) -> Self {
        Self {
            keybindings,
            timeout_ms: 500,
        }
    }

    pub fn with_default_bindings() -> Self {
        Self::new(KeyBindings::default())
    }

    pub fn with_vim_bindings() -> Self {
        Self::new(KeyBindings::vim_config())
    }

    pub fn with_emacs_bindings() -> Self {
        Self::new(KeyBindings::emacs_config())
    }

    pub fn set_keybindings(&mut self, keybindings: KeyBindings) {
        self.keybindings = keybindings;
    }

    pub fn keybindings(&self) -> &KeyBindings {
        &self.keybindings
    }

    fn process_key_event(&self, key_event: KeyEvent) -> Option<AppEvent> {
        // Suporte para modificadores no futuro (Ctrl, Alt, Shift)
        if key_event.modifiers.contains(KeyModifiers::CONTROL) {
            return None;
        }

        self.keybindings.translate(key_event.code)
    }
}

impl InputReader for KeyboardInputReader {
    fn read(&mut self) -> Result<Option<AppEvent>> {
        if let Event::Key(key_event) = event::read()? {
            return Ok(self.process_key_event(key_event));
        }
        Ok(None)
    }

    fn has_event(&self) -> Result<bool> {
        event::poll(Duration::from_millis(self.timeout_ms))
    }

    fn name(&self) -> &str {
        "KeyboardInputReader"
    }
}

impl TimedInputReader for KeyboardInputReader {
    fn set_timeout(&mut self, timeout_ms: u64) {
        self.timeout_ms = timeout_ms;
    }

    fn timeout(&self) -> u64 {
        self.timeout_ms
    }
}

pub struct ExtendedKeyboardReader {
    base_reader: KeyboardInputReader,
    last_key: Option<KeyCode>,
    key_count: usize,
}

impl ExtendedKeyboardReader {
    pub fn new(keybindings: KeyBindings) -> Self {
        Self {
            base_reader: KeyboardInputReader::new(keybindings),
            last_key: None,
            key_count: 0,
        }
    }

    pub fn last_key(&self) -> Option<KeyCode> {
        self.last_key
    }

    pub fn key_count(&self) -> usize {
        self.key_count
    }

    pub fn reset_stats(&mut self) {
        self.key_count = 0;
        self.last_key = None;
    }
}

impl InputReader for ExtendedKeyboardReader {
    fn read(&mut self) -> Result<Option<AppEvent>> {
        if let Event::Key(key_event) = event::read()? {
            self.last_key = Some(key_event.code);
            self.key_count += 1;
            return Ok(self.base_reader.process_key_event(key_event));
        }
        Ok(None)
    }

    fn has_event(&self) -> Result<bool> {
        self.base_reader.has_event()
    }

    fn name(&self) -> &str {
        "ExtendedKeyboardReader"
    }
}

impl TimedInputReader for ExtendedKeyboardReader {
    fn set_timeout(&mut self, timeout_ms: u64) {
        self.base_reader.set_timeout(timeout_ms);
    }

    fn timeout(&self) -> u64 {
        self.base_reader.timeout()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyboard_reader_creation() {
        let reader = KeyboardInputReader::with_default_bindings();
        assert_eq!(reader.timeout(), 500);
        assert_eq!(reader.name(), "KeyboardInputReader");
    }

    #[test]
    fn test_extended_reader_stats() {
        let reader = ExtendedKeyboardReader::new(KeyBindings::default());
        assert_eq!(reader.key_count(), 0);
        assert_eq!(reader.last_key(), None);
    }

    #[test]
    fn test_timeout_modification() {
        let mut reader = KeyboardInputReader::with_default_bindings();
        reader.set_timeout(1000);
        assert_eq!(reader.timeout(), 1000);
    }
}
