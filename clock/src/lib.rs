use std::fmt;

const MINS_PER_DAY: i32 = 24 * 60;

#[derive(Debug, Eq, PartialEq)]
pub struct Clock {
    mins: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mins = as_clock_time((hours * 60) + minutes);

        Clock { mins }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mins = as_clock_time(self.mins + minutes);

        Clock { mins }
    }
}

/// Calculates the least nonnegative remainder of mins
pub fn as_clock_time(minutes: i32) -> i32 {
    minutes.rem_euclid(MINS_PER_DAY)
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hours = self.mins / 60;
        let minutes = self.mins % 60;

        write!(f, "{:02}:{:02}", hours, minutes)
    }
}
