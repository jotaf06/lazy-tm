mod app;
mod task;

mod ui;
use ui::render::render;
use ui::state::UiState;

mod input;
use input::InputConfig;

mod storage;
use storage::{load, save};

use app::App;
use color_eyre::eyre::Result;
use ratatui::{
    DefaultTerminal,
    crossterm::event::{self, KeyCode},
    restore,
};

mod controler;
use controler::{
    Controller,
    events::{AppEvent, EventDispatcher},
};

use crate::ui::state::Mode;

const TASKS_PATH: &str = "tasks.json";

fn main() -> Result<()> {
    color_eyre::install()?;

    let tasks = load(TASKS_PATH)?;

    let mut app = App::new(tasks);
    let mut state = UiState::default();

    let terminal = ratatui::init();
    
    let result = run(terminal, &mut state, &mut app);

    restore();
    
    println!("Changes saved. See you soon! :)");
    result
}

fn run(mut terminal: DefaultTerminal, state: &mut UiState, app: &mut App) -> Result<()> {
    let mut input_config = InputConfig::default();
    let mut event_dispatcher = EventDispatcher::new();
    let mut should_exit = false;

    loop {
        state.ensure_selection(app.tasks.len());
        
        terminal.draw(|f| render(f, state, app))?;

        let has_running_timer = app.tasks.iter().any(|t| t.is_running());
        
        match state.mode {
            Mode::Normal => {
                should_exit = handle_normal_mode(&mut input_config, &mut event_dispatcher, state, app, has_running_timer)?;
            }
            Mode::Editing => {
                handle_editing_mode(state, app)?;
            }
        }

        if should_exit {
            app.pause_all_timers();
            break;
        }
    }

    save(TASKS_PATH, &app.tasks)?;

    Ok(())
}

fn handle_normal_mode(
    input_config: &mut InputConfig,
    event_dispatcher: &mut EventDispatcher,
    state: &mut UiState,
    app: &mut App,
    has_running_timer: bool,
) -> Result<bool> {
    let timeout = if has_running_timer { 100 } else { 500 };
    
    if event::poll(std::time::Duration::from_millis(timeout))? {
        if let Some(event) = input_config.read_event()? {
            if matches!(event, AppEvent::Quit) {
                return Ok(true);
            }
            
            let mut controller = Controller::new(app, state);
            
            event_dispatcher.dispatch(event, &mut controller);
        }
    }
    Ok(false)
}

fn handle_editing_mode(state: &mut UiState, app: &mut App) -> Result<()> {
    if let event::Event::Key(key) = event::read()? {
        match key.code {
            KeyCode::Esc => {
                state.cancel_editing();
            }

            KeyCode::Char(c) => {
                state.push_input_char(c);
            }

            KeyCode::Backspace => {
                state.pop_input_char();
            }

            KeyCode::Enter => {
                if state.is_editing {
                    if let Some(selected_index) = state.list_state.selected() {
                        if selected_index < app.tasks.len() {
                            let id = app.tasks[selected_index].id;
                            app.edit_task(
                                id,
                                state.title.clone(),
                                state.description.clone(),
                            );
                        }
                    }
                } else {
                    if !state.title.is_empty() {
                        app.add_task(state.title.clone(), state.description.clone());
                    }
                }

                state.finish_editing();
            }

            KeyCode::Tab => {
                state.toggle_input_focus();
            }

            _ => {}
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tasks_path() {
        assert_eq!(TASKS_PATH, "tasks.json");
    }
}

