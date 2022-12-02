use crate::Solution;

mod year_2022;

/// This function is to get a solutions for a year.
/// It maps the year to the trait object denoted by the prefix `dyn`
/// dyn Solution actually has two pointers which go to the instance of a struct
/// and another pointer goes to the map of a method call names to function pointers (vtable).
/// https://doc.rust-lang.org/std/keyword.dyn.html
/// The &'static prefix refers to reference lifetime. We have this so this map persists.
pub fn get_year(year: u32) -> &'static [&'static dyn Solution] {
    match year {
        2022 => &year_2022::ALL,
        _ => &[],
    }
}