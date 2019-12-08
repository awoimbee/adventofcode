#![feature(maybe_uninit_ref)]

use std::io::{self, BufRead};
use std::mem::MaybeUninit;


fn smaller(a: &[u8; 6], b: &[u8; 6]) -> bool {
    let a: u64 = unsafe {
        let a: *const u64 = std::mem::transmute(a.as_ptr());
        (*a & 0x0000FFFFFFFFFFFF).swap_bytes()  // ONLY NEEDED ON SMALL-ENDIAN SYSTEMS
    };
    let b: u64 = unsafe {
        let b: *const u64 = std::mem::transmute(b.as_ptr());
        (*b & 0x0000FFFFFFFFFFFF).swap_bytes()
    };
    a < b
}

fn increasing_digits(nb: &[u8; 6]) -> bool {
    (0..nb.len() - 1).all(|i| nb[i] <= nb[i + 1])
}

fn double_digits(nb: &[u8; 6]) -> bool {
    (0..nb.len() - 1).any(|i| nb[i] == nb[i + 1])
}

fn strict_double_digits(nb: &[u8; 6]) -> bool {
    let l = nb.len() - 1;
    let mut i = 0;
    while i < l {
        if nb[i] == nb[i + 1] {
            match ((i + 1) == l) || nb[i + 1] != nb[i + 2] {
                true => return true,
                false => while i != l && nb[i] == nb[i+1] { i+= 1 },
            }
        }
        i += 1;
    }
    false
}

fn increment(nb: &mut [u8]) {
    for i in (0..nb.len()).rev() {
        nb[i] += 1;
        match nb[i] == 10 {
            false => return,
            true => nb[i] = nb[i - i],
        };
    }
}

fn parse_input(inp: &str) -> [u8; 6] {
    let inp = inp.as_bytes();
    if inp.len() != 6 {
        panic!("input nb too long");
    }
    let mut res = MaybeUninit::<[u8; 6]>::uninit();
    let res = unsafe {
        let res_w = res.get_mut();
        for i in 0..res_w.len() {
            res_w[i] = inp[i] - b'0';
        }
        res.assume_init()
    };
    res
}

fn main() {
    println!("Input exemple: 134792-675810");
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();

    let now = std::time::Instant::now();

    let dash = input.find('-').unwrap();
    let mut nb = parse_input(&input[..dash]);
    let end = parse_input(&input[dash+1..]);

    let mut occurences = 0;
    let mut occurences_1 = 0;
    while smaller(&nb, &end) {
        if increasing_digits(&nb) {
            if double_digits(&nb) {
                occurences += 1;
            }
            if strict_double_digits(&nb) {
                occurences_1 += 1;
            }
        }
        increment(&mut nb);
    }

    println!("Took {}µs", now.elapsed().as_micros());
    println!("Part. I: {}", occurences);
    println!("Part. II: {}", occurences_1);
}
