pub mod day01;

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
    crate::day01::run(Box::new(reader));
}
