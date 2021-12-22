const INPUT: &str = include_str!("../input/day17.txt");

#[derive(Debug)]
struct Parsed {
    left: i64,
    right: i64,
    up: i64,
    down: i64,
}

type Point = (i64, i64);
type Velocity = (i64, i64);

impl Parsed {
    fn from_input(input: &str) -> Self {
        let val_str = input.trim_end().split_once(": ").unwrap().1;
        let (x_str, y_str) = val_str.split_once(", ").unwrap();
        let (xa, xb) = x_str[2..].split_once("..").unwrap();
        let (ya, yb) = y_str[2..].split_once("..").unwrap();
        let left = xa.parse().unwrap();
        let right = xb.parse().unwrap();
        let up = yb.parse().unwrap();
        let down = ya.parse().unwrap();
        debug_assert!(left < right);
        debug_assert!(up > down);

        Self {
            left,
            right,
            up,
            down,
        }
    }
    fn contains(&self, pt: &Point) -> bool {
        self.left <= pt.0 && pt.0 <= self.right && pt.1 <= self.up && pt.1 >= self.down
    }
}

fn simulate_point(initial_velocity: Velocity, bounds: &Parsed) -> Option<i64> {
    let mut point = (0, 0);
    let mut velocity = initial_velocity;
    let mut y_max = point.1;

    // terminate if point has overshot bounds
    while point.0 <= bounds.right && point.1 >= bounds.down {
        point = (point.0 + velocity.0, point.1 + velocity.1);
        y_max = point.1.max(y_max);

        if bounds.contains(&point) {
            return Some(y_max);
        } else {
            velocity = (0.max(velocity.0 - 1), velocity.1 - 1);
        }
    }

    None
}

fn find_hits(bounds: &Parsed) -> (i64, usize) {
    let mut max_y = i64::MIN;
    let mut nb_hits = 0;

    for vx in 0..=bounds.right {
        let vy_range = bounds.down..=(-bounds.down);
        for vy in vy_range {
            if let Some(y) = simulate_point((vx, vy), bounds) {
                max_y = max_y.max(y);
                nb_hits += 1;
            }
        }
    }
    (max_y, nb_hits)
}

fn solve(input: &Parsed) -> (i64, usize) {
    find_hits(input)
}

pub fn day17() -> (String, String) {
    let parsed = Parsed::from_input(INPUT);
    let (part1, part2) = solve(&parsed);

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simulate_point() {
        let p = Parsed::from_input("target area: x=20..30, y=-10..-5");
        let res = simulate_point((7, 2), &p);
        assert!(res.is_some());
        let res = simulate_point((6, 3), &p);
        assert!(res.is_some());
        let res = simulate_point((9, 0), &p);
        assert!(res.is_some());
        let res = simulate_point((17, -4), &p);
        assert!(res.is_none());
        let res = simulate_point((6, 9), &p);
        assert!(matches!(res, Some(45)));

        let res = simulate_point((6, 0), &p);
        assert!(res.is_some());
    }

    #[test]
    fn test_part1_test_input() {
        let p = Parsed::from_input("target area: x=20..30, y=-10..-5");
        let (p1, _) = solve(&p);
        assert_eq!(p1, 45);
    }

    #[test]
    fn test_part1() {
        let p = Parsed::from_input(INPUT);
        let (p1, _) = solve(&p);
        assert_eq!(p1, 12246);
    }

    #[test]
    fn test_part2() {
        let p = Parsed::from_input(INPUT);
        let (p1, _) = solve(&p);
        assert_eq!(p1, 3528);
    }

    #[test]
    fn test_part2_test_input() {
        let p = Parsed::from_input("target area: x=20..30, y=-10..-5");
        let (_, p2) = solve(&p);
        assert_eq!(p2, 112);
    }
}
