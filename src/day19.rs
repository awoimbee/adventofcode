const INPUT: &str = include_str!("../input/day19.txt");

const TOK_MAX_ID: usize = 150;

fn parse() -> (Vec<Token>, Vec<&'static str>) {
    let mut sep = INPUT.split("\n\n");

    let rules = {
        let mut rules = vec![Token::Char(b'\0'); TOK_MAX_ID];
        sep.next().unwrap().lines().for_each(|line| {
            let mut sep = line.split(": ");
            let id: usize = sep.next().unwrap().parse().unwrap();
            let rules_str = sep.next().unwrap().trim();
            rules[id] = if rules_str.starts_with('"') {
                Token::Char(rules_str.as_bytes()[1])
            } else {
                let mut rules_sep = rules_str.split('|');
                let rules0 = rules_sep.next().unwrap().trim().split(' ').map(|nb| nb.parse().unwrap()).collect();
                let rules1 = rules_sep.next().map(|r| {r.trim().split(' ').map(|nb| nb.parse().unwrap()).collect::<Vec<_>>()});
                Token::Rules((rules0, rules1))
            };
        });
        rules
    };
    let data = sep.next().unwrap().trim().lines().collect();

    (rules, data)
}

#[derive(Clone)]
enum Token {
    Char(u8),
    Rules((Vec<usize>, Option<Vec<usize>>)),
}

impl Token {
    pub fn inner_matches(&self, rules: &[Token], data: &str, mut idx: usize) -> (usize, bool) {
        println!("-> data: {}", &data[idx..]);
        let ret = match self {
            Self::Char(c) => {
                if data.len() > idx && data.as_bytes()[idx] == *c {
                    println!("matched ! {} {}", data.as_bytes()[idx] as char, *c as char);
                    (idx + 1, true)
                } else {
                    (idx, false)
                }
            },
            Self::Rules((r0, opt_r1)) => {
                let rules_ids = if let Some(r1) = opt_r1 {
                    vec![r0, r1]
                } else {
                    vec![r0]
                };
                let ret = rules_ids.iter().any(|r| {
                    let mut tmp_idx = idx;
                    let ret = r.iter().all(|i| {
                        println!("{}: matching token: {}", data, i);
                        let tok = &rules[*i];
                        let ret = tok.inner_matches(rules, data, tmp_idx);
                        tmp_idx = ret.0;
                        ret.1
                    });
                    if ret {
                        idx = tmp_idx;
                    }
                    ret
                });
                (idx, ret)
            }
        };
        println!("<- ret: {:?}", ret);
        ret
    }

    pub fn matches(&self, rules: &[Token], data: &str) -> bool {
        let (idx, res) = self.inner_matches(rules, data, 0);
        res && idx == data.len()
    }
}

fn p1(rules: &[Token], data: &[&str]) -> u64 {
    data.iter().filter(|d| rules[0].matches(&rules, d)).count() as u64
}

fn p2(rules: &mut [Token], data: &[&str]) -> u64 {
    rules[8] = Token::Rules((vec![42], Some(vec![42, 8])));
    rules[11] = Token::Rules((vec![42, 31], Some(vec![42, 11, 31])));
    data.iter().filter(|d| rules[0].matches(&rules, d)).count() as u64
}

pub fn day19() -> (String, String) {
    let (mut rules, data) = parse();

    // let p1 = "";
    let p2 = "";
    let p1 = p1(&rules, &data);
    // let p2 = p2(&mut rules, &data);

    (format!("{}", p1), format!("{}", p2))
}
