const INPUT: &str = include_str!("../input/day08.txt");

/// Can be solved in bash in 3ms:
/// `cut input/day08.txt -d'|' -f2 | tr ' ' '\n' | awk '{ print length }' | grep -E '7|4|2|3' | wc -l`
fn part_1(input: &str) -> usize {
    input
        .lines()
        .flat_map(|l| {
            l.rsplit_once('|')
                .unwrap()
                .1
                .split(' ')
                .map(|s| s.trim().len())
        })
        .filter(|&len| len == 7 || len == 3 || len == 4 || len == 2)
        .count()
}

fn signal_srt2byte(signal: &str) -> u8 {
    signal
        .as_bytes()
        .iter()
        .map(|b| match b {
            b'a' => 0b1,
            b'b' => 0b10,
            b'c' => 0b100,
            b'd' => 0b1000,
            b'e' => 0b10000,
            b'f' => 0b100000,
            b'g' => 0b1000000,
            _ => panic!("Invalid signal"),
        })
        .reduce(|acc, x| acc | x)
        .unwrap()
}

fn part_2(input: &str) -> usize {
    let mut total_sum = 0;
    for line in input.lines() {
        // define vars
        // signal_patterns[number_segment][candidate_id] => signal
        let mut signal_patterns: [Vec<u8>; 6] = Default::default();
        let mut output_value: [u8; 4] = Default::default();
        // decoder[number] => signal
        let mut decoder: [u8; 10] = [0; 10];

        // ingest
        let (raw_signals, raw_output) = line.split_once('|').unwrap();
        for rs in raw_signals.trim().split_whitespace() {
            let signal_len = rs.len();
            let signal = signal_srt2byte(rs);
            if !signal_patterns[signal_len - 2].contains(&signal) {
                signal_patterns[signal_len - 2].push(signal);
            }
        }
        for (i, ro) in raw_output.trim().split_whitespace().enumerate() {
            output_value[i] = signal_srt2byte(ro);
        }

        // Build decoder:
        // This is quite unreadable...
        // Here is my logic from my scratchpad:
        // 2seg: 1
        // 3seg: 7
        // 4seg: 4
        // 5seg: 2, 3, 5
        // 6seg: 0, 6, 9
        // 7seg: 8
        // ([5seg numbers]).filter(|nb| (nb|1) == 1)[0] -> 3
        // (3|4) -> 9
        // ([6seg numbers]).filter(|nb| (nb|1)==nb)[0] -> 0
        // ([6seg numbers]).filter(|nb| nb!=0 && nb!= 9) -> 6
        // ([5seg numbers]).filter(|nb| (nb|9) == 8) -> 2
        // [5seg numbers][0] -> 5

        decoder[1] = signal_patterns[0][0];
        decoder[4] = signal_patterns[2][0];
        decoder[7] = signal_patterns[1][0];
        decoder[8] = signal_patterns[5][0];
        decoder[3] = *signal_patterns[3]
            .iter()
            .find(|&&nb| (nb | decoder[1]) == nb)
            .unwrap();
        decoder[9] = decoder[3] | decoder[4];
        decoder[0] = *signal_patterns[4]
            .iter()
            .find(|&&nb| nb != decoder[9] && (nb | decoder[1]) == nb)
            .unwrap();
        decoder[6] = *signal_patterns[4]
            .iter()
            .find(|&&nb| nb != decoder[0] && nb != decoder[9])
            .unwrap();
        decoder[2] = *signal_patterns[3]
            .iter()
            .find(|&&nb| (nb | decoder[9]) == decoder[8])
            .unwrap();
        decoder[5] = *signal_patterns[3]
            .iter()
            .find(|&&nb| nb != decoder[2] && nb != decoder[3])
            .unwrap();

        // decode
        let mut decoded = 0;
        for ov in output_value.iter_mut() {
            decoded *= 10;
            decoded += decoder.iter().position(|x| x == ov).unwrap();
        }
        total_sum += decoded;
    }

    total_sum
}

pub fn day08() -> (String, String) {
    let part1 = part_1(INPUT);
    let part2 = part_2(INPUT);

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn test_part_1_test_input() {
        assert_eq!(part_1(TEST_INPUT), 26);
    }

    #[test]
    fn test_part_2_test_input() {
        assert_eq!(part_2(TEST_INPUT), 61229);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 409);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 1024649);
    }
}
