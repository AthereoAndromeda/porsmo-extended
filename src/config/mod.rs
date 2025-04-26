use crate::timers::pomodoro::PomodoroConfig;
use nanoserde::{DeJson, SerJson};

#[derive(Clone, Debug, DeJson, SerJson)]
/// A JSON-serializable version of `PomodoroConfig`
pub struct PomodoroConfigSerializable {
    pub work_time: u64,
    pub break_time: u64,
    pub long_break: u64,
}

impl From<PomodoroConfig> for PomodoroConfigSerializable {
    fn from(value: PomodoroConfig) -> Self {
        Self {
            work_time: value.work_time.as_secs(),
            break_time: value.break_time.as_secs(),
            long_break: value.long_break.as_secs(),
        }
    }
}

impl Default for PomodoroConfigSerializable {
    fn default() -> Self {
        Self {
            work_time: 25 * 60,
            break_time: 5 * 60,
            long_break: 10 * 60,
        }
    }
}

#[derive(Clone, Debug, Default, DeJson, SerJson)]
pub struct Config {
    pomodoro_timings: PomodoroConfigSerializable,
}

impl Config {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
