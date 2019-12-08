use std::io::{self, BufRead};

#[derive(Clone, Copy)]
struct Vert {
    x: i32,
    y: i32,
}

struct Seg {
    a: Vert,
    b: Vert,
}
impl Seg {
    pub fn new(a: Vert, b: Vert) -> Self {
        Seg { a, b }
    }
}

/// Returns (vertical, horizontal)
fn parse_input(input: String) -> (Vec<Seg>, Vec<Seg>) {
    let mut last_vert = Vert { x: 0, y: 0 };
    let mut vertical = Vec::new();
    let mut horizont = Vec::new();

    for p in input.split(',') {
        let v0 = last_vert;
        let dir = p.as_bytes()[0usize] as char;
        let dist: i32 = p[1..].parse().unwrap();
        match dir {
            'U' => {
                last_vert.y += dist;
                vertical.push(Seg::new(v0, last_vert))
            }
            'D' => {
                last_vert.y -= dist;
                vertical.push(Seg::new(last_vert, v0))
            }
            'L' => {
                last_vert.x -= dist;
                horizont.push(Seg::new(last_vert, v0))
            }
            'R' => {
                last_vert.x += dist;
                horizont.push(Seg::new(v0, last_vert))
            }
            _ => panic!("Bad input"),
        };
    }
    (vertical, horizont)
}

fn closest_intersect(vertis: Vec<Seg>, horizs: Vec<Seg>) -> i32 {
    let mut smallest_dist = std::i32::MAX;
    for v in &vertis {
        for h in &horizs {
            if (h.a.x <= v.a.x && v.a.x <= h.b.x) && (v.a.y <= h.a.y && h.a.y <= v.b.y) {
                let dist = v.a.x.abs() + h.a.y.abs();
                if dist < smallest_dist && dist != 0 {
                    smallest_dist = dist;
                }
            }
        }
    }
    smallest_dist
}

fn main() {
    let stdin = io::stdin();

    let input = stdin.lock().lines().next().unwrap().unwrap();
    let (v0, h0) = parse_input(input);
    let input = stdin.lock().lines().next().unwrap().unwrap();
    let (v1, h1) = parse_input(input);

    let smallest_dist = std::cmp::min(closest_intersect(v0, h1), closest_intersect(v1, h0));
    println!("Smallest distance: {}", smallest_dist);
}
