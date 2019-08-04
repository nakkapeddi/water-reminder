extern crate chrono;
use chrono::{DateTime, Duration, Utc};
use std::{thread, time};
mod notifier;

const PACKAGE_NAME: &str = "water-reminder";

fn hour_from_now(date_time: DateTime<Utc>) -> Option<DateTime<Utc>> {
    date_time.checked_add_signed(Duration::hours(1))
}

fn main() {
    let thirty_secs = time::Duration::from_secs(30);
    let now = Utc::now();
    let mut water_time = hour_from_now(now);
    
    loop {
        let now = Utc::now();
        if now >= water_time.unwrap() {
            water_time = hour_from_now(water_time.unwrap());
            let reminder_msg = format!("Drink up! \x07 I will remind you again at: {}", water_time.unwrap());
            println!("{}", reminder_msg);
            notifier::notify(&reminder_msg);
        }
        thread::sleep(thirty_secs);
    }
    
}
