use std::time::Duration;

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Margin, Rect},
    style::{Color, Style, Stylize},
    symbols::border::{DOUBLE, THICK},
    text::Line,
    widgets::{Block, BorderType, List, ListItem, Paragraph, Widget, Wrap},
};

use super::state::{ActivePanel, HomeMode, InputType, LayoutMode, Mode, Screen, UiState};
use crate::app::App;

fn format_duration(duration: Duration) -> String {
    let total = duration.as_secs();
    let hours = total / 3600;
    let minutes = (total % 3600) / 60;
    let seconds = total % 60;

    if hours > 0 {
        return format!("{:02}:{:02}:{:02}", hours, minutes, seconds);
    }

    format!("{:02}:{:02}", minutes, seconds)
}

pub fn render(frame: &mut Frame, state: &mut UiState, app: &mut App, list_names: &[String]) {
    match state.layout_mode {
        LayoutMode::TwoPanel => render_two_panel(frame, state, app, list_names),
        LayoutMode::Single => match state.screen {
            Screen::Home => render_home(frame, state, list_names),
            Screen::TaskList => render_task_list(frame, state, app),
        },
    }
}

fn render_two_panel(frame: &mut Frame, state: &mut UiState, app: &mut App, list_names: &[String]) {
    let [main_area, instructions_area]: [Rect; 2] =
        Layout::vertical([Constraint::Fill(1), Constraint::Length(2)])
            .margin(1)
            .areas(frame.area());

    let [left_area, right_area]: [Rect; 2] =
        Layout::horizontal([Constraint::Percentage(30), Constraint::Percentage(70)])
            .areas(main_area);

    let left_color = if state.active_panel == ActivePanel::Left { Color::LightYellow } else { Color::White };
    let right_color = if state.active_panel == ActivePanel::Right { Color::LightYellow } else { Color::White };

    Block::bordered()
        .border_type(BorderType::Rounded)
        .fg(left_color)
        .title(Line::from(" Lists ").bold().centered())
        .title_bottom(Line::from("[1]").right_aligned())
        .border_set(THICK)
        .render(left_area, frame.buffer_mut());

    let list_name = app.current_list.as_deref().unwrap_or("—");
    Block::bordered()
        .border_type(BorderType::Rounded)
        .fg(right_color)
        .title(Line::from(format!("  {}  ", list_name)).bold().centered())
        .title_bottom(Line::from("[2]").right_aligned())
        .border_set(THICK)
        .render(right_area, frame.buffer_mut());

    let left_inner = left_area.inner(Margin { vertical: 1, horizontal: 1 });
    let right_inner = right_area.inner(Margin { vertical: 1, horizontal: 1 });

    let home_list = build_home_list(list_names);
    frame.render_stateful_widget(home_list, left_inner, &mut state.home_list_state);

    let task_list = build_task_list(app);
    frame.render_stateful_widget(task_list, right_inner, &mut state.list_state);

    let instructions = match state.active_panel {
        ActivePanel::Left => match &state.home_mode {
            HomeMode::Normal => Line::from(vec![
                " Nav: ".into(), "j/k".light_yellow().bold(),
                " |".into(), " Focus Right: ".into(), "Enter".light_yellow().bold(),
                " |".into(), " New List: ".into(), "a".light_yellow().bold(),
                " |".into(), " Delete List: ".into(), "D".light_yellow().bold(),
                " |".into(), " Two-Panel Off: ".into(), "M".light_yellow().bold(),
                " |".into(), " Quit: ".into(), "q".light_yellow().bold(),
            ]),
            HomeMode::CreatingList | HomeMode::ConfirmingDelete => Line::from(vec![
                " Confirm: ".into(), "Enter".light_yellow().bold(),
                " |".into(), " Cancel: ".into(), "Esc".light_yellow().bold(),
            ]),
        },
        ActivePanel::Right => match &state.mode {
            Mode::Normal => Line::from(vec![
                " Nav: ".into(), "j/k".light_yellow().bold(),
                " |".into(), " Toggle: ".into(), "space".light_yellow().bold(),
                " |".into(), " Add: ".into(), "a".light_yellow().bold(),
                " |".into(), " Edit: ".into(), "e".light_yellow().bold(),
                " |".into(), " Timer: ".into(), "T/S".light_yellow().bold(),
                " |".into(), " Delete: ".into(), "D".light_yellow().bold(),
                " |".into(), " Clear: ".into(), "C".light_yellow().bold(),
                " |".into(), " Two-Panel Off: ".into(), "M".light_yellow().bold(),
                " |".into(), " Back: ".into(), "q".light_yellow().bold(),
            ]),
            Mode::Editing => Line::from(vec![
                " Switch Field: ".into(), "Tab".light_yellow().bold(),
                " |".into(), " Save: ".into(), "Enter".light_yellow().bold(),
                " |".into(), " Cancel: ".into(), "Esc".light_yellow().bold(),
            ]),
            Mode::ConfirmingDelete | Mode::ConfirmingClearAll => Line::from(vec![
                " Confirm: ".into(), "Enter".light_yellow().bold(),
                " |".into(), " Cancel: ".into(), "Esc".light_yellow().bold(),
            ]),
        },
    };

    Paragraph::new(instructions)
        .wrap(Wrap { trim: true })
        .style(Style::default().fg(Color::White))
        .render(instructions_area, frame.buffer_mut());

    // Popups rendered over full frame
    match state.home_mode {
        HomeMode::CreatingList => {
            let popup_area = centered_rect(50, 3, frame.area());
            frame.render_widget(
                Paragraph::new(state.new_list_name.as_str())
                    .style(Style::default().fg(Color::White))
                    .block(Block::bordered().title(" New list name ").border_set(DOUBLE)),
                popup_area,
            );
        }
        HomeMode::ConfirmingDelete => {
            if let Some(idx) = state.home_list_state.selected() {
                if let Some(name) = list_names.get(idx) {
                    render_confirm_popup(frame, name, frame.area());
                }
            }
        }
        HomeMode::Normal => {}
    }

    render_task_popups(frame, state, app);
}

