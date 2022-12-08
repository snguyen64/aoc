use crate::solution::{load, Solution};
use std::{io::BufRead};

pub struct Day08;

const YEAR: u32 = 2022;
const DAY: u32 = 08;

// TODO - see how other people have implemented it
impl Solution for Day08 {
    fn name(&self) -> String {
        "Treetop Tree House".to_string()
    }

    /// Part A
    /// Input is a grid, where each digit is a tree size. (0 smallest - 9 largest)
    /// A tree is visible if it is taller than the trees between [current tree] and the edge of the grid (cardinal directions)
    /// All trees around the edge are visible.
    /// How many trees are visible?
    fn part_a(&self) {
        let mut visible: usize = 0;
        let trees = load_in_trees();
        let height = trees.len();
        let width = trees[0].len();
        let tree_ref = &trees;
        
        for (r, row) in tree_ref.iter().enumerate() {
            for (c, _col) in row.iter().enumerate() {
                if is_bound(height, width, (r, c)) {
                    visible += 1;
                } else {
                    if check_nsew(&trees, (r, c)) {
                        visible += 1;
                    }
                }
            }
        }

        println!("Number visible: {}", visible);
    }

    /// Part B
    fn part_b(&self) {
        let mut max_tree_score: usize = 0;
        let trees = load_in_trees();
        let tree_ref = &trees;
        
        for (r, row) in tree_ref.iter().enumerate() {
            for (c, _col) in row.iter().enumerate() {
                let ss = get_scenic_score(&trees, (r, c));
                max_tree_score = if max_tree_score > ss { max_tree_score } else { ss };
            }
        }

        println!("Max scenic score: {}", max_tree_score);
    }
}

fn load_in_trees() -> Vec<Vec<u32>> {
    let br = load(YEAR, DAY);
    let mut trees: Vec<Vec<u32>> = vec![];
    br.lines().for_each(|line| {
        if let Ok(l) = line {
            let tree_row: Vec<u32> = l.chars()
                .map(|c| { c.to_digit(10).unwrap() } )
                .collect();
            trees.push(tree_row);
        }
    });
    trees
}

fn is_bound(height: usize, width: usize, (x,y): (usize, usize)) -> bool {
    return x <= 0 || y <= 0 || x >= height - 1 || y >= width - 1;
}

fn check_nsew(trees: &Vec<Vec<u32>>, (r, c): (usize, usize)) -> bool {
    let mut visible = false;
    // check west
    for n in (0..c).rev() {
        if trees[r][n] >= trees[r][c] {
            visible = false;
            break;
        } else {
            visible = true;
        }
    }
    if visible { return true; }
    // check east
    for s in (c + 1)..trees[0].len() {
        if trees[r][s] >= trees[r][c] {
            visible = false;
            break;
        } else {
            visible = true;
        }
    }
    if visible { return true; }
    // check south
    for e in (r + 1)..trees.len() {
        if trees[e][c] >= trees[r][c] {
            visible = false;
            break;
        } else {
            visible = true;
        }
    }
    if visible { return true; }
    // check north
    for w in (0..r).rev() {
        if trees[w][c] >= trees[r][c] {
            visible = false;
            break;
        } else {
            visible = true;
        }
    }
    visible
}

fn get_scenic_score(trees: &Vec<Vec<u32>>, (r, c): (usize, usize)) -> usize {
    let mut t_count = 1;
    // check west
    for n in (0..c).rev() {
        if trees[r][n] >= trees[r][c] || is_bound(trees.len(), trees[0].len(), (r,n)) {
            break;
        }
        t_count += 1;
    }
    let west = t_count;
    t_count = 1;
    // check east
    for s in (c + 1)..trees[0].len() {
        if trees[r][s] >= trees[r][c] || is_bound(trees.len(), trees[0].len(), (r,s)){
            break;
        }
        t_count += 1;
    }
    let east = t_count;
    t_count = 1;
    // check south
    for e in (r + 1)..trees.len() {
        if trees[e][c] >= trees[r][c] || is_bound(trees.len(), trees[0].len(), (e,c)) {
            break;
        }
        t_count += 1;
    }
    let south = t_count;
    t_count = 1;
    // check north
    for w in (0..r).rev() {
        if trees[w][c] >= trees[r][c] || is_bound(trees.len(), trees[0].len(), (w,c)) {
            break;
        }
        t_count += 1;
    }
    let north = t_count;
    return west * north * east * south;
}
