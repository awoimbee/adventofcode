use std::ops::Range;

const INPUT: &str = include_str!("../input/day23.txt");

fn parse(input: &'static str) -> Vec<u8> {
    input.as_bytes().iter().filter(|c| (b'0'..=b'9').contains(*c)).map(|c| c - b'0').collect()
}


// The crab picks up the three cups that are immediately clockwise of the current cup.
//      They are removed from the circle; cup spacing is adjusted as necessary to maintain the circle.

// The crab selects a destination cup: the cup with a label equal to the current cup's label minus one.
//      If this would select one of the cups that was just picked up,

//      the crab will keep subtracting one until it finds a cup that wasn't just picked up.
//      If at any point in this process the value goes below the lowest value on any cup's label,
//      it wraps around to the highest value on any cup's label instead.
// The crab places the cups it just picked up so that they are immediately clockwise of the destination cup.
//      They keep the same order as when they were picked up.
// The crab selects a new current cup: the cup which is immediately clockwise of the current cup.

// fn pick_cups(cups: &mut Vec<u8>, pos: usize, nb: usize) -> Vec<u8> {
//     let picked_cups = Vec::new();
//     for i in 0..nb {
//         picked_cups.push(cups.remove(pos % cups.len()));
//     }
//     picked_cups
// }

fn find_dest(cups: &[u8], org: &[usize], curr: u8) -> u8 {
    let mut target = curr;
    loop {
        target = if target == 1 {cups.len() as u8} else {target - 1};
        println!("Target: {}", target);
        for (i, c) in cups.iter().enumerate() {
            if *c == target && !org.contains(&i) {
                return target;
            }
        }
    }
}

// fn move_cups(cups: &mut Vec<u8>, org: usize, dst: usize) {
//     let org_wrapped = [org % cups.len(), (org + 3) % cups.len()];
//     let dst_wrapped = [dst % cups.len(), (dst + 3) % cups.len()];



// }

fn wrapped_range(start: usize, end: usize, max: usize) -> Vec<usize> {
    let wrapped_end = end % max;
    let wrapped_start = start % max;
    if wrapped_end < wrapped_start {
        (wrapped_start..max).chain(0..wrapped_end).collect()
    } else {
        (wrapped_start..wrapped_end).into_iter().collect()
    }
}


fn crab_move(cups: &mut Vec<u8>, mut pos: usize) {
    pos = pos % cups.len();
    let curcup = cups[pos];
    let org_ids = wrapped_range(pos+1, pos+4, cups.len());
    let org: Vec<_> = org_ids.iter().map(|i| cups[*i]).collect();
    let dest = find_dest(&cups, &org_ids, cups[pos]);

    println!("pos: {}: {}", pos, cups[pos]);
    println!("picked up: {:?}", org_ids.iter().map(|i| cups[*i]).collect::<Vec<_>>());
    println!("dest: {}", dest);

    let mut oi = 0;
    let mut di = 0;
    let mut new_cups = Vec::with_capacity(cups.len());
    while oi < cups.len() {
        if org_ids.contains(&oi) {
            oi += 1;
        } else if cups[oi] == dest {
            new_cups.push(cups[oi]);
            oi += 1;
            new_cups.extend(org.iter());
        } else {
            new_cups.push(cups[oi]);
            oi += 1;
        }
    }
    let delta = pos as isize - new_cups.iter().position(|c| *c == curcup).unwrap() as isize;
    if delta < 0 {
        new_cups.rotate_left((-delta) as usize);
    } else {
        new_cups.rotate_right(delta as usize);
    }

    *cups = new_cups;
}

fn part1(mut cups: Vec<u8>, nb_rounds: usize) -> String {
    for round in 0..nb_rounds {
        println!("ROUND {} CUPS: {:?}", round, cups);
        crab_move(&mut cups, round);
    }
    cups.iter_mut().for_each(|c| *c += b'0');
    let idx_one = cups.iter().position(|c| *c == b'1').unwrap();
    cups.rotate_left(idx_one);
    cups.remove(0);
    String::from_utf8(cups).unwrap()
}

pub fn day23() -> (String, String) {
    let mut t = parse(INPUT);

    let p1 = part1(t, 100);
    let p2 = "undefined";

    (p1.to_string(), p2.to_string())
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "389125467";

    #[test]
    fn test_part1() {
        let parsed = parse(TEST_INPUT);
        let p1 = part1(parsed, 10);
        println!("p1: {}", p1);
        assert!(p1 == "92658374");
    }
}
