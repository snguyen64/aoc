use crate::solution::{load, Solution};
use std::{io::BufRead, rc::Rc, cell::RefCell, collections::HashMap, borrow::{BorrowMut}};
use std::fmt;

pub struct Day07;

const YEAR: u32 = 2022;
const DAY: u32 = 7;

impl Solution for Day07 {
    fn name(&self) -> String {
        "No Space Left On Device".to_string()
    }

    /// Part A
    /// Find directories that are good candidates for deletion. (Determine total size of a directory)
    /// Find directories with total size of at most 100,000. Then calculate the sum of their total sizes.
    fn part_a(&self) {
        let br = load(YEAR, DAY);
        let lines = br.lines();
        let mut fs: FS = FS::new();
        // read our input into fs
        read_into(lines, &mut fs);
        fs.update_fs_dir_sizes();
        fs.get_directories_with_size_below(100000);
    }

    /// Part B
    /// We want to update our system but we dont have enough spacce
    /// 70000000 total disk space available
    /// We need at least 30000000 unused space
    /// Find the smallest directory that, if deleted, would free up space on the FS to run an update.
    fn part_b(&self) {
        let br = load(YEAR, DAY);
        let lines = br.lines();
        let mut fs: FS = FS::new();

        let total_available = 70000000;
        let min_free = 30000000;
        let total_limit = total_available - min_free;

        // read our input into fs
        read_into(lines, &mut fs);
        fs.update_fs_dir_sizes();
        let r = fs.root.borrow_mut();
        match r {
            Some(d) => {
                let cs = d.as_ref().borrow_mut().size;
                println!("Current size: {}, Limit: {}", cs, total_limit);
                if cs > total_limit {
                    let over_amt = cs - total_limit;
                    println!("We are over by: {}.. we need to free up at least this much space", over_amt);
                    fs.get_directories_with_size_above(over_amt);
                }
            },
            None => {}
        }
    }
}

fn read_into(lines: std::io::Lines<std::io::BufReader<std::fs::File>>, fs: &mut FS) {
    for (_i, l) in lines.enumerate().into_iter() {
        if let Ok(l) = l {
            let c_args: Vec<&str> = l.split_terminator(' ').collect();
            if c_args.get(0).unwrap().eq(&COMMAND) {
                let cmd = c_args.get(1).unwrap();
                if cmd.eq(&CHANGE_DIR) {
                    let arg = c_args.get(2).unwrap();
                    fs.cd(arg);
                } else if cmd.eq(&LIST) {
                    // no need, we process the other lines in the else statement
                }
            } else {
                // else it is an output and we should read until we get a command
                // if line starts with dir - then it is a directory and we should add it into the hashmap
                // if line starts with a number - then it is a file
                let arg0 = c_args.get(0).unwrap();
                let arg1 = c_args.get(1).unwrap();
                if arg0.eq(&DIR_KW) {
                    fs.add_dir(arg1)
                } else {
                    let size = arg0.parse::<usize>().unwrap();
                    fs.add_file_sizes(size);
                }
            }
        }     
    }
}

const COMMAND: &str = "$";
const CHANGE_DIR: &str = "cd";
const LIST: &str = "ls";
const PARENT_DIR: &str = "..";
const ROOT_DIR: &str = "/";
const DIR_KW: &str = "dir";

type Link = Option<Rc<RefCell<Dir>>>;

#[derive(Debug)]
struct Dir {
    name: String,
    size: usize,
    next: HashMap<String, Link>,
}

impl Dir {
    fn new(dir: String) -> Self {
        Self {
            name: dir,
            size: 0,
            next: HashMap::new(),
        }
    }

    fn update_total_size_helper(&mut self) {
        if self.next.len() == 0 {
            // do nothing
        } else {
            let mut total_size = 0;
            for (_k, v) in self.next.iter_mut() {
                if let Some(d) = v.borrow_mut() {
                    let mut drb = d.as_ref().borrow_mut();
                    drb.update_total_size_helper();
                    total_size += drb.size;
                }
            }
            self.size += total_size;
        }
    }

    fn get_sub_dir_with_size_limit(&mut self, limit: usize) -> usize {
        let mut sub_dir_limit_total = 0;
        if self.size < limit {
            sub_dir_limit_total += self.size;
        }
        if self.next.len() > 0 {
            for (_k, v) in self.next.iter_mut() {
                if let Some(d) = v.borrow_mut() {
                    let mut drb = d.as_ref().borrow_mut();
                    sub_dir_limit_total += drb.get_sub_dir_with_size_limit(limit);
                }
            }
        }
        sub_dir_limit_total
    }

    fn get_smallest_sub_dir_with_over_size(&mut self, limit: usize) -> usize {
        let mut min_over = 0;
        if self.size > limit {
            min_over = self.size;
        }
        if self.next.len() > 0 {
            for (_k, v) in self.next.iter_mut() {
                if let Some(d) = v.borrow_mut() {
                    let mut drb = d.as_ref().borrow_mut();
                    let sub_size = drb.get_smallest_sub_dir_with_over_size(limit);
                    if sub_size < min_over && sub_size > limit {
                        min_over = sub_size
                    }
                }
            }
        }
        min_over
    }
}

impl fmt::Display for Dir {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "Dir: {}, Size: {}, Children: {:?}", self.name, self.size, self.next.keys())
    }
}

#[derive(Debug)]
struct FS {
    root: Link,
    current: Vec<Link>,
}

impl FS {
    fn new() -> Self {
        let root = Some(Rc::new(RefCell::new(Dir::new("".to_string()))));
        let current = vec![root.clone()];
        Self {
            root,
            current,
        }
    }

    fn add_dir(&mut self, dir: &str) {
        let node = Dir::new(dir.to_string());
        // node.prev = self.current.clone();
        let node_ptr = Rc::new(RefCell::new(node));
        if let Some(c) = self.current.last().unwrap().borrow_mut() { // there should always be a root
            c.as_ref().borrow_mut()
                .next
                .insert(dir.to_string(), Some(Rc::clone(&node_ptr)));
        }
    }

    fn cd(&mut self, dir: &str) {
        if dir.eq(ROOT_DIR) {
            self.current = vec![self.root.borrow_mut().clone()];
        } else if dir.eq(PARENT_DIR) {
            // pop the stack
            self.current.pop();
        } else {
            let new_d = match self.current.last().unwrap() {
                Some(d) => {
                    let mut bm = d.as_ref().borrow_mut();
                    bm.next.borrow_mut().get(dir).unwrap().clone()
                },
                None => { None } // should not be the case here
            };
            self.current.push(new_d);
        }
    }

    fn add_file_sizes(&mut self, size: usize) {
        if let Some(d) = self.current.last().unwrap().borrow_mut() {
            d.as_ref().borrow_mut().size += size;
        }
    }

    fn update_fs_dir_sizes(&mut self) {
        if let Some(d) = self.root.borrow_mut() {
            let mut bm = d.as_ref().borrow_mut();
            bm.update_total_size_helper();
        }
    }

    fn get_directories_with_size_below(&mut self, limit: usize) {
        let mut total = 0;
        if let Some(d) = self.root.borrow_mut() {
            let mut bm = d.as_ref().borrow_mut();
            total = bm.get_sub_dir_with_size_limit(limit);
        }
        println!("Total: {}", total)
    }

    fn get_directories_with_size_above(&mut self, limit: usize) {
        if let Some(d) = self.root.borrow_mut() {
            let mut bm = d.as_ref().borrow_mut();
            let r =bm.get_smallest_sub_dir_with_over_size(limit);
            println!("Smallest sub size: {}", r);
        }
    }
}