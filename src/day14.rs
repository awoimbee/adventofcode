use std::collections::HashMap;

const INPUT: &str = include_str!("../input/day14.txt");

#[derive(Debug)]
struct BinaryMask {
    zero: u64,
    one: u64,
    float: Vec<u8>,
}
impl BinaryMask {
    pub fn default() -> Self {
        Self {
            zero: std::u64::MAX,
            one: 0,
            float: Vec::new(),
        }
    }
    pub fn from_str(line: &str) -> Self {
        let mut mask = Self::default();
        line.as_bytes().iter().enumerate().for_each(|(i, c)| {
            match *c {
                b'1' => mask.one += 1 << 35 - i,
                b'0' => mask.zero -= 1 << 35 - i,
                b'X' => mask.float.push((35 - i) as u8),
                _ => unreachable!(),
            };
        });
        mask
    }
    pub fn mask_p1(&self, val: u64) -> u64 {
        (val & self.zero) | self.one
    }
    fn _mask_p2_float<'a>(float_mask: &[u8], val: u64) -> Vec<u64> {
        if float_mask.is_empty() {
            vec![val]
        } else {
            let idx = float_mask[0];
            let val1 = val | (1 << idx);
            let val2 = val & (std::u64::MAX ^ (1 << idx));

            let mut a = Self::_mask_p2_float(&float_mask[1..], val1);
            let mut b = Self::_mask_p2_float(&float_mask[1..], val2);

            (a.drain(..).chain(b.drain(..))).collect()
        }
    }

    pub fn mask_p2(&self, mut val: u64) -> Vec<u64> {
        val = val | self.one;
        Self::_mask_p2_float(&self.float, val)
    }
}

#[derive(Debug)]
enum Line<'a> {
    Mask(&'a str),
    MemSet(usize, u64), // address: value
}

fn parse() -> impl Iterator<Item = Line<'static>> {
    INPUT.lines().map(|l| {
        if l.starts_with("mask") {
            Line::Mask(&l[l.find('=').unwrap() + 2..])
        } else {
            let memloc = &l[l.find('[').unwrap() + 1..l.find(']').unwrap()];
            let val = &l[l.find('=').unwrap() + 2..];
            Line::MemSet(memloc.parse().unwrap(), val.parse().unwrap())
        }
    })
}

fn p1(lines: &[Line]) -> usize {
    let mut mask: BinaryMask = BinaryMask::default();
    let mut mem = vec![0; 99047];
    for l in lines.iter() {
        match l {
            Line::Mask(m) => mask = BinaryMask::from_str(m),
            Line::MemSet(idx, val) => {
                mem[*idx] = mask.mask_p1(*val);
            }
        }
    }
    mem.iter().sum::<u64>() as usize
}

fn _bits(nb: u64) -> String {
    (0..36)
        .map(|i| match (nb >> (35 - i)) & 1 {
            1 => '1',
            0 => '0',
            _ => unreachable!(),
        })
        .collect()
}

fn p2(lines: &[Line]) -> u64 {
    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut mask = BinaryMask::default();
    for l in lines {
        match l {
            Line::Mask(m) => mask = BinaryMask::from_str(m),
            Line::MemSet(idx, val) => {
                mask.mask_p2(*idx as u64).iter().for_each(|id| {
                    mem.insert(*id, *val);
                });
            }
        }
    }

    mem.drain().map(|(_k, v)| v).sum()
}

pub fn day14() -> (String, String) {
    let lines = parse().collect::<Vec<_>>();

    let p1 = p1(&lines);
    let p2 = p2(&lines);
    (format!("{}", p1), format!("{}", p2))
}
