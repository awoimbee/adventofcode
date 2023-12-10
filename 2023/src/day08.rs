const INPUT: &str = include_str!("../input/day08.txt");

const fn name_to_idx(name: &str) -> u32 {
    let letters = name.as_bytes();
    let letters = [
        (letters[0] - b'A') as u32,
        (letters[1] - b'A') as u32,
        (letters[2] - b'A') as u32,
    ];
    letters[0] | letters[1] << 5 | letters[2] << 10
}

#[allow(dead_code)]
fn idx_to_name(idx: u32) -> &'static str {
    static mut NAME: [u8; 3] = [0; 3];
    unsafe {
        NAME[0] = (idx & 0b11111) as u8 + b'A';
        NAME[1] = (idx >> 5 & 0b11111) as u8 + b'A';
        NAME[2] = (idx >> 10 & 0b11111) as u8 + b'A';
        std::str::from_utf8_unchecked(&NAME)
    }
}

/// least common multiple
fn lcm(values: &[u64]) -> u64 {
    // a * b / gcd(a, b)

    let mut gcd_ = values[0];
    for value in values.iter().skip(1) {
        gcd_ = gcd(gcd_, *value);
    }
    gcd_ * values.iter().map(|value| value / gcd_).product::<u64>()
}

/// greatest common denominator/divisor/factor
fn gcd(a: u64, b: u64) -> u64 {
    let mut max = a.max(b);
    let mut min = b.min(a);

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn parse(input: &str) -> (&[u8], Vec<(u32, u32)>) {
    let mut nodes_vec = vec![(u32::MAX, u32::MAX); name_to_idx("ZZZ") as usize + 1];

    let (instructions, nodes) = input.split_once("\n\n").unwrap();
    for node in nodes.lines() {
        let (name, directions) = node.split_once(" = ").unwrap();
        let (left, right) = directions
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split_once(", ")
            .unwrap();
        let name_idx = name_to_idx(name);
        let left_idx = name_to_idx(left);
        let right_idx = name_to_idx(right);
        nodes_vec[name_idx as usize] = (left_idx, right_idx);
    }

    (instructions.as_bytes(), nodes_vec)
}

fn solve_p1(instructions: &[u8], nodes: &[(u32, u32)]) -> i64 {
    let mut cur_node = &nodes[name_to_idx("AAA") as usize];
    let mut instructions_it = instructions.iter().cycle();
    let mut num_steps = 0;
    loop {
        num_steps += 1;
        let instruction = instructions_it.next().unwrap();
        let direction = match instruction {
            b'L' => cur_node.0,
            b'R' => cur_node.1,
            _ => unreachable!(),
        };
        cur_node = &nodes[direction as usize];
        if direction == name_to_idx("ZZZ") {
            break;
        }
    }

    num_steps
}

/// For part 2 the solution is HUGE, impossible to compute using loops.
/// Some input analysis is required to find a pattern.
/// Basically each path neatly loops around (N steps to a **Z node, then N steps until it comes back to a **Z).
/// So the solution is the least common multiple of the number of steps for each path !
fn solve_p2(instructions: &[u8], nodes: &[(u32, u32)]) -> u64 {
    let current_nodes = (name_to_idx("AAA")..=name_to_idx("ZZA"))
        .map(|id| &nodes[id as usize])
        .filter(|directions| directions.0 != u32::MAX)
        .collect::<Vec<_>>();
    let is_arrived = |node: &u32| *node >= name_to_idx("AAZ");

    let steps_for_each: Vec<_> = current_nodes
        .iter()
        .map(|node| {
            let mut num_steps = 0;
            let mut cur_node: &(_, _) = *node;
            let mut instructions_it = instructions.iter().cycle();
            loop {
                num_steps += 1;
                let instruction = instructions_it.next().unwrap();
                let direction = match instruction {
                    b'L' => cur_node.0,
                    b'R' => cur_node.1,
                    _ => unreachable!(),
                };
                cur_node = &nodes[direction as usize];
                if is_arrived(&direction) {
                    break;
                }
            }
            num_steps
        })
        .collect();

    lcm(&steps_for_each)
}

pub fn day08() -> (String, String) {
    let (instructions, nodes) = parse(INPUT);

    let p1 = solve_p1(instructions, &nodes);
    let p2 = solve_p2(instructions, &nodes);

    (p1.to_string(), p2.to_string())
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT_1: &str = indoc! {r#"
        RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)
    "#};

    const TEST_INPUT_2: &str = indoc! {r#"
        LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)
    "#};

    const TEST_INPUT_3: &str = indoc! {r#"
        LR

        AAA = (AAB, XXX)
        AAB = (XXX, AAZ)
        AAZ = (AAB, XXX)
        BBA = (BBB, XXX)
        BBB = (BBC, BBC)
        BBC = (BBZ, BBZ)
        BBZ = (BBB, BBB)
        XXX = (XXX, XXX)
    "#};

    #[test]
    fn test_name_to_idx() {
        assert_eq!(name_to_idx("AAA"), 0);
        assert!(name_to_idx("YZA") < name_to_idx("ZZA"));
        assert!(name_to_idx("ZZA") < name_to_idx("AAB"));
        assert_eq!("AZF", idx_to_name(name_to_idx("AZF")));
        assert_eq!("ZZZ", idx_to_name(name_to_idx("ZZZ")));
    }

    #[test]
    fn test_part_1_input_1() {
        let (instructions, nodes) = parse(TEST_INPUT_1);
        assert_eq!(solve_p1(instructions, &nodes), 2);
    }

    #[test]
    fn test_part_1_input_2() {
        let (instructions, nodes) = parse(TEST_INPUT_2);
        assert_eq!(solve_p1(instructions, &nodes), 6);
    }

    #[test]
    fn test_part_2() {
        let (instructions, nodes) = parse(TEST_INPUT_3);
        assert_eq!(solve_p2(instructions, &nodes), 6);
    }

    // #[test]
    // fn test_part_2() {
    //     assert_eq!(solve_p2(TEST_INPUT), 5905);
    // }
}
