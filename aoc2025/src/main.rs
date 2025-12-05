use std::env;
use chrono::{Datelike, Local};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
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
    day01::run();
    day02::run();
    day03::run();
    day04::run();
    day05::run();
    match day {
        "1" => day01::run(),
        "2" => day02::run(),
        "3" => day03::run(),
        "4" => day04::run(),
        "5" => day05::run(),
        _ => println!("Day not implemented"),
    }
}
