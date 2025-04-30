use std::time::Instant;
use std::{io::Write, time::Duration};

use crate::terminal::running_color;
use crate::{CounterUI, new_line_queue, prelude::*};
use crate::{format::format_duration, input::Command};
use crossterm::style::Stylize;

#[derive(Debug, Clone, Copy)]
pub struct Stopwatch {
    start_time: Option<Instant>,
    elapsed_before: Duration,
}

impl Default for Stopwatch {
    fn default() -> Self {
        Self {
            start_time: Some(Instant::now()),
            elapsed_before: Duration::ZERO,
        }
    }
}

impl Stopwatch {
    pub fn new(start_time: Option<Instant>, elapsed_before: Duration) -> Self {
        Self {
            start_time,
            elapsed_before,
        }
    }

    pub fn elapsed(&self) -> Duration {
        match self.start_time {
            Some(start_time) => self.elapsed_before + start_time.elapsed(),
            None => self.elapsed_before,
        }
    }

    pub fn started(&self) -> bool {
        self.start_time.is_some()
    }

    pub fn start(&mut self) {
        if self.start_time.is_none() {
            self.start_time = Some(Instant::now());
        }
    }

    pub fn stop(&mut self) {
        if let Some(start_time) = self.start_time {
            self.elapsed_before += start_time.elapsed();
            self.start_time = None;
        }
    }

    pub fn toggle(&mut self) {
        match self.start_time {
            Some(start_time) => {
                self.elapsed_before += start_time.elapsed();
                self.start_time = None;
            }
            None => {
                self.start_time = Some(Instant::now());
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct StopwatchUI {
    stopwatch: Stopwatch,
}

const CONTROLS: &str = "[Q]: quit, [Space]: pause/resume";

impl CounterUI for StopwatchUI {
    fn show(&mut self, out: &mut impl Write) -> Result<()> {
        let elapsed = self.stopwatch.elapsed();
        let is_running = self.stopwatch.started();

        new_line_queue!(
            out,
            "Stopwatch",
            format_duration(elapsed).with(running_color(is_running)),
            CONTROLS,
        )?;

        out.flush()?;
        Ok(())
    }

    fn update(&mut self, command: Command) {
        match command {
            Command::Pause => self.stopwatch.stop(),
            Command::Resume => self.stopwatch.start(),
            Command::Toggle | Command::Enter => self.stopwatch.toggle(),
            _ => (),
        }
    }
}