fn render_home(frame: &mut Frame, state: &mut UiState, list_names: &[String]) {
    let title = Line::from("  Lazy Task Manager  ".bold());

    let [main_area, instructions_area]: [Rect; 2] =
        Layout::vertical([Constraint::Fill(1), Constraint::Length(2)])
            .margin(1)
            .areas(frame.area());

    Block::bordered()
        .border_type(BorderType::Rounded)
        .fg(Color::White)
        .title(title.centered())
        .border_set(THICK)
        .render(main_area, frame.buffer_mut());

    let inner_area = main_area.inner(Margin { vertical: 1, horizontal: 1 });

    let instructions = match &state.home_mode {
        HomeMode::Normal => Line::from(vec![
            " Next: ".into(), "k".light_yellow().bold(),
            " |".into(), " Previous: ".into(), "j".light_yellow().bold(),
            " |".into(), " Open List: ".into(), "Enter".light_yellow().bold(),
            " |".into(), " New List: ".into(), "a".light_yellow().bold(),
            " |".into(), " Delete List: ".into(), "D".light_yellow().bold(),
            " |".into(), " Two-Panel: ".into(), "M".light_yellow().bold(),
            " |".into(), " Quit: ".into(), "q".light_yellow().bold(),
        ]),
        HomeMode::CreatingList => Line::from(vec![
            " Confirm: ".into(), "Enter".light_yellow().bold(),
            " |".into(), " Cancel: ".into(), "Esc".light_yellow().bold(),
        ]),
        HomeMode::ConfirmingDelete => Line::from(vec![
            " Confirm Delete: ".into(), "Enter".light_yellow().bold(),
            " |".into(), " Cancel: ".into(), "Esc".light_yellow().bold(),
        ]),
    };

    Paragraph::new(instructions)
        .wrap(Wrap { trim: true })
        .style(Style::default().fg(Color::White))
        .render(instructions_area, frame.buffer_mut());

    let list = build_home_list(list_names);
    frame.render_stateful_widget(list, inner_area, &mut state.home_list_state);

    match state.home_mode {
        HomeMode::CreatingList => {
            let popup_area = centered_rect(50, 3, frame.area());
            frame.render_widget(
                Paragraph::new(state.new_list_name.as_str())
                    .style(Style::default().fg(Color::White))
                    .block(Block::bordered().title(" New list name ").border_set(DOUBLE)),
                popup_area,
            );
        }
        HomeMode::ConfirmingDelete => {
            if let Some(idx) = state.home_list_state.selected() {
                if let Some(name) = list_names.get(idx) {
                    render_confirm_popup(frame, name, frame.area());
                }
            }
        }
        HomeMode::Normal => {}
    }
}

