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
                let rules0 = rules_sep
                    .next()
                    .unwrap()
                    .trim()
                    .split(' ')
                    .map(|nb| nb.parse().unwrap())
                    .collect();
                let rules1 = rules_sep.next().map(|r| {
                    r.trim()
                        .split(' ')
                        .map(|nb| nb.parse().unwrap())
                        .collect::<Vec<_>>()
                });
                Token::Rules((rules0, rules1))
            };
        });
        rules
    };
    let data = sep.next().unwrap().trim().lines().collect();

    (rules, data)
}

#[derive(Clone, Debug)]
enum Token {
    Char(u8),
    Rules((Vec<usize>, Option<Vec<usize>>)),
}
impl std::fmt::Display for Token {
    fn fmt(&self, w: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Char(c) => w.write_fmt(format_args!("Char({})", *c as char)),
            Self::Rules((r0, r1)) => w.write_fmt(format_args!("Rules({:?}, {:?}", r0, r1)),
        }
    }
}

impl Token {
    fn inner_match_rules(
        &self,
        rules: &[Token],
        data: &str,
        idx: usize,
        r: &[usize],
    ) -> Vec<usize> {
        let mut tmp_res = vec![idx];
        r.iter().all(|i| {
            let tok = &rules[*i];
            tmp_res = tmp_res
                .drain(..)
                .map(|tmp_idx| tok.inner_matches(rules, data, tmp_idx))
                .flatten()
                .collect();
            !tmp_res.is_empty()
        });
        tmp_res
    }

    fn inner_matches(&self, rules: &[Token], data: &str, idx: usize) -> Vec<usize> {
        let ret = match self {
            Self::Char(c) => {
                if data.len() > idx && data.as_bytes()[idx] == *c {
                    vec![idx + 1]
                } else {
                    Vec::new()
                }
            }
            Self::Rules((r0, None)) => self.inner_match_rules(rules, data, idx, r0),
            Self::Rules((r0, Some(r1))) => {
                let mut lft = self.inner_match_rules(rules, data, idx, r0);
                let mut rgt = self.inner_match_rules(rules, data, idx, r1);
                if !lft.is_empty() && !rgt.is_empty() {
                    lft.drain(..).chain(rgt.drain(..)).collect()
                } else if !lft.is_empty() {
                    lft
                } else {
                    rgt
                }
            }
        };
        ret
    }

    pub fn matches(&self, rules: &[Token], data: &str) -> bool {
        let matches = self.inner_matches(rules, data, 0);
        for idx in matches {
            if idx == data.len() {
                return true;
            }
        }
        false
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
    // let p2 = "";
    let p1 = p1(&rules, &data);
    let p2 = p2(&mut rules, &data);

    (p1.to_string(), p2.to_string())
}
