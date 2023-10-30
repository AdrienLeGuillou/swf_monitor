use std::error;
use std::path::Path;
use swf::workflow::{Workflow, load_workflows};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// counter
    pub counter: u8,
    pub workflows: Vec<Workflow>,
    pub selected_wf: usize,
}

impl Default for App {
    fn default() -> Self {
        let wfs = match load_workflows(Path::new(".")) {
            Ok(x) => x,
            _ => Vec::new(),
        };

        Self {
            running: true,
            counter: 0,
            workflows: wfs,
            selected_wf: 0,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn select_next_wf(&mut self) {
        if self.selected_wf == self.workflows.len() - 1 {
            self.selected_wf = 0;
        } else {
            self.selected_wf += 1;
        }
    }

    pub fn select_prev_wf(&mut self) {
        if self.selected_wf == 0 {
            self.selected_wf = self.workflows.len() - 1;
        } else {
            self.selected_wf -= 1;
        }
    }

    pub fn increment_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
        }
    }

    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
    }
}
