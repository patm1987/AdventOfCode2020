use crate::map::{build_map, Map};
use std::{env, fs};

mod map;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please specify your input");
        return;
    }

    let file: &str = &args[1];
    let input_map = build_map(&fs::read_to_string(file).expect("Failed to read input"));
    let count = trees_hit(3, 1, &input_map);

    println!("hit {} trees", count);
}

fn trees_hit(dx: i32, dy: i32, map: &Map) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let mut count = 0;
    loop {
        if map.get(x, y) == '#' {
            count += 1;
        }
        x += dx;
        y += dy;
        if y >= map.height() { break; }
    }
    count
}

fn trees_hit_mul(deltas: &[(i32, i32)], map: &Map) -> i32 {
    0
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

        assert_eq!(7, trees_hit(3, 1, &build_map(input)))
    }

    #[test]
    fn hits_one_tree() {
        let input = "....
...#";
        assert_eq!(1, trees_hit(3, 1, &build_map(input)))
    }

    #[test]
    fn puzzle_validation_2() {
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

        assert_eq!(336, trees_hit_mul(&[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)], &build_map(input)))
    }
}