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

    fn is_set<T: Into<usize>>(&self, idx: T) -> bool {
        let idx = idx.into();

        (self.0[idx / 8] & 1 << (idx % 8)) != 0
    }

    fn count_ones(&self) -> usize {
        let mut sum = 0;
        for v in &self.0 {
            sum += v.count_ones() as usize
        }
        sum
    }
}

#[derive(Copy, Clone, PartialEq)]
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
        // assume that the guard never starts at a point surrounded by 3 obstructions
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
    let h = INPUT.trim_end().chars().filter(|c| *c == '\n').count() + 1;
    // track all nodes visited, the direction traveling when visited, and unique places to add a circuit establishing obstruction
    //
    // on start and after each rotation, cast a ray behind the guard and mark each node the ray visits up to an obstruction as visited
    //
    // when the guard visits a new node, check to see if the node has been visited before and if the guard were to rotate again there
    // if that would match a previous direction that has been taken
    //
    // assuming that the direction the guard is facing allows for an obstruction to be placed on the grid, then record the location as
    // a new cyclic opportunity point

    let mut grid: Vec<Vec<u8>> = Vec::with_capacity(h);
    let mut g = Guard {
        x: -1,
        y: -1,
        d: Direction::Up,
    };

    {
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
    }
    let w = grid[0].len();

    let mut visited = Bitmap::new(w * h * 5);
    let mut visited_clone = Bitmap::new(w * h * 5);
    let mut cycles = Bitmap::new(w * h);
    let mut cycle_directions = Bitmap::new(w * h * 4);
    loop {
        visited.set((g.y as usize * w + g.x as usize) * 5); // node visited
        visited.set((g.y as usize * w + g.x as usize) * 5 + 1 + g.d as usize); // node visited in specific direction
        let mut pg: Guard = Guard {
            x: g.x,
            y: g.y,
            d: g.d,
        };
        if !g.step(&grid) {
            break;
        }
        // step back one and check if the path leads to a cycle if that node would have been a valid obstruction
        //
        // a valid obstruction is one placed where we have not yet visited
        //
        // also avoid visiting any cycle spawning paths we've already visited
        if !visited.is_set((g.y as usize * w + g.x as usize) * 5)
            && !cycle_directions.is_set((g.y as usize * w + g.x as usize) * 4 + pg.d as usize)
        {
            grid[g.y as usize][g.x as usize] = b'#'; // set obstruction
            if will_cycle(&mut pg, &grid, &visited, &mut visited_clone) {
                cycles.set(g.y as usize * w + g.x as usize);
                cycle_directions.set((g.y as usize * w + g.x as usize) * 4 + pg.d as usize);
            }
            grid[g.y as usize][g.x as usize] = b'.'; // clear obstruction
        }
    }

    println!("{}", cycles.count_ones());
}

fn will_cycle(
    g: &mut Guard,
    grid: &Vec<Vec<u8>>,
    visited: &Bitmap,
    visited_clone: &mut Bitmap,
) -> bool {
    // note: this is a very good problem for parallel solving and centralized loop detection caching
    //
    // a path segment ending in a loop can be cached independent of prior vectors if the loop contributing
    // segments never have a direction change due to hitting the newly added obstruction. All preceding
    // nodes are also cacheable if not interacting with the obstruction. All backward projecting nodes
    // from all segments in the prior definition that align with a node vector in it are also part of
    // the cycle creating equivalance set as long as they do not interact with the new obstruction.
    //
    // Cycles that depend on the new obstruction are also potentially cacheable if one of the other three
    // sides is involved in the cycle path such that the path does not change direction due to hitting the
    // obstruction as either part of the cycle or a cycle prefix (after hitting the obstruction).
    //
    // Not doing it because this is fast enough for my lifetime and hardware.
    for v in visited.0.iter().enumerate() {
        visited_clone.0[v.0] = *v.1;
    }
    let visited = visited_clone;

    loop {
        if !g.step(&grid) {
            return false;
        }

        let idx = (g.y as usize * grid[0].len() + g.x as usize) * 5 + 1 + g.d as usize; // directed node index
        if visited.is_set(idx) {
            return true;
        }
        visited.set(idx);
    }
}
