const INPUT: &str = include_str!("../input/day02.txt");

#[derive(Debug)]
struct Rule {
    pub char: u8,
    pub min: u8,
    pub max: u8,
}

fn parse() -> impl Iterator<Item = (Rule, &'static [u8])> {
    INPUT.lines().map(|s| {
        let mut t = s.split(": ");
        let rule = t.next().unwrap();
        let passwd = t.next().unwrap();
        let mut t = rule.split(' ');
        let mut tt = t.next().unwrap().split('-');
        let min = tt.next().unwrap().parse().unwrap();
        let max = tt.next().unwrap().parse().unwrap();
        let char = t.next().unwrap().as_bytes()[0];
        (Rule { char, min, max }, passwd.as_bytes())
    })
}

fn is_valid_part1(r: &Rule, pass: &[u8]) -> bool {
    let nb = pass.iter().map(|&c| if c == r.char { 1 } else { 0 }).sum();
    r.min <= nb && nb <= r.max
}

fn is_valid_part2(r: &Rule, pass: &[u8]) -> bool {
    let char1 = pass[(r.min - 1) as usize];
    let char2 = pass[(r.max - 1) as usize];
    (char1 == r.char) ^ (char2 == r.char)
}

pub fn day02() -> (String, String) {
    let input = parse();

    let mut p1 = 0;
    let mut p2 = 0;
    for (r, p) in input {
        if is_valid_part1(&r, p) {
            p1 += 1
        };
        if is_valid_part2(&r, p) {
            p2 += 1
        };
    }

    (p1.to_string(), p2.to_string())
}
