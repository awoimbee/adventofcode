#![feature(const_fn)]

use std::io::{self, BufRead};

#[derive(Clone, Copy, Debug)]
struct Xy {
	x: f32,
	y: f32,
}

#[derive(Clone, Copy, Debug)]
struct Ray {
	org: Xy,
	dir: Xy,
}

impl Xy {
	pub fn add(&self, b: &Xy) -> Self {
		Xy {
			x: self.x + b.x,
			y: self.y + b.y,
		}
	}
	pub fn sub(&self, b: &Xy) -> Self {
		Xy {
			x: self.x - b.x,
			y: self.y - b.y,
		}
	}
	pub fn div(&self, b: &Xy) -> Self {
		Xy {
			x: self.x / b.x,
			y: self.y / b.y,
		}
	}
	pub fn divf(&self, b: f32) -> Self {
		Xy {
			x: self.x / b,
			y: self.y / b,
		}
	}
	pub fn mod2(&self) -> f32 {
		self.x * self.x + self.y * self.y
	}
	pub fn mod1(&self) -> f32 { // mod is reserved
		self.x * self.x + self.y * self.y
	}
	pub fn norm(&self) -> Self {
		self.divf(self.mod1())
	}
}

// fuck terminology
fn closest(ray: Ray, asteroids: &[Xy]) -> f32 {
	let mut nearest = std::f32::MAX;
	for a in asteroids.iter() {
		let dist = (a.x - ray.org.x) / ray.dir.x;
		let new_y = ray.org.y + ray.dir.y * dist;
		if new_y > a.y - 0.0001 && new_y < a.y + 0.0001 && dist < nearest {
			nearest = dist;
		}
	}
	nearest
}

fn main() -> Result<(), std::io::Error> {
	let stdin = io::stdin();
	let mut input = Vec::new();
	// let mut asteroids = Vec::new();

	for line in stdin.lock().lines() {
		let line = line?;
		if line.len() == 0 { break };
		let line: Vec<_> = line.into_bytes().iter().map(|c| match c {b'#' => true, b'.' => false, _ => panic!("BAD INPUT")}).collect();
		input.push(line);
	}

	for line in input.iter() {
		for c in line.iter() {
			if *c == false { continue };
			for line_ in input.iter() {
				for c_ in line.iter() {
					if *c_ == false { continue };
					
				}
			}

		}
	}


	// // let mut direct_sight = Vec::new();
	// let mut max_direct_sight = 0;
	// // let mut asteroids = asteroids.iter();
	// // while let Some(ast) = asteroids.next() {
	// for ast in asteroids.iter() {
	// 	let mut direct_sight = 0;
	// 	for a in asteroids.iter() {
	// 		let dir = a.sub(&ast);
	// 		let dist = dir.mod1();
	// 		let ray = Ray {org: *ast, dir};
	// 		if closest(ray, asteroids.as_slice()) >= dist - 0.0001 {
	// 			direct_sight += 1;
	// 		}
	// 	}
	// 	if direct_sight > max_direct_sight {max_direct_sight = direct_sight};
	// 	println!("nb sight: {}", direct_sight);
	// }
	// println!("SFSF {}", max_direct_sight);
	Ok(())
}
