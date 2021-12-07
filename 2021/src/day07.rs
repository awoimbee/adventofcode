const INPUT: &str = include_str!("../input/day07.txt");

struct Crabs {
    crabs: Vec<i32>,
}

impl Crabs {
    fn new(input: &str) -> Self {
        let mut crabs = input
            .trim_end()
            .split(',')
            .map(|crab| crab.parse().unwrap())
            .collect::<Vec<_>>();
        crabs.sort_unstable();
        Self { crabs }
    }

    fn min_distance_p1(&self) -> i32 {
        self.crabs[self.crabs.len() / 2]
    }

    fn fuel_p1(&self, position: i32) -> i32 {
        self.crabs.iter().map(|pos| (pos - position).abs()).sum()
    }

    fn part1(&self) -> i32 {
        self.fuel_p1(self.min_distance_p1())
    }

    /// with `x=|crab_sub[i] - position|`; Crab fuel is: `((x+1)*x)/2 <=> (x^2 + x)/2`
    ///
    /// mean minimizes `sum(x^2)` (https://math.stackexchange.com/questions/967138/formal-proof-that-mean-minimize-squared-error-function).
    ///
    /// we minimize `sum((x^2 + x) / 2)`.
    ///
    /// *close enough !*
    ///
    /// The minimum distance is either the `floor()` or the `ceil()` of the mean.
    fn min_distance_p2(&self) -> (i32, i32) {
        let mean = self.crabs.iter().map(|&c| c as f32).sum::<f32>() / self.crabs.len() as f32;
        (mean.floor() as i32, mean.ceil() as i32)
    }

    fn fuel_p2(&self, position: i32) -> i32 {
        self.crabs
            .iter()
            .map(|pos| {
                let dist = (pos - position).abs();
                ((dist + 1) * dist) / 2
            })
            .sum()
    }

    fn part2(&self) -> i32 {
        let (a, b) = self.min_distance_p2();

        let af = self.fuel_p2(a);
        let bf = self.fuel_p2(b);
        af.min(bf)
    }
}

pub fn day07() -> (String, String) {
    let crabs = Crabs::new(INPUT);
    let part1 = crabs.part1();
    let part2 = crabs.part2();

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_min_distance_part_1_test_input() {
        let crabs = Crabs::new(TEST_INPUT);
        assert_eq!(crabs.min_distance_p1(), 2);
    }

    #[test]
    fn test_fuel_needed_part_1_test_input() {
        let crabs = Crabs::new(TEST_INPUT);
        assert_eq!(crabs.fuel_p1(2), 37);
    }

    #[test]
    fn test_min_distance_part_2_test_input() {
        let crabs = Crabs::new(TEST_INPUT);
        let res = crabs.min_distance_p2();
        assert_eq!(res.0, 4);
        assert_eq!(res.1, 5);
    }

    #[test]
    fn test_min_distance_part_2() {
        let crabs = Crabs::new(INPUT);
        let res = crabs.min_distance_p2();
        assert_eq!(res.0, 446);
        assert_eq!(res.1, 447);
    }

    #[test]
    fn test_fuel_needed_part_2_test_input() {
        let crabs = Crabs::new(TEST_INPUT);
        assert_eq!(crabs.fuel_p2(5), 168);
    }

    #[test]
    fn test_fuel_needed_part_2() {
        let crabs = Crabs::new(INPUT);
        assert_eq!(crabs.fuel_p2(446), 87640209);
    }
}
