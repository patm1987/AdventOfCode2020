mod report_tracker;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let filename = &args[1];
        println!("File: {}", filename);
        let input = fs::read_to_string(filename).expect("Failed to open file");
        let lines = input.split("\n");
        let mut tracker = report_tracker::build_report_tracker(2020);
        for line in lines {
            match String::from(line).parse::<i32>(){
                Ok(parsed_line) => {
                    let (match_found, mult) = tracker.add_expense(parsed_line);
                    if match_found {
                        println!("Expense: {}", mult);
                        return;
                    }
                },
                Err(e) => println!("Failed to parse {} with {}", line, e)
            }
        }
    } else {
        println!("Please specify a file to parse!");
    }
}
