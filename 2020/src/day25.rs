const INPUT: &str = include_str!("../input/day25.txt");

const BASE_NB: u64 = 7;
const REMAINDER_NB: u64 = 20201227;

fn parse() -> Vec<u64> {
    INPUT.lines().map(|l| l.parse().unwrap()).collect()
}

fn calculate_loopsize(pubkey: u64) -> u64 {
    let mut loopsize = 0;
    let mut value = 1;
    let subject = BASE_NB;
    while value != pubkey {
        value = (value * subject) % REMAINDER_NB;
        loopsize += 1;
    }
    loopsize
}

fn transform(subject: u64, loopsize: u64) -> u64 {
    let mut value = 1;
    for _ in 0..loopsize {
        value = (value * subject) % REMAINDER_NB;
    }
    value
}

fn calculate_privkey(pubkeys: &[u64]) -> u64 {
    let card_loopsize = calculate_loopsize(pubkeys[0]);
    transform(pubkeys[1], card_loopsize)
}

pub fn day25() -> (String, String) {
    let pubkeys = parse();

    let p1 = calculate_privkey(&pubkeys);
    let p2 = "🌟";

    (p1.to_string(), p2.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn loopsize() {
        const PUBKEY: u64 = 17807724;
        let clp = calculate_loopsize(PUBKEY);
        assert!(clp == 11);
    }

    #[test]
    fn privkey() {
        let pubkeys = [5764801, 17807724];
        let privkey = calculate_privkey(&pubkeys);
        assert!(privkey == 14897079);
    }
}
