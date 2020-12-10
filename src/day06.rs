use std::collections::HashMap;

const INPUT: &[u8] = include_bytes!("../input/day06.txt");

pub fn day06() -> (String, String) {
    let mut count_part1: usize = 0;
    let mut count_part2: usize = 0;
    let mut answers: HashMap<u8, usize> = HashMap::new();
    let mut nb_people: usize = 0;

    for line in INPUT.split(|&c| c == b'\n') {
        if line.is_empty() {
            count_part1 += answers.len();
            count_part2 += answers.iter().filter(|(_, &nb)| nb == nb_people).count();
            answers.drain();
            nb_people = 0;
        } else {
            nb_people += 1;
            // letters are a-z, no need to handle utf-8
            for c in line.iter() {
                answers
                    .entry(*c)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
        }
    }
    (format!("{}", count_part1), format!("{}", count_part2))
}
