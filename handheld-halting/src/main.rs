use crate::Instruction::{Nop, Jmp, Acc};
use std::{env, fs};
use crate::ProgramResult::{Normal, Overflow, Loop};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please specify the input file");
        return;
    }

    let filename: &str = &args[1];
    let file = fs::read_to_string(filename).expect("Failed to parse file");
    let program = parse_program(&file);
    println!("Found cycle at {:?}", acc_before_loop(&program));
}

#[derive(PartialEq, Eq, Debug)]
enum Instruction {
    Nop,
    Jmp(i32),
    Acc(i32),
}

#[derive(PartialEq, Eq, Debug)]
enum ProgramResult {
    Loop(i32),
    Normal(i32),
    Overflow(i32),
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

fn parse_program(input: &str) -> Vec<Instruction> {
    input.trim().lines().filter_map(|line| parse_line(line).ok()).collect()
}

fn acc_before_loop(program: &Vec<Instruction>) -> ProgramResult {
    let mut program_state: Vec<Option<i32>> = vec![None; program.len()];
    let mut program_counter: i32 = 0;
    let mut accumulator = 0;
    while program_state[program_counter as usize] == None {
        match program[program_counter as usize] {
            Nop => {
                program_state[program_counter as usize] = Some(accumulator);
                program_counter += 1;
            }
            Jmp(amount) => {
                program_state[program_counter as usize] = Some(accumulator);
                program_counter += amount;
            }
            Acc(amount) => {
                accumulator += amount;
                program_state[program_counter as usize] = Some(accumulator);
                program_counter += 1;
            }
        }

        if program_counter == program.len() as i32 {
            return Normal(accumulator);
        } else if program_counter > program.len() as i32 {
            return Overflow(accumulator);
        }
    }
    Loop(accumulator)
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

    #[test]
    fn test_parses_sample_input() {
        let expected = vec![
            Nop,
            Acc(1),
            Jmp(4),
            Acc(3),
            Jmp(-3),
            Acc(-99),
            Acc(1),
            Jmp(-4),
            Acc(6)
        ];
        assert_eq!(expected, parse_program(SAMPLE_INPUT))
    }

    #[test]
    fn matches_sample_input() {
        assert_eq!(Loop(5), acc_before_loop(&parse_program(SAMPLE_INPUT)));
    }
}
