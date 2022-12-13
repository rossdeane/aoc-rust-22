use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("input2.txt") {
        let mut all_scores: Vec<i32> = Vec::new();
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            // if let Ok(calories) = line: string {
            let foo = line.unwrap();
            let strat: Vec<&str> = foo.split(" ").collect();
            let your_go = strat[1];
            let their_go = strat[0];
            let mut round_score;
            
            if their_go == "A" {
                // They chose rock
                if your_go == "X" {
                    round_score = 1;
                    round_score += 3;
                } else if your_go == "Y" {
                    round_score = 2;
                    round_score += 6;
                } else {
                    round_score = 3;
                    round_score += 0;
                }
            } else if their_go == "B" {
                // They chose Paper
                if your_go == "X" {
                    round_score = 1;
                    round_score += 0;
                } else if your_go == "Y" {
                    round_score = 2;
                    round_score += 3;
                } else {
                    round_score = 3;
                    round_score += 6;
                }
            } else {
                // They chose scissors
                if your_go == "X" {
                    round_score = 1;
                    round_score += 6;
                } else if your_go == "Y" {
                    round_score = 2;
                    round_score += 0;
                } else {
                    round_score = 3;
                    round_score += 3;
                }
            }
            // }
            // println!("{}", round_score);
            all_scores.push(round_score);
        }
        println!("{}", all_scores.iter().sum::<i32>());
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}