// pub mod commands;
pub mod events;
// pub mod triggers;

use crate::{App, UiState};
//use commands::CommandManager;

pub struct Controller<'a> {
    pub app: &'a mut App,
    pub state: &'a mut UiState,
    //pub command_manager: CommandManager,
}

impl<'a> Controller<'a> {
    pub fn new(app: &'a mut App, state: &'a mut UiState) -> Self {
        Self {
            app,
            state,
            //pub command_manager: CommandManager,
        }
    }

    pub fn select_next(&mut self) {
        if let Some(current_idx) = self.state.list_state.selected() {
            if let Some(task) = self.app.tasks.get_mut(current_idx) {
                task.stop_timer();
            }
        }
        
        let i = match self.state.list_state.selected() {
            Some(i) => {
                if i >= self.app.tasks.len().saturating_sub(1) {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.list_state.select(Some(i));
    }

    pub fn select_previous(&mut self) {
        if let Some(current_idx) = self.state.list_state.selected() {
            if let Some(task) = self.app.tasks.get_mut(current_idx) {
                task.stop_timer();
            }
        }
        
        let i = match self.state.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.app.tasks.len().saturating_sub(1)
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.list_state.select(Some(i));
    }

    pub fn selected_index(&self) -> Option<usize> {
        self.state.list_state.selected()
    }

    pub fn selected_task(&self) -> Option<&crate::task::Task> {
        self.selected_index()
            .and_then(|i| self.app.tasks.get(i))
    }

    pub fn selected_task_mut(&mut self) -> Option<&mut crate::task::Task> {
        self.selected_index()
            .and_then(|i| self.app.tasks.get_mut(i))
    }

    pub fn ensure_valid_selection(&mut self) {
        let len = self.app.tasks.len();
        if len == 0 {
            self.state.list_state.select(None);
        } else if let Some(selected) = self.state.list_state.selected() {
            if selected >= len {
                self.state.list_state.select(Some(len - 1));
            }
        }
    }

    // pub fn can_undo(&self) -> bool {
    //     self.command_manager.can_undo()
    // }

    // pub fn can_redo(&self) -> bool {
    //     self.command_manager.can_redo()
    // }
}
