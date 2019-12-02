use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut tot_fuel_req: i64 = 0;

    for line in stdin.lock().lines() {
        let line = line?;
        if line.len() == 0 {
            break;
        }
        let mut mass: i64 = line.parse().unwrap();
        while mass > 6 {
            mass = fuel_req / 3 - 2;
            tot_fuel_req += mass;
        }
    }
    println!("Fuel required: {}", tot_fuel_req);

    Ok(())
}
