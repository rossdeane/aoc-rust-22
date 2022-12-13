use advent::read_lines;

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("inputs/day1.txt") {
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
        let sum: Vec<&[i32]> = total_cals.chunks(3).collect();
        println!("{}", sum.first().unwrap().iter().sum::<i32>());
    }
}
