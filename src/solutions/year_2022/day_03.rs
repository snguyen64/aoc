use crate::solution::{load, Solution};
use std::{io::BufRead};
use array_tool::vec::{Intersect};
use itertools::Itertools;

pub struct Day03;

const YEAR: u32 = 2022;
const DAY: u32 = 3;

impl Solution for Day03 {
    fn name(&self) -> String {
        "Rucksack Reorganization".to_string()
    }

    /// Part A
    /// Find the item type that appears in both compartments of each rucksack. What is the sum of the priorities of those item types?
    /// Each char is an item (case insensitive). a-z => prio 1-26. A=Z => prio 27-52.
    /// 
    /// We know that each compartment has the same # of items. (2 compartments per line). 1 line = 1 ransack.
    /// 
    fn part_a(&self) {
        let br = load(YEAR, DAY);
        let mut total_prio: usize = 0;
        br.lines().for_each(|l| {
            let mut array: [i32; 52] = [0; 52];
            let mut total_comp = 0;
            if let Ok(line) = l {
                // for each char in the line (that's not end line character...)
                let (comp_one, comp_two) = line.split_at(line.len() / 2);
                for c in comp_one.chars() {
                    let index = get_index(c);
                    array[index] += 1;
                }
                for c in comp_two.chars() {
                    let index = get_index(c);
                    if array[index] != 0 {
                        // found an item in first comp that was in the 2nd one
                        total_comp += index + 1;
                    }
                    array[index] = 0; // reset so no duplicates
                }
            }
            total_prio += total_comp;
        });
        println!("Total prio in the bag: {}", total_prio);        
    }
    
    /// Part B
    /// Search groups of 3 ransacks for the common item between the bags. This will be the id card.
    /// What is the sum of the priorities of every three-elf group?
    fn part_b(&self) {
        let br = load(YEAR, DAY);
        let mut total_prio: usize = 0;
        for chunk in br.lines().into_iter().chunks(3).into_iter() {
            let vec: Vec<String> = chunk.map(|item| {
                item.unwrap()
            }).collect();
            
            let x: Vec<char> = vec.get(0).unwrap().chars().collect();
            let y: Vec<char> = vec.get(1).unwrap().chars().collect();
            let z: Vec<char> = vec.get(2).unwrap().chars().collect();

            let intersection = x.intersect(y).intersect(z);

            let mut prio = 0;
            for i in intersection {
                prio += get_index(i) + 1;
            }
            total_prio += prio
        }
        println!("Total prio in the bag: {}", total_prio);  
    }
}

fn get_index(c: char) -> usize {
    if c.is_lowercase() {
        c as usize - 'a' as usize
    } else {
        c as usize - 'A' as usize + 26
    }
}