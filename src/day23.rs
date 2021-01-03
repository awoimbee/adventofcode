const INPUT: &str = include_str!("../input/day23.txt");

/// Ring buffer is represented as array with each idx = cup label,
/// and value under that index/label is the index/label of the next cup.
/// `[3, 8, 9, 1, 2, 5, 4, 6, 7]` becomes
/// `[0 => (3), 1 => 2, 2 => 5, 3 => 8, 4 => 6, 5 => 4,
///  6 => 7, 7 => 3, 8 => 9, 9 => 1]`.
///
/// `self.ring[0]` is the currently selected cup
struct Cups {
    ring: Vec<u32>,
}

impl Cups {
    pub fn new(input: &str, nb_cups: Option<usize>) -> Self {
        let cups = input
            .trim_end()
            .chars()
            .map(|ch| (ch as u32) - b'0' as u32)
            .collect::<Vec<_>>();
        let nb_cups = match nb_cups {
            Some(s) if s >= cups.len() => s,
            _ => cups.len(),
        };
        let mut s = Self {
            ring: vec![std::u32::MAX; nb_cups + 1],
        };
        //  Set head & body
        s.ring[0] = cups[0];
        cups.windows(2)
            .for_each(|two_cups| s.ring[two_cups[0] as usize] = two_cups[1]);
        //  Set tail
        if nb_cups != cups.len() {
            *s.ring.last_mut().unwrap() = s.ring[0];

            let mut prev = *cups.last().unwrap() as usize;
            let mut next = cups.len() + 1;
            while prev < nb_cups {
                s.ring[prev] = next as u32;
                prev = next;
                next += 1;
            }
        } else {
            s.ring[*cups.last().unwrap() as usize] = cups[0] as u32;
        }

        s
    }
    pub fn run_rounds(&mut self, nb: usize) {
        for _ in 0..nb {
            let p0 = self.ring[0];
            // pick up three cups
            let p1 = self.ring[p0 as usize];
            let p2 = self.ring[p1 as usize];
            let p3 = self.ring[p2 as usize];

            // pick destination
            let mut dst = p0;
            while [p0, p1, p2, p3].contains(&dst) {
                dst -= 1;
                if dst == 0 {
                    dst = (self.ring.len() - 1) as u32;
                }
            }

            // we need to go from
            // [ current => pick1 => pick2 => pick3 => after_pick3 ... //
            //   ... => dst => after_dst => ... ]
            // to
            // [ current => after_pick3 ... //
            //   ... => dst => pick1 => pick2 => pick3 => after_dst => ... ]

            // redirect current to after_pick3
            self.ring[p0 as usize] = self.ring[p3 as usize];

            // place picks between dst and after_dst
            let after_dst = self.ring[dst as usize];
            self.ring[dst as usize] = p1;
            self.ring[p3 as usize] = after_dst;

            // select new current cup
            self.ring[0] = self.ring[p0 as usize];
        }
    }
    pub fn two_cups_clockwise_of_one(&self) -> usize {
        self.ring[1] as usize * self.ring[self.ring[1] as usize] as usize
    }
    pub fn labels_on_cups_after_cup_one(&self) -> String {
        let mut output = String::new();
        let mut id = self.ring[1];
        while id != 1 {
            output.push((id as u8 + b'0') as char);
            id = self.ring[id as usize];
        }
        output
    }
}

pub fn part1(input: &str, nb_rounds: usize) -> String {
    let mut cups = Cups::new(input, None);
    cups.run_rounds(nb_rounds);
    cups.labels_on_cups_after_cup_one()
}

pub fn part2(input: &str) -> usize {
    const TOTAL_CUPS: usize = 1_000_000;

    let mut cups = Cups::new(input, Some(TOTAL_CUPS));
    cups.run_rounds(10_000_000);
    cups.two_cups_clockwise_of_one()
}

pub fn day23() -> (String, String) {
    let p1 = part1(INPUT.trim(), 100);
    let p2 = part2(INPUT.trim());
    (p1.to_string(), p2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_small() {
        assert_eq!(part1("389125467", 10), "92658374");
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1("467528193", 100), "43769582");
    }
    #[test]
    fn test_part2_small() {
        assert_eq!(part2("389125467"), 149245887792);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2("467528193"), 264692662390);

    }
}
