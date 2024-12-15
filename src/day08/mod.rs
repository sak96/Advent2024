use fxhash::FxHashSet;
use std::io::BufRead;

pub fn run<'a>(reader: Box<dyn BufRead + 'a>) {
    let mut memory = vec![vec![]; b'z' as usize + 1];

    let map: Vec<_> = reader
        .lines()
        .map_while(Result::ok)
        .map(|line| line.as_bytes().to_vec())
        .collect();

    for (r, row) in map.iter().enumerate() {
        for (c, &ch) in row.iter().enumerate() {
            if ch == b'.' || ch == b'#' {
                continue;
            }
            memory[ch as usize].push((r as isize, c as isize));
        }
    }
    let rows = map.len() as isize;
    let cols = map[0].len() as isize;

    let mut antinodes = FxHashSet::default();
    let mut antinodes2 = FxHashSet::default();
    for nodes in memory {
        if !nodes.is_empty() {
            let mut nodes = nodes.as_slice();
            while let Some(((x, y), n1)) = nodes.split_first() {
                nodes = n1;
                for (p, q) in nodes {
                    let (dx, dy) = ((p - x), (q - y));
                    let (m, n) = ((2 * p - x), (2 * q - y));
                    if m >= 0 && m < rows && n >= 0 && n < cols {
                        antinodes.insert((m, n));
                    }
                    let (m, n) = ((2 * x - p), (2 * y - q));
                    if m >= 0 && m < rows && n >= 0 && n < cols {
                        antinodes.insert((m, n));
                    }
                    let (mut m, mut n) = (p - dx, q - dy);
                    while m >= 0 && m < rows && n >= 0 && n < cols {
                        antinodes2.insert((m, n));
                        m -= dx;
                        n -= dy;
                    }
                    let (mut m, mut n) = (x + dx, y + dy);
                    while m >= 0 && m < rows && n >= 0 && n < cols {
                        antinodes2.insert((m, n));
                        m += dx;
                        n += dy;
                    }
                }
            }
        }
    }
    println!("{}", antinodes.len());
    println!("{}", antinodes2.len());
}
