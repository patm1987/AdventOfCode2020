fn main() {
    println!("Hello, world!");
}

fn trees_hit(dx: i32, dy: i32, map: &str) -> i32{
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

        assert_eq!(7, trees_hit(3, 1, input))
    }
}