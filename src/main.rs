mod days;
use std::io::{self, Write};

fn main() {
    // Pergunta o dia
    print!("Selecione o dia (1-25): ");
    io::stdout().flush().unwrap();

    let mut day_input = String::new();
    io::stdin().read_line(&mut day_input).unwrap();
    let day: u8 = day_input.trim().parse().expect("Dia inválido");

    // Pergunta a parte
    print!("Selecione a parte (1 ou 2): ");
    io::stdout().flush().unwrap();

    let mut part_input = String::new();
    io::stdin().read_line(&mut part_input).unwrap();
    let part: u8 = part_input.trim().parse().expect("Parte inválida");

    // Dispatch
    match days::run(day, part) {
        Some(result) => println!("\nResultado: {}", result),
        None => eprintln!("Dia ou parte não implementado."),
    }
}