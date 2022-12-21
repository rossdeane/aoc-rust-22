use std::{collections::HashSet};

use advent::read_lines;

fn main() {

    const ALPHA: &str = " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWYXZ";
    let mut total: usize = 0;
    
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("inputs/day3.txt") {

        let lines_array = lines.flatten().collect::<Vec<String>>();
        let elf_groups = lines_array.chunks(3);

        for (i, group) in elf_groups.enumerate() {
            let elf1 = group[0].chars().collect::<HashSet<_>>();
            let elf2 = group[1].chars().collect::<HashSet<_>>();
            let elf3 = group[2].chars().collect::<HashSet<_>>();

            let c = elf1.intersection(&elf2).cloned().collect::<HashSet<_>>();
            let d: Vec<&char> = c.intersection(&elf3).collect::<Vec<_>>();
            println!("Group {} badge is {:?}", i, d);

            for &ch in d {
                total += ALPHA.find(ch).unwrap(); 
            }
        }
    }

    println!("{}", total)
}

