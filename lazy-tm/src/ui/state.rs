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

impl Default for UiState {
    fn default() -> Self {
        Self {
            list_state: ListState::default(),
            mode: Mode::Normal,
            title: String::new(),
            description: String::new(),
            input_type: InputType::Title,
            is_editing: false,
        }
    }
}

impl UiState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ensure_selection(&mut self, len: usize) {
        if len == 0 {
            self.list_state.select(None);
        } else if self.list_state.selected().is_none() {
            self.list_state.select(Some(0));
        };
    }

    pub fn enter_create_mode(&mut self) {
        self.mode = Mode::Editing;
        self.input_type = InputType::Title;
        self.title.clear();
        self.description.clear();
        self.is_editing = false;
    }

    pub fn enter_edit_mode(&mut self, title: &str, description: &str) {
        self.mode = Mode::Editing;
        self.input_type = InputType::Title;
        self.title = title.to_string();
        self.description = description.to_string();
        self.is_editing = true;
    }

    pub fn cancel_editing(&mut self) {
        self.mode = Mode::Normal;
        self.input_type = InputType::Title;
        self.title.clear();
        self.description.clear();
        self.is_editing = false;
    }

    pub fn toggle_input_focus(&mut self) {
        self.input_type = match self.input_type {
            InputType::Title => InputType::Description,
            InputType::Description => InputType::Title,
        };
    }

    pub fn push_input_char(&mut self, c: char) {
        match self.input_type {
            InputType::Title => self.title.push(c),
            InputType::Description => self.description.push(c),
        }
    }

    pub fn pop_input_char(&mut self) {
        match self.input_type {
            InputType::Title => {
                self.title.pop();
            }
            InputType::Description => {
                self.description.pop();
            }
        }
    }

    pub fn finish_editing(&mut self) {
        self.mode = Mode::Normal;
        self.input_type = InputType::Title;
        self.title.clear();
        self.description.clear();
        self.is_editing = false;
    }
}
