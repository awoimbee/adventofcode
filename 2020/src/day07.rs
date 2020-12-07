use std::collections::HashMap;

const INPUT: &str = unsafe { std::str::from_utf8_unchecked(include_bytes!("../input/day07.txt")) };

type BagMap = HashMap<&'static str, Vec<(&'static str, u16)>>;

fn parse() -> BagMap {
    let mut bags = HashMap::new();

    for line in INPUT.split('\n') {
        if line.is_empty() {
            break;
        }
        let mut tmp = line.split(" bags contain ");
        let parent_color = tmp.next().unwrap();
        let children_str = tmp.next().unwrap();
        if children_str.starts_with('n') {
            continue; // 'no other bags.'
        }
        for child_str in children_str.split(", ") {
            let first_non_nb = child_str.find(|c: char| !c.is_numeric()).unwrap();
            let color_end = child_str.rfind("bag").unwrap();
            let nb = child_str[..first_non_nb].parse::<u16>().unwrap();
            let color = &child_str[first_non_nb + 1..color_end - 1];
            bags.entry(parent_color)
                .or_insert_with(Vec::new)
                .push((color, nb));
        }
    }
    bags
}

fn contains_bag(bag: &str, color: &str, bag_map: &BagMap) -> bool {
    let v = match bag_map.get(bag) {
        Some(v) => v,
        None => return false,
    };
    for (b, _) in v {
        if *b == color || contains_bag(b, color, bag_map) {
            return true;
        }
    }
    false
}

fn part1(bags: &BagMap) {
    let mut sum = 0;
    for &k in bags.keys() {
        if contains_bag(k, "shiny gold", bags) {
            sum += 1;
        }
    }
    println!("Part 1: {}", sum);
}

fn part2(bags: &BagMap, parent_bag: &str) -> usize {
    let mut count = 1;
    let children = match bags.get(parent_bag) {
        Some(v) => v,
        None => return count,
    };
    for (bag, nb) in children {
        count += *nb as usize * part2(bags, bag);
    }
    count
}

pub fn day07() {
    let bag_map = parse();
    part1(&bag_map);
    println!("Part 2: {}", part2(&bag_map, "shiny gold") - 1);
}
