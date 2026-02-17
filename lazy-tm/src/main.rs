mod app;
mod task;

mod ui;
use ui::render::render;
use ui::state::UiState;

mod input;
use input::{AppEvent, read_event};

use app::App;
use color_eyre::eyre::Result;
use ratatui::{
    DefaultTerminal,
    restore,
};

fn main() -> Result<()> {
    let mut app = App::default();
    let mut state = UiState::default();

    color_eyre::install()?;

    app.add_task(String::from("Buy Milk"), String::from("Buy Milk"));
    app.add_task(String::from("Buy Milk"), String::from("Buy Milk"));
    app.add_task(String::from("Buy Milk"), String::from("Buy Milk"));

    let terminal = ratatui::init();
    let result = run(terminal, &mut state, &mut app);

    restore();
    result
}

fn run(mut terminal: DefaultTerminal, state: &mut UiState, app: &mut App) -> Result<()> {
    loop {
        state.ensure_selection(app.tasks.len());
        terminal.draw(|f| render(f, state, app))?;

        let event = read_event()?;
        match event {
            AppEvent::Quit => break,
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
                        app.toggle_task(id);
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
            AppEvent::Add => {}

            _ => {}
        }

    }

    Ok(())
}
