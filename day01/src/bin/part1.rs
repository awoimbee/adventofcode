use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut fuel_req: u64 = 0;

    for line in stdin.lock().lines() {
        let line = line?;
        if line.len() == 0 {
            break;
        }
        let mass: u64 = line.parse().unwrap();
        fuel_req += mass / 3 - 2;
    }
    println!("Fuel required: {}", tot_fuel_req);

    Ok(())
}
