use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock{
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let new_minutes = minutes.rem_euclid(60);
        if minutes < 0 {
            let overflow_hours;
            if minutes.rem_euclid(60) == 0 {
                overflow_hours = minutes / 60;
            } else {
                overflow_hours = (minutes / 60) - 1;
            }
            let new_hours = (hours + overflow_hours).rem_euclid(24);
            return Clock { hours: new_hours, minutes: new_minutes }
        } else {
            let overflow_hours: i32 = minutes / 60;
            let new_hours = (hours + overflow_hours).rem_euclid(24);
            return Clock { hours: new_hours, minutes: new_minutes }
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }

    pub fn to_string(&self) -> String {
        format!("{hours:0>2}:{minutes:02}", hours=self.hours, minutes=self.minutes)
    }

}

impl fmt::Display for Clock {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{hours:0>2}:{minutes:02}", hours=self.hours, minutes=self.minutes)
    }
}
