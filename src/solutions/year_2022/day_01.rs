use aoc2022::load;

use crate::Solution;
use std::io::{BufRead};
use std::collections::BinaryHeap;

pub struct Day01;

const YEAR: u32 = 2022;
const DAY: u32 = 1;

impl Solution for Day01 {
    /// Part A of Day 1 to get the top 1 elf holding the most calories
    fn part_a(&self) {
        let br = load(YEAR, DAY);
        let mut max_elf_calories: u32 = 0;
        let mut current_elf_calories: u32 = 0;

        br.lines().for_each(|l| {
            if let Ok(line) = l {
                match line.parse::<u32>() {
                    Ok(n) => {
                        current_elf_calories += n;
                    }
                    Err (_) => {
                        if max_elf_calories < current_elf_calories {
                            max_elf_calories = current_elf_calories;
                        }
                        current_elf_calories = 0;
                    }
                }
            }
        });

        println!("Max Calories for elf: {max_elf_calories}");
    }

    /// Part B of Day 1 to get the top 3 elves holding the most calories
    fn part_b(&self) {
        let br = load(YEAR, DAY);
        let mut current_elf_calories: u32 = 0;
        let mut bh: BinaryHeap<u32> = BinaryHeap::with_capacity(3);

        br.lines().for_each(|l| {
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
        });
        // fence post for last elf not covered by the loop
        bh.push(current_elf_calories);
        let top_three_sum = bh.pop().unwrap() + bh.pop().unwrap() + bh.pop().unwrap(); // part 2
        println!("Top Three Elves Calorie Sum: {top_three_sum}");
    }
}