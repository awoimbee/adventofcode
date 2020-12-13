const INPUT: &str = include_str!("../input/day13.txt");

fn parse() -> (i64, Vec<i64>, Vec<i64>) {
    let mut lines = INPUT.lines();
    let disponibility = lines.next().unwrap().parse().unwrap();
    let (residues, buses) = lines
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|(_, s)| s != &"x")
        .map(|(dt, s)| (dt as i64, s.parse::<i64>().unwrap()))
        // .fold(())
        .map(|(dt, b_id)| (b_id - dt, b_id))
        .fold((Vec::new(), Vec::new()), |mut acc, x| {
            acc.0.push(x.0);
            acc.1.push(x.1);
            acc
        });
    (disponibility, buses, residues)
}

fn p1(disp: i64, buses: &[i64]) -> i64 {
    let mut min_wait = std::i64::MAX;
    let mut best_bus = -1;
    for b in buses {
        let wait_time = b - (disp % b);
        if wait_time < min_wait {
            min_wait = wait_time;
            best_bus = *b;
        }
    }
    min_wait * best_bus
}

/// [Extended Euclidean algorithm](https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm)
/// Returns Greatest Common Divisor, but also the coefficients of Bézout's identity
#[inline(always)]
fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

/// [Chinese remainder theorem](https://en.wikipedia.org/wiki/Chinese_remainder_theorem)
fn p2(re: &[i64], mo: &[i64]) -> i64 {
    let prod = mo.iter().product::<i64>(); // N
    re.iter()
        .zip(mo)
        .map(|(&re, &mo)| {
            let p = prod / mo;
            re * (egcd(p, mo).1 % mo) * p
        })
        .sum::<i64>()
        % prod
}

pub fn day13() -> (String, String) {
    let (disp, buses, residues) = parse();
    let p1 = p1(disp, &buses);
    let p2 = p2(&residues, &buses);
    (format!("{}", p1), format!("{}", p2))
}
