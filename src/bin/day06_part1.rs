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
    let mut nb = 0;
    for (_child, mut parent) in objects.values() {
        while let Some(p) = objects.get(parent) {
            parent = p;
            nb += 1;
        }
    }
    println!("Nb orbits: {}", nb);

    Ok(())
}
