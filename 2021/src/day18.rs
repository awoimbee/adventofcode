const INPUT: &str = include_str!("../input/day18.txt");

trait Nesting: std::fmt::Display {}

#[derive(Debug, Clone)]
struct NumComp {
    value: u32,
    closes: u32, // The number of ']' after
    opens: u32,  // the number of '[' before
}

#[derive(Debug, Clone)]
struct Num {
    inner: Vec<NumComp>,
}

impl Num {
    fn from_input(input: &str) -> Self {
        let mut inner: Vec<NumComp> = Vec::new();
        let mut next_opens = 0;
        let mut next_closes = 0;
        for c in input.trim_end().chars() {
            match c {
                '[' => next_opens += 1,
                ']' => next_closes += 1,
                '0'..='9' => {
                    if next_closes > 0 {
                        inner.last_mut().unwrap().closes = next_closes;
                    }
                    inner.push(NumComp {
                        value: c.to_digit(10).unwrap(),
                        closes: 0,
                        opens: next_opens,
                    });
                    next_closes = 0;
                    next_opens = 0;
                }
                _ => {}
            };
        }
        inner.last_mut().unwrap().closes = next_closes;
        Self { inner }
    }

    /// [[1,[2,3]],4] => [[3, 0], 7]
    fn explode(&mut self, i: usize) {
        debug_assert!(self.inner[i + 1].opens == 0);
        if i > 0 {
            self.inner[i - 1].value += self.inner[i].value;
        }
        if i < self.inner.len() - 2 {
            self.inner[i + 2].value += self.inner[i + 1].value;
        }
        self.inner[i].closes = self.inner[i + 1].closes - 1;
        self.inner[i].value = 0;
        self.inner[i].opens -= 1;
        self.inner.remove(i + 1);
    }

    fn split(&mut self, i: usize) {
        let val = self.inner[i].value as f32 / 2.;
        self.inner[i].value = val.floor() as u32;
        self.inner[i].opens += 1;

        self.inner.insert(
            i + 1,
            NumComp {
                value: val.ceil() as u32,
                opens: 0,
                closes: self.inner[i].closes + 1,
            },
        );
        self.inner[i].closes = 0;
    }

    fn reduce(&mut self) {
        let mut nesting = 0;
        for (i, n) in self.inner.iter().enumerate() {
            nesting = nesting + n.opens - n.closes;
            if nesting > 4 && self.inner[i + 1].opens == 0 {
                self.explode(i);
                return self.reduce();
            }
        }
        for (i, n) in self.inner.iter().enumerate() {
            if n.value >= 10 {
                self.split(i);
                return self.reduce();
            }
        }
    }

    fn add(&mut self, mut rhs: Self) {
        self.inner[0].opens += 1;
        rhs.inner.last_mut().unwrap().closes += 1;
        self.inner.append(&mut rhs.inner);
        self.reduce();
    }

    fn _format(&self) -> String {
        let mut res = String::new();
        for n in &self.inner {
            for _ in 0..n.opens {
                res.push('[');
            }
            res.push_str(&n.value.to_string());
            for _ in 0..n.closes {
                res.push(']');
            }
            res.push(',');
        }
        res.pop();

        res
    }

    fn magnitude(mut self) -> u32 {
        let mut i = 0;
        while self.inner.len() > 1 {
            for (i, n) in self.inner.iter().enumerate() {
                if n.closes != 0 || self.inner[i + 1].opens != 0 {
                    continue;
                }
                self.inner[i].value = self.inner[i].value * 3 + self.inner[i + 1].value * 2;
                self.inner[i].opens -= 1;
                self.inner[i].closes = self.inner[i + 1].closes - 1;
                self.inner.remove(i + 1);
                break;
            }
            i += 1;
            if i > 25 {
                break;
            }
        }
        self.inner[0].value
    }
}

fn part1(input: &[Num]) -> u32 {
    let num = input
        .iter()
        .cloned()
        .reduce(|mut acc, rhs| {
            acc.add(rhs);
            acc
        })
        .unwrap();
    num.magnitude()
}

fn part2(input: Vec<Num>) -> u32 {
    let mut max_magnitude = 0;
    for a in input.iter() {
        for mut b in input.iter().cloned() {
            let mut a_1 = a.clone();
            let a_2 = a.clone();

            a_1.add(b.clone());
            let magnitude = a_1.magnitude();
            if magnitude > max_magnitude {
                max_magnitude = magnitude;
            }
            b.add(a_2);
            let magnitude = b.magnitude();
            if magnitude > max_magnitude {
                max_magnitude = magnitude;
            }
        }
    }
    max_magnitude
}

fn parse(input: &str) -> Vec<Num> {
    input.split_terminator('\n').map(Num::from_input).collect()
}

pub fn day18() -> (String, String) {
    let parsed = parse(INPUT);
    let part1 = part1(&parsed);
    let part2 = part2(parsed);

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

    #[test]
    fn test_num_parsing() {
        let s = "[[[[4,3],4],4],[7,[[8,4],9]]]";
        let num = Num::from_input(s);
        assert_eq!(num._format(), s);
    }

    #[test]
    fn test_reductions_steps() {
        let s = "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]";
        let mut num = Num::from_input(s);
        num.explode(0);
        assert_eq!(num._format(), "[[[[0,7],4],[7,[[8,4],9]]],[1,1]]");
        num.explode(4);
        assert_eq!(num._format(), "[[[[0,7],4],[15,[0,13]]],[1,1]]");
        num.split(3);
        assert_eq!(num._format(), "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]");
        num.split(6);
        assert_eq!(num._format(), "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]");
        num.explode(6);
        assert_eq!(num._format(), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
    }

    #[test]
    fn test_reduction() {
        let s = "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]";
        let mut num = Num::from_input(s);
        num.reduce();
        assert_eq!(num._format(), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
    }

    #[test]
    fn test_add() {
        let mut a = Num::from_input("[[[[4,3],4],4],[7,[[8,4],9]]]");
        let b = Num::from_input("[1,1]");
        a.add(b);
        assert_eq!(a._format(), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
    }

    #[test]
    fn test_magnitude() {
        assert_eq!(Num::from_input("[[1,2],[[3,4],5]]").magnitude(), 143);
        assert_eq!(
            Num::from_input("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").magnitude(),
            1384
        );
        assert_eq!(
            Num::from_input("[[[[1,1],[2,2]],[3,3]],[4,4]]").magnitude(),
            445
        );
        assert_eq!(
            Num::from_input("[[[[3,0],[5,3]],[4,4]],[5,5]]").magnitude(),
            791
        );
        assert_eq!(
            Num::from_input("[[[[5,0],[7,4]],[5,5]],[6,6]]").magnitude(),
            1137
        );
        assert_eq!(
            Num::from_input("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").magnitude(),
            3488
        );
    }

    #[test]
    fn test_part1_test_input() {
        assert_eq!(part1(&parse(TEST_INPUT)), 4140);
    }

    #[test]
    fn test_part2_test_input() {
        assert_eq!(part2(parse(TEST_INPUT)), 3993);
    }
}
