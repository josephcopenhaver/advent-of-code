use std::{collections::HashSet, error::Error};

const INPUT: &str = include_str!("../../../input.txt");

fn main() -> Result<(), Box<dyn Error>> {
    let w = INPUT.find("\n").expect("no newline") - 2;
    let map_end = INPUT.find("\n\n").expect("no double newline");
    let h = (&INPUT[0..map_end]).chars().filter(|c| *c == '\n').count() - 1;

    let mut grid = vec![vec![0; w * 2]; h];
    let mut p: (i32, i32) = (-1, -1);
    for (y, mut v) in (&INPUT[(w + 3)..(map_end - w - 3)]).lines().enumerate() {
        v = &v[1..(w + 1)];
        for (x, v) in v.as_bytes().iter().cloned().enumerate() {
            match v {
                b'.' => continue,
                b'@' => {
                    p = (x as i32 * 2, y as i32);
                    continue;
                }
                b'#' => {
                    grid[y][x * 2] = v;
                    grid[y][x * 2 + 1] = v;
                }
                _ => {
                    grid[y][x * 2] = b'[';
                    grid[y][x * 2 + 1] = b']';
                }
            }
        }
    }
    let w = w * 2;

    let mut hs_a = HashSet::<i32>::new();
    let mut hs_b = HashSet::<i32>::new();
    let mut buf = Vec::<(i32, i32)>::new();

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

        let v = grid[np.1 as usize][np.0 as usize];
        match v {
            b'#' => continue,
            0 => {
                p = np;
                continue;
            }
            _ => {}
        }

        // there is a box in the way

        if d.1 == 0 {
            // direction must be left or right
            // search along it for empty space or wall

            let mut x = np.0 + d.0 * 2;
            while x >= 0 && (x as usize) < w {
                match grid[p.1 as usize][x as usize] {
                    b'#' => break,
                    0 => {
                        let v = if d.0 > 0 { (b']', b'[') } else { (b'[', b']') };
                        loop {
                            grid[p.1 as usize][x as usize] = v.0;
                            x -= d.0;
                            grid[p.1 as usize][x as usize] = v.1;
                            x -= d.0;
                            if x == np.0 {
                                break;
                            }
                        }

                        grid[p.1 as usize][x as usize] = 0;
                        p = np;
                        break;
                    }
                    _ => {
                        // must be a box here, keep on moving
                        x += d.0 * 2;
                    }
                }
            }

            continue;
        }

        // direction must be up or down
        let mut y = np.1;
        {
            let mut x = np.0;

            buf.push((x, y));
            hs_a.insert(x);

            x += if v == b'[' { 1 } else { -1 };

            buf.push((x, y));
            hs_a.insert(x);
        }

        let mut blocked = false;
        while hs_a.len() > 0 {
            y += d.1;

            if y < 0 || y as usize >= h {
                blocked = true;
                break;
            }

            for x in hs_a.iter().cloned() {
                let v = grid[y as usize][x as usize];
                match v {
                    0 => continue,
                    b'#' => {
                        blocked = true;
                        break;
                    }
                    _ => {
                        hs_b.insert(x as i32);
                        hs_b.insert(x as i32 + (if v == b'[' { 1 } else { -1 }));
                    }
                }
            }
            if blocked {
                hs_b.clear();
                break;
            }

            // track the full set of box grid nodes to move
            (hs_a, hs_b) = (hs_b, hs_a);
            hs_b.clear();
            for v in hs_a.iter().cloned() {
                buf.push((v, y));
            }
        }

        if blocked {
            hs_a.clear();
            buf.clear();
            continue;
        }

        // shift all box nodes starting with
        // those closest to the opposite side of the robot
        buf.reverse();
        for (x, y) in buf.iter().cloned() {
            grid[(y + d.1) as usize][x as usize] = grid[y as usize][x as usize];
            grid[y as usize][x as usize] = 0;
        }
        buf.clear();

        p = np;
    }

    let mut sum = 0;
    for (y, v) in grid.iter().enumerate() {
        for (x, v) in v.iter().cloned().enumerate() {
            if v == b'[' {
                sum += 100 * (y + 1) + x + 2;
            }
        }
    }

    println!("{}", sum);
    Ok(())
}
