const INPUT: &str = unsafe { std::str::from_utf8_unchecked(include_bytes!("../input/day12.txt")) };
// E S W N
const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

struct Ship {
    p1_dir: (i32, i32),      // pos coeff (y, x)
    p1_pos: (i32, i32),      // y, x
    p2_pos: (i32, i32),      // y, x
    p2_waypoint: (i32, i32), // y, x
}
impl Ship {
    pub fn default() -> Self {
        Self {
            p1_dir: (0, 1),
            p1_pos: (0, 0),
            p2_pos: (0, 0),
            p2_waypoint: (-1, 10),
        }
    }
    fn forward(&mut self, val: i32) {
        self.p1_pos.0 += self.p1_dir.0 * val;
        self.p1_pos.1 += self.p1_dir.1 * val;
        self.p2_pos.0 += self.p2_waypoint.0 * val;
        self.p2_pos.1 += self.p2_waypoint.1 * val;
    }
    fn move_direction(&mut self, dir_id: usize, val: i32) {
        let dir = DIRECTIONS[dir_id];
        self.p1_pos.0 += dir.0 * val;
        self.p1_pos.1 += dir.1 * val;
        self.p2_waypoint.0 += dir.0 * val;
        self.p2_waypoint.1 += dir.1 * val;
    }
    fn rotate(&mut self, degrees: usize, rot: &dyn Fn(&mut (i32, i32))) {
        for _ in 0..((degrees % 360) / 90) {
            rot(&mut self.p1_dir);
            rot(&mut self.p2_waypoint);
        }
    }
    fn rotate_left(&mut self, degrees: usize) {
        self.rotate(degrees, &|angle| {
            let tmp = angle.0;
            angle.0 = -angle.1;
            angle.1 = tmp;
        });
    }
    fn rotate_right(&mut self, degrees: usize) {
        self.rotate(degrees, &|angle| {
            let tmp = angle.0;
            angle.0 = angle.1;
            angle.1 = -tmp;
        });
    }
    pub fn read_instr(&mut self, instr: &str) {
        let what = instr.as_bytes()[0];
        let val = (&instr[1..]).parse::<u32>().unwrap() as i32;
        match what {
            b'R' => self.rotate_right(val as usize),
            b'L' => self.rotate_left(val as usize),
            b'F' => self.forward(val),
            b'E' => self.move_direction(0, val),
            b'S' => self.move_direction(1, val),
            b'W' => self.move_direction(2, val),
            b'N' => self.move_direction(3, val),
            _ => unreachable!(),
        }
    }
    pub fn get_manhattan_dist1(&self) -> usize {
        (self.p1_pos.0.abs() + self.p1_pos.1.abs()) as usize
    }
    pub fn get_manhattan_dist2(&self) -> usize {
        (self.p2_pos.0.abs() + self.p2_pos.1.abs()) as usize
    }
}

fn parse() -> Ship {
    INPUT
        .split('\n')
        .filter(|s| !str::is_empty(s))
        .fold(Ship::default(), |mut boat, s| {
            boat.read_instr(s);
            boat
        })
}

pub fn day12() -> (String, String) {
    let boat = parse();

    let p1 = boat.get_manhattan_dist1();
    let p2 = boat.get_manhattan_dist2();

    (format!("{}", p1), format!("{}", p2))
}
