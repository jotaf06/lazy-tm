use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub is_checked: bool,
    pub elapsed: Duration,

    #[serde(skip)]
    pub timer_start: Option<Instant>,
}

impl Task {
    pub fn new(id: u64, title: String, description: String) -> Task {
        Task {
            id,
            title,
            description,
            is_checked: false,
            elapsed: Duration::ZERO,
            timer_start: None,
        }
    }

    pub fn is_running(&self) -> bool {
        self.timer_start.is_some()
    }

    pub fn current_elapsed(&self) -> Duration {
        match self.timer_start {
            Some(start) => self.elapsed + start.elapsed(),
            None => self.elapsed,
        }
    }

    pub fn start_timer(&mut self) {
        if self.timer_start.is_none() {
            self.timer_start = Some(Instant::now());
        }
    }

    pub fn stop_timer(&mut self) {
        if let Some(start) = self.timer_start {
            self.elapsed += start.elapsed();
            self.timer_start = None;
        }
    }

    pub fn toggle_timer(&mut self) {
        if self.is_running() {
            self.stop_timer();
        } else {
            self.start_timer();
        }
    }
}
