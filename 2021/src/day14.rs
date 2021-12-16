use heapless::FnvIndexMap;

const INPUT: &str = include_str!("../input/day14.txt");

struct Rule {
    pattern: [u8; 2],
    result: u8,
}

impl Rule {
    pub fn from_line(line: &str) -> Self {
        let (pattern, result) = line.split_once(" -> ").expect("invalid rule");
        let pattern = pattern.as_bytes();
        let pattern = [pattern[0], pattern[1]];
        let result = result.bytes().next().expect("invalid rule");
        Self { pattern, result }
    }
}

#[inline]
fn fnv_map_inc<K>(map: &mut FnvIndexMap<K, u64, 128>, key: &K, increment: u64)
where
    K: std::cmp::Eq + std::hash::Hash + hash32::Hash + std::fmt::Debug + Copy,
{
    if let Some(count) = map.get_mut(key) {
        *count += increment;
    } else {
        map.insert(*key, increment).unwrap();
    }
}

struct FastPolymerizer {
    polymer: FnvIndexMap<[u8; 2], u64, 128>,
    first_char: u8,
    rules: Vec<Rule>,
}

impl FastPolymerizer {
    pub fn new(input: &str) -> Self {
        let (polymer, rules) = input
            .split_once("\n\n")
            .expect("delimiter between polymer and rules not found");
        let first_char = polymer.bytes().next().expect("empty polymer");
        let mut polymer_map = FnvIndexMap::new();
        for elems in polymer.as_bytes().windows(2) {
            let key = [elems[0], elems[1]];
            fnv_map_inc(&mut polymer_map, &key, 1);
        }
        let mut rules = rules
            .split_terminator('\n')
            .map(Rule::from_line)
            .collect::<Vec<_>>();
        rules
            .iter()
            .for_each(|r| fnv_map_inc(&mut polymer_map, &r.pattern, 0));
        rules.sort_by_key(|rule| rule.pattern);
        Self {
            polymer: polymer_map,
            rules,
            first_char,
        }
    }

    pub fn next_step(&mut self) {
        let mut new_polymer = self.polymer.clone();
        for rule in &self.rules {
            let elems = self.polymer.get_mut(&rule.pattern).unwrap();
            if *elems == 0 {
                continue;
            }
            let a = [rule.pattern[0], rule.result];
            let b = [rule.result, rule.pattern[1]];
            let val = *elems;
            *new_polymer.get_mut(&rule.pattern).unwrap() -= val;
            *new_polymer.get_mut(&a).unwrap() += val;
            *new_polymer.get_mut(&b).unwrap() += val;
        }
        self.polymer = new_polymer;
    }

    pub fn run(&mut self, steps: usize) {
        for _ in 0..steps {
            self.next_step();
        }
    }

    pub fn most_common_minus_least_common(&self) -> usize {
        let mut counts = FnvIndexMap::new();
        for (elems, count) in self.polymer.iter() {
            fnv_map_inc(&mut counts, &elems[1], *count);
        }
        fnv_map_inc(&mut counts, &self.first_char, 1);

        let mut min = std::u64::MAX;
        let mut max = std::u64::MIN;

        for elem_count in counts.values().filter(|&&x| x != 0) {
            min = std::cmp::min(min, *elem_count);
            max = std::cmp::max(max, *elem_count);
        }
        (max - min) as usize
    }

    pub fn _print(&self) {
        for (elems, &count) in self.polymer.iter() {
            if count == 0 {
                continue;
            }
            print!("{}{}:{} ", elems[0] as char, elems[1] as char, count);
        }
        println!();
    }

    pub fn _print_individual(&self) {
        let mut counts = FnvIndexMap::new();
        for (elems, count) in self.polymer.iter() {
            fnv_map_inc(&mut counts, &elems[1], *count);
        }
        fnv_map_inc(&mut counts, &self.first_char, 1);

        println!("Fast polymer:");
        for (elem, count) in counts.iter() {
            print!("{}:{} ", *elem as char, count);
        }
        println!();
    }
}

pub fn day14() -> (String, String) {
    let mut parsed = FastPolymerizer::new(INPUT);
    parsed.run(10);

    let part1 = parsed.most_common_minus_least_common();
    parsed.run(30);
    let part2 = parsed.most_common_minus_least_common();

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn test_part_1_test_input() {
        let mut parsed = FastPolymerizer::new(TEST_INPUT);
        parsed.run(10);
        assert_eq!(parsed.most_common_minus_least_common(), 1588);
    }

    #[test]
    fn test_part_1() {
        let mut parsed = FastPolymerizer::new(INPUT);
        parsed.run(10);
        assert_eq!(parsed.most_common_minus_least_common(), 3906);
    }

    #[test]
    fn test_part_2_test_input() {
        let mut parsed = FastPolymerizer::new(TEST_INPUT);
        parsed.run(40);
        assert_eq!(parsed.most_common_minus_least_common(), 2188189693529);
    }
}
