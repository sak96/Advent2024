pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;

use std::{
    env::args,
    fs::File,
    io::BufRead,
    io::{stdin, BufReader},
};

fn main() {
    let stdin = stdin();
    let reader: Box<dyn BufRead> = match args().nth(1) {
        Some(path) => Box::new(BufReader::new(File::open(path).unwrap())),
        None => Box::new(BufReader::new(stdin.lock())),
    };
    crate::day07::run(Box::new(reader));
}
