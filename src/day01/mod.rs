use std::io::BufRead;

pub fn run<'a>(reader: Box<dyn BufRead + 'a>) {
    let mut loc1 = vec![];
    let mut loc2 = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        let mut split = line.split("   ");
        loc1.push(split.next().unwrap().parse::<u64>().unwrap());
        loc2.push(split.next().unwrap().parse::<u64>().unwrap());
    }
    loc1.sort();
    loc2.sort();
    let distance = std::iter::zip(&loc1, &loc2)
        .map(|(&a, &b)| a.abs_diff(b))
        .sum::<u64>();
    println!("{distance}");

    let mut it1 = loc1.iter().peekable();
    let mut it2 = loc2.iter().peekable();

    let mut similarity = 0;
    while let Some(i1) = it1.next() {
        let mut repeat1 = 1;
        while let Some(&p1) = it1.peek() {
            if p1 == i1 {
                repeat1 += 1;
                it1.next();
            } else {
                break;
            }
        }

        let mut repeat2 = 0;
        while let Some(&&i2) = it2.peek() {
            match i2.cmp(i1) {
                std::cmp::Ordering::Less => {
                    it2.next();
                    continue;
                }
                std::cmp::Ordering::Equal => {
                    it2.next();
                    repeat2 += 1;
                }
                std::cmp::Ordering::Greater => {
                    break;
                }
            }
        }
        similarity += repeat1 * repeat2 * i1;
    }
    println!("{similarity}");
}
