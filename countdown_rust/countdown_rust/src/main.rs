use time::{Date, OffsetDateTime, UtcOffset, Duration, Time, Month};
use std::ops::Sub;


fn main() {
    let utc = OffsetDateTime::now_utc();
    println!("The time is: {}", utc.time());
    println!("The date is: {}", utc.date());

   // let date = utc.date();

    let dt = OffsetDateTime::new_in_offset(
    Date::from_calendar_date(2024, Month::December, 15).expect("mumma mia"),
    Time::from_hms_nano(12, 59, 59, 500_000_000).expect("seconds error"),
    UtcOffset::from_hms(-5, 0, 0).expect("timezone error"),
);
    println!("Calculated date is {}",dt.date());

    let diff = dt.sub(utc);
    println!("{}", diff);
    println!("Weeks until deadline: {}", diff.whole_weeks()); 
    println!("Days until deadline: {}", diff.whole_days());
    println!("Hours until deadline: {}", diff.whole_hours());
    println!("Seconds until deadline: {}", diff.whole_seconds());



    println!("Days until deadline minus: {}", diff.whole_days());
    let diff_hours = diff.whole_hours() -(24*(diff.whole_days()));
    println!("Hours until deadline: {}", diff_hours);
    //let diff3 = diff2.sub(diff2.whole_hours());
    let diff_minutes = diff.whole_minutes() - (60*diff.whole_hours());
    println!("minutes: {}", diff_minutes);

    let diff_seconds = diff.whole_seconds() - (60*diff.whole_minutes());
    println!("second until deadline: {}", diff_seconds);
}
