use crate::days;
use std::io::BufRead;
use std::io;

pub fn part1() -> io::Result<()> {
    let reader = days::load_input_reader(1)?;

    let mut counter = 0;
    let mut curr_value = 50;

    for line in reader.lines() {
        let line = line?;

        let dir: char = line.chars().next().unwrap();
        let valor: u32 = line[1..].parse().unwrap();

        if dir == 'R'{
            curr_value = (curr_value + valor) % 100;
        }
        else {
            curr_value = (curr_value + 100 - (valor % 100)) % 100;
        }

        if curr_value == 0 {
            counter += 1;
        }
    }

    print!("{}", counter);

    Ok(())
}

pub fn part2() -> io::Result<()> {
    let reader = days::load_input_reader(1)?;

    let mut counter = 0;
    let mut curr_value = 50;

    for line in reader.lines() {
        let line = line?;

        let dir: char = line.chars().next().unwrap();
        let valor: i32 = line[1..].parse().unwrap();

        if dir == 'R'{
            let multiplier = (curr_value + valor) / 100;
            curr_value = (curr_value + valor) % 100;
            counter += multiplier;
        }
        else {
            let multiplier = if curr_value != 0 {(100 - curr_value + valor) / 100} else { valor / 100};
            curr_value = (curr_value + 100 - (valor % 100)) % 100;
            counter += multiplier;
        }
    }

    print!("{}", counter);

    Ok(())
}
