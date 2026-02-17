use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Stylize, Style},
    text::Line,
    widgets::{Block, List, ListItem, BorderType, Widget},
    Frame,
    symbols::border::THICK,

};

use crate::app::App;
use super::state::UiState;

pub fn render(frame: &mut Frame, state: &mut UiState, app: &mut App) {
    let title = Line::from("  Lazy Task Manager  ".bold());

    let instructions = Line::from(vec![
        " Next Task ".into(),
        "< k >".blue().bold(),
        " Previous Task ".into(),
        "< j >".blue().bold(),
        " Toggle Task ".into(),
        "< space >".blue().bold(),
        " Delete Task ".into(),
        "< D >".blue().bold(),
        " Quit ".into(),
        "< q > ".blue().bold(),
    ]);

    let [border_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(frame.area());

    let [inner_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(border_area);

    Block::bordered()
        .border_type(BorderType::Rounded)
        .fg(Color::LightYellow)
        .title(title.centered())
        .title_bottom(instructions)
        .border_set(THICK)
        .render(border_area, frame.buffer_mut());

    let list = List::new(app.tasks.iter().map(|x| {
        let status = if x.is_checked { "[x]" } else { "[ ]" };
        ListItem::new(format!(
            " {}. {}: {} {}",
            x.id, x.title, x.description, status
        ))
    }))
    .highlight_style(
        Style::default()
            .fg(Color::Yellow)
            .bg(Color::Black)
            .add_modifier(ratatui::style::Modifier::BOLD),
    );

    frame.render_stateful_widget(list, inner_area, &mut state.list_state);
}