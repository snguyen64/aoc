use crate::solution::Solution;

mod day_01;
mod day_02;

/// This contains a map of all the solutions that we have for module 2022
pub const ALL: [&dyn Solution; 2] = [
    &day_01::Day01, 
    &day_02::Day02
];