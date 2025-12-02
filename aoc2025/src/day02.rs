use crate::helpers;
use std::path::Path;

fn is_invalid1(id: i64) -> bool {
    let _id_str = id.to_string();
    let _half = _id_str.len() / 2;
    let _first_half = &_id_str[0.._half];
    let _second_half = &_id_str[_half.._id_str.len()];
    if _first_half == _second_half {
        return true;
    }
    return false;
}

fn is_invalid2(id: i64) -> bool {
    let _str_id = id.to_string();
    let len = _str_id.len();
    let mut _number_of_splits = 2;
    let mut split = len / _number_of_splits;

    while split > 0 {
        if len % _number_of_splits != 0 {
            _number_of_splits += 1;
            split = len / _number_of_splits;
            continue;
        }
        let mut parts: Vec<&str> = Vec::new();
        let mut i = 0;
        while i + split <= len {
            parts.push(&_str_id[i..i + split]);
            i += split;
        }
        let mut are_equal = true;
        for j in 0..parts.len() {
            if parts[j] != parts[0] {
                are_equal = false;
                break;
            }
        }
        if are_equal {
            return true;
        }
        _number_of_splits += 1;
        split = len / _number_of_splits;
    }
    false
}

pub fn run() {
    let _input = helpers::read_file(Path::new("inputs/input/02"));
    let _lines: Vec<String> = _input.lines().map(|line| line.trim().to_string()).collect();
    let mut _sum_of_invalid_ids1 = 0;
    let mut _sum_of_invalid_ids2 = 0;
    for _line in _lines {
        let _ranges = _line.split(",");
        for _range in _ranges {
            let _first_id = _range.split("-").collect::<Vec<&str>>()[0].parse().unwrap();
            let _last_id: i64 = _range.split("-").collect::<Vec<&str>>()[1].parse().unwrap();
            for _id in _first_id..=_last_id {
                if is_invalid1(_id) {
                    _sum_of_invalid_ids1 += _id;
                }
                if is_invalid2(_id) {
                    _sum_of_invalid_ids2 += _id;
                    // println!("Invalid ID2: {}", _id);

                }
            }
        }
    }
    println!("Day 02 Part 1: {}", _sum_of_invalid_ids1);
    println!("Day 02 Part 2: {}", _sum_of_invalid_ids2);
}
