use chrono::{Local, Timelike};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeContext {
    pub hour: u32,
    pub period: String,
}

impl Default for TimeContext {
    fn default() -> Self {
        Self {
            hour: 0,
            period: "morning".into(),
        }
    }
}

pub fn detect_time_context() -> TimeContext {
    let now = Local::now();
    let hour = now.hour();
    let period = match hour {
        0..=5 => "late-night",
        6..=11 => "morning",
        12..=17 => "afternoon",
        _ => "evening",
    }
    .to_string();

    TimeContext { hour, period }
}
