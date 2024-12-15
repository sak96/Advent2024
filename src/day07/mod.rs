use std::io::BufRead;

pub fn solve(value: u64, eq: &[u64], test_value: u64) -> bool {
    if value > test_value {
        false
    } else if let Some((&first, eq)) = eq.split_first() {
        solve(value * first, eq, test_value) || solve(value + first, eq, test_value)
    } else {
        test_value == value
    }
}

pub fn solve2(value: u64, eq: &[u64], test_value: u64) -> bool {
    if value > test_value {
        false
    } else if let Some((&first, eq)) = eq.split_first() {
        solve2(value * first, eq, test_value)
            || solve2(value + first, eq, test_value)
            || solve2(
                value * 10_u64.pow(first.ilog10() + 1) + first,
                eq,
                test_value,
            )
    } else {
        test_value == value
    }
}
pub fn run<'a>(reader: Box<dyn BufRead + 'a>) {
    let equations: Vec<_> = reader
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            let mut split = line.split(": ");
            let test_value = split.next().unwrap().parse::<u64>().unwrap();
            let split = split.next().unwrap().split(' ');
            let nums: Vec<_> = split
                .map(|num| num.trim().parse::<u64>().unwrap())
                .collect();
            (test_value, nums)
        })
        .collect();

    let mut total = 0;
    let mut total2 = 0;
    for (test_value, eq) in equations {
        let (&first, eq) = eq.split_first().unwrap();
        if solve(first, eq, test_value) {
            total += test_value;
            total2 += test_value;
        } else if solve2(first, eq, test_value) {
            total2 += test_value;
        }
    }
    println!("{total}");
    println!("{total2}");
}
