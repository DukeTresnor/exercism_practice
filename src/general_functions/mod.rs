// general_functions / mod.rs

use std::fmt;

// We want to add the Display trait for Clock, ot give it the to_string() method
#[derive(Debug, PartialEq)]
pub struct Clock {
    hour_hand: i32,
    minute_hand: i32,
}


impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        if self.hour_hand < 10 && self.minute_hand < 10 {
            write!(f, "0{}:0{}", self.hour_hand, self.minute_hand)
        } else if self.hour_hand >= 10 && self.minute_hand < 10 {
            write!(f, "{}:0{}", self.hour_hand, self.minute_hand)
        } else if self.hour_hand < 10 && self.minute_hand >= 10 {
            write!(f, "0{}:{}", self.hour_hand, self.minute_hand)
        } else {
            write!(f, "{}:{}", self.hour_hand, self.minute_hand)
        }
        
    }
}

// implement display for clock to give handle different desired write formatting
// handle hour and minute overflows
// handle overflow from clock creation
// handle negative times


// Implement Clock's methods
// Doesn't use dates
impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let clock = Clock {
            hour_hand: hours,
            minute_hand: minutes,
        };
        clock.add_minutes(0)       
    }
    
    // ie clock_entity.add_minutes(minutes)
    // first try just creates a new clock with a modded minute time
    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut current_minutes = self.minute_hand + minutes;
        let mut current_hour = self.hour_hand;
        while current_minutes < 0 {
            current_minutes += 60;
            current_hour -= 1;
        }
        while current_hour < 0 {
            current_hour += 24;
        }
        while current_minutes >= 60 {
            current_minutes -= 60;
            current_hour += 1;
        }
        while current_hour >= 24 {
            current_hour -= 24;
        }
        Clock {
            hour_hand: current_hour,
            minute_hand: current_minutes
        }
    }
}
