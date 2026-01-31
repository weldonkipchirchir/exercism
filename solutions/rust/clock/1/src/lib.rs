use std::fmt;

#[derive(Debug, Clone)]
pub struct Clock {
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self {
            minutes: Self::normalize(hours * 60 + minutes)
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self {
            minutes: Self::normalize(self.minutes + minutes)
        }
    }

    //ensures that any number of minutes passed in is normalized to a 24-hour clock -convered into a va lue between 0 and 1439 (inclusive), clock.
    // For example, passing in 1500 minutes would normalize to 60 minutes (1:00 AM), and passing in -15 minutes would normalize to 1425 minutes (11:45 PM).
    fn normalize(minutes: i32) -> i32 {
        // There are 1440 minutes in a day
        let day = 24 * 60;
        // Use modulo to wrap around the minutes within a day
        // Adding 'day' before modulo ensures positive result for negative inputs
        // For example, -15 % 1440 would yield -15, but (-15 + 1440) % 1440 yields 1425
        // This handles both positive and negative minute values correctly
        ((minutes % day) + day) % day
    }
}

impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hours = self.minutes / 60;
        let minutes = self.minutes % 60;
        write!(f, "{:02}:{:02}", hours, minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.minutes == other.minutes
    }
}
