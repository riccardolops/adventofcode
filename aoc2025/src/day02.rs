use crate::helpers;
use std::path::Path;

fn is_invalid1(id: i64) -> bool {
    let id_str = id.to_string();
    let half = id_str.len() / 2;
    let first_half = &id_str[0..half];
    let second_half = &id_str[half..id_str.len()];
    if first_half == second_half {
        return true;
    }
    return false;
}

fn is_invalid2(id: i64) -> bool {
    let str_id = id.to_string();
    let len = str_id.len();
    let mut number_of_splits = 2;
    let mut split = len / number_of_splits;

    while split > 0 {
        if len % number_of_splits != 0 {
            number_of_splits += 1;
            split = len / number_of_splits;
            continue;
        }
        let mut parts: Vec<&str> = Vec::new();
        let mut i = 0;
        while i + split <= len {
            parts.push(&str_id[i..i + split]);
            i += split;
        }
        let mut are_parts_equal = true;
        for j in 0..parts.len() {
            if parts[j] != parts[0] {
                are_parts_equal = false;
                break;
            }
        }
        if are_parts_equal {
            return true;
        }
        number_of_splits += 1;
        split = len / number_of_splits;
    }
    false
}

pub fn run() {
    let input = helpers::read_file(Path::new("inputs/input/02"));
    let lines: Vec<String> = input.lines().map(|line| line.trim().to_string()).collect();
    let mut sum_of_invalid_ids1 = 0;
    let mut sum_of_invalid_ids2 = 0;
    for line in lines {
        let ranges = line.split(",");
        for range in ranges {
            let first_id = range.split("-").collect::<Vec<&str>>()[0].parse().unwrap();
            let last_id: i64 = range.split("-").collect::<Vec<&str>>()[1].parse().unwrap();
            for id in first_id..=last_id {
                if is_invalid1(id) {
                    sum_of_invalid_ids1 += id;
                }
                if is_invalid2(id) {
                    sum_of_invalid_ids2 += id;
                    // println!("Invalid ID2: {}", id);
                }
            }
        }
    }
    println!("Day 02 Part 1: {}", sum_of_invalid_ids1);
    println!("Day 02 Part 2: {}", sum_of_invalid_ids2);
}
