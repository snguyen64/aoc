use aoc2022::load;
use crate::Solution;
use std::io::BufRead;
pub struct Day02;

const YEAR: u32 = 2022;
const DAY: u32 = 2;

// todo: is there a better solution for these parts?
// https://www.reddit.com/r/rust/comments/zanek7/aoc_day_2/
// see solutions for readability / performance / neat tricks and methods
impl Solution for Day02 {
    /// Given an "strat guide" with two columns,
    /// The first column will have A, B, or C (rock paper scissors)
    /// The second column will have X, Y, or Z (same as above)
    /// rock = 1, paper = 2, scissors = 3
    /// lose = 0, draw = 3, win = 6
    /// Assuming the strat guide is accurate, how many points will you get?
    fn part_a(&self) {
        let br = load(YEAR, DAY);
        let mut sum_points = 0;
        let mut matches = 0;
        br.lines().for_each(|l| {
            if let Ok(line) = l {
                let mut play = line.chars();
                let player_one = play.next().unwrap() as i32 - 'A' as i32 + 1;
                let player_two = play.next_back().unwrap() as i32 - 'X' as i32 + 1;
                let res = player_two - player_one;            

                let final_res = if res == 0 {
                    player_two + 3
                } else if res == 1 || res == -2 {
                    player_two + 6
                } else {
                    player_two
                };

                sum_points += final_res;
                matches += 1;
            }
        });
        println!("Here's the result of the match {sum_points} in {matches} matches");
    }

    /// Similar to part 1, except the X Y Z stands for lose, draw, and win - you want to lose some to not be sus
    /// Figure out how many points you get at the end of this run!
    fn part_b(&self) {
        let br = load(YEAR, DAY);
        let mut sum_points = 0;
        let mut matches = 0;
        br.lines().for_each(|l| {
            if let Ok(line) = l {
                let mut play = line.chars();
                let player_one = play.next().unwrap() as i32 - 'A' as i32 + 1;
                let game_res = play.next_back().unwrap();
                let pts = match game_res {
                    'X' => 0,
                    'Y' => 3,
                    'Z' => 6,
                    _ => 0,
                };

                let total = if pts == 3 {
                    pts + player_one
                } else if pts == 6 {
                    // win stuff
                    if player_one == 3 {
                        pts + 1
                    } else {
                        pts + player_one + 1
                    }
                } else {
                    if player_one == 1 {
                        pts + 3
                    } else {
                        player_one - 1
                    }
                };

                sum_points += total;
                matches += 1;
            }
        });
        println!("Here's the result of the match {sum_points} in {matches} matches");
    }
}