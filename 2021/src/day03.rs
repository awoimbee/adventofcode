use heapless::Vec;

const INPUT: &str = include_str!("../input/day03.txt");
const MAX_INPUT_LINES: usize = 1000;

struct ParsedDay03 {
    bits: Vec<i32, MAX_INPUT_LINES>,
    bits_len: usize,
}

impl ParsedDay03 {
    pub fn new(input: &str) -> Self {
        let bits_len = input.find('\n').unwrap();
        let res = input
            .split_terminator('\n')
            .map(|line| i32::from_str_radix(line, 2).unwrap())
            .collect::<Vec<_, MAX_INPUT_LINES>>();
        Self {
            bits: res,
            bits_len,
        }
    }
}

fn part1(input: &ParsedDay03) -> String {
    let most_common = input
        .bits
        .iter()
        .fold(vec![0i32; input.bits_len], |acc, x| {
            acc.iter().zip(bit_iter(*x)).map(|(a, b)| a + b).collect()
        });
    let gamma_rate = vec2nb(&most_common);
    let epsilon_rate = (!gamma_rate) & !(i32::MAX << input.bits_len);

    (gamma_rate * epsilon_rate).to_string()
}

fn bit_iter(nb: i32) -> impl Iterator<Item = i32> {
    (0..i32::BITS)
        .into_iter()
        .map(move |x| ((nb >> x) & 1) * 2 - 1)
}

fn vec2nb(bits: &[i32]) -> i32 {
    bits.iter()
        .enumerate()
        .fold(0, |acc, (i, x)| acc | if *x > 0 { 1 << i } else { 0 })
}

fn is_bit_set(nb: i32, i: usize) -> bool {
    (nb >> i & 1) == 1
}

// lots to optimize here
fn part2(input: &ParsedDay03) -> String {
    let mut indexes = (0..input.bits.len()).collect::<Vec<_, MAX_INPUT_LINES>>();
    let mut pos = 0;
    while indexes.len() > 1 {
        // println!("{:?}", indexes.iter().map(|i| format!("{:#07b}", input.bits[*i])).collect::<std::vec::Vec<_>>());
        let one = indexes
            .iter()
            .copied()
            .filter(|i| is_bit_set(input.bits[*i], input.bits_len - 1 - pos))
            .collect::<Vec<_, MAX_INPUT_LINES>>();
        let zero = indexes
            .into_iter()
            .filter(|i| !is_bit_set(input.bits[*i], input.bits_len - 1 - pos))
            .collect::<Vec<_, MAX_INPUT_LINES>>();
        indexes = if one.len() >= zero.len() { one } else { zero };
        pos += 1;
    }
    let oxygen_generator_rating = &input.bits[indexes[0]];

    let mut indexes = (0..input.bits.len()).collect::<Vec<_, MAX_INPUT_LINES>>();
    let mut pos = 0;
    while indexes.len() > 1 {
        let one = indexes
            .iter()
            .copied()
            .filter(|i| is_bit_set(input.bits[*i], input.bits_len - 1 - pos))
            .collect::<Vec<_, MAX_INPUT_LINES>>();
        let zero = indexes
            .into_iter()
            .filter(|i| !is_bit_set(input.bits[*i], input.bits_len - 1 - pos))
            .collect::<Vec<_, MAX_INPUT_LINES>>();
        indexes = if one.len() >= zero.len() { zero } else { one };
        pos += 1;
    }
    let co2_scrubber_rating = &input.bits[indexes[0]];

    (oxygen_generator_rating * co2_scrubber_rating).to_string()
}

pub fn day03() -> (String, String) {
    let parsed = ParsedDay03::new(INPUT);
    (part1(&parsed), part2(&parsed))
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str =
        "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010\n";

    #[test]
    fn test_parsing_ok() {
        let parsed = ParsedDay03::new(TEST_INPUT);
        let recreated = parsed
            .bits
            .into_iter()
            .map(|nb| format!("{:#07b}", nb).split_off(2))
            .collect::<std::vec::Vec<_>>()
            .join("\n")
            + "\n";
        assert_eq!(recreated, TEST_INPUT);
    }

    #[test]
    fn test_part1_test_input() {
        assert_eq!(part1(&ParsedDay03::new(TEST_INPUT)), "198".to_string());
    }

    #[test]
    fn test_part1_real_input() {
        assert_eq!(part1(&ParsedDay03::new(INPUT)), "2583164".to_string());
    }

    #[test]
    fn test_part2_test_input() {
        assert_eq!(part2(&ParsedDay03::new(TEST_INPUT)), "230".to_string());
    }
}
