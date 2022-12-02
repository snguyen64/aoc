use crate::aoc::{day_two::{self, part_two}};

pub mod aoc; // tells compiler to include code found in src/aoc.rs

// todo
// 1. add command line to take inputs to run code
//      take inputs such as year, day, part, or all
// 2. add modules for year and day
// 3. refactor
// 4. add time measurements to read executed code

fn main() {
    println!("Starting ~AoC 2022~");
    println!("Executing AoC Day One Problem");

    // execute day 1 code
    // aoc::DayChallenge::new(day_one::INPUT, get_highest_calories)
    //     .exec();

    // execute day 2 code
    aoc::DayChallenge::new(day_two::INPUT, part_two)
        .exec();
    // todo
}
