use crate::Instruction::{Nop, Jmp, Acc};

fn main() {
    println!("Hello, world!");
}

#[derive(PartialEq, Eq, Debug)]
enum Instruction {
    Nop,
    Jmp(i32),
    Acc(i32),
}

fn parse_line(line: &str) -> Result<Instruction, &'static str> {
    let instruction = &line[..3];
    let value = &line[4..];
    match instruction {
        "nop" => { Ok(Nop) }
        "jmp" => { Ok(Jmp(value.parse::<i32>().expect("Failed to parse jmp value"))) }
        "acc" => { Ok(Acc(value.parse::<i32>().expect("Failed to parse acc value"))) }
        _ => { Err("unknown instruction") }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Instruction::{Jmp, Acc};

    const SAMPLE_INPUT: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn test_parses_noop() {
        assert_eq!(Nop, parse_line("nop +0").unwrap());
    }

    #[test]
    fn test_parses_jmp_positive() {
        assert_eq!(Jmp(1337), parse_line("jmp +1337").unwrap())
    }

    #[test]
    fn test_parses_jmp_negative() {
        assert_eq!(Jmp(-42), parse_line("jmp -42").unwrap())
    }

    #[test]
    fn test_parses_acc() {
        assert_eq!(Acc(42), parse_line("acc +42").unwrap());
        assert_eq!(Acc(-1337), parse_line("acc -1337").unwrap());
    }
}
