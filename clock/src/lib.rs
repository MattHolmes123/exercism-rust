use std::fmt;
use chrono::{NaiveTime, Duration, Timelike};


#[derive(Debug, Eq, PartialEq)]
pub struct Clock {
    nt: NaiveTime
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut nt = NaiveTime::from_hms(0, 0, 0);

        nt += Duration::hours(hours as i64);
        nt += Duration:: minutes(minutes as i64);

        Clock {nt: nt}
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut nt = NaiveTime::from_hms(
            self.nt.hour(), self.nt.minute(), self.nt.second()
        );
        nt += Duration::minutes(minutes as i64);

        Clock {nt: nt}
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.nt.hour(), self.nt.minute())
    }
}
