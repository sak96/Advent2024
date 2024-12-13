use std::io::BufRead;

fn is_safe<'a, I>(mut level: I) -> bool
where
    I: Iterator<Item = &'a i64>,
{
    let first = *level.next().unwrap();
    let mut iter = level.peekable();
    let diff = *iter.peek().unwrap() - first;
    if !(1..=3).contains(&diff.abs()) {
        return false;
    }
    (iter.collect::<Vec<_>>())
        .windows(2)
        .map(|i| {
            let diff = (i[1] - i[0]) * diff.signum();
            (1..=3).contains(&diff)
        })
        .all(|x| x)
}

pub fn run<'a>(reader: Box<dyn BufRead + 'a>) {
    let data: Vec<Vec<i64>> = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            line.split(' ').map(|n| n.trim().parse().unwrap()).collect()
        })
        .collect();
    let safe = data
        .iter()
        .filter_map(|level| is_safe(level.iter()).then_some(()))
        .count();
    println!("{safe}");
    let safe = data
        .iter()
        .filter_map(|level| {
            (0..(level.len()))
                .map(|i| is_safe(level.iter().take(i).chain(level.iter().skip(i + 1))))
                .any(|x| x)
                .then_some(())
        })
        .count();
    println!("{safe}");
}
