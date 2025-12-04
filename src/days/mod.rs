use std::fs::File;
use std::io::{self, BufReader};

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
// ... add more days as needed

pub fn load_input_reader(day: u8) -> io::Result<BufReader<File>> {
    let path = format!("data/day{:02}.txt", day);
    let file = File::open(path)?;
    Ok(BufReader::new(file))
}

pub fn run(day: u8, part: u8) -> Option<String> {
    match (day, part) {
        (1, 1) => match day01::part1() {
            Ok(_) => Some(String::from("Part 1 completed successfully.")),
            Err(e) => Some(format!("Error: {}", e)),
        },
        (1, 2) => match day01::part2() {
            Ok(_) => Some(String::from("Part 2 completed successfully.")),
            Err(e) => Some(format!("Error: {}", e)),
        },
        (2, 1) => match day02::part1() {
            Ok(_) => Some(String::from("Part 1 completed successfully.")),
            Err(e) => Some(format!("Error: {}", e)),
        },
        (2, 2) => match day02::part2() {
            Ok(_) => Some(String::from("Part 2 completed successfully.")),
            Err(e) => Some(format!("Error: {}", e)),
        },
        (3, 1) => match day03::part1() {
            Ok(_) => Some(String::from("Part 1 completed successfully.")),
            Err(e) => Some(format!("Error: {}", e)),
        },
        (3, 2) => match day03::part2() {
            Ok(_) => Some(String::from("Part 2 completed successfully.")),
            Err(e) => Some(format!("Error: {}", e)),
        },
        (4, 1) => match day04::part1() {
            Ok(_) => Some(String::from("Part 1 completed successfully.")),
            Err(e) => Some(format!("Error: {}", e)),
        },
        (4, 2) => match day04::part2() {
            Ok(_) => Some(String::from("Part 2 completed successfully.")),
            Err(e) => Some(format!("Error: {}", e)),
        },
        // Add more days and parts as needed
        _ => None,
    }
}