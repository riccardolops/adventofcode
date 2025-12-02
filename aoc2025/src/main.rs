use std::env;
use chrono::{Datelike, Local};

mod day01;
mod day02;
mod helpers;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = if let Some(day_arg) = args.get(1) {
        day_arg.as_str()
    } else {
            let today = Local::now().date_naive();
            let month = today.month();
            let day_num = today.day();

            let default_day = if month == 12 && (1..=25).contains(&day_num) {
                day_num
            } else {
                1
            };
            &default_day.to_string()
    };
    match day {
        "1" => day01::run(),
        "2" => day02::run(),
        _ => println!("Day not implemented"),
    }
}
