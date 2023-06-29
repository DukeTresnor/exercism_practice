// datetime_functions / mod.rs

use time::PrimitiveDateTime as DateTime;
use time::Duration;

// Returns a DateTime one billion seconds after start.
// A gigasecond is one thousand million seconds
pub fn after(start: DateTime) -> DateTime {


/*
    Make a new DateTime example:
        DateTime::new(
            Date::from_calendar_date(year, month.try_into().unwrap(), day).unwrap(),
            Time::from_hms(hour, minute, second).unwrap(),
        )

 */

    // use as_hms(), use date, use time
    // use new -- DateTime::new{date: Date, time: Time}
    // extract start data
    // convert one billion seconds into days, hours, minutes, and seconds
    // add your new values to the start data
    // create a new DateTime using new data
    //let date = start.date();
    //let time = start.time();
    //let mut year = date.year();
    //let month = date.month();
    //let mut ordinal_day = date.ordinal();
    //let (mut hours, mut minutes, mut seconds) = time.as_hms();
/*
    let giga_second = 1000000000.0;
    let seconds_per_year = 31560000.0;
    let days_per_year = 365.0;
    let hours_per_day = 24.0;
    let minutes_per_hour = 60.0;
    let seconds_per_minute = 60.0;
    let year_percentage_leftover = (seconds_per_year % seconds_per_year) / seconds_per_year;
    let years_added = giga_second / seconds_per_year - year_percentage_leftover;
    let days_percentage_leftover = (year_percentage_leftover % days_per_year) / days_per_year;
    let days_added = year_percentage_leftover / days_per_year - days_percentage_leftover;
    let hours_percentage_leftover = (days_percentage_leftover % hours_per_day) / hours_per_day;
    let hours_added = days_added / hours_per_day - hours_percentage_leftover;
    let minutes_percentage_leftover = ( hours_percentage_leftover % minutes_per_hour ) / minutes_per_hour;
    let minutes_added = hours_added / minutes_per_hour - minutes_percentage_leftover;
    let seconds_percentage_leftover = ( minutes_percentage_leftover % seconds_per_minute ) / seconds_per_minute;
    let seconds_added = minutes_added / seconds_per_minute - seconds_percentage_leftover;
*/

    // 1 gigasecond = 1,000,000,000 seconds
    // 1 year = 3.156e7 seconds
    // 1 day = 86400 seconds
    // 1 hour = 3600 seconds
    // 1 minute = 60 seconds


    // This is the main portion of the after() function. Idea is that you create a Duration entity with the desired added time, then you
    //   apply this Duration to the start param using checked_add().
    let added_duration = Duration::new(1000000000, 0);
    start.checked_add(added_duration).unwrap()

    

    
    
}
