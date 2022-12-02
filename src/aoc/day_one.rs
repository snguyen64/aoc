use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::BinaryHeap;

/// AoC Day 1. Get the elf with the most calories!
/// Given a file which is a list of calories, where the elves are separated by an empty line
/// Find the elf with the most calories. How many total Calories is that elf carrying?
/// Find the sum of the top 3 elves holding the most calories. 
/// https://adventofcode.com/2022/day/1
pub fn get_highest_calories() {
    println!("Executing AoC Day One Problem");
    let path = "day_one.txt";
    let f = File::open(path).expect("Unable to open file");
    let br = BufReader::new(f);
    let mut current_elf_calories: u32 = 0;
    let mut bh: BinaryHeap<u32> = BinaryHeap::with_capacity(3);

    br.lines().for_each(|l| {
        // Here's another way to do it
        if let Ok(line) = l {
            match line.parse::<u32>() {
                Ok(n) => {
                    current_elf_calories += n;
                }
                Err (_) => {
                    bh.push(current_elf_calories);
                    current_elf_calories = 0;
                }
            }
        }
        // Here's 1 way to do it
        // let line = l.unwrap();
        // if !line.is_empty() {
        //     let line_calorie: u32 = line.parse().unwrap();
        //     current_elf_calories += line_calorie;
        // } else {
        //     bh.push(current_elf_calories);
        //     current_elf_calories = 0;
        // }
    });
    // fence post for last elf not covered by the loop
    bh.push(current_elf_calories);

    let top_elf = bh.pop().unwrap(); // part one
    let top_three_sum = top_elf + bh.pop().unwrap() + bh.pop().unwrap(); // part 2
    println!("Most calories an elf is holding: {top_elf}\nTop Three Elves Calorie Sum: {top_three_sum}");
}
