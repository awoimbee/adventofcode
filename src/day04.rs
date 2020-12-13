const INPUT: &str = include_str!("../input/day04.txt");

struct Passport {
    present: u8,
    valid: u8,
}
impl Passport {
    pub fn new(data: impl Iterator<Item = &'static str>) -> Self {
        let mut present = 0;
        let mut valid = 0;

        for d in data {
            let mut t = d.split(':');
            let key = t.next().unwrap();
            let value = t.next().unwrap();
            match key {
                "byr" => {
                    // byr (Birth Year) - four digits; at least 1920 and at most 2002.
                    present |= 1 << 0;
                    if let Ok(yr) = value.parse::<u32>() {
                        if (1920..=2002).contains(&yr) {
                            valid |= 1 << 0;
                        }
                    }
                }
                "iyr" => {
                    // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
                    present |= 1 << 1;
                    if let Ok(yr) = value.parse::<u32>() {
                        if (2010..=2020).contains(&yr) {
                            valid |= 1 << 1;
                        }
                    }
                }
                "eyr" => {
                    // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
                    present |= 1 << 2;
                    if let Ok(yr) = value.parse::<u32>() {
                        if (2020..=2030).contains(&yr) {
                            valid |= 1 << 2;
                        }
                    }
                }
                "hgt" => {
                    // hgt (Height) - a number followed by either cm or in:
                    //   If cm, the number must be at least 150 and at most 193.
                    //   If in, the number must be at least 59 and at most 76.
                    present |= 1 << 3;
                    if value.len() >= 4 {
                        if let Ok(h) = &value[..value.len() - 2].parse::<u32>() {
                            if (&value[value.len() - 2..] == "in" && (59..=76).contains(h))
                                || (&value[value.len() - 2..] == "cm" && (150..=193).contains(h))
                            {
                                valid |= 1 << 3;
                            }
                        }
                    }
                }
                "hcl" => {
                    // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
                    present |= 1 << 4;
                    if value.len() == 7 && value.as_bytes()[0] == b'#' {
                        for i in 1..7 {
                            if !matches!(value.as_bytes()[i], b'0'..=b'9'|b'a'..=b'f') {
                                continue;
                            }
                        }
                        valid |= 1 << 4;
                    }
                }
                // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
                "ecl" => {
                    present |= 1 << 5;
                    if matches!(value, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth") {
                        valid |= 1 << 5;
                    }
                }
                // pid (Passport ID) - a nine-digit number, including leading zeroes.
                "pid" => {
                    present |= 1 << 6;
                    if value.len() == 9 {
                        for i in 0..9 {
                            if !value.as_bytes()[i].is_ascii_digit() {
                                continue;
                            }
                        }
                        valid |= 1 << 6;
                    }
                }
                // cid (Country ID) - ignored, missing or not.
                _ => (), // skip cid
            }
        }
        Self { valid, present }
    }
    pub fn is_complete(&self) -> bool {
        self.present == 127
    }
    pub fn is_valid(&self) -> bool {
        self.valid == 127
    }
}

fn parse() -> Vec<Passport> {
    INPUT
        .split("\n\n")
        .map(|s| s.split(|c| c == ' ' || c == '\n').filter(|s| !s.is_empty()))
        .map(Passport::new)
        .collect()
}

pub fn day04() -> (String, String) {
    let passports = parse();
    let mut p1 = 0;
    let mut p2 = 0;

    for p in passports {
        if p.is_complete() {
            p1 += 1
        };
        if p.is_valid() {
            p2 += 1
        };
    }
    (format!("{}", p1), format!("{}", p2))
}
