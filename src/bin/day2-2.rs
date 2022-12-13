use advent::read_lines;

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
                    round_score = 0; //lose
                    round_score += 3; //scissors
                } else if your_go == "Y" {
                    round_score = 3; //draw
                    round_score += 1; //rock
                } else {
                    round_score = 6; // win
                    round_score += 2; //paper
                }
            } else if their_go == "B" {
                // They chose Paper
                if your_go == "X" {
                    round_score = 0; //lose
                    round_score += 1; //rock
                } else if your_go == "Y" {
                    round_score = 3; //draw
                    round_score += 2; //paper  
                } else {
                    round_score = 6; //win
                    round_score += 3; //scissors
                }
            } else {
                // They chose scissors
                if your_go == "X" {
                    round_score = 0; //lose
                    round_score += 2; //paper
                } else if your_go == "Y" {
                    round_score = 3; //draw 
                    round_score += 3; //scissors
                } else {
                    round_score = 6; //win
                    round_score += 1; //rock
                }
            }
            // }
            // println!("{}", round_score);
            all_scores.push(round_score);
        }
        println!("{}", all_scores.iter().sum::<i32>());
    }
}

