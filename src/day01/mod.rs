use std::io::BufRead;

pub fn run<'a>(reader: Box<dyn BufRead + 'a>) {}
