extern crate regex;

use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Specify the file to open!");
        return;
    }
    let filename = &args[1];
    let input = fs::read_to_string(filename).expect("Failed to open file");
    let count = count_valid_lines(&input.as_str());
    println!("There are {} valid lines", count);
}

struct ParsedInput {
    min: i32,
    max: i32,
    letter: char,
    password: String,
}

fn parse_line(line: &str) -> Result<ParsedInput, &'static str> {
    let r = regex::Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").expect("Malformed regex");
    match r.captures(line) {
        Some(cap) => {
            Ok(ParsedInput {
                min: cap[1].parse().unwrap(),
                max: cap[2].parse().unwrap(),
                letter: cap[3].chars().next().unwrap(),
                password: String::from(&cap[4]),
            })
        }
        None => Err("Failed to parse")
    }
}

fn validate_line(line: &str) -> bool {
    match parse_line(line) {
        Ok(parsed) => {
            let mut sum = 0;
            for i in parsed.password.chars() {
                if i == parsed.letter {
                    sum += 1;
                }
            }
            return sum >= parsed.min && sum <= parsed.max;
        },
        Err(_) => false
    }
}

fn count_valid_lines(input: &str) -> i32 {
    let mut count = 0;
    for line in input.split('\n') {
        if validate_line(line) {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenizes_min() {
        let input = "1-3 a: abcde";
        let parsed = parse_line(input).expect("Failed to parse");
        assert_eq!(1, parsed.min);
    }

    #[test]
    fn test_tokenizes_max() {
        let input = "1-3 a: abcde";
        let parsed = parse_line(input).expect("Failed to parse");
        assert_eq!(3, parsed.max);
    }

    #[test]
    fn test_tokenizes_letter() {
        let input = "1-3 a: abcde";
        let parsed = parse_line(input).expect("Failed to parse");
        assert_eq!('a', parsed.letter);
    }

    #[test]
    fn test_tokenizes_password() {
        let input = "1-3 a: abcde";
        let parsed = parse_line(input).expect("Failed to parse");
        assert_eq!("abcde", parsed.password);
    }

    #[test]
    fn test_handles_empty_string() {
        let input = "";
        let parsed = parse_line(input);
        match parsed {
            Ok(_) => assert!(false),
            Err(_) => assert!(true)
        }
    }

    #[test]
    fn test_validates_min_occurance() {
        let input = "1-3 a: abcd";
        assert!(validate_line(input));
    }

    #[test]
    fn test_validates_max_occurance() {
        let input = "1-3 a: aaaa";
        assert!(!validate_line(input));
    }

    #[test]
    fn test_validates_example_input() {
        let input = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";
        assert_eq!(2, count_valid_lines(input));
    }

    #[test]
    fn test_validates_empty_line() {
        let input = "";
        assert!(!validate_line(input));
    }
}