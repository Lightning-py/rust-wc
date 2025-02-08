use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::{self, BufRead};

fn counter(filename: &str) -> Result<(usize, usize, usize), io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut line_counter = 0;
    let mut word_counter = 0;
    let mut char_counter = 0;

    for line in reader.lines() {
        let line = line?;
        line_counter += 1;
        char_counter += line.chars().count() + 1; // на один больше из-за символа переноса строки
        word_counter += line.split_whitespace().count();
    }

    Ok((line_counter, word_counter, char_counter))
}

fn main() {
    // Получаем аргументы командной строки
    let args: Vec<String> = env::args().collect();

    // Если аргументов недостаточно или больше чем нужно, выходжим с ошибкой
    if args.len() != 2 {
        eprint!("Filename not specified");
        std::process::exit(1);
    }

    match counter(&args[1]) {
        Ok((line_count, word_count, char_count)) => {
            println!("Words: {}", word_count);
            println!("Lines: {}", line_count);
            println!("Chars: {}", char_count);
        }
        Err(_e) => {
            eprintln!("Can't open file: {}", args[1]);
            std::process::exit(1);
        }
    }
}
