pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self {
            hours: hours,
            minutes: minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let total_minutes = self.hours * 60 + self.minutes + minutes;
        let new_hour = (total_minutes / 60) % 24;
        let new_minutes = total_minutes % 60;
        Self {
            hours: new_hour,
            minutes: new_minutes,
        }
    }
}
