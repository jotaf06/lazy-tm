use crate::input::keyboard::{ExtendedKeyboardReader, KeyboardInputReader};
use crate::input::keybindings::KeyBindings;
use crate::input::reader::{CompositeInputReader, InputReader, TimedInputReader};

pub struct InputReaderFactory;

impl InputReaderFactory {
    pub fn create_keyboard() -> Box<dyn InputReader> {
        Box::new(KeyboardInputReader::with_default_bindings())
    }

    pub fn create_keyboard_with_bindings(keybindings: KeyBindings) -> Box<dyn InputReader> {
        Box::new(KeyboardInputReader::new(keybindings))
    }

    pub fn create_vim_keyboard() -> Box<dyn InputReader> {
        Box::new(KeyboardInputReader::with_vim_bindings())
    }

    pub fn create_emacs_keyboard() -> Box<dyn InputReader> {
        Box::new(KeyboardInputReader::with_emacs_bindings())
    }

    pub fn create_extended_keyboard(keybindings: KeyBindings) -> Box<dyn InputReader> {
        Box::new(ExtendedKeyboardReader::new(keybindings))
    }

    pub fn create_composite() -> CompositeInputReader {
        CompositeInputReader::new()
    }

    pub fn create_from_config(config: &str) -> Box<dyn InputReader> {
        match config.to_lowercase().as_str() {
            "vim" => Self::create_vim_keyboard(),
            "emacs" => Self::create_emacs_keyboard(),
            "extended" => Self::create_extended_keyboard(KeyBindings::default()),
            _ => Self::create_keyboard(),
        }
    }

    pub fn create_with_timeout(timeout_ms: u64) -> Box<KeyboardInputReader> {
        let mut reader = KeyboardInputReader::with_default_bindings();
        reader.set_timeout(timeout_ms);
        Box::new(reader)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputReaderType {
    Default,
    Vim,
    Emacs,
    Extended,
}

impl InputReaderType {
    pub fn create(&self) -> Box<dyn InputReader> {
        match self {
            Self::Default => InputReaderFactory::create_keyboard(),
            Self::Vim => InputReaderFactory::create_vim_keyboard(),
            Self::Emacs => InputReaderFactory::create_emacs_keyboard(),
            Self::Extended => {
                InputReaderFactory::create_extended_keyboard(KeyBindings::default())
            }
        }
    }

    pub fn all() -> Vec<Self> {
        vec![Self::Default, Self::Vim, Self::Emacs, Self::Extended]
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Default => "default",
            Self::Vim => "vim",
            Self::Emacs => "emacs",
            Self::Extended => "extended",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "default" => Some(Self::Default),
            "vim" => Some(Self::Vim),
            "emacs" => Some(Self::Emacs),
            "extended" => Some(Self::Extended),
            _ => None,
        }
    }
}

pub struct InputConfigBuilder {
    reader_type: InputReaderType,
    custom_bindings: Option<KeyBindings>,
    timeout_ms: Option<u64>,
}

impl InputConfigBuilder {
    pub fn new() -> Self {
        Self {
            reader_type: InputReaderType::Default,
            custom_bindings: None,
            timeout_ms: None,
        }
    }

    pub fn reader_type(mut self, reader_type: InputReaderType) -> Self {
        self.reader_type = reader_type;
        self
    }

    pub fn with_bindings(mut self, bindings: KeyBindings) -> Self {
        self.custom_bindings = Some(bindings);
        self
    }

    pub fn with_timeout(mut self, timeout_ms: u64) -> Self {
        self.timeout_ms = Some(timeout_ms);
        self
    }

    pub fn build(self) -> Box<dyn InputReader> {
        let reader = if let Some(bindings) = self.custom_bindings {
            InputReaderFactory::create_keyboard_with_bindings(bindings)
        } else {
            self.reader_type.create()
        };

        if let Some(_timeout) = self.timeout_ms {
        }

        reader
    }
}

impl Default for InputConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factory_creates_readers() {
        let keyboard = InputReaderFactory::create_keyboard();
        assert_eq!(keyboard.name(), "KeyboardInputReader");

        let vim = InputReaderFactory::create_vim_keyboard();
        assert_eq!(vim.name(), "KeyboardInputReader");
    }

    #[test]
    fn test_reader_type_enum() {
        assert_eq!(InputReaderType::from_str("vim"), Some(InputReaderType::Vim));
        assert_eq!(InputReaderType::from_str("default"), Some(InputReaderType::Default));
        assert_eq!(InputReaderType::from_str("invalid"), None);
    }

    #[test]
    fn test_config_builder() {
        let reader = InputConfigBuilder::new()
            .reader_type(InputReaderType::Vim)
            .build();

        assert_eq!(reader.name(), "KeyboardInputReader");
    }

    #[test]
    fn test_all_reader_types() {
        let types = InputReaderType::all();
        assert_eq!(types.len(), 4);
    }
}
