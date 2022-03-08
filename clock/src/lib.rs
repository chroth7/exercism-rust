use std::fmt::Display;

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let hours_from_minutes = minutes / 60;
        let mut hours = (hours + hours_from_minutes) % 24;

        let mut minutes = minutes % 60;
        if minutes < 0 {
            minutes += 60;
            hours -= 1;
        }

        if hours < 0 {
            hours += 24
        }

        Clock {
            hours,
            minutes: minutes % 60,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}
