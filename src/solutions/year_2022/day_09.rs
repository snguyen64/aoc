use crate::solution::{load, Solution};
use std::{io::BufRead};
use itertools::Itertools;
use linked_hash_set::LinkedHashSet;

pub struct Day09;

const YEAR: u32 = 2022;
const DAY: u32 = 9;

impl Solution for Day09 {
    fn name(&self) -> String {
        "Rope Bridge".to_string()
    }

    /// Part A
    /// Given an input with instructions for the head of the rope to move,
    /// How many positions does the tail of the rope visit at least once?
    /// Where the tail is always next to the head (cardinal/diagnol too).
    fn part_a(&self) {
        let mut visited: LinkedHashSet<(i32, i32)> = LinkedHashSet::new();
        visited.insert((0, 0));

        let mut h_pos: (i32, i32) = (0, 0);
        let mut t_pos: (i32, i32) = (0, 0);

        let br = load(YEAR, DAY);
        let h_ref: (&mut i32, &mut i32) = (&mut h_pos.0, &mut h_pos.1);
        let t_ref: (&mut i32, &mut i32) = (&mut t_pos.0, &mut t_pos.1);

        for line in br.lines() {
            if let Ok(l) = line {
                // println!("~{l}~");
                // read instructions and move
                let (dir, amt) = l.split_once(" ").unwrap();
                let amt = amt.parse::<i32>().unwrap();

                // operation to determine our move
                let op = match dir {
                    "R" | "U" => { 1 },
                    _ => { -1 }, // left and down are -1
                };
                
                // position of one of the values in head coordinates
                let is_x: bool = match dir {
                    "R" | "L" => true,
                    _ => false,
                };

                let mut begin_pos = if is_x {*h_ref.0} else {*h_ref.1} ;
                let final_pos = begin_pos + op * amt;
                
                // loop through until we move to the final position
                while begin_pos != final_pos {
                    begin_pos += op * 1;
                    // println!("  is_x: {is_x}: Moving {begin_pos} to {final_pos}");
                    // update head coordinates
                    if is_x { *h_ref.0 = begin_pos } else { *h_ref.1 = begin_pos };

                    // calculate the head and tail difference
                    let xdiff = *h_ref.0 - *t_ref.0;
                    let ydiff = *h_ref.1 - *t_ref.1;

                    if (xdiff.abs() > 1 && ydiff.abs() > 0) || (xdiff.abs() > 0 && ydiff.abs() > 1) {
                        if xdiff > 0 { *t_ref.0 += 1 } else { *t_ref.0 -= 1 }
                        if ydiff > 0 { *t_ref.1 += 1 } else { *t_ref.1 -= 1 }
                    } else if xdiff.abs() > 1 {
                        // move along the x axis
                        if xdiff > 0 { *t_ref.0 += 1 } else { *t_ref.0 -= 1 }
                    } else if ydiff.abs() > 1 {
                        // move along the y axis             
                        if ydiff > 0 { *t_ref.1 += 1 } else { *t_ref.1 -= 1 }           
                    }

                    visited.insert((*t_ref.0, *t_ref.1));
                    // println!("  Visited: {:?}\n-----------", visited);
                }
            }
        }
        println!("We visited {}", visited.len());
        // println!("We visited {:?}", visited);
    }

    /// Part B
    /// We have a longer rope with x knots now. What is the # of passes of the tail?
    /// The tail follows the same logic and path, except it wont move from the head this time,
    /// it'll be moved from the  knot it's next to.
    fn part_b(&self) {
        let num_knots = 10;
        let mut knots: Vec<Knots> = vec![Knots::new((0, 0)); num_knots];
        knots[0].is_head = true;

        let br = load(YEAR, DAY);

        for line in br.lines() {
            if let Ok(l) = line {
                // println!("Instruction: {}", &l);
                let (amt, op, is_x) = get_instructions(l);
                move_knots(is_x, op, amt, &mut knots);
            }
        }
        // println!("Head {:?}", knots[0]);
        // println!("Connected knot 2 {:?}", knots[1]);
        // println!("Connected knot 3 {:?}", knots[2]);
        println!("For the tail, we visited {:?}", knots[num_knots - 1].visited.len());
    }
}

fn get_instructions(l: String) -> (i32, i32, bool) {
    let (dir, amt) = l.split_once(" ").unwrap();
    let amt = amt.parse::<i32>().unwrap();
    // operation to determine our move
    let op = match dir {
        "R" | "U" => { 1 },
        _ => { -1 }, // left and down are -1
    };
    // position of one of the values in head coordinates
    let is_x: bool = match dir {
        "R" | "L" => true,
        _ => false,
    };
    (amt, op, is_x)
}

fn move_knots(is_x: bool, op: i32, amt: i32, knots: &mut Vec<Knots>) {
    let mut begin_pos = if is_x {knots[0].coords.0} else {knots[0].coords.1} ;
    let final_pos = begin_pos + op * amt;
    while begin_pos != final_pos {
        // move head
        begin_pos += op * 1;
        if is_x { knots[0].coords.0 = begin_pos } else { knots[0].coords.1 = begin_pos };

        for (a, b) in (0..knots.len()).tuple_windows() {
            let a_coords = knots[a].coords;
            let b_coords = knots[b].coords;
            let coords = get_connected_knot_new_coords(a_coords, b_coords);
            knots[b].coords = coords;
            knots[b].visited.insert(coords);
        }
    }
}

fn get_connected_knot_new_coords(knot_a: (i32, i32), knot_b: (i32, i32)) -> (i32, i32) {
    // calculate the head and tail difference
    let xdiff = knot_a.0 - knot_b.0;
    let ydiff = knot_a.1 - knot_b.1;

    let mut bx = knot_b.0;
    let mut by = knot_b.1;

    if (xdiff.abs() > 1 && ydiff.abs() > 0) || (xdiff.abs() > 0 && ydiff.abs() > 1) {
        if xdiff > 0 { bx += 1 } else { bx -= 1 }
        if ydiff > 0 { by += 1 } else { by -= 1 }
    } else if xdiff.abs() > 1 {
        // move along the x axis
        if xdiff > 0 { bx += 1 } else { bx -= 1 }
    } else if ydiff.abs() > 1 {
        // move along the y axis             
        if ydiff > 0 { by += 1 } else { by -= 1 }           
    }
    (bx, by)
}

#[derive(Clone, Debug)]
struct Knots {
    is_head: bool,
    coords: (i32, i32),
    visited: Box<LinkedHashSet<(i32, i32)>>,
}

impl Knots {
    fn new(value: (i32, i32)) -> Self {
        Knots {
            is_head: false,
            coords: value,
            visited: Box::new(LinkedHashSet::new()),
        }
    }
}