use std::{
    io::BufReader,
    fs::File,
};


pub mod day_one; // tells compiler to include day_one code
pub mod day_two;


pub struct DayChallenge {
    input: String,
    exec_day: fn(BufReader<File>),
}

impl DayChallenge {
    pub fn new(i: &str, f: fn(BufReader<File>)) -> DayChallenge {
        DayChallenge {
            input: i.to_string(),
            exec_day: f,
        }
    }

    pub fn exec(&self) {
        let fo = File::open(self.input.as_str()).expect("Unable to open file");
        let br = BufReader::new(fo);
        let f = self.exec_day;
        f(br);
    }
}