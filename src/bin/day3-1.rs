use std::collections::HashSet;
use advent::read_lines;

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("test3.txt") {
        for line in lines {
            if let Ok(l) = line {
                let(compart1, compart2) = l.split_at(l.len() / 2);

                let unique_a = compart1.chars().collect::<HashSet<_>>();
                let unique_b = compart2.chars().collect::<HashSet<_>>();

                let c = unique_a.intersection(&unique_b).collect::<Vec<_>>();

                println!("{:?}", c.first().unwrap());
            }
        }

    }
}

