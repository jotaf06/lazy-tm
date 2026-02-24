mod app;
mod task;

mod ui;
use ui::render::render;
use ui::state::UiState;

mod input;
use input::{AppEvent, read_event};

mod storage;
use storage::{load, save};

use app::App;
use color_eyre::eyre::Result;
use ratatui::{
    DefaultTerminal,
    crossterm::event::{self, KeyCode},
    restore,
};

use crate::ui::state::{InputType, Mode};

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
    loop {
        state.ensure_selection(app.tasks.len());
        terminal.draw(|f| render(f, state, app))?;

        let mode = &state.mode;
        match mode {
            Mode::Normal => {
                let event = read_event()?;
                match event {
                    AppEvent::Quit => {
                        app.pause_all_timers();
                        break;
                    }
                    AppEvent::SelNext => {
                        let i = match state.list_state.selected() {
                            Some(i) => {
                                if i >= app.tasks.len().saturating_sub(1) {
                                    0
                                } else {
                                    i + 1
                                }
                            }
                            None => 0,
                        };

                        state.list_state.select(Some(i));
                    }

                    AppEvent::SelPrevious => {
                        let i = match state.list_state.selected() {
                            Some(i) => {
                                if i == 0 {
                                    app.tasks.len().saturating_sub(1)
                                } else {
                                    i - 1
                                }
                            }
                            None => 0,
                        };

                        state.list_state.select(Some(i));
                    }

                    AppEvent::ToggleTask => {
                        if let Some(selected_index) = state.list_state.selected() {
                            if let Some(task) = app.tasks.get(selected_index) {
                                let id = task.id;
                                let is_checked = task.is_checked;
                                app.toggle_task(id);

                                if is_checked {
                                    app.toggle_timer(id);
                                }
                            }
                        }
                    }

                    AppEvent::Delete => {
                        if let Some(selected_index) = state.list_state.selected() {
                            if selected_index < app.tasks.len() {
                                app.tasks.remove(selected_index);
                            }

                            let len = app.tasks.len();

                            if len == 0 {
                                state.list_state.select(None);
                            } else if selected_index >= len {
                                state.list_state.select(Some(len - 1));
                            } else {
                                state.list_state.select(Some(selected_index));
                            }
                        }
                    }

                    AppEvent::ClearAll => {
                        app.clear_all_tasks();
                        state.list_state.select(None);
                    }

                    AppEvent::Add => {
                        state.mode = Mode::Editing;
                        state.title.clear();
                        state.description.clear();
                        state.input_type = InputType::Title;
                    }

                    AppEvent::EditTask => {
                        state.mode = Mode::Editing;
                        state.input_type = InputType::Title;
                        state.is_editing = true;
                        state.title = app.tasks[state.list_state.selected().unwrap()]
                            .title
                            .clone();
                        state.description = app.tasks[state.list_state.selected().unwrap()]
                            .description
                            .clone();
                    }

                    AppEvent::StartTimer => {
                        if let Some(selected_index) = state.list_state.selected() {
                            if selected_index < app.tasks.len() {
                                if !app.tasks[selected_index].is_checked {
                                    if !app.tasks[selected_index].is_running() {
                                        let id = app.tasks[selected_index].id;
                                        app.toggle_timer(id);
                                    }
                                }
                            }
                        }
                    }

                    _ => {}
                }
            }

            Mode::Editing => {
                if let event::Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Esc => {
                            state.mode = Mode::Normal;
                            state.title.clear();
                            state.description.clear();
                        }

                        KeyCode::Char(c) => {
                            match state.input_type {
                                InputType::Title => state.title.push(c),
                                InputType::Description => state.description.push(c),
                            };
                        }

                        KeyCode::Backspace => {
                            match state.input_type {
                                InputType::Title => state.title.pop(),
                                InputType::Description => state.description.pop(),
                            };
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

                            state.title.clear();
                            state.description.clear();

                            state.mode = Mode::Normal;
                            state.input_type = InputType::Title;
                        }

                        KeyCode::Tab => {
                            state.input_type = match state.input_type {
                                InputType::Title => InputType::Description,
                                InputType::Description => InputType::Title,
                            }
                        }

                        _ => {}
                    }
                }
            }
        }
    }

    save(TASKS_PATH, &app.tasks)?;

    Ok(())
}
