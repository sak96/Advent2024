use std::io::BufRead;

use fxhash::FxHashSet;

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

    pub fn turn(&mut self) {
        *self = match self {
            Direction::N => Direction::E,
            Direction::S => Direction::W,
            Direction::W => Direction::N,
            Direction::E => Direction::S,
        };
    }
}

pub fn run<'a>(reader: Box<dyn BufRead + 'a>) {
    let (mut sx, mut sy) = (0, 0);
    let mut obstacles = FxHashSet::default();

    let (mut row, mut col) = (0, 0);
    for (r, line) in reader.lines().map_while(Result::ok).enumerate() {
        for (c, &ch) in line.as_bytes().iter().enumerate() {
            if ch == b'#' {
                obstacles.insert((r as isize, c as isize));
            } else if ch == b'^' {
                (sx, sy) = (r, c);
            }
        }
        col = line.len();
        row = row.max(r + 1);
    }

    let (mut i, mut j) = (sx as isize, sy as isize);
    let mut direction = Direction::N;
    let mut moves = FxHashSet::default();
    loop {
        if i < 0 || j < 0 || i as usize > row || j as usize > col {
            println!("{}", moves.len());
            break;
        }
        moves.insert((i, j));
        let (x, y) = direction.move_();
        if obstacles.contains(&(i + x, j + y)) {
            direction.turn();
        } else {
            i += x;
            j += y;
        }
    }

    let mut count = 0;
    moves.remove(&(sx as _, sy as _));
    for (r, c) in moves {
        obstacles.insert((r, c));
        let mut moves = FxHashSet::default();
        let (mut i, mut j) = (sx as isize, sy as isize);
        let mut direction = Direction::N;
        loop {
            if i < 0 || j < 0 || i as usize > row || j as usize > col {
                break;
            }
            if !moves.insert((i, j, direction)) {
                count += 1;
                break;
            }
            let (x, y) = direction.move_();
            if obstacles.contains(&(i + x, j + y)) {
                direction.turn();
            } else {
                i += x;
                j += y;
            }
        }
        obstacles.remove(&(r, c));
    }
    println!("{count}");
}
