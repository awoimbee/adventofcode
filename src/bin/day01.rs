use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut fuel_req_p1: u64 = 0;
    let mut fuel_req_p2 = 0;

    for line in stdin.lock().lines() {
        let line = line?;
        if line.len() == 0 {
            break;
        }
        let mut mass: u64 = line.parse().unwrap();
        fuel_req_p1 += mass / 3 - 2;
        while mass > 6 {
            mass = mass / 3 - 2;
            fuel_req_p2 += mass;
        }
    }
    println!("Part I  Fuel required: {}", fuel_req_p1);
    println!("Part II Fuel required: {}", fuel_req_p2);

    Ok(())
}
