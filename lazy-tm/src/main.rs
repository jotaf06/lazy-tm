mod app;
mod task;

mod ui;
use ui::render::render;
use ui::state::{ActivePanel, HomeMode, InputType, LayoutMode, Mode, Screen, UiState};

mod input;
use input::{AppEvent, read_event};

mod storage;

use app::App;
use color_eyre::eyre::Result;
use ratatui::{
    DefaultTerminal,
    crossterm::event::{self, KeyCode},
    restore,
};

fn main() -> Result<()> {
    color_eyre::install()?;

    storage::ensure_lists_dir()?;
    storage::migrate_legacy("tasks.json", "Default")?;

    let list_names = storage::list_names()?;

    let mut app = App::default();
    let mut state = UiState::default();

    let terminal = ratatui::init();
    let result = run(terminal, &mut state, &mut app, list_names);

    restore();
    println!("Changes saved. See you soon! :)");
    result
}

fn run(
    mut terminal: DefaultTerminal,
    state: &mut UiState,
    app: &mut App,
    mut list_names: Vec<String>,
) -> Result<()> {
    loop {
        match state.layout_mode {
            LayoutMode::TwoPanel => {
                state.ensure_home_selection(list_names.len());
                state.ensure_selection(app.tasks.len());
                terminal.draw(|f| render(f, state, app, &list_names))?;
                if handle_two_panel_input(state, app, &mut list_names)? {
                    break;
                }
            }
            LayoutMode::Single => match state.screen {
                Screen::Home => {
                    state.ensure_home_selection(list_names.len());
                    terminal.draw(|f| render(f, state, app, &list_names))?;
                    if handle_home_input(state, app, &mut list_names)? {
                        break;
                    }
                }
                Screen::TaskList => {
                    state.ensure_selection(app.tasks.len());
                    terminal.draw(|f| render(f, state, app, &list_names))?;
                    handle_task_input(state, app, &mut list_names)?;
                }
            },
        }
    }

    Ok(())
}

fn save_current_list(app: &mut App) -> Result<()> {
    app.pause_all_timers();
    if let Some(name) = &app.current_list.clone() {
        storage::save_list(name, &app.tasks)?;
    }
    Ok(())
}

fn switch_to_two_panel(state: &mut UiState, app: &App, list_names: &[String]) {
    state.layout_mode = LayoutMode::TwoPanel;
    if let Some(current) = &app.current_list {
        let idx = list_names.iter().position(|n| n == current).unwrap_or(0);
        state.home_list_state.select(Some(idx));
        state.active_panel = ActivePanel::Right;
    } else {
        state.active_panel = ActivePanel::Left;
    }
}

fn switch_to_single(state: &mut UiState, app: &App) {
    state.layout_mode = LayoutMode::Single;
    state.screen = if app.current_list.is_some() {
        Screen::TaskList
    } else {
        Screen::Home
    };
}

/// Returns true when the app should quit.
fn handle_two_panel_input(
    state: &mut UiState,
    app: &mut App,
    list_names: &mut Vec<String>,
) -> Result<bool> {
    match state.active_panel {
        ActivePanel::Left => handle_two_panel_left(state, app, list_names),
        ActivePanel::Right => handle_two_panel_right(state, app, list_names),
    }
}