fn render_task_list(frame: &mut Frame, state: &mut UiState, app: &mut App) {
    let list_name = app.current_list.as_deref().unwrap_or("Tasks");
    let title = Line::from(format!("  {}   |   Lazy Task Manager ", list_name).bold());

    let [main_area, instructions_area]: [Rect; 2] =
        Layout::vertical([Constraint::Fill(1), Constraint::Length(2)])
            .margin(1)
            .areas(frame.area());

    Block::bordered()
        .border_type(BorderType::Rounded)
        .fg(Color::White)
        .title(title.centered())
        .border_set(THICK)
        .render(main_area, frame.buffer_mut());

    let inner_area = main_area.inner(Margin { vertical: 1, horizontal: 1 });

    let instructions = match &state.mode {
        Mode::Normal => Line::from(vec![
            " Next Task: ".into(), "k".light_yellow().bold(),
            " |".into(), " Previous Task: ".into(), "j".light_yellow().bold(),
            " |".into(), " Toggle Task: ".into(), "space".light_yellow().bold(),
            " |".into(), " Add Task: ".into(), "a".light_yellow().bold(),
            " |".into(), " Edit Task: ".into(), "e".light_yellow().bold(),
            " |".into(), " Create Timer: ".into(), "T".light_yellow().bold(),
            " |".into(), " Start/Stop Timer: ".into(), "S".light_yellow().bold(),
            " |".into(), " Delete Task: ".into(), "D".light_yellow().bold(),
            " |".into(), " Clear All: ".into(), "C".light_yellow().bold(),
            " |".into(), " Two-Panel: ".into(), "M".light_yellow().bold(),
            " |".into(), " Back: ".into(), "q".light_yellow().bold(),
        ]),
        Mode::Editing => Line::from(vec![
            " Change Focus: ".into(), "Tab".light_yellow().bold(),
            " |".into(), " Save Task: ".into(), "Enter".light_yellow().bold(),
            " |".into(), " Cancel: ".into(), "Esc".light_yellow().bold(),
        ]),
        Mode::ConfirmingDelete | Mode::ConfirmingClearAll => Line::from(vec![
            " Confirm: ".into(), "Enter".light_yellow().bold(),
            " |".into(), " Cancel: ".into(), "Esc".light_yellow().bold(),
        ]),
    };

    Paragraph::new(instructions)
        .wrap(Wrap { trim: true })
        .style(Style::default().fg(Color::White))
        .render(instructions_area, frame.buffer_mut());

    let list = build_task_list(app);
    frame.render_stateful_widget(list, inner_area, &mut state.list_state);

    render_task_popups(frame, state, app);
}

