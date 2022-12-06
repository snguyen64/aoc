use crate::solution::{load, Solution};
use std::io::{BufRead};

pub struct Day06;

const YEAR: u32 = 2022;
const DAY: u32 = 06;

impl Solution for Day06 {
    fn name(&self) -> String {
        "Tuning Trouble".to_string()
    }

    /// Part A
    /// Identify the first position where the 4 most recently received characters were all different.
    /// How many characters need to be processed before the first start-of-packet marker is detected?
    /// -> the marker is the first character after the 4 unique characters.
    /// mjqjpqmgbljsphdztnvjfqwrcgsmlb
    /// jpqm 
    /// [Sliding window problem]
    fn part_a(&self) {
        let br = load(YEAR, DAY);
        let line = br.lines().next().unwrap().unwrap();
        let window_size = 4;
        // window_end index = size - 1
        let mut window_i = 0; // window start index
        // ensure that the window is within bounds
        while (window_i + window_size - 1) < line.len() {
            let window = line.get(window_i..window_i + window_size).unwrap();
            // println!("{:?}", window);
            /*
            given window [abab]
            this is invalid,
            we get the first index of the last invalid occurrence [1] + 1 - this is where we slide the window
            */
            let mut alpha: [i32; 26] = [-1; 26];
            let mut i = 0;
            let mut is_valid = true;
            for (_x, c) in window.chars().enumerate().into_iter() {
                let char_index = c as usize - 'a' as usize;
                if alpha[char_index] > -1 {
                    window_i += (alpha[char_index] + 1) as usize;
                    is_valid = false;
                    break;
                }
                alpha[char_index] = i;
                i += 1;
            }
            if is_valid {
                println!("Substring:{} and start of packet: {}", window, (window_i + window_size));
                return;
            }
        }
    }

    /// Part B
    /// Similar to Part A except with window size = 14.
    fn part_b(&self) {
        let br = load(YEAR, DAY);
        let line = br.lines().next().unwrap().unwrap();
        let window_size = 14;
        // window_end index = size - 1
        let mut window_i = 0; // window start index
        // ensure that the window is within bounds
        while (window_i + window_size - 1) < line.len() {
            let window = line.get(window_i..window_i + window_size).unwrap();
            let mut alpha: [i32; 26] = [-1; 26];
            let mut i = 0;
            let mut is_valid = true;
            for (_x, c) in window.chars().enumerate().into_iter() {
                let char_index = c as usize - 'a' as usize;
                if alpha[char_index] > -1 {
                    window_i += (alpha[char_index] + 1) as usize;
                    is_valid = false;
                    break;
                }
                alpha[char_index] = i;
                i += 1;
            }
            if is_valid {
                println!("Substring:{} and start of packet: {}", window, (window_i + window_size));
                return;
            }
        }
    }
}
