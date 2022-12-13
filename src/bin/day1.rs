use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("input.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut elf_cals: Vec<i32> = Vec::new();
        let mut total_cals: Vec<i32> = Vec::new();
        for line in lines {
            if let Ok(calories) = line {
                if calories == "" {
                    let sum:i32 = elf_cals.iter().sum();
                    total_cals.push(sum);
                    elf_cals.clear();
                } else {
                    let line_cal: i32 = calories.parse().unwrap();
                    elf_cals.push(line_cal);
                }
            }
        }
        total_cals.sort();
        let sum: i32 = total_cals.chunks(3).collect().first().unwrap().iter().sum();
        
        println!("{:?}", sum);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}