const INPUT: &str = unsafe { std::str::from_utf8_unchecked(include_bytes!("../input/day03.txt")) };

#[derive(Debug)]
struct Rule {
    pub char: u8,
    pub min: u8,
    pub max: u8,
}

struct Pt<T> {
    pub x: T,
    pub y: T,
}
impl<T> Pt<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

fn parse() -> Vec<Vec<bool>> {
    INPUT
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.as_bytes().iter().map(|&c| c == b'#').collect())
        .collect()
}

fn check_slope(map: &[Vec<bool>], slope: &Pt<usize>) -> u32 {
    let mut pos = Pt::new(0, 0);
    let mut nb_tree = 0;

    while pos.y < map.len() {
        if map[pos.y][pos.x] {
            nb_tree += 1
        };
        pos.y += slope.y;
        pos.x = (pos.x + slope.x) % map[0].len();
    }
    nb_tree
}

pub fn day03() -> (String, String) {
    let input = parse();
    let p1 = check_slope(&input, &Pt::new(3, 1));

    let p2 = [Pt::new(1, 1), Pt::new(5, 1), Pt::new(7, 1), Pt::new(1, 2)]
        .iter()
        .map(|slope| check_slope(&input, slope))
        .product::<u32>()
        * p1;

    (format!("{}", p1), format!("{}", p2))
}
