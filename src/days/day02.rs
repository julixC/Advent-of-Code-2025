use crate::days;
use std::io::BufRead;
use std::io;
use std::collections::{HashSet};

pub fn part1() -> io::Result<()> {
    let reader = days::load_input_reader(2)?;
    let line = reader.lines().next().unwrap()?;
    let values = line.split(",");

    // Parse only ranges with even number of digits
    let ranges: Vec<(u128, u128)> = values.into_iter().filter_map(|v| {
        let bounds: Vec<u128> = v.split("-")
            .map(|b| b.parse::<u128>().unwrap())
            .collect();

        let mut b0 = bounds[0];
        let mut b1 = bounds[1];

        let length_b0 = b0.to_string().len() as u32;
        let length_b1 = b1.to_string().len() as u32;

        if length_b0 % 2 != 0 {
            b0 = 10u128.pow(length_b0);
        }

        if length_b1 % 2 != 0 {
            b1 = 10u128.pow(length_b1 - 1) - 1;
        }
        
        if b0 > b1 {
            None
        } else {
            Some((b0, b1))
        }
    }).collect();

    // Sum total of every invalid id on ranges

    let mut total = 0;

    for (min, max) in ranges{
        let range: (u128, u128) = {
            let min_converted = min.to_string();
            let max_converted = max.to_string();

            let min_half_length = min_converted.len() / 2;
            let left_part = &min_converted[..min_half_length];
            
            let max_half_length = max_converted.len() / 2;
            let right_part = &max_converted[..max_half_length];

            let left_min = left_part.parse::<u128>().unwrap();
            let right_min = right_part.parse::<u128>().unwrap();
            (left_min, right_min)
        };

        for number in range.0..=range.1 {
            let candidate = format!("{}{}", number, number);
            let candidate_parsed = candidate.parse::<u128>().unwrap();

            if candidate_parsed >= min && candidate_parsed <= max {
                println!("Invalid id: {}", candidate_parsed);
                total += candidate_parsed;
            }
        }
    }

    print!("{}", total);

    Ok(())
}

fn split_ranges(min: u128, max: u128) -> Vec<(u128, u128)> {
    let mut ranges = Vec::new();
    let min_len = min.to_string().len();
    let max_len = max.to_string().len();

    if min_len == max_len {
        ranges.push((min, max));
    } else {
        // First range
        let first_range_max = 10u128.pow(min_len as u32) - 1;
        ranges.push((min, first_range_max));

        // Middle ranges
        for length in (min_len + 1)..max_len {
            let range_min = 10u128.pow(length as u32 - 1);
            let range_max = 10u128.pow(length as u32) - 1;
            ranges.push((range_min, range_max));
        }

        // Last range
        let last_range_min = 10u128.pow(max_len as u32 - 1);
        ranges.push((last_range_min, max));
    }

    ranges
}

pub fn part2() -> io::Result<()> {
    let reader = days::load_input_reader(2)?;
    let line = reader.lines().next().unwrap()?;
    let values = line.split(",");
    let divisors = vec![vec![], vec![1], vec![1], vec![1, 2], vec![1], vec![1, 2, 3], vec![1], vec![1, 2, 4], vec![1, 3], vec![1, 2, 5]];

    // Parse ranges
    let ranges: Vec<(u128, u128)> = values.into_iter().flat_map(|v| {
        let bounds: Vec<u128> = v.split("-")
            .map(|b| b.parse::<u128>())
            .filter_map(Result::ok)
            .collect();
        
        split_ranges(bounds[0], bounds[1]).into_iter()
    }).collect();

    // Sum total of every invalid id on ranges

    let mut invalid_ids = HashSet::new();

    for (min, max) in ranges{
        let min_converted = min.to_string();
        let max_converted = max.to_string();
        
        let sizes: HashSet<u128> = {
            let mut set = HashSet::new();
            for size in divisors[min.to_string().len()-1 as usize].iter() {
                    set.insert(*size);
            }
            for size in divisors[max.to_string().len()-1 as usize].iter() {
                    set.insert(*size);
            }
            set
        };

        for size in sizes {
            let (local_min, local_max) = {
                let left_part = &min_converted[..size as usize];
                let right_part = &max_converted[..size as usize];
                (left_part.parse::<u128>().unwrap(), right_part.parse::<u128>().unwrap())
            };

            for number in local_min..=local_max {                

                let number_of_repeats = min_converted.len() / size as usize;

                let candidate = number.to_string().repeat(number_of_repeats);

                let candidate_parsed = candidate.parse::<u128>().unwrap();

                if candidate_parsed >= min && candidate_parsed <= max {
                    invalid_ids.insert(candidate_parsed);
                }
            }
        }
    }

    let total: u128 = invalid_ids.iter().sum::<u128>();
    println!("{}", total);

    Ok(())
}