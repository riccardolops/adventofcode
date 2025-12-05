use crate::helpers;
use std::path::Path;

pub fn run() {
    let initial_position = 50;
    let mut current_position1 = initial_position;
    let mut current_position2 = initial_position;
    let mut pass_code1 = 0;
    let mut pass_code2 = 0;
    let input = helpers::read_file(Path::new("inputs/input/01"));
    for line in input.lines() {
        let (direction, distance_str) = line.split_at(1);
        let distance: i32 = distance_str.parse().unwrap();
        pass_code2 += distance / 100;
        let reminder = distance % 100;

        if direction == "R" {
            current_position1 += distance;
        } else if direction == "L" {
            current_position1 -= distance;
        }
        current_position1 = current_position1 % 100;
        if current_position1 == 0 {
            pass_code1 += 1;
        }

        for _ in 0..reminder {
            if direction == "R" {
                current_position2 += 1;
            } else if direction == "L" {
                current_position2 -= 1;
            }
            current_position2 = current_position2 % 100;
            if current_position2 == 0 {
                pass_code2 += 1;
            }
        }
    }
    println!("Day 01 Part 1: {}", pass_code1);
    println!("Day 01 Part 2: {}", pass_code2);
}
