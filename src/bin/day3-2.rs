use std::{collections::HashSet};

use advent::read_lines;

fn main() {
    const ALPHA: &str = " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWYXZ";
    let mut total: usize = 0;
    
    if let Ok(lines) = read_lines("inputs/day3.txt") {
        let lines_vec = lines.flatten().collect::<Vec<String>>();
        let elf_groups = lines_vec.chunks(3);

        for group in elf_groups {
            // need to make this owned so we can pop it
            let mut g = group.to_owned();
            // initialise the prev_inventory to the first elf in the group, and remove them from the group
            let mut prev_inventory: HashSet<char> = g.pop().unwrap().chars().collect();
            
            // iteratively find the interesection between this and the last intersection, this should work for any amount of elves in a group
            for elf in g {
                let inventory:HashSet<char> = elf.chars().collect();
                prev_inventory = prev_inventory.intersection(&inventory).cloned().collect();
            }

            // By this point there should only be one char in the vector anyway, so lets just get it out
            let badge_vec = prev_inventory.into_iter().collect::<Vec<char>>();
            let badge = badge_vec.first().unwrap();
            println!("Group badge is {}", badge);
            
            // find the char in the list to get priority number
            total += ALPHA.find(*badge).unwrap(); 
        }
    }

    println!("{}", total)
}

