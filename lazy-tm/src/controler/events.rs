use crate::controler::Controller;
use crate::ui::state::{InputType, Mode};

#[derive(Debug, Clone, PartialEq)]
pub enum AppEvent {
    Quit,
    SelPrevious,
    SelNext,
    ToggleTask,
    Delete,
    Add,
    EditTask,
    ClearAll,
    StartTimer,
}

pub trait EventHandler {
    fn handle(&mut self, evt: AppEvent, ctx: &mut Controller);
}

pub struct NormalModeHandler;

impl EventHandler for NormalModeHandler {
    fn handle(&mut self, evt: AppEvent, ctx: &mut Controller) {
        match evt {
            AppEvent::Quit => {
            }

            AppEvent::SelNext => {
                ctx.select_next();
            }

            AppEvent::SelPrevious => {
                ctx.select_previous();
            }

            AppEvent::ToggleTask => {
                if let Some(task) = ctx.selected_task() {
                    let id = task.id;
                    let is_checked = task.is_checked;
                    ctx.app.toggle_task(id);

                    if is_checked {
                        ctx.app.toggle_timer(id);
                    }
                }
            }

            AppEvent::Delete => {
                if let Some(selected_index) = ctx.selected_index() {
                    if selected_index < ctx.app.tasks.len() {
                        ctx.app.tasks.remove(selected_index);
                    }
                    ctx.ensure_valid_selection();
                }
            }

            AppEvent::ClearAll => {
                ctx.app.clear_all_tasks();
                ctx.state.list_state.select(None);
            }

            AppEvent::Add => {
                ctx.state.mode = Mode::Editing;
                ctx.state.title.clear();
                ctx.state.description.clear();
                ctx.state.input_type = InputType::Title;
                ctx.state.is_editing = false;
            }

            AppEvent::EditTask => {
                if let Some(selected_index) = ctx.selected_index() {
                    if let Some(task) = ctx.app.tasks.get(selected_index) {
                        ctx.state.mode = Mode::Editing;
                        ctx.state.input_type = InputType::Title;
                        ctx.state.is_editing = true;
                        ctx.state.title = task.title.clone();
                        ctx.state.description = task.description.clone();
                    }
                }
            }

            AppEvent::StartTimer => {
                if let Some(selected_index) = ctx.selected_index() {
                    if selected_index >= ctx.app.tasks.len() {
                        return;
                    }
                    
                    let task = &ctx.app.tasks[selected_index];
                    if !task.is_checked && !task.is_running() {
                        let id = task.id;
                        ctx.app.toggle_timer(id);
                    }
                }
            }
        }
    }
}

pub struct EditingModeHandler;

impl EventHandler for EditingModeHandler {
    fn handle(&mut self, evt: AppEvent, ctx: &mut Controller) {
        match evt {
            AppEvent::Quit => {
                ctx.state.mode = Mode::Normal;
                ctx.state.title.clear();
                ctx.state.description.clear();
                ctx.state.is_editing = false;
            }
            _ => {}
        }
    }
}

pub struct EventDispatcher {
    normal_handler: NormalModeHandler,
    editing_handler: EditingModeHandler,
}

impl EventDispatcher {
    pub fn new() -> Self {
        Self {
            normal_handler: NormalModeHandler,
            editing_handler: EditingModeHandler,
        }
    }

    pub fn dispatch(&mut self, evt: AppEvent, ctx: &mut Controller) {
        match ctx.state.mode {
            Mode::Normal => self.normal_handler.handle(evt, ctx),
            Mode::Editing => self.editing_handler.handle(evt, ctx),
        }
    }
}

impl Default for EventDispatcher {
    fn default() -> Self {
        Self::new()
    }
}
