use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../input/day24.txt");

type InnerPt2 = i16;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(align(1))]
struct Pt2 {
    pub x: InnerPt2,
    pub y: InnerPt2,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct PackedPt2(u32);

impl PackedPt2 {
    pub fn unpack(self) -> Pt2 {
        unsafe { std::mem::transmute::<Self, Pt2>(self) }
    }
}

impl Pt2 {
    pub const fn default() -> Self {
        Self { x: 0, y: 0 }
    }
    pub const fn new(x: InnerPt2, y: InnerPt2) -> Self {
        Self { x, y }
    }
    pub const fn pack(self) -> PackedPt2 {
        unsafe { std::mem::transmute::<Self, PackedPt2>(self) }
    }
}

impl std::ops::AddAssign<Self> for Pt2 {
    fn add_assign(&mut self, b: Self) {
        self.x += b.x;
        self.y += b.y;
    }
}

impl std::ops::AddAssign<(InnerPt2, InnerPt2)> for Pt2 {
    fn add_assign(&mut self, b: (InnerPt2, InnerPt2)) {
        self.x += b.0;
        self.y += b.1;
    }
}

impl std::ops::Add<Self> for Pt2 {
    type Output = Self;
    fn add(mut self, b: Pt2) -> Self::Output {
        self.x += b.x;
        self.y += b.y;
        self
    }
}

fn parse() -> HashSet<PackedPt2> {
    // axial coordinates (https://gamedevelopment.tutsplus.com/tutorials/introduction-to-axial-coordinates-for-hexagonal-tile-based-games--cms-28820)
    let raw_tiles = INPUT.lines().map(|l| {
        let lbytes = l.as_bytes();
        let mut pos = Pt2::default();
        let mut i = 0;
        while i < l.len() {
            pos += match lbytes[i] {
                b0 @ (b'n' | b's') => {
                    i += 1;
                    match (b0, lbytes[i]) {
                        (b'n', b'e') => (0, -1),
                        (b'n', b'w') => (1, -1),
                        (b's', b'e') => (-1, 1),
                        (b's', b'w') => (0, 1),
                        _ => unreachable!(),
                    }
                }
                b'e' => (-1, 0),
                b'w' => (1, 0),
                _ => unreachable!(),
            };
            i += 1;
        }
        pos.pack()
    });
    let mut tiles = HashSet::new();
    for t in raw_tiles {
        if !tiles.remove(&t) {
            tiles.insert(t);
        }
    }
    tiles
}

const NEIGHBORS: [Pt2; 6] = [
    Pt2::new(0, -1),
    Pt2::new(1, -1),
    Pt2::new(1, 0),
    Pt2::new(0, 1),
    Pt2::new(-1, 1),
    Pt2::new(-1, 0),
];

fn p2(mut tiles: HashSet<PackedPt2>) -> u64 {
    const NB_DAYS: u64 = 100;

    for _ in 0..NB_DAYS {
        let mut new_tiles = HashSet::new();
        let mut white_neighbors = HashMap::new();

        for t in tiles.iter() {
            let nb_neighbors = {
                let pos = t.unpack();
                let mut nb = 0;
                for neighbor in NEIGHBORS.iter().map(|pt| (pos + *pt).pack()) {
                    if tiles.contains(&neighbor) {
                        nb += 1;
                    } else {
                        white_neighbors
                            .entry(neighbor)
                            .and_modify(|v| *v += 1)
                            .or_insert(1);
                    }
                }
                nb
            };
            if nb_neighbors == 1 || nb_neighbors == 2 {
                new_tiles.insert(*t);
            }
        }
        for (t, _) in white_neighbors
            .drain()
            .filter(|(_, nb_neighbors)| *nb_neighbors == 2)
        {
            new_tiles.insert(t);
        }
        tiles = new_tiles;
    }
    tiles.len() as u64
}

fn p1(tiles: &HashSet<PackedPt2>) -> u64 {
    tiles.len() as u64
}

pub fn day24() -> (String, String) {
    let tiles = parse();

    let p1 = p1(&tiles);
    let p2 = p2(tiles);

    (p1.to_string(), p2.to_string())
}

mod test {
    use super::*;

    #[test]
    fn pack_unpack() {
        for x in -128..=127 {
            for y in -128..=127 {
                let pt = Pt2::new(x, y);
                let packed = pt.pack();
                assert!(pt == packed.unpack());
            }
        }
    }

    #[test]
    fn correct_output() {
        let (p1, p2) = day24();
        assert!(p1 == "485");
        assert!(p2 == "3933");
    }
}
