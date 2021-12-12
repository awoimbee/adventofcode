use std::intrinsics::{likely, unlikely};

const INPUT: &str = include_str!("../input/day12.txt");

struct Cave {
    name: &'static str,
    links: Vec<usize>,
    is_big: bool,
}

impl Cave {
    fn new(name: &'static str) -> Self {
        debug_assert!(name.to_uppercase() == name || name.to_lowercase() == name);
        debug_assert!(!name.is_empty());
        Cave {
            name,
            links: Vec::new(),
            is_big: name.chars().next().unwrap().is_uppercase(),
        }
    }
}

struct Map {
    caves: Vec<Cave>,
    start: usize,
    end: usize,
}

impl Map {
    pub fn new(input: &'static str) -> Self {
        let mut map = Self {
            caves: Vec::new(),
            start: 0,
            end: 0,
        };

        for line in input.split_terminator('\n') {
            let link = line
                .split_once('-')
                .expect("Invalid input: could not split on '-'");
            let link_room = (
                map.get_or_create_room(link.0),
                map.get_or_create_room(link.1),
            );
            map.caves[link_room.0].links.push(link_room.1);
            map.caves[link_room.1].links.push(link_room.0);
        }

        map
    }

    fn get_or_create_room(&mut self, name: &'static str) -> usize {
        let i = self
            .caves
            .iter()
            .position(|r| r.name == name)
            .unwrap_or_else(|| {
                self.caves.push(Cave::new(name));
                self.caves.len() - 1
            });
        if name == "start" {
            self.start = i;
        } else if name == "end" {
            self.end = i;
        }
        i
    }

    pub fn _print(&self) {
        for room in &self.caves {
            println!("{}:", room.name);
            for link in &room.links {
                println!("  ->{}", self.caves[*link].name);
            }
        }
    }
}

struct TraversalData {
    map: Map,
    visited: Vec<usize>,
    visited_twice: Option<usize>,
}

impl TraversalData {
    fn new(map: Map) -> Self {
        Self {
            visited: Vec::with_capacity(map.caves.len() * 2),
            visited_twice: None,
            map,
        }
    }
    fn reset(&mut self) {
        self.visited.clear();
        self.visited_twice = None;
    }
    fn without_double_visits(&mut self) {
        self.visited_twice = Some(usize::MAX);
    }

    pub fn traverse(&mut self, id: usize) -> i32 {
        if unlikely(id == self.map.end) {
            return 1;
        }
        let cave = &self.map.caves[id];
        if likely(!cave.is_big && self.visited.contains(&id)) {
            if likely(self.visited_twice.is_some()) || unlikely(id == self.map.start) {
                return 0;
            }
            self.visited_twice = Some(id);
        }
        self.visited.push(id);
        let mut total = 0;
        let untracked_cave = unsafe { (cave as *const Cave).as_ref().unwrap() };
        for c in &untracked_cave.links {
            total += self.traverse(*c);
        }
        self.visited.pop();
        if self.visited_twice.contains(&id) {
            self.visited_twice = None;
        }
        total
    }
}

pub fn day12() -> (String, String) {
    // Part 1 is very fast
    // Part 2 in horribly slow

    let parsed = Map::new(INPUT);
    let mut ctx = TraversalData::new(parsed);
    // no double visit for part 1
    ctx.without_double_visits();
    let start = ctx.map.start;
    let part1 = ctx.traverse(start);
    ctx.reset();
    let part2 = ctx.traverse(start);
    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    const SMALL_TEST_INPUT: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    const MEDIUM_TEST_INPUT: &str = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

    const LARGE_TEST_INPUT: &str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    fn test_part_1(input: &'static str) -> i32 {
        let parsed = Map::new(input);
        let mut ctx = TraversalData::new(parsed);
        // no double visit for part 1
        ctx.without_double_visits();
        let start = ctx.map.start;
        ctx.traverse(start)
    }

    fn test_part_2(input: &'static str) -> i32 {
        let parsed = Map::new(input);
        let mut ctx = TraversalData::new(parsed);
        let start = ctx.map.start;
        ctx.traverse(start)
    }

    #[test]
    fn test_part_1_small_test_input() {
        assert_eq!(test_part_1(SMALL_TEST_INPUT), 10);
    }

    #[test]
    fn test_part_1_medium_test_input() {
        assert_eq!(test_part_1(MEDIUM_TEST_INPUT), 19);
    }

    #[test]
    fn test_part_1_large_test_input() {
        assert_eq!(test_part_1(LARGE_TEST_INPUT), 226);
    }

    #[test]
    fn test_part_2_small_test_input() {
        assert_eq!(test_part_2(SMALL_TEST_INPUT), 36);
    }

    #[test]
    fn test_part_2_medium_test_input() {
        assert_eq!(test_part_2(MEDIUM_TEST_INPUT), 103);
    }

    #[test]
    fn test_part_2_large_test_input() {
        assert_eq!(test_part_2(LARGE_TEST_INPUT), 3509);
    }
}
