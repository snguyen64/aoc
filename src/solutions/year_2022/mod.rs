use crate::solution::Solution;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;

/// This contains a map of all the solutions that we have for module 2022
pub const ALL: [&dyn Solution; 5] = [
    &day_01::Day01, 
    &day_02::Day02,
    &day_03::Day03,
    &day_04::Day04,
    &day_05::Day05,
];