use std::io::BufRead;

use regex::Regex;

pub fn run<'a>(mut reader: Box<dyn BufRead + 'a>) {
    let mut input = String::new();
    let _ = reader.read_to_string(&mut input);

    let re = Regex::new(r"(mul\((\d+),(\d+)\))|(do\(\))|(don't\(\))").unwrap();
    let mut enabled = true;
    let mut sol1 = 0;
    let mut sol2 = 0;
    for cap in re.captures_iter(&input) {
        if cap[0].starts_with("mul") {
            let a = cap[2].parse::<u64>().unwrap();
            let b = cap[3].parse::<u64>().unwrap();
            sol1 += a * b;
            if enabled {
                sol2 += a * b;
            }
        } else {
            match cap[0].starts_with("don't") {
                true => {
                    enabled = false;
                }
                false => {
                    enabled = true;
                }
            }
        }
    }
    println!("{sol1}");
    println!("{sol2}");
}
