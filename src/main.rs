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

#[cfg(test)]

mod tests {

    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]

    fn test_counter_with_empty_file() {
        let file = NamedTempFile::new().unwrap();

        let path = file.path().to_str().unwrap();

        let result = counter(path).unwrap();

        assert_eq!(result, (0, 0, 0));
    }

    #[test]

    fn test_counter_with_single_line() {
        let file = NamedTempFile::new().unwrap();

        let path = file.path().to_str().unwrap();

        writeln!(file.as_file(), "Hello, world!").unwrap();

        let result = counter(path).unwrap();

        assert_eq!(result, (1, 2, 14));
    }

    #[test]

    fn test_counter_with_multiple_lines() {
        let file = NamedTempFile::new().unwrap();

        let path = file.path().to_str().unwrap();

        writeln!(file.as_file(), "Hello, world!").unwrap();
        writeln!(file.as_file(), "This is a test.").unwrap();

        let result = counter(path).unwrap();

        assert_eq!(result, (2, 6, 30));
    }

    #[test]

    fn test_counter_with_whitespace() {
        let file = NamedTempFile::new().unwrap();

        let path = file.path().to_str().unwrap();

        writeln!(file.as_file(), "   ").unwrap();
        writeln!(file.as_file(), "Hello, world!").unwrap();

        let result = counter(path).unwrap();

        assert_eq!(result, (2, 2, 18));
    }

    #[test]

    fn test_counter_with_special_characters() {
        let file = NamedTempFile::new().unwrap();

        let path = file.path().to_str().unwrap();
        writeln!(file.as_file(), "Hello, world! @2023").unwrap();

        let result = counter(path).unwrap();
        assert_eq!(result, (1, 3, 20));
    }
}
