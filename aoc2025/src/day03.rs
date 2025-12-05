use crate::helpers;
use std::path::Path;

pub fn run() {
    let input = helpers::read_file(Path::new("inputs/input/03"));
    let lines: Vec<String> = input.lines().map(|line| line.trim().to_string()).collect();

    let bank_len = 12;

    let mut total_joltage1 = 0;
    let mut total_joltage2 = 0u64;

    for line in lines {
        let digits: Vec<u32> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();

        let mut digits_copy = digits.clone();
        let mut joltages: Vec<u32> = Vec::new();

        while joltages.len() < bank_len {
            let mut max_digit = 0;
            let mut max_digit_index = 0;

            for i in 0..digits_copy.len() {
                if digits_copy[i] > max_digit && digits_copy.len() - i >= bank_len - joltages.len() {
                    max_digit = digits_copy[i];
                    max_digit_index = i;
                }
            }

            joltages.push(max_digit);
            digits_copy.drain(0..=max_digit_index);
        }

        let mut max_first_digit = digits[0];
        let mut second_max_digit = digits[1];

        for i in 1..digits.len() - 1 {
            let current_digit = digits[i];
            if current_digit > max_first_digit {
                max_first_digit = current_digit;
                second_max_digit = digits[i + 1];
            } else if current_digit > second_max_digit {
                second_max_digit = current_digit;
            }
        }

        if digits[digits.len() - 1] > second_max_digit {
            second_max_digit = digits[digits.len() - 1];
        }

        let joltage_str: String = joltages.iter().map(|d| d.to_string()).collect();
        let joltage_num: u64 = joltage_str.parse().unwrap();
        total_joltage2 += joltage_num;
        total_joltage1 += max_first_digit * 10 + second_max_digit;
    }

    println!("Day 03 Part 1: {}", total_joltage1);
    println!("Day 03 Part 2: {}", total_joltage2);
}

