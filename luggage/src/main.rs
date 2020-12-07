fn main() {
    println!("Hello, world!");
}

#[derive(PartialEq, Debug)]
struct Bag {
    name: String,
    bags: Vec<(i32, String)>,
}

impl Bag {
    fn new(name: &str, bags: &[(i32, &str)]) -> Bag {
        Bag {
            name: String::from(name),
            bags: bags.iter().map(|x| (x.0, String::from(x.1))).collect(),
        }
    }
    fn parse(input: String) -> Bag {
        Bag { name: String::from(""), bags: Vec::new() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    #[test]
    fn test_parses_bag() {
        let sample_input = "light red bags contain 1 bright white bag, 2 muted yellow bags.";
        let expected = Bag::new("light red", &[(1, "bright white"), (2, "muted yellow")]);
        assert_eq!(expected, Bag::parse(String::from(sample_input)));
    }
}
