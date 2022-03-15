use std::fmt::Display;

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
        let total_hours = hours + minutes.div_euclid(MINS_IN_HOUR);

        // keep remaining minutes
        let minutes = minutes.rem_euclid(MINS_IN_HOUR);

        // reduce hours to valid value
        let hours = total_hours.rem_euclid(HOURS_IN_DAY);

        Clock {
            hours: hours as u8,
            minutes: minutes as u8,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let hours = minutes.div_euclid(MINS_IN_HOUR);
        let minutes = minutes.rem_euclid(MINS_IN_HOUR);

        let total_minutes = minutes + self.minutes as i32;
        let total_hours = hours + self.hours as i32;

        Clock::new(total_hours, total_minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
