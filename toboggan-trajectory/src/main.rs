use crate::map::build_map;
use std::{env, fs};

mod map;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please specify your input");
        return;
    }

    let file: &str = &args[1];
    let input_map = fs::read_to_string(file).expect("Failed to read input");
    let count = trees_hit(1, 3, &input_map);

    println!("hit {} trees", count);
}

fn trees_hit(dx: i32, dy: i32, map: &str) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let parsed_map = build_map(map);
    let mut count = 0;
    loop {
        if parsed_map.get(x, y) == '#' {
            count += 1;
        }
        x += dx;
        y += dy;
        if y >= parsed_map.height() { break; }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle_validation() {
        let input = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

        assert_eq!(7, trees_hit(3, 1, input))
    }

    #[test]
    fn hits_one_tree() {
        let input = "....
...#";
        assert_eq!(1, trees_hit(3, 1, input))
    }
}