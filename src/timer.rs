use crate::alert::Alerter;
use crate::stopwatch::Stopwatch;
use crate::terminal::running_color;
use crate::{CounterUI, new_line_queue, prelude::*};
use crate::{format::format_duration, input::Command};
use crossterm::{
    cursor::MoveToNextLine,
    queue,
    style::{Print, Stylize},
};
use std::io::Write;
use std::time::Duration;

use chrono::{DateTime, Local, TimeDelta};
fn timer_show(
    out: &mut impl Write,
    elapsed: Duration,
    target: Duration,
    is_running: bool,
    alerter: &mut Alerter,
    finish_time: &DateTime<Local>,
) -> Result<()> {
    let formatted_finish_time = finish_time
        .format("%H:%M:%S")
        .to_string()
        .with(crossterm::style::Color::Blue);

    let (title, timer, controls, tim) = if elapsed < target {
        let time_left = target.saturating_sub(elapsed);
        (
            "Timer",
            format_duration(time_left).with(running_color(is_running)),
            format!("ETA: {}", formatted_finish_time),
            "[Q]: quit, [Space]: pause/resume",
        )
    } else {
        alerter.alert_once(
            "The timer has ended!",
            format!(
                "Your Timer of {initial} has ended",
                initial = format_duration(target)
            ),
        );
        let excess_time = format_duration(elapsed.saturating_sub(target));
        (
            "Timer has ended",
            format!("+{excess_time}").with(running_color(is_running)),
            format!("ETA: {}", formatted_finish_time),
            "[Q]: quit, [Space]: pause/resume",
        )
    };

    new_line_queue!(out, title, timer, controls, tim)?;

    out.flush()?;
    Ok(())
}

fn timer_update(command: Command, stopwatch: &mut Stopwatch) {
    match command {
        Command::Pause => stopwatch.stop(),
        Command::Resume => stopwatch.start(),
        Command::Toggle | Command::Enter => stopwatch.toggle(),
        _ => (),
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct TimerUI {
    stopwatch: Stopwatch,
    target: Duration,
    alerter: Alerter,
    finish_time: DateTime<Local>,
}

impl TimerUI {
    pub fn new(target: Duration) -> Self {
        let dt1: DateTime<Local> = Local::now();
        let finish_time = dt1
            .checked_add_signed(TimeDelta::from_std(target).expect("Failed to convert Duration"))
            .expect("Failed to calculate estimated time");

        Self {
            target,
            finish_time,
            ..Default::default()
        }
    }
}

impl CounterUI for TimerUI {
    fn show(&mut self, out: &mut impl Write) -> Result<()> {
        let elapsed = self.stopwatch.elapsed();
        let is_running = self.stopwatch.started();
        timer_show(
            out,
            elapsed,
            self.target,
            is_running,
            &mut self.alerter,
            &self.finish_time,
        )
    }

    fn update(&mut self, command: Command) {
        timer_update(command, &mut self.stopwatch)
    }
}
