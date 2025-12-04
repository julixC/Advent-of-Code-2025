use crate::days;
use std::io::BufRead;
use std::io;

fn check_surroundings(matrix: &Vec<Vec<char>>, i: isize, j: isize) -> i32 {
    let directions = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
    let mut count = 0;

    for (di, dj) in directions.iter() {
        let ni = i + di;
        let nj = j + dj;

        if ni >= 0 && ni < matrix.len() as isize && nj >= 0 && nj < matrix[0].len() as isize {
            if matrix[ni as usize][nj as usize] == '@' {
                count += 1;
            }
        }
    }

    count
}

pub fn part1() -> io::Result<()> {
    let reader = days::load_input_reader(4)?;
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let mut row: Vec<char> = Vec::new();
        for char in line.chars() {
            row.push(char);
        }
        matrix.push(row);
    }

    let mut result = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] != '@' {
                continue;
            }

            result += if check_surroundings(&matrix, i as isize, j as isize) < 4 { 1 } else { 0 };
        }
    }

    println!("Result: {}", result);

    Ok(())
}

pub fn part2() -> io::Result<()> {
    let reader = days::load_input_reader(4)?;
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let mut row: Vec<char> = Vec::new();
        for char in line.chars() {
            row.push(char);
        }
        matrix.push(row);
    }

    for liinha in &matrix {
        for c in liinha {
            print!("{} ", c);
        }
        println!();
    }

    let mut count_matrix: Vec<Vec<i32>> = vec![vec![0; matrix[0].len()]; matrix.len()];

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            count_matrix[i][j] = check_surroundings(&matrix, i as isize, j as isize);
        }
    }

    let mut result = 0;
    let directions = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

    // Naive approach (not efficient, but works for the input size)
    for _ in 0..50 {
        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                if count_matrix[i][j] < 4 && matrix[i][j] == '@' {
                    result += 1;
                    matrix[i][j] = '.';

                    for (di, dj) in directions.iter() {
                        let ni = i as isize + di;
                        let nj = j as isize + dj;

                        if ni >= 0 && ni < matrix.len() as isize && nj >= 0 && nj < matrix[0].len() as isize {
                            if matrix[ni as usize][nj as usize] == '@' {
                                count_matrix[ni as usize][nj as usize] -= 1;
                            }
                        }
                    }
                }
            }
        }

        for liinha in &matrix {
            for c in liinha {
                print!("{} ", c);
            }
            println!();
        }
    }


    println!("Result: {}", result);
    println!("Lines: {}, Cols: {}", matrix.len(), matrix[0].len());

    Ok(())
}