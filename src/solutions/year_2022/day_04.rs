use crate::solution::{load, Solution};
use std::io::BufRead;

pub struct Day04;

const YEAR: u32 = 2022;
const DAY: u32 = 4;

impl Solution for Day04 {
    fn name(&self) -> String {
        "Camp Cleanup".to_string()
    }

    /// Part A
    /// Given a list where each line is a pair of ranges
    /// In how many lines are there a complete overlap (one range fully contains the other)
    fn part_a(&self) {
        let br = load(YEAR, DAY);
        let mut total_overlaps = 0;
        br.lines().for_each(|l| {
            let l = l.unwrap();
            let (r1, r2) = l.split_once(",").unwrap();
            let (r1_a, r1_b) = extract_ranges(r1);
            let (r2_a, r2_b) = extract_ranges(r2);
            if (r1_a <= r2_a && r1_b >= r2_b) || (r2_a <= r1_a && r2_b >= r1_b) {
                total_overlaps += 1;
            }
        });
        println!("Total overlaps: {total_overlaps}");
    }

    /// Part B
    /// Now instead of completely overlapping, let's count on any kind of overlap at all
    fn part_b(&self) {
        let br = load(YEAR, DAY);
        let mut total_overlaps = 0;
        br.lines().for_each(|l| {
            let l = l.unwrap();
            let (r1, r2) = l.split_once(",").unwrap();
            let (a, b) = extract_ranges(r1);
            let (c, d) = extract_ranges(r2);
            if (a >= c && a <= d) || (b >= c && b <= d) || (c >= a && c <= b) || (d >= a && d <= b) {
                total_overlaps += 1;
            }
        });
        println!("Total overlaps: {total_overlaps}");
    }
}

fn extract_ranges(r: &str) -> (usize, usize) {
    let (r2_b, r2_c) = r.split_once("-").unwrap();
    let r2_b = r2_b.parse::<usize>().unwrap();
    let r2_c = r2_c.parse::<usize>().unwrap();
    (r2_b, r2_c)
}