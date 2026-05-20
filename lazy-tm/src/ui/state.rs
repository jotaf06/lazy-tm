use ratatui::widgets::ListState;

#[derive(Debug, PartialEq)]
pub enum LayoutMode {
    Single,
    TwoPanel,
}

#[derive(Debug, PartialEq)]
pub enum ActivePanel {
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
pub enum Screen {
    Home,
    TaskList,
}

#[derive(Debug, PartialEq)]
pub enum HomeMode {
    Normal,
    CreatingList,
    ConfirmingDelete,
}

#[derive(Debug, PartialEq)]
pub enum Mode {
    Normal,
    Editing,
    ConfirmingDelete,
    ConfirmingClearAll,
}

#[derive(Debug, PartialEq)]
pub enum InputType {
    Title,
    Description,
}

pub struct UiState {
    pub layout_mode: LayoutMode,
    pub active_panel: ActivePanel,
    pub screen: Screen,
    pub home_mode: HomeMode,
    pub home_list_state: ListState,
    pub new_list_name: String,
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
            layout_mode: LayoutMode::Single,
            active_panel: ActivePanel::Left,
            screen: Screen::Home,
            home_mode: HomeMode::Normal,
            home_list_state: ListState::default(),
            new_list_name: String::new(),
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
        }
    }

    pub fn ensure_home_selection(&mut self, len: usize) {
        if len == 0 {
            self.home_list_state.select(None);
        } else if self.home_list_state.selected().is_none() {
            self.home_list_state.select(Some(0));
        }
    }
}
