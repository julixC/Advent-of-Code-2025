use crate::days;
use std::io::{self, BufRead};
use std::collections::HashSet;

fn bfs_split(matrix: &Vec<Vec<char>>, start: (usize, usize), result: &mut i32) {
    let mut queue: Vec<(usize, usize)> = Vec::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    queue.push(start);
    while !queue.is_empty() {
        let (x, y) = queue.remove(0);
        if visited.contains(&(x, y)) {
            continue;
        }
        visited.insert((x, y));

        if x < matrix.len() && y < matrix[0].len() {
            if x + 1 < matrix.len() && matrix[x + 1][y] == '.' {
                queue.push((x + 1, y));
            }
            else if x + 1 < matrix.len() && matrix[x + 1][y] == '^' {
                *result += 1;
                if y > 0 && matrix[x + 1][y - 1] == '.' {
                    queue.push((x + 1, y - 1));
                }
                if y + 1 < matrix[0].len() && matrix[x + 1][y + 1] == '.' {
                    queue.push((x + 1, y + 1));
                }
            }
        }
    }
}

pub fn part1() -> io::Result<()> {

    let reader = days::load_input_reader(7)?;
    let lines = reader.lines();

    let mut start = (0, 0);
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for line in lines {
        let line = line?;
        for i in 0..line.len() {
            if line.chars().nth(i).unwrap() == 'S' {
                start = (0, i);
            }
        }
        matrix.push(line.chars().collect());
    }

    let mut result = 0;

    bfs_split(&matrix, start, &mut result);

    println!("Result: {}", result);

    Ok(())
}

fn dfs_paths(matrix: &Vec<Vec<char>>, pos: (usize, usize), visited: &mut HashSet<(usize, usize)>, matrix_count: &mut Vec<Vec<i128>>) {
    if visited.contains(&pos) {
        return;
    }
    visited.insert(pos);

    if pos.0 == matrix.len() - 1 {
        matrix_count[pos.0][pos.1] = 1;
        return;
    }

    if pos.0 + 1 < matrix.len() && pos.1 < matrix[0].len() {

        if pos.0 + 1 < matrix.len() && matrix[pos.0 + 1][pos.1] == '.' {
            dfs_paths(matrix, (pos.0 + 1, pos.1), visited, matrix_count);

            matrix_count[pos.0][pos.1] += matrix_count[pos.0 + 1][pos.1];
        }
        else if pos.0 + 1 < matrix.len() && matrix[pos.0 + 1][pos.1] == '^' {
            if pos.1 > 0 && matrix[pos.0 + 1][pos.1 - 1] == '.' {
                dfs_paths(matrix, (pos.0 + 1, pos.1 - 1), visited, matrix_count);
                matrix_count[pos.0][pos.1] += matrix_count[pos.0 + 1][pos.1 - 1];
            }
            if pos.1 + 1 < matrix[0].len() && matrix[pos.0 + 1][pos.1 + 1] == '.' {
                dfs_paths(matrix, (pos.0 + 1, pos.1 + 1), visited, matrix_count);
                matrix_count[pos.0][pos.1] += matrix_count[pos.0 + 1][pos.1 + 1];
            }
        }
    }
}

pub fn part2() -> io::Result<()> {
    let reader = days::load_input_reader(7)?;
    let lines = reader.lines();

    let mut start = (0, 0);
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for line in lines {
        let line = line?;
        for i in 0..line.len() {
            if line.chars().nth(i).unwrap() == 'S' {
                start = (0, i);
            }
        }
        matrix.push(line.chars().collect());
    }

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut matrix_count: Vec<Vec<i128>> = vec![vec![0; matrix[0].len()]; matrix.len()];

    dfs_paths(&matrix, start, &mut visited, &mut matrix_count);

    println!("Result: {}", matrix_count[start.0][start.1]);

    Ok(())
}