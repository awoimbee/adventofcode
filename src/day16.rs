const INPUT: &str = include_str!("../input/day16.txt");

fn p1_bis(
    nearby_tickets: impl Iterator<Item = Vec<u64>>,
    schema: &[(&'static str, [[u64; 2]; 2])],
) -> (Vec<Vec<u64>>, u64) {
    let mut invalid_sum = 0;
    let valid_tickets = nearby_tickets
        .filter(|ticket| {
            ticket.iter().all(|val| {
                let valid = schema.iter().any(|(_, ranges)| {
                    ranges
                        .iter()
                        .any(|range| (range[0]..=range[1]).contains(val))
                });
                if !valid {
                    invalid_sum += val;
                }
                valid
            })
        })
        .collect();

    (valid_tickets, invalid_sum)
}

fn p2(schema: Vec<(&str, [[u64; 2]; 2])>, ticket: Vec<u64>, nearby: Vec<Vec<u64>>) -> u64 {
    let indexes: Vec<_> = schema
        .iter()
        .map(|(name, _)| name)
        .enumerate()
        .filter(|(_, name)| name.starts_with("departure"))
        .map(|(index, _)| index)
        .collect();

    let confusion_matrix = p2_generate_confusion_matrix(&nearby, &schema);
    let matching = p2_solve(&confusion_matrix);

    let ans: u64 = indexes
        .into_iter()
        .map(|i| ticket[matching.iter().position(|&x| x == i).unwrap()])
        .product();
    ans
}

fn p2_generate_confusion_matrix(
    nearby: &[Vec<u64>],
    schema: &[(&str, [[u64; 2]; 2])],
) -> Vec<Vec<bool>> {
    let columns = p2_transpose(&nearby);
    let m = schema.len();
    let mut ans = vec![vec![false; m]; m];
    for i in 0..m {
        for j in 0..m {
            if p2_check(&columns[i], &schema[j].1) {
                ans[i][j] = true;
            }
        }
    }
    ans
}

fn p2_transpose(nearby: &[Vec<u64>]) -> Vec<Vec<u64>> {
    let m = nearby[0].len();

    let mut ans = vec![Vec::new(); m];
    for numbers in nearby {
        for (i, &number) in numbers.iter().enumerate() {
            ans[i].push(number);
        }
    }
    ans
}

fn p2_check(numbers: &[u64], intervals: &[[u64; 2]]) -> bool {
    numbers.iter().all(|num| {
        intervals
            .iter()
            .any(|range| range[0] <= *num && *num <= range[1])
    })
}

fn p2_solve(valid: &[Vec<bool>]) -> Vec<usize> {
    let mut valid = valid.to_owned();

    let mut ans = vec![999_999; valid.len()];
    for _ in 0..valid.len() {
        let (i, v) = valid
            .iter()
            .enumerate()
            .min_by_key(|&(_, item)| {
                let v = item.iter().filter(|x| **x).count();
                if v == 0 {
                    return 999_999;
                }
                v
            })
            .unwrap();
        let indexes: Vec<_> = v
            .iter()
            .enumerate()
            .filter(|(_, x)| **x)
            .map(|(i, _)| i)
            .collect();
        if indexes.len() > 1 {
            unreachable!(format!("{:?}", indexes));
        }
        for i in 0..valid.len() {
            valid[i][indexes[0]] = false;
        }
        ans[i] = indexes[0];
    }

    ans
}

pub fn day16() -> (String, String) {
    let mut sections = INPUT.trim().split("\n\n");

    let schema: Vec<_> = sections
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            let mut t = l.split(": ");
            let name = t.next().unwrap();
            let mut values = [[0; 2]; 2];
            for (i, range) in t.next().unwrap().split(" or ").enumerate() {
                for (j, nb) in range.split('-').enumerate() {
                    values[i][j] = nb.parse::<u64>().unwrap();
                }
            }
            (name, values)
        })
        .collect();

    let my_ticket: Vec<u64> = sections
        .next()
        .unwrap()
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .map(|nb| nb.parse().unwrap())
        .collect();

    let nearby_tickets = sections
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|l| l.split(',').map(|nb| nb.parse().unwrap()).collect());

    let (nearby_tickets, p1) = p1_bis(nearby_tickets, &schema);
    let p2 = p2(schema, my_ticket, nearby_tickets);

    (p1.to_string(), p2.to_string())
}
