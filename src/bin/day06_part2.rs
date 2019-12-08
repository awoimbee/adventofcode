use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut objects: HashMap<String, String> = HashMap::new();
    objects.insert("COM".to_owned(), "".to_owned());

    for line in stdin.lock().lines() {
        let line = line?;
        if line.is_empty() {
            break;
        };
        let (parent, child) = {
            let del = line.find(')').unwrap();
            (&line[..del], &line[del + 1..])
        };
        objects.insert(child.to_owned(), parent.to_owned());
    }
    let mut stantas_route = Vec::new();
    let mut my_route = Vec::new();

    let mut parent = "SAN";
    while let Some(p) = objects.get(parent) {
        parent = p;
        stantas_route.push(parent);
    }
    let mut parent = "YOU";
    while let Some(p) = objects.get(parent) {
        parent = p;
        my_route.push(parent);
    }

    let nb_steps = (|| {
        for (my_steps, me) in my_route.iter().enumerate() {
            for (santas_steps, santa) in stantas_route.iter().enumerate() {
                if me == santa {
                    return my_steps + santas_steps;
                }
            }
        }
        panic!("NO ROUTE FOUND BETWEEN ME & SANTA");
    })();

    println!("Nb steps: {}", nb_steps);

    Ok(())
}
