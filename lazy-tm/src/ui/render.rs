use std::time::Duration;

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Margin, Rect},
    style::{Color, Style, Stylize},
    symbols::border::{DOUBLE, THICK},
    text::Line,
    widgets::{Block, BorderType, List, ListItem, Paragraph, Widget, Wrap},
};

use super::state::UiState;
use crate::{
    app::App,
    ui::state::{InputType, Mode},
};

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

pub fn render(frame: &mut Frame, state: &mut UiState, app: &mut App) {
    let title = Line::from("  Lazy Task Manager  ".bold());

    /*  Layout principal: conteúdo + barra inferior */

    let [main_area, instructions_area]: [Rect; 2] =
        Layout::vertical([Constraint::Fill(1), Constraint::Length(2)])
            .margin(1)
            .areas(frame.area());

    /* Render da moldura principal */
    Block::bordered()
        .border_type(BorderType::Rounded)
        .fg(Color::White)
        .title(title.centered())
        .border_set(THICK)
        .render(main_area, frame.buffer_mut());

    let inner_area = main_area.inner(Margin {
        vertical: 1,
        horizontal: 1,
    });

    let instructions = match &state.mode {
        Mode::Normal => Line::from(vec![
            " Next Task: ".into(),
            "k".light_yellow().bold(),
            " |".into(),
            " Previous Task: ".into(),
            "j".light_yellow().bold(),
            " |".into(),
            " Toggle Task: ".into(),
            "space".light_yellow().bold(),
            " |".into(),
            " Add Task: ".into(),
            "a".light_yellow().bold(),
            " |".into(),
            " Edit Task: ".into(),
            "e".light_yellow().bold(),
            " |".into(),
            " Start Timer: ".into(),
            "S".light_yellow().bold(),
            " |".into(),
            " Delete Task: ".into(),
            "D".light_yellow().bold(),
            " |".into(),
            " Clear All: ".into(),
            "C".light_yellow().bold(),
            " |".into(),
            " Quit: ".into(),
            "q".light_yellow().bold(),
        ]),
        Mode::Editing => Line::from(vec![
            " Change Focus: ".into(),
            "Tab".light_yellow().bold(),
            " |".into(),
            " Save Task: ".into(),
            "Enter".light_yellow().bold(),
            " |".into(),
            " Cancel: ".into(),
            "Esc".light_yellow().bold(),
        ]),
    };

    Paragraph::new(instructions)
        .wrap(Wrap { trim: true })
        .style(Style::default().fg(Color::White))
        .render(instructions_area, frame.buffer_mut());

    let list = List::new(app.tasks.iter().map(|x| {
        let status = if x.is_checked { "[x]" } else { "[ ]" };

        let timer = if x.is_running() {
            format!(" ⏱  {}", format_duration(x.current_elapsed()))
        } else if x.elapsed > Duration::ZERO {
            format!(" ⏸ {}", format_duration(x.current_elapsed()))
        } else {
            String::new()
        };

        ListItem::new(format!(
            " {}. {}: {} {} {}",
            x.id, x.title, x.description, status, timer
        ))
    }))
    .highlight_style(
        Style::default()
            .fg(Color::LightYellow)
            .bg(Color::Black)
            .add_modifier(ratatui::style::Modifier::BOLD),
    );

    if state.mode == Mode::Editing {
        let [list_area, input_area]: [Rect; 2] =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(6)]).areas(inner_area);

        frame.render_stateful_widget(list, list_area, &mut state.list_state);

        let [title_area, description_area]: [Rect; 2] =
            Layout::vertical([Constraint::Length(3), Constraint::Length(3)]).areas(input_area);

        let title_block = if state.input_type == InputType::Title {
            Block::bordered().title(" Title ").border_set(DOUBLE)
        } else {
            Block::bordered().title(" Title ")
        };

        let title_widget = Paragraph::new(state.title.as_str())
            .style(Style::default().fg(Color::White))
            .block(title_block);

        frame.render_widget(title_widget, title_area);

        let description_block = if state.input_type == InputType::Description {
            Block::bordered().title(" Description ").border_set(DOUBLE)
        } else {
            Block::bordered().title(" Description ")
        };

        let description_widget = Paragraph::new(state.description.as_str())
            .style(Style::default().fg(Color::White))
            .block(description_block);

        frame.render_widget(description_widget, description_area);
    } else {
        frame.render_stateful_widget(list, inner_area, &mut state.list_state);
    }
}
