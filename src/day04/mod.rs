use std::io::BufRead;

struct Grid(Vec<Vec<u8>>);

impl Grid {
    pub fn get(&self, r: isize, c: isize) -> Option<u8> {
        (r >= 0 && c >= 0)
            .then(|| self.0.get(r as usize).and_then(|r| r.get(c as usize)))
            .flatten()
            .copied()
    }

    const DIRECTIONS: [(isize, isize); 8] = [
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];

    fn xmas(&self, i: isize, j: isize) -> usize {
        Self::DIRECTIONS
            .iter()
            .filter(|(x, y)| {
                for (k, &ch) in b"XMAS".iter().enumerate() {
                    let o = self.get(x * k as isize + i, y * k as isize + j);
                    if o != Some(ch) {
                        return false;
                    }
                }
                true
            })
            .count()
    }

    fn xmas_2(&self, i: isize, j: isize) -> bool {
        if self.get(i, j) == Some(b'A') {
            let a = self.get(i + 1, j + 1);
            let b = self.get(i - 1, j - 1);
            let x = self.get(i - 1, j + 1);
            let y = self.get(i + 1, j - 1);
            ((a == Some(b'M') && b == Some(b'S')) || (a == Some(b'S') && b == Some(b'M')))
                && ((x == Some(b'M') && y == Some(b'S')) || (x == Some(b'S') && y == Some(b'M')))
        } else {
            false
        }
    }
}

pub fn run<'a>(reader: Box<dyn BufRead + 'a>) {
    let input: Vec<_> = reader
        .lines()
        .map(|line| line.unwrap().into_bytes())
        .collect();
    let rows = input.len() as isize;
    let cols = input[0].len() as isize;
    let grid = Grid(input);
    let mut count_1 = 0;
    let mut count_2 = 0;

    for i in 0..rows {
        for j in 0..cols {
            count_1 += grid.xmas(i, j);
            if grid.xmas_2(i, j) {
                count_2 += 1;
            }
        }
    }
    println!("{count_1}");
    println!("{count_2}");
}
