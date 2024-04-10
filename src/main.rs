use std::env;
use chrono::{Datelike, Local, NaiveTime, Timelike};


fn main() {
    print_greeting();

    print_current_time();
}

fn print_current_time() {
    let now = Local::now();
    println!("Now = {}", now.timestamp());
    println!("Now Formatted = {}", now.to_rfc2822());
    let date = now.date_naive();
    let month = date.month();
    let year = date.year();
    let day = date.day();
    println!("Year: {}\nMonth: {}\nDay: {}", year, month, day);
    let time = now.time();
    let hour = time.hour();
    let minutes = time.minute();
    let seconds = time.second();
    println!("Hour: {}\nMinute: {}\nSeconds: {}", hour, minutes, seconds);
}

fn print_day_stage(time: NaiveTime) -> String {
    return match time.hour() {
        0..=11 => "morning".to_string(),
        12..=17 => "afternoon".to_string(),
        _ => "evening".to_string(),
    }
}

fn print_greeting() {
    let args: Vec<String> = env::args().collect();
    let name = args.get(1);
    let day_stage = print_day_stage(Local::now().time());
    match name {
        Some(n) => println!("Good, {} {}!", day_stage, n),
        None => {
            let name = "Luke Skywalker".to_string();
            println!("Good, {} [{}]!", day_stage, name);
        }
    }
}
