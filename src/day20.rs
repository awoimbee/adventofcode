#![allow(dead_code)]

const INPUT: &str = include_str!("../input/day20.txt");

const TILE_SIZE: usize = 10; // tiles are squares

use std::cell::UnsafeCell;

use bitvec::prelude::*;

struct Rotation(u8);

struct TileEdge {
    pub val: u16,
    pub rev: u16,
}

impl TileEdge {
    pub fn new<const SIDE: &'static str>(tile: &[[bool; 10]; 10]) -> Self {
        let mut val: u16 = 0;
        let val_window = val.view_bits_mut::<Lsb0>();
        match SIDE {
            "UP" => {
                for (i, px) in tile[0].iter().enumerate() {
                    if *px {
                        val_window.set(i, true);
                    }
                }
            }
            "DOWN" => {
                for (i, px) in tile[9].iter().enumerate() {
                    if *px {
                        val_window.set(i, true);
                    }
                }
            }
            "LEFT" => {
                for (i, t_l) in tile.iter().enumerate() {
                    if t_l[0] {
                        val_window.set(i, true);
                    }
                }
            }
            "RIGHT" => {
                for (i, t_l) in tile.iter().enumerate() {
                    if t_l[9] {
                        val_window.set(i, true);
                    }
                }
            }
            _ => unreachable!(),
        };
        let rev = val.reverse_bits() >> 6;
        // println!("Side {:5}: {:#b} {:#b}", SIDE, val, rev);
        Self { val, rev }
    }
}

struct Tile {
    pub id: u16,
    pub rot: Rotation,
    pub edges: [TileEdge; 4],
    pub matches: u8,
}

impl Tile {
    pub fn from_str(s: &str) -> Self {
        let mut lines = s.lines();
        let id = lines.next().unwrap()[5..9].parse().unwrap();
        let tile = {
            let mut t = [[false; 10]; 10];
            lines.enumerate().for_each(|(y, l)| {
                l.as_bytes()
                    .iter()
                    .enumerate()
                    .for_each(|(x, c)| t[y][x] = *c == b'#');
            });
            t
        };
        let edges = [
            TileEdge::new::<"UP">(&tile),
            TileEdge::new::<"DOWN">(&tile),
            TileEdge::new::<"LEFT">(&tile),
            TileEdge::new::<"RIGHT">(&tile),
        ];

        Self {
            id,
            rot: Rotation(0),
            edges,
            matches: 0,
        }
    }
    pub unsafe fn matches(&mut self, tiles: &[UnsafeCell<Tile>]) {
        for t in tiles
            .into_iter()
            .map(|t| std::mem::transmute::<*mut Tile, &mut Tile>(t.get()))
        {
            for host_side in 0..4 {
                for guest_side in 0..4 {
                    if self.edges[host_side].val == t.edges[guest_side].val
                        || self.edges[host_side].rev == t.edges[guest_side].val
                    {
                        self.matches += 1;
                    }
                }
            }
        }
    }
}

fn parse() -> Vec<UnsafeCell<Tile>> {
    INPUT
        .split("\n\n")
        .map(|t| UnsafeCell::new(Tile::from_str(t)))
        .collect()
}

pub fn day20() -> (String, String) {
    let mut tiles = parse();

    for t in tiles.iter() {
        let tt = unsafe { std::mem::transmute::<*mut Tile, &mut Tile>(t.get()) };

        // println!("{}", tt.matches);
    }

    // let int: u16 = 1;
    // println!("{:#x} -> {:#x}", int, int.reverse_bits() >> 15);

    let p1 = "undefined";
    let p2 = "undefined";
    // let p1 = p1(&rules, &data);
    // let p2 = p2(&mut rules, &data);

    (p1.to_string(), p2.to_string())
}