fn render_task_popups(frame: &mut Frame, state: &mut UiState, app: &mut App) {
    if state.mode == Mode::ConfirmingClearAll {
        let popup_area = centered_rect(44, 3, frame.area());
        Paragraph::new(Line::from(vec![
            " Clear all tasks? ".into(),
            "Enter".light_yellow().bold(),
            " / ".into(),
            "Esc".light_yellow().bold(),
            " ".into(),
        ]))
        .block(Block::bordered().border_set(DOUBLE).fg(Color::Red))
        .render(popup_area, frame.buffer_mut());
    }

    if state.mode == Mode::ConfirmingDelete {
        if let Some(idx) = state.list_state.selected() {
            if let Some(task) = app.tasks.get(idx) {
                let popup_area = centered_rect(50, 3, frame.area());
                Paragraph::new(Line::from(vec![
                    " Delete \"".into(),
                    task.title.as_str().light_red().bold(),
                    "\"? ".into(),
                    "Enter".light_yellow().bold(),
                    " / ".into(),
                    "Esc".light_yellow().bold(),
                    " ".into(),
                ]))
                .block(Block::bordered().border_set(DOUBLE).fg(Color::Red))
                .render(popup_area, frame.buffer_mut());
            }
        }
    }

    if state.mode == Mode::Editing {
        let popup_area = centered_rect(60, 6, frame.area());
        let [title_area, description_area]: [Rect; 2] =
            Layout::vertical([Constraint::Length(3), Constraint::Length(3)]).areas(popup_area);

        let title_block = if state.input_type == InputType::Title {
            Block::bordered().title(" Title ").border_set(DOUBLE)
        } else {
            Block::bordered().title(" Title ")
        };
        frame.render_widget(
            Paragraph::new(state.title.as_str())
                .style(Style::default().fg(Color::White))
                .block(title_block),
            title_area,
        );

        let description_block = if state.input_type == InputType::Description {
            Block::bordered().title(" Description ").border_set(DOUBLE)
        } else {
            Block::bordered().title(" Description ")
        };
        frame.render_widget(
            Paragraph::new(state.description.as_str())
                .style(Style::default().fg(Color::White))
                .block(description_block),
            description_area,
        );
    }
}

fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let x = area.x + area.width.saturating_sub(width) / 2;
    let y = area.y + area.height.saturating_sub(height) / 2;
    Rect::new(x, y, width.min(area.width), height.min(area.height))
}

fn render_confirm_popup(frame: &mut Frame, list_name: &str, area: Rect) {
    let popup_area = centered_rect(44, 3, area);
    Paragraph::new(Line::from(vec![
        " Delete \"".into(),
        list_name.light_red().bold(),
        "\"? ".into(),
        "Enter".light_yellow().bold(),
        " / ".into(),
        "Esc".light_yellow().bold(),
        " ".into(),
    ]))
    .block(Block::bordered().border_set(DOUBLE).fg(Color::Red))
    .render(popup_area, frame.buffer_mut());
}

fn build_home_list(list_names: &[String]) -> List<'_> {
    List::new(list_names.iter().map(|name| ListItem::new(format!("  {}", name))))
        .highlight_style(
            Style::default()
                .fg(Color::LightYellow)
                .bg(Color::Black)
                .add_modifier(ratatui::style::Modifier::BOLD),
        )
}

fn build_task_list(app: &App) -> List<'_> {
    List::new(app.tasks.iter().map(|x| {
        let status = if x.is_checked { "[x]" } else { "[ ]" };
        let timer = if x.is_running() {
            format!(" ⏱  {}", format_duration(x.current_elapsed()))
        } else if x.has_timer {
            format!(" ⏸ {}", format_duration(x.current_elapsed()))
        } else {
            String::new()
        };
        ListItem::new(format!(" {}. {}: {} {} {}", x.id, x.title, x.description, status, timer))
    }))
    .highlight_style(
        Style::default()
            .fg(Color::LightYellow)
            .bg(Color::Black)
            .add_modifier(ratatui::style::Modifier::BOLD),
    )
}
