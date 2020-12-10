const INPUT: &str = unsafe { std::str::from_utf8_unchecked(include_bytes!("../input/day10.txt")) };

fn parse() -> impl Iterator<Item = u64> {
    INPUT
        .split('\n')
        .filter(|s| !str::is_empty(s))
        .map(|s| s.parse().unwrap())
}

pub fn day10() -> (String, String) {
    let mut jolts = Vec::with_capacity(105);
    jolts.push(0);
    jolts.extend(parse());
    jolts.sort_unstable();
    jolts.push(jolts.last().unwrap() + 3);

    let (p1, p2) = {
        let mut dp = vec![0; jolts.len()];
        dp[0] = 1;
        let mut j = 0;
        let mut sum: u64 = 1;
        let mut nb_jmp_1 = 0;
        let mut nb_jmp_3 = 0;
        for i in 1..(jolts.len()) {
            match jolts[i] - jolts[i - 1] {
                1 => nb_jmp_1 += 1,
                3 => nb_jmp_3 += 1,
                _ => (),
            }
            while jolts[i] - jolts[j] > 3 {
                sum -= dp[j];
                j += 1;
            }
            dp[i] = sum;
            sum *= 2;
        }
        (nb_jmp_1 * nb_jmp_3, *dp.last().unwrap())
    };

    (format!("{}", p1), format!("{}", p2))
}
