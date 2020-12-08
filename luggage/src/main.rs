use regex::Regex;
use std::collections::{HashMap, HashSet};

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
    fn parse(input: String) -> Result<Bag, &'static str> {
        let re_descriptor = Regex::new(r"(?P<count>\d+) (?P<name>[a-zA-Z ]+) bags?").unwrap();
        let mut split = input.split(" bags contain ");
        let name = split.next().unwrap();
        let remainder = split.next().unwrap();
        Ok(Bag {
            name: name.to_string(),
            bags: re_descriptor.captures_iter(remainder).filter_map(|x| {
                Some((x.name("count").unwrap().as_str().parse::<i32>().unwrap(), x.name("name").unwrap().as_str().to_string()))
            }).collect(),
        })
    }
}

fn parse_bags(input: &str) -> HashMap<String, Bag> {
    let mut map = HashMap::new();
    input.trim().split('\n').for_each(|x| {
        match Bag::parse(x.to_string()) {
            Ok(bag) => { map.insert(bag.name.clone(), bag); }
            Err(_) => {}
        }
    });
    map
}

fn can_contain(bag_map: &HashMap<String, Bag>, bag_name: &str) -> i32 {
    let mut count = 0;
    bag_map.iter().for_each(|(_name, bag)| {
        let mut pending: Vec<String> = Vec::new();
        let mut checked: HashSet<String> = HashSet::new();
        bag.bags.iter().for_each(|(_count, test_name)| {
            try_insert(&mut pending, &checked, test_name);
        });
        while !pending.is_empty() {
            let test_bag = pending.pop().unwrap();
            if test_bag == bag_name {
                count += 1;
                return;
            }

            checked.insert(test_bag.clone());
            bag_map.get(test_bag.as_str()).unwrap().bags.iter().for_each(|(_count, name)| try_insert(&mut pending, &checked, name));
        }
    });
    count
}

fn try_insert(pending: &mut Vec<String>, checked: &HashSet<String>, bag_name: &String) {
    if !pending.contains(bag_name) && !checked.contains(bag_name) {
        pending.push(bag_name.clone())
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
        assert_eq!(expected, Bag::parse(String::from(sample_input)).unwrap());
    }

    #[test]
    fn test_parses_to_hash() {
        let bag_map = parse_bags(SAMPLE_INPUT);
        let expected_light_red = Bag::new("light red", &[(1, "bright white"), (2, "muted yellow")]);
        let expected_dotted_black = Bag::new("dotted black", &[]);
        assert_eq!(expected_light_red, *bag_map.get("light red").unwrap());
        assert_eq!(expected_dotted_black, *bag_map.get("dotted black").unwrap());
    }

    #[test]
    fn test_handles_sample() {
        let bag_map = parse_bags(SAMPLE_INPUT);
        assert_eq!(4, can_contain(&bag_map, "shiny gold"));
    }
}
