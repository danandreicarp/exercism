use std::{
    fmt::Display,
    ops::{Div, Rem},
};

const MINS_IN_HOUR: i32 = 60;
const HOURS_IN_DAY: i32 = 24;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: u8,
    minutes: u8,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        // transform minutes into hours
        let mut total_hours = hours + minutes.div(MINS_IN_HOUR);

        // keep remaining minutes
        let mut minutes = minutes.rem(MINS_IN_HOUR);

        // handle negative minutes e.g. -15 -> 45
        if minutes < 0 {
            minutes += MINS_IN_HOUR;
            total_hours -= 1;
        }

        // reduce hours to valid value
        let mut hours = total_hours.rem(HOURS_IN_DAY);

        // handle negative hours e.g. -1 -> 23
        if hours < 0 {
            hours += HOURS_IN_DAY;
        }

        Clock {
            hours: hours as u8,
            minutes: minutes as u8,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let hours = minutes.div(MINS_IN_HOUR);
        let minutes = minutes.rem(MINS_IN_HOUR);

        let total_minutes = minutes + self.minutes as i32;
        let total_hours = hours + self.hours as i32;

        Clock::new(total_hours, total_minutes)
    }

    // pub fn to_string(&self) -> String {
    //     format!("{:02}:{:02}", self.hours, self.minutes)
    // }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
