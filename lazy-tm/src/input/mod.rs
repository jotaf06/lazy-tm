
pub mod factory;
pub mod keybindings;
pub mod keyboard;
pub mod reader;

#[cfg(feature = "examples")]
pub mod examples;

pub use factory::{InputReaderFactory, InputReaderType};
pub use keyboard::KeyboardInputReader;
pub use reader::InputReader;

use crate::controler::events::AppEvent;
use std::io::Result;

pub fn create_default_reader() -> Box<dyn InputReader> {
    InputReaderFactory::create_keyboard()
}

pub fn read_event() -> Result<Option<AppEvent>> {
    let mut reader = KeyboardInputReader::with_default_bindings();
    if !reader.has_event()? {
        return Ok(None);
    }
    reader.read()
}

pub struct InputConfig {
    reader: Box<dyn InputReader>,
}

impl InputConfig {
    pub fn new() -> Self {
        Self {
            reader: create_default_reader(),
        }
    }

    pub fn with_reader(reader: Box<dyn InputReader>) -> Self {
        Self { reader }
    }

    pub fn from_type(reader_type: InputReaderType) -> Self {
        Self {
            reader: reader_type.create(),
        }
    }

    pub fn read_event(&mut self) -> Result<Option<AppEvent>> {
        self.reader.read()
    }

    pub fn has_event(&self) -> Result<bool> {
        self.reader.has_event()
    }

    pub fn reader_name(&self) -> &str {
        self.reader.name()
    }
}

impl Default for InputConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_config_default() {
        let config = InputConfig::default();
        assert_eq!(config.reader_name(), "KeyboardInputReader");
    }

    #[test]
    fn test_input_config_from_type() {
        let config = InputConfig::from_type(InputReaderType::Vim);
        assert_eq!(config.reader_name(), "KeyboardInputReader");
    }

    #[test]
    fn test_create_default_reader() {
        let reader = create_default_reader();
        assert_eq!(reader.name(), "KeyboardInputReader");
    }
}
