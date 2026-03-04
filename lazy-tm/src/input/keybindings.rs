use crate::controler::events::AppEvent;
use ratatui::crossterm::event::KeyCode;
use std::collections::HashMap;

#[derive(Clone)]
pub struct KeyBindings {
    bindings: HashMap<KeyCode, AppEvent>,
}

impl KeyBindings {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }

    pub fn bind(mut self, key: KeyCode, event: AppEvent) -> Self {
        self.bindings.insert(key, event);
        self
    }


    pub fn bind_many(mut self, bindings: Vec<(KeyCode, AppEvent)>) -> Self {
        for (key, event) in bindings {
            self.bindings.insert(key, event);
        }
        self
    }

    pub fn unbind(mut self, key: KeyCode) -> Self {
        self.bindings.remove(&key);
        self
    }

    pub fn translate(&self, key: KeyCode) -> Option<AppEvent> {
        self.bindings.get(&key).cloned()
    }

    pub fn is_bound(&self, key: KeyCode) -> bool {
        self.bindings.contains_key(&key)
    }

    pub fn all_bindings(&self) -> &HashMap<KeyCode, AppEvent> {
        &self.bindings
    }

    pub fn default_config() -> Self {
        Self::new()
            .bind(KeyCode::Char('j'), AppEvent::SelPrevious)
            .bind(KeyCode::Up, AppEvent::SelPrevious)
            .bind(KeyCode::Char('k'), AppEvent::SelNext)
            .bind(KeyCode::Down, AppEvent::SelNext)
            
            .bind(KeyCode::Char(' '), AppEvent::ToggleTask)
            .bind(KeyCode::Char('a'), AppEvent::Add)
            .bind(KeyCode::Char('e'), AppEvent::EditTask)
            .bind(KeyCode::Char('D'), AppEvent::Delete)
            .bind(KeyCode::Char('C'), AppEvent::ClearAll)
            .bind(KeyCode::Char('S'), AppEvent::StartTimer)
            
            .bind(KeyCode::Esc, AppEvent::Quit)
            .bind(KeyCode::Char('q'), AppEvent::Quit)
            .bind(KeyCode::Char('Q'), AppEvent::Quit)
    }

    pub fn vim_config() -> Self {
        Self::new()
            .bind(KeyCode::Char('k'), AppEvent::SelPrevious)
            .bind(KeyCode::Up, AppEvent::SelPrevious)
            .bind(KeyCode::Char('j'), AppEvent::SelNext)
            .bind(KeyCode::Down, AppEvent::SelNext)
            
            .bind(KeyCode::Char(' '), AppEvent::ToggleTask)
            .bind(KeyCode::Char('a'), AppEvent::Add)
            .bind(KeyCode::Char('i'), AppEvent::EditTask)
            .bind(KeyCode::Char('d'), AppEvent::Delete)
            .bind(KeyCode::Char('s'), AppEvent::StartTimer)
            
            .bind(KeyCode::Esc, AppEvent::Quit)
            .bind(KeyCode::Char('q'), AppEvent::Quit)
    }

    pub fn emacs_config() -> Self {
        Self::new()
            .bind(KeyCode::Char('p'), AppEvent::SelPrevious)
            .bind(KeyCode::Up, AppEvent::SelPrevious)
            .bind(KeyCode::Char('n'), AppEvent::SelNext)
            .bind(KeyCode::Down, AppEvent::SelNext)
            
            .bind(KeyCode::Char(' '), AppEvent::ToggleTask)
            .bind(KeyCode::Char('a'), AppEvent::Add)
            .bind(KeyCode::Char('e'), AppEvent::EditTask)
            .bind(KeyCode::Char('d'), AppEvent::Delete)
            
            .bind(KeyCode::Char('x'), AppEvent::Quit)
    }
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self::default_config()
    }
}

pub struct KeyBindingsBuilder {
    bindings: KeyBindings,
}

impl KeyBindingsBuilder {
    pub fn new() -> Self {
        Self {
            bindings: KeyBindings::new(),
        }
    }

    pub fn from_preset(preset: KeyBindings) -> Self {
        Self { bindings: preset }
    }

    pub fn with_binding(mut self, key: KeyCode, event: AppEvent) -> Self {
        self.bindings = self.bindings.bind(key, event);
        self
    }

    pub fn without_binding(mut self, key: KeyCode) -> Self {
        self.bindings = self.bindings.unbind(key);
        self
    }

    pub fn build(self) -> KeyBindings {
        self.bindings
    }
}

impl Default for KeyBindingsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keybindings_builder() {
        let bindings = KeyBindings::new()
            .bind(KeyCode::Char('q'), AppEvent::Quit)
            .bind(KeyCode::Char('j'), AppEvent::SelNext);

        assert_eq!(bindings.translate(KeyCode::Char('q')), Some(AppEvent::Quit));
        assert_eq!(bindings.translate(KeyCode::Char('j')), Some(AppEvent::SelNext));
        assert_eq!(bindings.translate(KeyCode::Char('x')), None);
    }

    #[test]
    fn test_fluent_builder() {
        let bindings = KeyBindingsBuilder::new()
            .with_binding(KeyCode::Char('q'), AppEvent::Quit)
            .with_binding(KeyCode::Char('a'), AppEvent::Add)
            .build();

        assert!(bindings.is_bound(KeyCode::Char('q')));
        assert!(bindings.is_bound(KeyCode::Char('a')));
    }
}
