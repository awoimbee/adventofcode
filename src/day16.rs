use std::{cell::RefCell, collections::HashMap};

const INPUT: &str = include_str!("../input/day16.txt");

struct Parsed {
    values: HashMap<&'static str, [[u64; 2]; 2]>,
    my_ticket: Vec<u64>,
    nearby_tickets: RefCell<Vec<Vec<u64>>>,
}

impl Parsed {
    pub fn new() -> Self {
        let sep_your = INPUT.find("your ticket:").unwrap();
        let sep_nearby = sep_your + INPUT[sep_your..].find("nearby tickets:").unwrap();
        let values = INPUT[..sep_your]
            .trim()
            .lines()
            .map(|l| {
                let mut t = l.split(": ");
                let name = t.next().unwrap();
                let mut values = [[0; 2]; 2];
                for (i, range) in t.next().unwrap().split(" or ").enumerate() {
                    for (j, nb) in range.split('-').enumerate() {
                        values[i][j] = nb.parse().unwrap();
                    }
                }
                (name, values)
            })
            .collect();
        let my_ticket = INPUT[sep_your + 13..sep_nearby]
            .trim()
            .split(',')
            .map(|nb| nb.parse().unwrap())
            .collect();
        let nearby_tickets = RefCell::new(
            INPUT[sep_nearby + 16..]
                .trim()
                .lines()
                .map(|l| l.split(',').map(|nb| nb.parse().unwrap()).collect())
                .collect(),
        );
        Self {
            values,
            my_ticket,
            nearby_tickets,
        }
    }
}

fn p1(input: &mut Parsed) -> u64 {
    let mut invalid_sum = 0;
    input.nearby_tickets.borrow_mut().retain(|ticket| {
        let ticket_invalid_sum = ticket
            .iter()
            .map(|val| {
                let mut sum = 0;
                let valid = input
                    .values
                    .iter()
                    .map(|(_, v)| {
                        for r in v {
                            if (r[0]..=r[1]).contains(val) {
                                return true;
                            };
                        }
                        false
                    })
                    .any(|x| x);
                if !valid {
                    sum += *val;
                }
                sum
            })
            .sum::<u64>();
        invalid_sum += ticket_invalid_sum;
        ticket_invalid_sum == 0
    });
    invalid_sum
}

fn p2(input: &Parsed) -> u64 {
    0
}

pub fn day16() -> (String, String) {
    let mut parsed = Parsed::new();

    let p1 = p1(&mut parsed);
    let p2 = p2(&parsed);

    (format!("{}", p1), format!("{}", p2))
}
