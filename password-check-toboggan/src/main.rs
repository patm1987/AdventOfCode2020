fn main() {
    println!("Hello, world!");
}

struct ParsedInput {
    index0: i32,
    index1: i32,
    letter: char,
    password: String,
}

fn parse_line(line: &str) -> Result<ParsedInput, &'static str> {
    let r = regex::Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").expect("Malformed regex");
    match r.captures(line) {
        Some(cap) => {
            Ok(ParsedInput {
                index0: cap[1].parse().unwrap(),
                index1: cap[2].parse().unwrap(),
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
            let chars = parsed.password.chars().collect::<Vec<char>>();
            return chars[(parsed.index0 - 1) as usize] == parsed.letter || chars[(parsed.index1 - 1) as usize] == parsed.letter;
        }
        Err(_) => false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenizes_min() {
        let input = "1-3 a: abcde";
        let parsed = parse_line(input).expect("Failed to parse");
        assert_eq!(1, parsed.index0);
    }

    #[test]
    fn test_tokenizes_max() {
        let input = "1-3 a: abcde";
        let parsed = parse_line(input).expect("Failed to parse");
        assert_eq!(3, parsed.index1);
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
    fn handles_empty_input() {
        let input = "";
        assert!(!validate_line(input))
    }

    #[test]
    fn handles_first_index() {
        let input = "1-3 a: abc";
        assert!(validate_line(input));
    }

    #[test]
    fn handles_second_index() {
        let input = "1-3 a: cba";
        assert!(validate_line(input));
    }
}
