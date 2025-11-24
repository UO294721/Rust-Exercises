use std::fmt::Display;
use std::fmt::Formatter;

#[derive(PartialEq, Debug)]
pub struct Clock {
    minutes: i32,
    hours: i32,
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f,"{:02}:{:02}", self.hours, self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (mut new_hours, new_minutes) = calculate_minutes(minutes); 
        let (another_hour, another_minute) = calculate_minutes(new_minutes);
        new_hours = calculate_hours(hours, new_hours + another_hour);
        Self {
            hours: new_hours,
            minutes: another_minute,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self{
        let (mut new_hours, new_minutes) = calculate_minutes(self.minutes + minutes);
        let (another_hour, another_minute) = calculate_minutes(new_minutes);
        new_hours = calculate_hours(self.hours, new_hours + another_hour);
        if new_hours >= 24 {
            new_hours = new_hours % 24;
        }
        Self {
            hours: new_hours,
            minutes: another_minute,
        }
    }
    
}

    pub fn calculate_hours(passed_hours: i32, carried_hours: i32) -> i32 {
        let hours_and_minutes = passed_hours + carried_hours;
        if hours_and_minutes >= 24 {
            hours_and_minutes % 24
        } else if hours_and_minutes < 0 {
            24 + hours_and_minutes % 24
        } else {
            hours_and_minutes
        }
    }

    pub fn calculate_minutes(minutes: i32) -> (i32, i32) {
        if minutes >= 60 {
            (minutes / 60, minutes % 60)
        } else if minutes < 0 {
            (minutes / 60 - 1, 60 + minutes % 60)
        } else {
            (0, minutes)
        }
    }

pub fn validate_time(hours: i32, minutes: i32) -> (i32, i32) {

    
    
    let mut new_hours = if hours >= 24 {
        hours % 24 + minutes / 60
    } else if hours < 0 {
        24 - (-hours) % 24 + minutes / 60
    } else {hours};
    let new_minutes = if minutes >= 60 {
        minutes % 60
    } else if minutes < 0 {
        new_hours = new_hours - (-minutes) / 60;
        60 - (-minutes) % 60
    } else {minutes};
    (new_hours, new_minutes)
}
