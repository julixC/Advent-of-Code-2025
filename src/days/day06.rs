use crate::days;
use std::io::{self, BufRead};

pub fn part1() -> io::Result<()> {
    let reader = days::load_input_reader(6)?;
    let lines = reader.lines();

    let mut numbers_matrix: Vec<Vec<i128>> = Vec::new();
    let mut operators: Vec<String> = Vec::new();
    let mut checking_operators = false;

    for line in lines {
        let line = line?;
        
        if line.is_empty() {
            checking_operators = true;
            continue;
        }

        if checking_operators {
            operators = line
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();
        } else {
            let row: Vec<i128> = line
                .split_whitespace()
                .filter_map(|s| s.parse::<i128>().ok())
                .collect();
            numbers_matrix.push(row);
        }
    }

    let mut result_final = 0;

    for i in 0..numbers_matrix[0].len() {
        let operator = &operators[i];
        let mut result = if operator == "+" { 0 } else { 1 };
        for j in 0..numbers_matrix.len() {
            match operator.as_str() {
                "+" => result += numbers_matrix[j][i],
                "*" => result *= numbers_matrix[j][i],
                _ => panic!("Invalid operator"),
            }
        }
        result_final += result;
    }

    println!("Final Result: {}", result_final);

    Ok(())
}

// ATTENTION: For this to work you need to change the 
// input file to have the operators in the first line
pub fn part2() -> io::Result<()> {
    let reader = days::load_input_reader(6)?;
    let mut lines = reader.lines();

    let operators_line = lines.next().unwrap().unwrap();

    let mut operators_and_spaces: Vec<(char, usize)> = Vec::new();

    let mut count = 0;
    let mut previous_char = ' ';
    for c in operators_line.chars() {
        if previous_char == ' ' && c != ' '{
            previous_char = c;
            continue;
        }

        if c != ' ' {
            operators_and_spaces.push((previous_char, count));
            count = 0;
            previous_char = c;
        }
        else {
            count += 1;
        }
    }
    
    operators_and_spaces.push((previous_char, count));

    let mut input: Vec<String> = Vec::new();
    
    for line in lines {
        let line = line?;
        input.push(line);
    }

    let mut start = 0;
    let mut final_result = 0;
    for (operator, space) in &operators_and_spaces {
        let mut result = if *operator == '+' { 0 } else { 1 };

        for i in start..start + space {
            let mut number = String::new();
            for line in &input {
                number.push(line.chars().nth(i).unwrap());
            }

            let trimmed = number.trim();
            if trimmed.is_empty() {
                continue;
            }

            if *operator == '+' {
                result += trimmed.parse::<i128>().unwrap();
            } else {
                result *= trimmed.parse::<i128>().unwrap();
            }
        }
        start += space + 1;
        final_result += result;
    }

    println!("Final Result: {}", final_result - 1);
    Ok(())
}