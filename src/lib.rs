use std::io::BufReader;
use std::fs::File;

/// Define a solution trait to solve for part a and b on each day
/// We use &self here to ensure that it is object safe
/// https://doc.rust-lang.org/reference/items/traits.html#object-safet
pub trait Solution {
    fn part_a(&self);
    fn part_b(&self);
}

pub fn load(year: u32, day: u32) -> BufReader<File> {
    let filename = format!("data/{year}/day_{:02}.txt", day);
    let f = File::open(filename).expect("Unable to open input file");
    BufReader::new(f)
}