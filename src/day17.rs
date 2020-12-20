const INPUT: &str = include_str!("../input/day17.txt");

const MAP_HALF_SIZE: usize = 13;
const MAP_SIZE: usize = MAP_HALF_SIZE * 2;
const MAP_SIZE_IDX: Idx = MAP_SIZE as Idx;

use bitvec::prelude::*;
use std::fmt::{write, Display};

type Idx = u8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pt3 {
    x: Idx,
    y: Idx,
    z: Idx,
}

impl Pt3 {
    const fn new(x: Idx, y: Idx, z: Idx) -> Self {
        Self { x, y, z }
    }
    const fn to_1d(&self) -> usize {
        self.x as usize
            + self.y as usize * MAP_SIZE as usize
            + self.z as usize * (MAP_SIZE as usize).pow(2)
    }
}

impl From<usize> for Pt3 {
    fn from(pt: usize) -> Self {
        let x = pt % MAP_SIZE;
        let y = (pt / MAP_SIZE) % MAP_SIZE;
        let z = pt / MAP_SIZE.pow(2);
        Self::new(x as Idx, y as Idx, z as Idx)
    }
}

#[derive(Debug, Clone)]
struct GameOfLife3D {
    map: BitVec,
}

impl GameOfLife3D {
    pub fn default() -> Self {
        Self {
            map: bitvec![0; MAP_SIZE * MAP_SIZE * MAP_SIZE],
        }
    }
    pub fn from_map(input: &str) -> Self {
        let mut map = Self::default();
        let mut pt = Pt3::new(
            MAP_HALF_SIZE as Idx,
            MAP_HALF_SIZE as Idx,
            MAP_HALF_SIZE as Idx,
        );

        input.lines().for_each(|l| {
            l.as_bytes().iter().for_each(|c| {
                if *c == b'#' {
                    *map.map.get_mut(pt.to_1d()).unwrap() = true
                }
                pt.x += 1;
            });
            pt.x = MAP_HALF_SIZE as Idx;
            pt.y += 1;
        });
        map
    }
    fn count_neighbors(&self, org: Pt3) -> usize {
        let mut neighbors = 0;
        let add = |a: Idx, b: i32| (a as i32 + b) as Idx;
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    if dx == 0 && dy == 0 && dz == 0 {
                    } else {
                        let pos = Pt3::new(add(org.x, dx), add(org.y, dy), add(org.z, dz));
                        if *self.map.get(pos.to_1d()).unwrap() {
                            neighbors += 1;
                        }
                    }
                }
            }
        }
        neighbors
    }
    pub fn run_cycle(&mut self) {
        let mut new_map = self.map.clone();
        for (x, y, z) in (1..MAP_SIZE_IDX - 1).flat_map(|x| {
            (1..MAP_SIZE_IDX - 1).flat_map(move |y| (1..MAP_SIZE_IDX - 1).map(move |z| (x, y, z)))
        }) {
            let cell_pos = Pt3::new(x, y, z);
            let idx = cell_pos.to_1d();
            let cell = *self.map.get(idx).unwrap();
            let nb_neighbors: usize = self.count_neighbors(cell_pos);
            if cell && nb_neighbors != 2 && nb_neighbors != 3 {
                new_map.set(idx, false);
            } else if !cell && nb_neighbors == 3 {
                new_map.set(idx, true);
            }
        }
        self.map = new_map;
    }
    pub fn nb_live(&self) -> usize {
        self.map.count_ones()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pt4 {
    x: Idx,
    y: Idx,
    z: Idx,
    w: Idx,
}

impl Pt4 {
    const fn new(x: Idx, y: Idx, z: Idx, w: Idx) -> Self {
        Self { x, y, z, w }
    }
    const fn to_1d(&self) -> usize {
        self.x as usize
            + self.y as usize * MAP_SIZE as usize
            + self.z as usize * (MAP_SIZE as usize).pow(2)
            + self.w as usize * (MAP_SIZE as usize).pow(3)
    }
}

impl From<usize> for Pt4 {
    fn from(pt: usize) -> Self {
        let x = pt % MAP_SIZE;
        let y = (pt / MAP_SIZE) % MAP_SIZE;
        let z = pt / MAP_SIZE.pow(2);
        let w = pt / MAP_SIZE.pow(3);
        Self::new(x as Idx, y as Idx, z as Idx, w as Idx)
    }
}

impl Display for Pt4 {
    fn fmt(&self, w: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write(
            w,
            format_args!(
                "{{x: {}, y: {}, z: {}, w: {}}}",
                self.x, self.y, self.z, self.w
            ),
        )
    }
}

#[derive(Debug, Clone)]
struct GameOfLife4D {
    map: BitVec,
}

impl GameOfLife4D {
    pub fn default() -> Self {
        Self {
            map: bitvec![0; MAP_SIZE * MAP_SIZE.pow(2) * MAP_SIZE.pow(3)],
        }
    }

    pub fn from_map(input: &str) -> Self {
        let mut map = Self::default();
        let mut pt = Pt4::new(
            MAP_HALF_SIZE as Idx,
            MAP_HALF_SIZE as Idx,
            MAP_HALF_SIZE as Idx,
            MAP_HALF_SIZE as Idx,
        );

        input.lines().for_each(|l| {
            l.as_bytes().iter().for_each(|c| {
                if *c == b'#' {
                    *map.map.get_mut(pt.to_1d()).unwrap() = true
                }
                pt.x += 1;
            });
            pt.x = MAP_HALF_SIZE as Idx;
            pt.y += 1;
        });
        map
    }

    fn count_neighbors(&self, org: Pt4) -> usize {
        let mut neighbors = 0;

        let add = |a: Idx, b: i32| (a as i32 + b) as Idx;
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    for dw in -1..=1 {
                        if dx == 0 && dy == 0 && dz == 0 && dw == 0 {
                        } else {
                            let pos = Pt4::new(
                                add(org.x, dx),
                                add(org.y, dy),
                                add(org.z, dz),
                                add(org.w, dw),
                            );
                            if *self.map.get(pos.to_1d()).unwrap() {
                                neighbors += 1;
                                if neighbors > 4 {
                                    return neighbors;
                                }
                            }
                        }
                    }
                }
            }
        }
        neighbors
    }

    pub fn run_cycle(&mut self) {
        let mut new_map = self.map.clone();
        for (x, y, z, w) in (1..MAP_SIZE_IDX - 1).flat_map(|x| {
            (1..MAP_SIZE_IDX - 1).flat_map(move |y| {
                (1..MAP_SIZE_IDX - 1)
                    .flat_map(move |z| (1..MAP_SIZE_IDX - 1).map(move |w| (x, y, z, w)))
            })
        }) {
            let cell_pos = Pt4::new(x, y, z, w);
            let idx = cell_pos.to_1d();
            let cell = *self.map.get(idx).unwrap();
            let nb_neighbors: usize = self.count_neighbors(cell_pos);
            if cell && nb_neighbors != 2 && nb_neighbors != 3 {
                new_map.set(idx, false);
            } else if !cell && nb_neighbors == 3 {
                new_map.set(idx, true);
            }
        }
        self.map = new_map;
    }
    pub fn nb_live(&self) -> usize {
        self.map.count_ones()
    }
}

pub fn day17() -> (String, String) {
    let mut gol_3d = GameOfLife3D::from_map(INPUT);

    (0..6).for_each(|_| gol_3d.run_cycle());
    let p1 = gol_3d.nb_live();

    let mut gol_4d = GameOfLife4D::from_map(INPUT);
    (0..6).for_each(|_| gol_4d.run_cycle());
    let p2 = gol_4d.nb_live();

    (format!("{}", p1), format!("{}", p2))
}
