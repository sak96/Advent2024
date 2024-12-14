use std::io::BufRead;

use fxhash::{FxHashMap, FxHashSet};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    N,
    S,
    W,
    E,
}

impl Direction {
    pub fn move_(&self) -> (isize, isize) {
        match self {
            Direction::N => (-1, 0),
            Direction::S => (1, 0),
            Direction::W => (0, -1),
            Direction::E => (0, 1),
        }
    }

    pub fn turn(&self) -> Self {
        match self {
            Direction::N => Direction::E,
            Direction::S => Direction::W,
            Direction::W => Direction::N,
            Direction::E => Direction::S,
        }
    }
}

pub fn run<'a>(reader: Box<dyn BufRead + 'a>) {
    let (mut sx, mut sy) = (0, 0);
    let mut memory = FxHashMap::default();

    let map: Vec<_> = reader
        .lines()
        .map_while(Result::ok)
        .map(|line| line.as_bytes().to_vec())
        .collect();

    for (r, row) in map.iter().enumerate() {
        for (c, &ch) in row.iter().enumerate() {
            for d in [Direction::N, Direction::E, Direction::W, Direction::S] {
                if ch == b'#' {
                    continue;
                } else if ch == b'^' {
                    (sx, sy) = (r, c);
                }
                let r = r as isize;
                let c = c as isize;
                let (x, y) = d.move_();
                let (x, y) = (r + x, c + y);
                if x < 0 || y < 0 || x as usize >= map.len() || y as usize >= map[x as usize].len()
                {
                    memory.insert((r, c, d), None);
                } else if map[x as usize][y as usize] == b'#' {
                    memory.insert((r, c, d), Some((r, c, d.turn())));
                } else {
                    memory.insert((r, c, d), Some((x, y, d)));
                }
            }
        }
    }

    let (mut x, mut y, mut d) = (sx as isize, sy as isize, Direction::N);
    let mut moves = FxHashSet::default();
    moves.insert((x, y));
    while let Some((x1, y1, d1)) = memory.get(&(x, y, d)).unwrap() {
        (x, y, d) = (*x1, *y1, *d1);
        moves.insert((x, y));
    }
    println!("{}", moves.len());

    let mut count = 0;
    for (r, c) in moves {
        let (mut x, mut y, mut d) = (sx as isize, sy as isize, Direction::N);
        let mut moves = FxHashSet::default();
        while let Some((x1, y1, d1)) = memory.get(&(x, y, d)).unwrap() {
            if (x1, y1) == (&r, &c) {
                d = d.turn();
            } else {
                (x, y, d) = (*x1, *y1, *d1);
            }
            if !moves.insert((x, y, d)) {
                count += 1;
                break;
            }
        }
    }
    println!("{count}");
}
