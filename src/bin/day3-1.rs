use std::collections::HashSet;
use advent::read_lines;

fn main() {

    const ALPHA: &str = " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWYXZ";
    let mut total: usize = 0;
    
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("inputs/day3.txt") {
        for line in lines {
            if let Ok(l) = line {
                let(compart1, compart2) = l.split_at(l.len() / 2);

                let unique_a = compart1.chars().collect::<HashSet<_>>();
                let unique_b = compart2.chars().collect::<HashSet<_>>();

                let c: Vec<&char> = unique_a.intersection(&unique_b).collect::<Vec<_>>();

                for &ch in c {
                    total += ALPHA.find(ch).unwrap(); 
                }
            }
        }

    }

    println!("{}", total)
}

