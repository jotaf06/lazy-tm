use std::time::Duration;

use color_eyre::eyre::Result;
use ratatui::crossterm::event::{self, Event, KeyCode};

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
    None,
}

pub fn read_event() -> Result<AppEvent> {
    if !event::poll(Duration::from_millis(500))? {
        return Ok(AppEvent::None);
    }

    if let Event::Key(key) = event::read()? {
        let event = match key.code {
            KeyCode::Esc => AppEvent::Quit,
            KeyCode::Char(char) => match char {
                'q' => AppEvent::Quit,
                'j' => AppEvent::SelPrevious,
                'k' => AppEvent::SelNext,
                ' ' => AppEvent::ToggleTask,
                'D' => AppEvent::Delete,
                'a' => AppEvent::Add,
                'C' => AppEvent::ClearAll,
                'e' => AppEvent::EditTask,
                'S' => AppEvent::StartTimer,
                _ => AppEvent::None,
            },

            KeyCode::Down => AppEvent::SelNext,
            KeyCode::Up => AppEvent::SelPrevious,
            _ => AppEvent::None,
        };

        return Ok(event);
    }

    Ok(AppEvent::None)
}
