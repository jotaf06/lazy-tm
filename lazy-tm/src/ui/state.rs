use ratatui::widgets::ListState;

#[derive(Debug, PartialEq)]
pub enum Mode {
    Normal,
    Editing,
}

#[derive(Debug, PartialEq)]
pub enum InputType {
    Title,
    Description,
}

pub struct UiState {
    pub list_state: ListState,
    pub mode: Mode,
    pub title: String,
    pub description: String,
    pub input_type: InputType,
    pub is_editing: bool,
}

impl UiState {
    pub fn default() -> Self {
        Self {
            list_state: ListState::default(),
            mode: Mode::Normal,
            title: String::new(),
            description: String::new(),
            input_type: InputType::Title,
            is_editing: false,
        }
    }

    pub fn ensure_selection(&mut self, len: usize) {
        if len == 0 {
            self.list_state.select(None);
        } else if self.list_state.selected().is_none() {
            self.list_state.select(Some(0));
        };
    }
}
