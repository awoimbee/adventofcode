//
// This code is disgusting
//

const INPUT: &str = include_str!("../input/day18.txt");

use std::io::Write;

macro_rules! next_nb {
    ($it:ident) => {{
        let s = $it.next().unwrap();
        s.parse::<i64>().unwrap()
    }};
}

fn do_maffs_p1(slice: &mut str) -> i64 {
    while let Some(idx_open) = slice.rfind('(') {
        let idx_close = slice[idx_open..].find(')').unwrap() + 1 + idx_open;
        let len = idx_close - idx_open;
        let res = format!(
            "{:width$}",
            do_maffs_p1(&mut slice[idx_open + 1..idx_close - 1]),
            width = len
        );
        unsafe {
            (slice[idx_open..idx_close].as_bytes_mut())
                .write(res.as_bytes())
                .unwrap();
        }
    }
    let mut it = slice.split(' ').filter(|s| !s.is_empty());
    let mut res = next_nb!(it);
    while let Some(tok) = it.next() {
        match tok {
            "+" => res += next_nb!(it),
            "*" => res *= next_nb!(it),
            _ => unreachable!(tok),
        };
    }
    res
}

macro_rules! next_tok_nb {
    ($it:ident) => {{
        match $it.next().unwrap() {
            Token::Nb(n) => *n,
            _ => unreachable!(),
        }
    }};
}

macro_rules! next_tok_op {
    ($it:ident) => {{
        match $it.next() {
            Some(Token::Op(o)) => Some(o),
            None => None,
            _ => unreachable!(),
        }
    }};
}

#[derive(Debug)]
enum Token {
    Op(char),
    Nb(i64),
}

impl From<&str> for Token {
    fn from(s: &str) -> Self {
        match s.as_bytes()[0] {
            b'*' => Self::Op('*'),
            b'+' => Self::Op('+'),
            _ => Self::Nb(s.parse().unwrap()),
        }
    }
}

fn do_maffs_p2(slice: &mut str) -> i64 {
    while let Some(idx_open) = slice.rfind('(') {
        let idx_close = slice[idx_open..].find(')').unwrap() + 1 + idx_open;
        let len = idx_close - idx_open;
        let res = format!(
            "{:width$}",
            do_maffs_p2(&mut slice[idx_open + 1..idx_close - 1]),
            width = len
        );
        unsafe {
            (slice[idx_open..idx_close].as_bytes_mut())
                .write(res.as_bytes())
                .unwrap();
        }
    }
    let mut tokens = slice
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|t| Token::from(t))
        .collect::<Vec<_>>();

    while let Some((idx, _)) = tokens
        .iter()
        .enumerate()
        .filter(|(_, t)| matches!(t, Token::Op('+')))
        .next()
    {
        tokens[idx - 1] = match (&tokens[idx - 1], &tokens[idx + 1]) {
            (Token::Nb(l), Token::Nb(r)) => Token::Nb(l + r),
            _ => unreachable!(),
        };
        let mut i = 0;
        tokens.retain(|_item| {
            let keep = !(idx..=idx + 1).contains(&i);
            i += 1;
            keep
        });
    }
    let mut it = tokens.iter();
    let mut res = next_tok_nb!(it);

    while let Some(_tok) = next_tok_op!(it) {
        res *= next_tok_nb!(it);
    }
    res
}

fn p1() -> i64 {
    INPUT
        .lines()
        .map(|line| {
            let mut l2 = line.trim().to_owned();
            do_maffs_p1(&mut l2)
        })
        .sum()
}

fn p2() -> i64 {
    INPUT
        .lines()
        .map(|line| {
            let mut l2 = line.trim().to_owned();
            do_maffs_p2(&mut l2)
        })
        .sum()
}

pub fn day18() -> (String, String) {
    let p1 = p1();
    let p2 = p2();

    (format!("{}", p1), format!("{}", p2))
}
