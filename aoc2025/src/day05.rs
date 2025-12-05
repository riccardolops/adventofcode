use crate::helpers;
use std::path::Path;

fn merge_ranges(ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut sorted_ranges = ranges.clone();
    sorted_ranges.sort_by(|a, b| a.0.cmp(&b.0));
    let mut merged_ranges: Vec<(u64, u64)> = Vec::new();
    for range in sorted_ranges {
        if merged_ranges.is_empty() {
            merged_ranges.push(range);
        } else {
            let last_range = merged_ranges.last_mut().unwrap();
            if range.0 <= last_range.1 {
                last_range.1 = last_range.1.max(range.1);
            } else {
                merged_ranges.push(range);
            }
        }
    }
    merged_ranges
}

pub fn run() {
    let input = helpers::read_file(Path::new("inputs/input/05"));
    let lines: Vec<String> = input.lines().map(|line| line.trim().to_string()).collect();
    let mut ids: Vec<u64> = Vec::new();
    let mut ranges: Vec<(u64, u64)> = Vec::new();
    for line in lines {
        if line.contains('-') {
            let parts: Vec<&str> = line.split('-').collect();
            let start = parts[0].parse::<u64>().unwrap();
            let end = parts[1].parse::<u64>().unwrap();
            ranges.push((start, end));
        } else if !line.is_empty() {
            let id = line.parse::<u64>().unwrap();
            ids.push(id);
        }
    }

    let mut total_fresh1 = 0;

    for id in ids {
        for (range_start, range_end) in &ranges {
            if id >= *range_start && id <= *range_end {
                total_fresh1 += 1;
                break;
            }
        }
    }

    println!("Day 05 Part 1: {}", total_fresh1);
    let ranges = merge_ranges(ranges.clone());
    let mut total_fresh2 = 0u64;
    for (range_start, range_end) in ranges {
        total_fresh2 += range_end - range_start + 1;
    }
    println!("Day 05 Part 2: {}", total_fresh2);
}


