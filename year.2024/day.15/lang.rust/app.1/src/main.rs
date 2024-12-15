use std::error::Error;

const INPUT: &str = include_str!("../../../input.txt");

fn main() -> Result<(), Box<dyn Error>> {
    let w = INPUT.find("\n").expect("no newline") - 2;
    let map_end = INPUT.find("\n\n").expect("no double newline");
    let h = (&INPUT[0..map_end]).chars().filter(|c| *c == '\n').count() - 1;

    let mut grid = vec![vec![0; w]; h];
    let mut p: (i32, i32) = (-1, -1);
    for (y, mut v) in (&INPUT[(w + 3)..(map_end - w - 3)]).lines().enumerate() {
        v = &v[1..(w + 1)];
        for (x, v) in v.as_bytes().iter().cloned().enumerate() {
            match v {
                b'.' => continue,
                b'@' => {
                    p = (x as i32, y as i32);
                    continue;
                }
                _ => grid[y][x] = v,
            }
        }
    }

    // process moves
    for v in (&INPUT[(map_end + 2)..]).as_bytes().iter().cloned() {
        let d = match v {
            b'<' => (-1, 0),
            b'>' => (1, 0),
            b'^' => (0, -1),
            b'v' => (0, 1),
            _ => continue,
        };

        let np = (p.0 + d.0, p.1 + d.1);
        if np.0 < 0 || np.1 < 0 || np.0 as usize >= w || np.1 as usize >= h {
            continue;
        }

        match grid[np.1 as usize][np.0 as usize] {
            b'#' => continue,
            0 => {
                p = np;
                continue;
            }
            _ => {}
        }

        // there is a box in the way

        // search along vector d for an empty space or wall
        let mut v = (np.0 + d.0, np.1 + d.1);
        while v.0 >= 0 && v.1 >= 0 && (v.0 as usize) < w && (v.1 as usize) < h {
            match grid[v.1 as usize][v.0 as usize] {
                b'#' => break,
                0 => {
                    grid[v.1 as usize][v.0 as usize] = b'O';
                    grid[np.1 as usize][np.0 as usize] = 0;
                    p = np;
                    break;
                }
                _ => {
                    // must be a box here, keep on moving
                    v = (v.0 + d.0, v.1 + d.1);
                }
            }
        }
    }

    let mut sum = 0;
    for (y, v) in grid.iter().enumerate() {
        for (x, v) in v.iter().cloned().enumerate() {
            if v == b'O' {
                sum += 100 * (y + 1) + x + 1;
            }
        }
    }

    println!("{}", sum);
    Ok(())
}
