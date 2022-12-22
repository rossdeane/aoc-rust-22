use std::collections::HashSet;
use advent::read_lines;

fn build_assignment_set(pair: &str) -> HashSet<u32> {
    let sections: Vec<&str> = pair.split('-').collect();
    let range_start = sections[0].parse::<u32>().unwrap();
    let range_end = sections[1].parse::<u32>().unwrap();

    let assignment:HashSet<u32> = (range_start..range_end + 1).collect();
    return assignment;
}

fn main() {
    let mut total: usize = 0;
    
    if let Ok(lines) = read_lines("inputs/day4.txt") {
        for line in lines.flatten() {
            let pair: Vec<&str> = line.split(',').collect();
            let assignment1 = build_assignment_set(pair[0]);
            let assignment2:HashSet<u32> = build_assignment_set(pair[1]);

            let overlap = assignment1.intersection(&assignment2).collect::<Vec<&u32>>().len();
            if overlap > 0 {
                total += 1;
            }
        }
    }
    println!("Total assignments that overlap at all is: {:?}", total);

}

