const INPUT: &str = include_str!("../../../input.txt");

struct Bitmap(Vec<u8>);

// Bitmap is quick and dirty
// it has zero bounds checking
// and can end in up to 7 unused bits
//
// Ideally a full implementation would have
// these aspects covered.
impl Bitmap {
    fn new(size: impl Into<usize>) -> Bitmap {
        let size = size.into();

        Bitmap(vec![0; (size + 7) / 8])
    }

    fn set<T: Into<usize>>(&mut self, idx: T) {
        let idx = idx.into();

        self.0[idx / 8] |= 1 << (idx % 8);
    }

    fn count_ones(&self) -> usize {
        let mut sum = 0;
        for v in &self.0 {
            sum += v.count_ones() as usize
        }
        sum
    }
}

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn parse(v: impl Into<char>) -> Option<Direction> {
        match v.into() {
            'V' => Some(Direction::Up),
            '^' => Some(Direction::Down),
            '<' => Some(Direction::Left),
            '>' => Some(Direction::Right),
            _ => None,
        }
    }
}

struct Guard {
    x: i32,
    y: i32,
    d: Direction,
}

impl Guard {
    fn step(&mut self, grid: &Vec<Vec<u8>>) -> bool {
        // assumes that the guard never starts at a point surrounded by 3 obstructions
        loop {
            // get next position
            let p = match self.d {
                Direction::Up => (self.x, self.y + 1),
                Direction::Down => (self.x, self.y - 1),
                Direction::Left => (self.x - 1, self.y),
                Direction::Right => (self.x + 1, self.y),
            };

            // if not in grid, short circuit
            if p.0 < 0 || p.0 as usize >= grid[0].len() {
                return false;
            }
            if p.1 < 0 || p.1 as usize >= grid.len() {
                return false;
            }

            // if landing where there is no obstruction
            // update state and return true
            if grid[p.1 as usize][p.0 as usize] != b'#' {
                self.x = p.0;
                self.y = p.1;
                return true;
            }

            // hit an obstruction, lets rotate
            self.d = match self.d {
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
            };
        }
    }
}

fn main() {
    let size = INPUT.trim_end().chars().filter(|c| *c == '\n').count() + 1;
    // assumes guard will leave the bounded zone
    // this means there are no cycles and there is clearly determinism
    // no state needs to be remembered other than visited squares before
    // terminal state is reached

    let mut g = Guard {
        x: -1,
        y: -1,
        d: Direction::Up,
    };

    let mut grid: Vec<Vec<u8>> = Vec::with_capacity(size);
    let mut y = -1;
    for v in INPUT.lines() {
        y += 1;
        let m = v
            .chars()
            .enumerate()
            .filter(|v| Direction::parse(v.1).is_some())
            .nth(0);
        grid.push(Vec::from(v.as_bytes()));
        if let Some((x, c)) = m {
            g.x = x as i32;
            g.y = y;
            g.d = Direction::parse(c).expect("unreachable");
            let idx = grid.len() - 1;
            grid[idx][x] = b'.';
        }
    }

    let mut bm = Bitmap::new(grid.len() * grid[0].len());
    loop {
        bm.set((g.y * grid[0].len() as i32 + g.x) as usize);
        if !g.step(&grid) {
            break;
        }
    }

    println!("{}", bm.count_ones());
}
