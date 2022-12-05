use crate::solution::{load, Solution};
use std::collections::VecDeque;
use std::io::{BufRead, BufReader, Lines};
use std::fs::File;
use itertools::Itertools;
use std::cell::RefCell;
use regex::Regex;

pub struct Day05;

const YEAR: u32 = 2022;
const DAY: u32 = 5;

impl Solution for Day05 {
    fn name(&self) -> String {
        "Supply Stacks".to_string()
    }

    /// Part A
    /// Given an input of 3 stacks and some move commands.
    /// When moving multiple, treat it as a stack. It will move 1 at a time.
    /// Which crate will end up on the top of each stack?
    /// 
    /// The result should be each a string of each character. No delimiters.
    fn part_a(&self) {
        let br = load(YEAR, DAY);
        let lines = br.lines();
        let (lines, stacks) = read_stacks(lines);

        // println!("{:?}", stacks);
        // now read the move set and operate on the stacks
        lines.for_each(|l| {
            if let Ok(l) = l {
                do_moves(&stacks, l)
            }
        });

        // println!("{:?}", stacks);
        get_top_of_stacks(&stacks);        
    }

    /// Part B
    /// Same as Part A except: This time it can pick up and move multiple crates at once...
    fn part_b(&self) {
        let br = load(YEAR, DAY);
        let lines = br.lines();
        let (lines, stacks) = read_stacks(lines);

        // println!("{:?}", stacks);

        // now read the move set and operate on the stacks
        lines.for_each(|l| {
            if let Ok(l) = l {
                do_moves_b(&stacks, l)
            }
        });

        // println!("{:?}", stacks);
        get_top_of_stacks(&stacks);    
    }
}

fn get_top_of_stacks(stacks: &Stacks) {
    let mut res: String = String::from("");
    let stacks = stacks.stacks.borrow_mut();
    for (n, s) in stacks.iter().enumerate() {
        // would it be 0 or the other side?
        let c = s.borrow_mut().get(0).unwrap().clone();
        print!("Top of Stack: {} is {}, ", n, c);
        res.push(c);
    }
    println!("\nResult: {res}");
}

fn do_moves(stacks: &Stacks, moves: String) {
    // get moves as [x,y,z]
    let nums = moves.trim_matches(char::is_alphabetic).trim();
    let reg = Regex::new(r"[a-zA-Z\s]+").unwrap();
    let split: Vec<_> = reg
        .split(nums)
        .map(|d| d.parse::<usize>().unwrap())
        .collect();

    let n = *split.get(0).unwrap();
    let f_index = *split.get(1).unwrap();
    let t_index = *split.get(2).unwrap();

    // move x from s1 to s2
    let s = stacks.stacks.borrow_mut();
    let from_stack = s.get(f_index - 1).unwrap();
    let to_stack = s.get(t_index - 1).unwrap();
    for _i in 0..n {
        println!("Moving: {} from {} to {}", n, split.get(1).unwrap(), split.get(2).unwrap());
        let popped = from_stack.borrow_mut().pop_front();
        match popped {
            Some(p) => {
                to_stack.borrow_mut().push_front(p);
            },
            None => {}
        }
    }
}

fn do_moves_b(stacks: &Stacks, moves: String) {
    // get moves as [x,y,z]
    let nums = moves.trim_matches(char::is_alphabetic).trim();
    let reg = Regex::new(r"[a-zA-Z\s]+").unwrap();
    let split: Vec<_> = reg
        .split(nums)
        .map(|d| d.parse::<usize>().unwrap())
        .collect();

    let n = *split.get(0).unwrap();
    let f_index = *split.get(1).unwrap();
    let t_index = *split.get(2).unwrap();

    // move x from s1 to s2
    let s = stacks.stacks.borrow_mut();
    let from_stack = s.get(f_index - 1).unwrap();
    let to_stack = s.get(t_index - 1).unwrap();
    let mut middle_man: VecDeque<char> = VecDeque::new();
    for _i in 0..n {
        println!("Moving: {} from {} to {}", n, split.get(1).unwrap(), split.get(2).unwrap());
        let popped = from_stack.borrow_mut().pop_front();
        match popped {
            Some(p) => {
                middle_man.push_back(p);
            },
            None => {}
        }
    }
    for _i in 0..middle_man.len() {
        to_stack.borrow_mut().push_front(middle_man.pop_back().unwrap())  
    }
}

// reads the inputs in to stacks
fn read_stacks(mut lines: Lines<BufReader<File>>) -> (Lines<BufReader<File>>, Stacks) {
    // y = 3x + x-1 
    // so if you have 3 stacks, you have 11 characters
    // reversely, if you have 11 characters, you have 3 stacks
    // 
    // So given a line input of length 11
    // 11 = 3x + x - 1
    // to get x, we add 1 to length and divid by 4
    // We can split the line on every 4th character until the end

    // then we can read from move
    let mut read_stack: bool = true;
    // create a new struct that contains a vec with a specific size
    let mut stacks: Stacks = Stacks::new();


    while read_stack {
        let line = lines.next().unwrap();
        if let Ok(l) = line {
            /*
            if contains [], we are in the stack
                move each into respective arr[queue.push,]
            if contains move, we are in instructs
            else, go next
            */
            if !l.contains("[") {
                read_stack = false;
            } else {
                let len = l.len();
                let num_stacks = (len + 1) / 4;
                if !stacks.is_initialized { stacks = Stacks::init(num_stacks); }
                let subs = l.chars()
                    .chunks(4).into_iter()
                    .map(|c| c.collect::<String>().chars().nth(1).unwrap())
                    .collect::<Vec<_>>();
                // println!("{:?}, len: {}, num_stacks: {}", subs, len, num_stacks);
                stacks.add_to_stack(subs)
            }
        }
    }
    lines.next(); // skip the empty line
    (lines, stacks)
}

#[derive(Debug)]
struct Stacks {
    stacks: RefCell<Vec<RefCell<VecDeque<char>>>>,
    is_initialized: bool,
}

impl Stacks {
    fn new() -> Stacks {
        Stacks { stacks: RefCell::new(vec![]), is_initialized: false, }
    }

    fn init(size: usize) -> Stacks {
        Stacks {
            stacks: RefCell::new(vec![RefCell::new(VecDeque::new()); size]), // init a vector(size) of vectors
            is_initialized: true,
        }
    }

    fn add_to_stack(&self, crates: Vec<char>) {
        for (i, c) in crates.iter().enumerate() {
            if !c.is_whitespace() {
                self.stacks.borrow_mut().get(i).unwrap().borrow_mut().push_back(*c);
            }
        }
    }
}