use std::fmt::Display;

const DAY_IN_MINUTES: i32 = 24 * 60;
const HOUR_IN_MINUTES: i32 = 60;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    minutes: i32,
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hours = self.minutes / HOUR_IN_MINUTES;
        let minutes = self.minutes % HOUR_IN_MINUTES;
        write!(f, "{:02}:{:02}", hours, minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self {
            minutes: (hours * HOUR_IN_MINUTES + minutes).rem_euclid(DAY_IN_MINUTES),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self {
            minutes: (self.minutes + minutes).rem_euclid(DAY_IN_MINUTES),
        }
    }
}
