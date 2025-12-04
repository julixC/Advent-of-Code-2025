use crate::days;
use std::io::BufRead;
use std::io;

pub fn part1() -> io::Result<()> {
    let reader = days::load_input_reader(3)?;

    let mut result = 0;

    for line in reader.lines() {
        let line = line?;

        // Create a vector that saves the biggest number yet from right to left

        let mut prefix: Vec<u32> = vec![0; line.len()];
        prefix[line.len() - 1] = line.chars().last().unwrap().to_digit(10).unwrap();

        let size = line.len() - 1;

        for i in (0..size).rev() {
            let char = line.chars().nth(i).unwrap();
            let num: u32 = char.to_digit(10).unwrap();

            if num > prefix[i + 1] {
                prefix[i] = num;
            } else {
                prefix[i] = prefix[i + 1];
            }
        }

        let mut biggest = 0;

        for i in 0..size {
            let char = line.chars().nth(i).unwrap();
            let num: u32 = char.to_digit(10).unwrap() * 10 + prefix[i + 1];

            if num > biggest {
                biggest = num;
            }
        }

        result += biggest;
    }

    println!("Result: {}", result);
    Ok(())
}

pub fn part2() -> io::Result<()> {
    let reader = days::load_input_reader(3)?;

    let mut result = 0;

    for line in reader.lines() {
        let line: Vec<u32> = {
            let temp = line?;

            let converted: Vec<u32> = temp
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect();
            
            converted
        };

        let mut pos_biggest = 0;
        let mut result_vec: Vec<u32> = vec![];
        
        while pos_biggest < line.len() - 12 + result_vec.len() && result_vec.len() < 12 {
            for i in pos_biggest..line.len() - 12 + result_vec.len() + 1 {
                if line[i] > line[pos_biggest] {
                    pos_biggest = i;
                }
            }
            result_vec.push(line[pos_biggest]);
            pos_biggest +=1;
        }

        let new_line = line[pos_biggest..].to_vec();
        let mut activation: Vec<bool> = vec![false; new_line.len()];

        for i in (0..new_line.len()).rev() {
            if result_vec.len() == 12{
                break;
            }

            if i + result_vec.len() + 12 >= new_line.len() {
                activation[i] = true;
            }
            else{
                let mut lower_active_pos = i + 1;

                for j in (i + 1)..new_line.len() {
                    if new_line[j] < new_line[lower_active_pos as usize] && activation[j] {
                        lower_active_pos = j as usize;
                    }
                }

                if new_line[i] >= new_line[lower_active_pos as usize] {
                    activation[i] = true;
                    activation[lower_active_pos as usize] = false;
                }
            }
        }

        for i in 0..new_line.len() {
            if activation[i] {
                result_vec.push(new_line[i]);
            }
        }

        let string_number = result_vec.iter().map(|n| n.to_string()).collect::<String>();
        let number = string_number.parse::<u64>().unwrap();

        result += number;
        println!("Number formed: {}", number);
    }

    println!("Result: {}", result);

    Ok(())
}