use crate::lib::load_file::load_data_file;
use std::ops::Range;

use super::ExecuteResponse;

pub fn execute() -> ExecuteResponse {
    let data = load_data_file("day_4.txt")?;

    let mut overlaps = 0u32;

    let score: u32 = data
        .lines()
        .map(|l| {
            let groups = l.split(",").collect::<Vec<&str>>();
            let ranges: Vec<Range<u8>> = groups
                .iter()
                .map(|r: &&str| {
                    // Create the ranges for this line (eg. 3-4,5-99 > <Range(3..4), Range(5..99)>)
                    let range = r.split("-").collect::<Vec<&str>>();
                    let start: u8 = match range.get(0) {
                        Some(s) => match s.parse::<u8>() {
                            Ok(n) => n,
                            _ => 0,
                        },
                        _ => 0,
                    };

                    let finish: u8 = match range.get(1) {
                        Some(s) => match s.parse::<u8>() {
                            Ok(n) => n,
                            _ => 0,
                        },
                        _ => 0,
                    };

                    start..finish
                })
                .collect();

            ranges
        })
        .map(|ranges| {
            let first_range = match ranges.get(0) {
                Some(range) => range,
                _ => &(0..0),
            };

            let second_range = match ranges.get(1) {
                Some(range) => range,
                _ => &(0..0),
            };

            // Trick for pt 2 :)
            if first_range.end >= second_range.start && first_range.start <= second_range.end {
                overlaps += 1;
            }

            // If first range contains second range
            if first_range.start <= second_range.start && first_range.end >= second_range.end {
                return 1u32;
            }

            // If second range contains first range
            if second_range.start <= first_range.start && second_range.end >= first_range.end {
                return 1u32;
            }

            0u32
        })
        .sum();

    println!("part 1 - {:?}", score);
    println!("part 2 - {:?}", overlaps);

    Ok(())
}
