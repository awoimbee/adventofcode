use std::io::{self, BufRead};

#[derive(Clone,Copy)]
struct Vert {
    x: i32,
    y: i32,
}

/// Returns (vertical, horizontal)
fn parse_input(input: String) -> Vec<Vert> {
    let mut last_vert = Vert{x: 0, y: 0};
    let mut path = Vec::new();

    for p in input.split(',') {
        let dir = p.as_bytes()[0usize] as char;
        let dist: i32 = p[1..].parse().unwrap();
        match dir {
            'U' => last_vert.y += dist,
            'D' => last_vert.y -= dist,
            'L' => last_vert.x -= dist,
            'R' => last_vert.x += dist,
            _ => panic!("Bad input"),
        };
        path.push(last_vert.clone())
    }
    path
}

// fn closest_intersect(vertis: Vec<Seg>, horizs: Vec<Seg>) -> i32 {
//     let mut smallest_dist = 9999999;
//     for v in &vertis {
//         for h in &horizs {
//             if (h.a.x <= v.a.x && v.a.x <= h.b.x) && (v.a.y <= h.a.y && h.a.y <= v.b.y) {
//                 let dist = v.a.x.abs() + h.a.y.abs();
//                 if dist < smallest_dist && dist != 0 {
//                     smallest_dist = dist;
//                 }
//             }
//         }
//     }
//     smallest_dist
// }

fn main() {
    let stdin = io::stdin();

    let input = stdin.lock().lines().next().unwrap().unwrap();
    let path0 = parse_input(input);
    let input = stdin.lock().lines().next().unwrap().unwrap();
    let path1 = parse_input(input);

    let mut smallest_steps = 9999999;
    for (stepw0, w0) in path0.chunks(2).enumerate() {
        if w0.len() == 1 { break; }
        for (stepw1, w1) in path1.chunks(2).enumerate() {
            if w1.len() == 1 { break; }
            if (w0[0].x <= w1[0].x && w1[0].x <= w0[1].x) && (w1[0].y <= w0[0].y && w0[0].y <= w1[1].y) {
                let dist = stepw1 + stepw0;
                if dist < smallest_steps && dist != 0 {
                    smallest_steps = dist;
                }
            }
        }
    }
    // smallest_dist

    // let smallest_dist = std::cmp::min(closest_intersect(v0, h1), closest_intersect(v1, h0));
    println!("Smallest distance: {}", smallest_steps);
}
