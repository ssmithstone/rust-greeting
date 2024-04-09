use std::env;
use chrono::{Datelike, Local, Timelike};

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

fn print_greeting() {
    let args: Vec<String> = env::args().collect();
    let name = args.get(1);
    match name {
        Some(n) => println!("Hello, {}!", n),
        None => {
            let name = "Luke Skywalker".to_string();
            println!("Hello, [{}]!", name);
        }
    }
}
