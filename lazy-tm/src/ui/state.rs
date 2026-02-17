use ratatui::widgets::ListState;

pub struct UiState {
    pub list_state: ListState,
}

impl UiState {
    pub fn default() -> Self {
        Self {
            list_state: ListState::default(),
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
