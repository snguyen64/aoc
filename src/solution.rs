use crate::solutions;

use std::io::BufReader;
use std::fs::File;

const BEGINNING_YEAR: u32 = 2022;
pub const CURRENT_YEAR: u32 = 2022;

/// Define a solution trait to solve for part a and b on each day
/// We use &self here to ensure that it is object safe
/// https://doc.rust-lang.org/reference/items/traits.html#object-safet
pub trait Solution {
    fn name(&self) -> String;
    fn part_a(&self);
    fn part_b(&self);
}

pub fn load(year: u32, day: u32) -> BufReader<File> {
    let filename = format!("data/{year}/day_{:02}.txt", day);
    let f = File::open(filename).expect("Unable to open input file");
    BufReader::new(f)
}

pub fn run_solutions(year: Option<u32>, day: Option<usize>, part: Option<char>) {      
    let (begin, end) = match year {
        Some(y) => {
            (y, y + 1)
        },
        None => {
            (BEGINNING_YEAR, CURRENT_YEAR + 1)
        }
    };
    for year in begin..(end) {
        println!("--Solutions for year: {year}--");
        let solutions = solutions::get_year(year);
        match day {
            Some(day) => {
                let solution = match solutions.get(day) {
                    Some(s) => {
                        s
                    },
                    None => {
                        println!("Solution not found");
                        return;
                    },
                };
                match part {
                    Some(part) => {
                        match part {
                            'a' => {
                                solution.part_a()
                            },
                            'b' => {
                                solution.part_b()
                            },
                            _ => println!("Challenge part does not exist"),
                        }
                    },
                    None => {
                        solution.part_a();
                        solution.part_b();
                    }
                }
                
            },
            None => {
                for (i, e) in solutions.iter().enumerate() {
                    println!("-Day {}:-", i + 1);
        
                    // if part was provided.. then execute the specific part..... else execute both
                    e.part_a();
                    e.part_b();
                }
            },
        };
    }
}
