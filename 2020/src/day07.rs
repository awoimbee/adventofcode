use fnv::FnvHashMap;
use fnv::FnvHasher;
use std::hash::Hasher;

const INPUT: &str = include_str!("../input/day07.txt");

type BagMap = FnvHashMap<u64, Vec<(u64, u16)>>;

fn hash_str(s: &str) -> u64 {
    let mut hasher = FnvHasher::default();
    hasher.write(s.as_bytes());
    hasher.finish()
}

fn parse() -> BagMap {
    let mut bags = BagMap::default();

    for line in INPUT.lines() {
        let mut tmp = line.split(" bags contain ");
        let parent_color = tmp.next().unwrap();
        let parent_color_hash = hash_str(parent_color);
        let children_str = tmp.next().unwrap();
        if children_str.starts_with('n') {
            continue; // 'no other bags.'
        }

        for child_str in children_str.split(", ") {
            let first_non_nb = child_str.find(|c: char| !c.is_numeric()).unwrap();
            let color_end = child_str.rfind("bag").unwrap();
            let nb = child_str[..first_non_nb].parse::<u16>().unwrap();
            let color_hash = hash_str(&child_str[first_non_nb + 1..color_end - 1]);
            bags.entry(parent_color_hash)
                .or_insert_with(Vec::new)
                .push((color_hash, nb));
        }
    }
    bags
}

fn contains_bag(bag: u64, color: u64, bag_map: &BagMap) -> bool {
    let v = match bag_map.get(&bag) {
        Some(v) => v,
        None => return false,
    };
    for (b, _) in v {
        if *b == color || contains_bag(*b, color, bag_map) {
            return true;
        }
    }
    false
}

fn part1(bags: &BagMap) -> u32 {
    let mut sum = 0;
    for &k in bags.keys() {
        if contains_bag(k, hash_str("shiny gold"), bags) {
            sum += 1;
        }
    }
    sum
}

fn part2(bags: &BagMap, parent_bag: u64) -> usize {
    let mut count = 1;
    let children = match bags.get(&parent_bag) {
        Some(v) => v,
        None => return count,
    };
    for (bag, nb) in children {
        count += *nb as usize * part2(bags, *bag);
    }
    count
}

pub fn day07() -> (String, String) {
    let bag_map = parse();
    (
        format!("{}", part1(&bag_map)),
        format!("{}", part2(&bag_map, hash_str("shiny gold")) - 1),
    )
}
