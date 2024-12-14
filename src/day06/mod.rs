use std::io::BufRead;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    N,
    S,
    W,
    E,
}

impl Direction {
    pub const DIRS: [Direction; 4] = [Direction::N, Direction::E, Direction::W, Direction::S];
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

    pub fn index(&self) -> isize {
        match self {
            Direction::N => 0,
            Direction::E => 1,
            Direction::W => 2,
            Direction::S => 3,
        }
    }
}

type State = (isize, isize, Direction);

struct CustomHashMap<T> {
    store: Vec<T>,
    cols: isize,
}

impl<T: Clone + Default> CustomHashMap<T> {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            store: vec![T::default(); rows * cols * 4],
            cols: cols as isize,
        }
    }

    pub fn insert(&mut self, (r, c, d): State, value: T) {
        self.store[(r * self.cols * 4 + c * 4 + d.index()) as usize] = value
    }

    pub fn get(&self, (r, c, d): &State) -> &T {
        &self.store[(r * self.cols * 4 + c * 4 + d.index()) as usize]
    }
}

struct CustomHashSet {
    map: CustomHashMap<bool>,
}

impl CustomHashSet {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            map: CustomHashMap::new(rows, cols),
        }
    }

    pub fn insert(&mut self, state: State) -> bool {
        if *self.map.get(&state) {
            false
        } else {
            self.map.insert(state, true);
            true
        }
    }

    pub fn to_vec(&self) -> Vec<State> {
        let (mut r, mut c, mut d) = (0, 0, 0);
        let cols = self.map.cols;
        self.map
            .store
            .iter()
            .filter_map(move |&i| {
                let output = if i {
                    Some((r, c, Direction::DIRS[d]))
                } else {
                    None
                };
                d += 1;
                if d >= 4 {
                    c += 1;
                    d %= 4;
                }
                if c >= cols {
                    r += 1;
                    c %= cols
                }
                output
            })
            .collect()
    }
}

pub fn run<'a>(reader: Box<dyn BufRead + 'a>) {
    let (mut sx, mut sy) = (0, 0);

    let map: Vec<_> = reader
        .lines()
        .map_while(Result::ok)
        .map(|line| line.as_bytes().to_vec())
        .collect();

    let mut memory = CustomHashMap::new(map.len(), map[0].len());

    for (r, row) in map.iter().enumerate() {
        for (c, &ch) in row.iter().enumerate() {
            for d in Direction::DIRS {
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
    let mut moves = CustomHashSet::new(map.len(), map[0].len());
    moves.insert((x, y, Direction::N));
    while let &Some((x1, y1, d1)) = memory.get(&(x, y, d)) {
        (x, y, d) = (x1, y1, d1);
        moves.insert((x, y, Direction::N));
    }
    let moves = moves.to_vec();
    println!("{}", moves.len());

    let mut count = 0;
    for (r, c, _) in moves {
        let (mut x, mut y, mut d) = (sx as isize, sy as isize, Direction::N);
        let mut moves = CustomHashSet::new(map.len(), map[0].len());
        while let &Some((x1, y1, d1)) = memory.get(&(x, y, d)) {
            if (x1, y1) == (r, c) {
                d = d.turn();
            } else {
                (x, y, d) = (x1, y1, d1);
            }
            if !moves.insert((x, y, d)) {
                count += 1;
                break;
            }
        }
    }
    println!("{count}");
}
