use crate::days;
use std::io::{self, BufRead};

pub fn part1() -> io::Result<()> {
    let reader = days::load_input_reader(5)?;
    let lines = reader.lines();
    
    let mut ranges: Vec<(i128, i128)> = Vec::new();
    let mut numbers_to_check: Vec<i128> = Vec::new();
    let mut checking_numbers = false;
    let mut result = 0;

    for line in lines {
        let line = line?;

        if line.is_empty() {
            checking_numbers = true;
            continue;
        }

        if checking_numbers {
            numbers_to_check.push(line.parse::<i128>().unwrap());
            continue;
        }

        let mut range = line.split("-");
        ranges.push((range.next().unwrap().parse::<i128>().unwrap(), range.next().unwrap().parse::<i128>().unwrap()));
    }

    ranges.sort_by_key(|r| r.0);

    for number in numbers_to_check {
        for range in &ranges {
            if range.0 <= number && range.1 >= number {
                result += 1;
                break;
            }
        }
    }

    println!("Result: {}", result);

    Ok(())
}

pub fn part2() -> io::Result<()> {
    let reader = days::load_input_reader(5)?;
    let lines = reader.lines();
    
    let mut ranges: Vec<(i128, i128)> = Vec::new();
    let mut result = 0;

    for line in lines {
        let line = line?;

        if line.is_empty() {
            break;
        }

        let mut range = line.split("-");
        ranges.push((range.next().unwrap().parse::<i128>().unwrap(), range.next().unwrap().parse::<i128>().unwrap()));
    }

    ranges.sort_by_key(|r| r.0);

    let mut inicial_range = ranges[0].0;
    let mut final_range = ranges[0].1;

    for range in &ranges {
        if range.0 > final_range {
            result += final_range - inicial_range + 1;
            inicial_range = range.0;
            final_range = range.1;
        } else {
            if range.1 > final_range {
                final_range = range.1;
            }
        }
    }

    result += final_range - inicial_range + 1;

    println!("Result: {}", result);
    Ok(())
}