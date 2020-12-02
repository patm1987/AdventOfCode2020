fn main() {
    println!("Hello, world!");
}

struct ParsedInput {
    min: i32,
    max: i32,
    letter: char,
    password: String
}

fn parse_line(line: &str) -> ParsedInput {
    ParsedInput {
        min: 0,
        max: 0,
        letter: '\0',
        password: String::from(""),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenizes_min() {
        let input = "1-3 a: abcde";
        let parsed = parse_line(input);
        assert_eq!(1, parsed.min);
    }

    #[test]
    fn test_tokenizes_max() {
        let input = "1-3 a: abcde";
        let parsed = parse_line(input);
        assert_eq!(3, parsed.max);
    }

    #[test]
    fn test_tokenizes_letter() {
        let input = "1-3 a: abcde";
        let parsed = parse_line(input);
        assert_eq!('a', parsed.letter);
    }

    #[test]
    fn test_tokenizes_password() {
        let input = "1-3 a: abcde";
        let parsed = parse_line(input);
        assert_eq!("abcde", parsed.password);
    }
}