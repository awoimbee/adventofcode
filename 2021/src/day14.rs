const INPUT: &str = include_str!("../input/day14.txt");

struct Rule {
    pattern: [u8; 2],
    result: u8,
}

// This is the dumb implementation
// I should store every pair possible and give them a count
struct Polymeriser {
    polymer: Vec<u8>,
    rules: Vec<Rule>,
}

impl Polymeriser {
    pub fn new(input: &str) -> Self {
        let (polymer, rules) = input
            .split_once("\n\n")
            .expect("delimiter between polymer and rules not found");
        let polymer = polymer.bytes().collect::<Vec<_>>();
        let mut rules = rules
            .split_terminator('\n')
            .map(|line| {
                let (pattern, result) = line.split_once(" -> ").expect("invalid rule");
                let pattern = pattern.as_bytes();
                let pattern = [pattern[0], pattern[1]];
                let result = result.bytes().next().expect("invalid rule");
                Rule { pattern, result }
            })
            .collect::<Vec<_>>();
        rules.sort_by_key(|rule| rule.pattern);
        Self { polymer, rules }
    }

    pub fn next_step(&mut self) {
        let mut new_polymer = Vec::with_capacity(self.polymer.len() * 2);
        self.polymer.push(b'_'); // dummy for iter::windows()
        for pair in self.polymer.windows(2) {
            new_polymer.push(pair[0]);
            for rule in &self.rules {
                if pair == rule.pattern {
                    new_polymer.push(rule.result);
                    break;
                }
            }
        }
        self.polymer = new_polymer;
    }

    pub fn run(&mut self, steps: usize) {
        for _ in 0..steps {
            self.next_step();
        }
    }

    pub fn _as_str(&self) -> &str {
        std::str::from_utf8(&self.polymer).unwrap()
    }

    pub fn most_common_minus_least_common(&self) -> usize {
        let mut elems = [0; 27];
        self.polymer
            .iter()
            .for_each(|elem| elems[(elem - b'A') as usize] += 1);
        let most_common_elem = elems.iter().max().unwrap();
        let least_common_elem = elems.iter().filter(|&&x| x != 0).min().unwrap();
        most_common_elem - least_common_elem
    }
}

pub fn day14() -> (String, String) {
    let mut parsed = Polymeriser::new(INPUT);
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
    fn test_next_step_test_input() {
        let mut parsed = Polymeriser::new(TEST_INPUT);
        parsed.next_step();
        assert_eq!(parsed._as_str(), "NCNBCHB");
        parsed.next_step();
        assert_eq!(parsed._as_str(), "NBCCNBBBCBHCB");
        parsed.next_step();
        assert_eq!(parsed._as_str(), "NBBBCNCCNBBNBNBBCHBHHBCHB");
        parsed.next_step();
        assert_eq!(
            parsed._as_str(),
            "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB"
        );
        parsed.next_step();
        assert_eq!(parsed._as_str().len(), 97);
        for _ in 0..5 {
            parsed.next_step();
        }
        assert_eq!(parsed._as_str().len(), 3073);
    }

    #[test]
    fn test_run_test_input() {
        let mut parsed = Polymeriser::new(TEST_INPUT);
        parsed.run(4);
        assert_eq!(
            parsed._as_str(),
            "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB"
        );
    }

    #[test]
    fn test_part_1_test_input() {
        let mut parsed = Polymeriser::new(TEST_INPUT);
        parsed.run(10);
        assert_eq!(parsed.most_common_minus_least_common(), 1588);
    }

    #[test]
    fn test_part_1() {
        let mut parsed = Polymeriser::new(INPUT);
        parsed.run(10);
        assert_eq!(parsed.most_common_minus_least_common(), 3906);
    }

    #[test]
    fn test_part_2_test_input() {
        let mut parsed = Polymeriser::new(TEST_INPUT);
        parsed.run(40);
        assert_eq!(parsed.most_common_minus_least_common(), 2188189693529);
    }
}
