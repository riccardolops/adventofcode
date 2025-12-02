use crate::helpers;
use std::path::Path;

pub fn run() {
    let _dial_start = 50;
    let mut _current_pos1 = _dial_start;
    let mut _current_pos2 = _dial_start;
    let mut _pass_code1 = 0;
    let mut _pass_code2 = 0;
    let _input = helpers::read_file(Path::new("inputs/input/01"));
    for line in _input.lines() {
        let (direction, distance) = line.split_at(1);
        let distance: i32 = distance.parse().unwrap();
        _pass_code2 += distance / 100;
        let _reminder = distance % 100;

        if direction == "R" {
            _current_pos1 += distance;
        } else if direction == "L" {
            _current_pos1 -= distance;
        }
        _current_pos1 = _current_pos1 % 100;
        if _current_pos1 == 0 {
            _pass_code1 += 1;
        }

        for _ in 0.._reminder {
            if direction == "R" {
                _current_pos2 += 1;
            } else if direction == "L" {
                _current_pos2 -= 1;
            }
            _current_pos2 = _current_pos2 % 100;
            if _current_pos2 == 0 {
                _pass_code2 += 1;
            }
        }
    }
    println!("Day 01 Part 1: {}", _pass_code1);
    println!("Day 01 Part 2: {}", _pass_code2);

    
}