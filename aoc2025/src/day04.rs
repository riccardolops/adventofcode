use crate::helpers;
use std::path::Path;

fn count_valid_rolls(lines: Vec<String>) -> u32 {
    let mut total_valid_rolls = 0;
    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            if lines[i].chars().nth(j).unwrap() == '@' {
                let mut adiacent_rolls = 0;
                for di in -1..=1 {
                    for dj in -1..=1 {
                        if di == 0 && dj == 0 {
                            continue;
                        }
                        let ni = i as isize + di;
                        let nj = j as isize + dj;
                        if ni >= 0 && ni < lines.len() as isize && nj >= 0 && nj < lines[i].len() as isize {
                            if lines[ni as usize].chars().nth(nj as usize).unwrap() == '@' {
                                adiacent_rolls += 1;
                            }
                        }
                    }
                }
                if adiacent_rolls < 4 {
                    total_valid_rolls += 1;
                }
            }
        }
    }
    total_valid_rolls
}

pub fn run() {
    let input = helpers::read_file(Path::new("inputs/input/04"));
    let mut lines: Vec<String> = input.lines().map(|line| line.trim().to_string()).collect();
    let total_valid_rolls = count_valid_rolls(lines.clone());
    println!("Day 04 Part 1: {}", total_valid_rolls);

    let mut total_removable_rolls = 0;
    let mut just_removed = true;
    while just_removed {
        let removed = count_valid_rolls(lines.clone());
        if removed == 0 {
            just_removed = false;
        } else {
            total_removable_rolls += removed;
            let mut lines_copy = lines.clone();
            for i in 0..lines.len() {
                for j in 0..lines[i].len() {
                    if lines[i].chars().nth(j).unwrap() == '@' {
                        let mut adiacent_rolls = 0;
                        for di in -1..=1 {
                            for dj in -1..=1 {
                                if di == 0 && dj == 0 {
                                    continue;
                                }
                                let ni = i as isize + di;
                                let nj = j as isize + dj;
                                if ni >= 0 && ni < lines.len() as isize && nj >= 0 && nj < lines[i].len() as isize {
                                    if lines[ni as usize].chars().nth(nj as usize).unwrap() == '@' {
                                        adiacent_rolls += 1;
                                    }
                                }
                            }
                        }
                        if adiacent_rolls < 4 {
                            let mut line_chars: Vec<char> = lines_copy[i].chars().collect();
                            line_chars[j] = '.';
                            lines_copy[i] = line_chars.iter().collect();
                        }
                    }
                }
            }
            lines = lines_copy;
        }
    }
    println!("Day 04 Part 2: {}", total_removable_rolls);
}
    