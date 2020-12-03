use crate::map::build_map;

mod map;

fn main() {
    println!("Hello, world!");
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