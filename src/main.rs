mod solutions;

// todo
// 1. Add command line to take inputs to run code
//      take inputs such as year, day, part, or all
// 2. Add time measurements to read executed code
// 3. Update the readme

use aoc2022::Solution;

fn main() {
    println!("Starting ~AoC 2022~");
    let year = 2022;
    let day = 1; // todo input and sub 1
    let solutions = solutions::get_year(year);
    let solution = match solutions.get(day) {
        Some(s) => s,
        None => {
            println!("No solution for the day and year");
            return;
        }
    };
    solution.part_a();
    solution.part_b();
}