fn handle_two_panel_left(
    state: &mut UiState,
    app: &mut App,
    list_names: &mut Vec<String>,
) -> Result<bool> {
    match state.home_mode {
        HomeMode::Normal => {
            let event = read_event()?;
            match event {
                AppEvent::Quit => {
                    save_current_list(app)?;
                    return Ok(true);
                }
                AppEvent::ToggleLayout => {
                    save_current_list(app)?;
                    switch_to_single(state, app);
                }
                AppEvent::SelNext => {
                    let len = list_names.len();
                    if len > 0 {
                        let i = match state.home_list_state.selected() {
                            Some(i) => if i >= len - 1 { 0 } else { i + 1 },
                            None => 0,
                        };
                        state.home_list_state.select(Some(i));
                        auto_load_list(state, app, list_names, i)?;
                    }
                }
                AppEvent::SelPrevious => {
                    let len = list_names.len();
                    if len > 0 {
                        let i = match state.home_list_state.selected() {
                            Some(i) => if i == 0 { len - 1 } else { i - 1 },
                            None => 0,
                        };
                        state.home_list_state.select(Some(i));
                        auto_load_list(state, app, list_names, i)?;
                    }
                }
                AppEvent::Confirm | AppEvent::FocusRight => {
                    state.active_panel = ActivePanel::Right;
                }
                AppEvent::Add => {
                    state.home_mode = HomeMode::CreatingList;
                    state.new_list_name.clear();
                }
                AppEvent::Delete => {
                    if !list_names.is_empty() {
                        state.home_mode = HomeMode::ConfirmingDelete;
                    }
                }
                _ => {}
            }
        }
        HomeMode::ConfirmingDelete => {
            let event = read_event()?;
            match event {
                AppEvent::Confirm => {
                    if let Some(idx) = state.home_list_state.selected() {
                        if idx < list_names.len() {
                            let name = list_names[idx].clone();
                            storage::delete_list(&name)?;
                            list_names.remove(idx);
                            let new_len = list_names.len();
                            if new_len == 0 {
                                state.home_list_state.select(None);
                                app.tasks.clear();
                                app.current_list = None;
                            } else {
                                let new_idx = idx.min(new_len - 1);
                                state.home_list_state.select(Some(new_idx));
                                auto_load_list(state, app, list_names, new_idx)?;
                            }
                        }
                    }
                    state.home_mode = HomeMode::Normal;
                }
                AppEvent::Back => {
                    state.home_mode = HomeMode::Normal;
                }
                _ => {}
            }
        }
        HomeMode::CreatingList => {
            if let event::Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc => {
                        state.home_mode = HomeMode::Normal;
                        state.new_list_name.clear();
                    }
                    KeyCode::Char(c) => state.new_list_name.push(c),
                    KeyCode::Backspace => { state.new_list_name.pop(); }
                    KeyCode::Enter => {
                        let name = state.new_list_name.trim().to_string();
                        if !name.is_empty() && !list_names.contains(&name) {
                            storage::save_list(&name, &[])?;
                            list_names.push(name.clone());
                            list_names.sort();
                            let idx = list_names.iter().position(|n| n == &name).unwrap_or(0);
                            state.home_list_state.select(Some(idx));
                            auto_load_list(state, app, list_names, idx)?;
                        }
                        state.new_list_name.clear();
                        state.home_mode = HomeMode::Normal;
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(false)
}

fn auto_load_list(
    state: &mut UiState,
    app: &mut App,
    list_names: &[String],
    idx: usize,
) -> Result<()> {
    if idx < list_names.len() {
        let name = list_names[idx].clone();
        let tasks = storage::load_list(&name)?;
        app.load_list(name, tasks);
        state.list_state.select(None);
    }
    Ok(())
}

fn handle_two_panel_right(
    state: &mut UiState,
    app: &mut App,
    list_names: &mut Vec<String>,
) -> Result<bool> {
    match &state.mode {
        Mode::Normal => {
            let event = read_event()?;
            match event {
                AppEvent::ToggleLayout => {
                    save_current_list(app)?;
                    switch_to_single(state, app);
                }
                AppEvent::FocusLeft => {
                    save_current_list(app)?;
                    *list_names = storage::list_names()?;
                    state.active_panel = ActivePanel::Left;
                }
                AppEvent::Quit | AppEvent::Back => {
                    save_current_list(app)?;
                    *list_names = storage::list_names()?;
                    state.active_panel = ActivePanel::Left;
                    state.mode = Mode::Normal;
                }
                AppEvent::SelNext => {
                    let i = match state.list_state.selected() {
                        Some(i) => if i >= app.tasks.len().saturating_sub(1) { 0 } else { i + 1 },
                        None => 0,
                    };
                    state.list_state.select(Some(i));
                }
                AppEvent::SelPrevious => {
                    let i = match state.list_state.selected() {
                        Some(i) => if i == 0 { app.tasks.len().saturating_sub(1) } else { i - 1 },
                        None => 0,
                    };
                    state.list_state.select(Some(i));
                }
                AppEvent::ToggleTask => {
                    if let Some(idx) = state.list_state.selected() {
                        if let Some(task) = app.tasks.get(idx) {
                            let id = task.id;
                            app.toggle_task(id);
                        }
                    }
                }
                AppEvent::Delete => {
                    if state.list_state.selected().is_some() && !app.tasks.is_empty() {
                        state.mode = Mode::ConfirmingDelete;
                    }
                }
                AppEvent::ClearAll => {
                    if !app.tasks.is_empty() {
                        state.mode = Mode::ConfirmingClearAll;
                    }
                }
                AppEvent::Add => {
                    state.mode = Mode::Editing;
                    state.title.clear();
                    state.description.clear();
                    state.input_type = InputType::Title;
                    state.is_editing = false;
                }
                AppEvent::EditTask => {
                    if let Some(idx) = state.list_state.selected() {
                        if idx < app.tasks.len() {
                            state.mode = Mode::Editing;
                            state.input_type = InputType::Title;
                            state.is_editing = true;
                            state.title = app.tasks[idx].title.clone();
                            state.description = app.tasks[idx].description.clone();
                        }
                    }
                }
                AppEvent::CreateTimer => {
                    if let Some(idx) = state.list_state.selected() {
                        if idx < app.tasks.len() {
                            let task = &app.tasks[idx];
                            if !task.is_checked && !task.has_timer {
                                let id = task.id;
                                app.create_timer(id);
                            }
                        }
                    }
                }
                AppEvent::ToggleTimer => {
                    if let Some(idx) = state.list_state.selected() {
                        if idx < app.tasks.len() {
                            let task = &app.tasks[idx];
                            if !task.is_checked && task.has_timer {
                                let id = task.id;
                                app.toggle_timer(id);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        Mode::ConfirmingClearAll => {
            let event = read_event()?;
            match event {
                AppEvent::Confirm => {
                    app.clear_all_tasks();
                    state.list_state.select(None);
                    state.mode = Mode::Normal;
                }
                AppEvent::Back => state.mode = Mode::Normal,
                _ => {}
            }
        }
        Mode::ConfirmingDelete => {
            let event = read_event()?;
            match event {
                AppEvent::Confirm => {
                    if let Some(idx) = state.list_state.selected() {
                        if idx < app.tasks.len() {
                            app.tasks.remove(idx);
                        }
                        let len = app.tasks.len();
                        if len == 0 {
                            state.list_state.select(None);
                        } else if idx >= len {
                            state.list_state.select(Some(len - 1));
                        }
                    }
                    state.mode = Mode::Normal;
                }
                AppEvent::Back => state.mode = Mode::Normal,
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
                        state.is_editing = false;
                    }
                    KeyCode::Char(c) => match state.input_type {
                        InputType::Title => state.title.push(c),
                        InputType::Description => state.description.push(c),
                    },
                    KeyCode::Backspace => match state.input_type {
                        InputType::Title => { state.title.pop(); }
                        InputType::Description => { state.description.pop(); }
                    },
                    KeyCode::Enter => {
                        if state.is_editing {
                            if let Some(idx) = state.list_state.selected() {
                                if idx < app.tasks.len() {
                                    let id = app.tasks[idx].id;
                                    app.edit_task(id, state.title.clone(), state.description.clone());
                                }
                            }
                        } else if !state.title.is_empty() {
                            app.add_task(state.title.clone(), state.description.clone());
                        }
                        state.title.clear();
                        state.description.clear();
                        state.mode = Mode::Normal;
                        state.input_type = InputType::Title;
                        state.is_editing = false;
                    }
                    KeyCode::Tab => {
                        state.input_type = match state.input_type {
                            InputType::Title => InputType::Description,
                            InputType::Description => InputType::Title,
                        };
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(false)
}

/// Returns true when the app should quit.
fn handle_home_input(
    state: &mut UiState,
    app: &mut App,
    list_names: &mut Vec<String>,
) -> Result<bool> {
    match state.home_mode {
        HomeMode::Normal => {
            let event = read_event()?;
            match event {
                AppEvent::Quit => return Ok(true),
                AppEvent::ToggleLayout => {
                    switch_to_two_panel(state, app, list_names);
                    if let Some(idx) = state.home_list_state.selected() {
                        auto_load_list(state, app, list_names, idx)?;
                    }
                }
                AppEvent::SelNext => {
                    let len = list_names.len();
                    if len > 0 {
                        let i = match state.home_list_state.selected() {
                            Some(i) => if i >= len - 1 { 0 } else { i + 1 },
                            None => 0,
                        };
                        state.home_list_state.select(Some(i));
                    }
                }
                AppEvent::SelPrevious => {
                    let len = list_names.len();
                    if len > 0 {
                        let i = match state.home_list_state.selected() {
                            Some(i) => if i == 0 { len - 1 } else { i - 1 },
                            None => 0,
                        };
                        state.home_list_state.select(Some(i));
                    }
                }
                AppEvent::Confirm => {
                    if let Some(idx) = state.home_list_state.selected() {
                        if idx < list_names.len() {
                            let name = list_names[idx].clone();
                            let tasks = storage::load_list(&name)?;
                            app.load_list(name, tasks);
                            state.list_state.select(None);
                            state.screen = Screen::TaskList;
                        }
                    }
                }
                AppEvent::Add => {
                    state.home_mode = HomeMode::CreatingList;
                    state.new_list_name.clear();
                }
                AppEvent::Delete => {
                    if !list_names.is_empty() {
                        state.home_mode = HomeMode::ConfirmingDelete;
                    }
                }
                _ => {}
            }
        }
        HomeMode::ConfirmingDelete => {
            let event = read_event()?;
            match event {
                AppEvent::Confirm => {
                    if let Some(idx) = state.home_list_state.selected() {
                        if idx < list_names.len() {
                            let name = list_names[idx].clone();
                            storage::delete_list(&name)?;
                            list_names.remove(idx);
                            let new_len = list_names.len();
                            if new_len == 0 {
                                state.home_list_state.select(None);
                            } else if idx >= new_len {
                                state.home_list_state.select(Some(new_len - 1));
                            }
                        }
                    }
                    state.home_mode = HomeMode::Normal;
                }
                AppEvent::Back => {
                    state.home_mode = HomeMode::Normal;
                }
                _ => {}
            }
        }
        HomeMode::CreatingList => {
            if let event::Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc => {
                        state.home_mode = HomeMode::Normal;
                        state.new_list_name.clear();
                    }
                    KeyCode::Char(c) => state.new_list_name.push(c),
                    KeyCode::Backspace => { state.new_list_name.pop(); }
                    KeyCode::Enter => {
                        let name = state.new_list_name.trim().to_string();
                        if !name.is_empty() && !list_names.contains(&name) {
                            storage::save_list(&name, &[])?;
                            list_names.push(name.clone());
                            list_names.sort();
                            let idx = list_names.iter().position(|n| n == &name).unwrap_or(0);
                            state.home_list_state.select(Some(idx));
                        }
                        state.new_list_name.clear();
                        state.home_mode = HomeMode::Normal;
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(false)
}

fn handle_task_input(
    state: &mut UiState,
    app: &mut App,
    list_names: &mut Vec<String>,
) -> Result<()> {
    match &state.mode {
        Mode::Normal => {
            let event = read_event()?;
            match event {
                AppEvent::Quit | AppEvent::Back => {
                    save_current_list(app)?;
                    *list_names = storage::list_names()?;
                    state.screen = Screen::Home;
                    state.mode = Mode::Normal;
                }
                AppEvent::ToggleLayout => {
                    switch_to_two_panel(state, app, list_names);
                }
                AppEvent::SelNext => {
                    let i = match state.list_state.selected() {
                        Some(i) => if i >= app.tasks.len().saturating_sub(1) { 0 } else { i + 1 },
                        None => 0,
                    };
                    state.list_state.select(Some(i));
                }
                AppEvent::SelPrevious => {
                    let i = match state.list_state.selected() {
                        Some(i) => if i == 0 { app.tasks.len().saturating_sub(1) } else { i - 1 },
                        None => 0,
                    };
                    state.list_state.select(Some(i));
                }
                AppEvent::ToggleTask => {
                    if let Some(idx) = state.list_state.selected() {
                        if let Some(task) = app.tasks.get(idx) {
                            let id = task.id;
                            app.toggle_task(id);
                        }
                    }
                }
                AppEvent::Delete => {
                    if state.list_state.selected().is_some() && !app.tasks.is_empty() {
                        state.mode = Mode::ConfirmingDelete;
                    }
                }
                AppEvent::ClearAll => {
                    if !app.tasks.is_empty() {
                        state.mode = Mode::ConfirmingClearAll;
                    }
                }
                AppEvent::Add => {
                    state.mode = Mode::Editing;
                    state.title.clear();
                    state.description.clear();
                    state.input_type = InputType::Title;
                }
                AppEvent::EditTask => {
                    if let Some(idx) = state.list_state.selected() {
                        state.mode = Mode::Editing;
                        state.input_type = InputType::Title;
                        state.is_editing = true;
                        state.title = app.tasks[idx].title.clone();
                        state.description = app.tasks[idx].description.clone();
                    }
                }
                AppEvent::CreateTimer => {
                    if let Some(idx) = state.list_state.selected() {
                        if idx < app.tasks.len() {
                            let task = &app.tasks[idx];
                            if !task.is_checked && !task.has_timer {
                                let id = task.id;
                                app.create_timer(id);
                            }
                        }
                    }
                }
                AppEvent::ToggleTimer => {
                    if let Some(idx) = state.list_state.selected() {
                        if idx < app.tasks.len() {
                            let task = &app.tasks[idx];
                            if !task.is_checked && task.has_timer {
                                let id = task.id;
                                app.toggle_timer(id);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        Mode::ConfirmingClearAll => {
            let event = read_event()?;
            match event {
                AppEvent::Confirm => {
                    app.clear_all_tasks();
                    state.list_state.select(None);
                    state.mode = Mode::Normal;
                }
                AppEvent::Back => state.mode = Mode::Normal,
                _ => {}
            }
        }
        Mode::ConfirmingDelete => {
            let event = read_event()?;
            match event {
                AppEvent::Confirm => {
                    if let Some(idx) = state.list_state.selected() {
                        if idx < app.tasks.len() {
                            app.tasks.remove(idx);
                        }
                        let len = app.tasks.len();
                        if len == 0 {
                            state.list_state.select(None);
                        } else if idx >= len {
                            state.list_state.select(Some(len - 1));
                        }
                    }
                    state.mode = Mode::Normal;
                }
                AppEvent::Back => state.mode = Mode::Normal,
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
                        state.is_editing = false;
                    }
                    KeyCode::Char(c) => match state.input_type {
                        InputType::Title => state.title.push(c),
                        InputType::Description => state.description.push(c),
                    },
                    KeyCode::Backspace => match state.input_type {
                        InputType::Title => { state.title.pop(); }
                        InputType::Description => { state.description.pop(); }
                    },
                    KeyCode::Enter => {
                        if state.is_editing {
                            if let Some(idx) = state.list_state.selected() {
                                if idx < app.tasks.len() {
                                    let id = app.tasks[idx].id;
                                    app.edit_task(id, state.title.clone(), state.description.clone());
                                }
                            }
                        } else if !state.title.is_empty() {
                            app.add_task(state.title.clone(), state.description.clone());
                        }
                        state.title.clear();
                        state.description.clear();
                        state.mode = Mode::Normal;
                        state.input_type = InputType::Title;
                        state.is_editing = false;
                    }
                    KeyCode::Tab => {
                        state.input_type = match state.input_type {
                            InputType::Title => InputType::Description,
                            InputType::Description => InputType::Title,
                        };
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(())
}
