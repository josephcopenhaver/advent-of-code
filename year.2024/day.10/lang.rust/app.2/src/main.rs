use std::collections::HashMap;
use std::error::Error;

const INPUT: &str = include_str!("../../../input.txt");

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: impl Into<usize>, y: impl Into<usize>) -> Point {
        Point {
            x: x.into(),
            y: y.into(),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let w = INPUT.find("\n").expect("does not contain newlines");
    let h = INPUT.trim_end().chars().filter(|c| *c == '\n').count() + 1;
    let mut grid = vec![vec![0 as u8; w]; h];

    struct PointPaths {
        point: Point,
        path_count: i32,
    }
    let mut scan_points = Vec::<PointPaths>::new();

    let mut next_elevation = b'9';
    for (y, v) in INPUT.lines().enumerate() {
        for (x, v) in v.as_bytes().iter().enumerate() {
            let v = *v;
            grid[y][x] = v;
            if v == next_elevation {
                let p = Point::new(x, y);
                scan_points.push(PointPaths {
                    point: p,
                    path_count: 1,
                });
            }
        }
    }
    next_elevation -= 1;

    let mut buf = Vec::<PointPaths>::with_capacity(scan_points.len());
    let mut m = HashMap::<Point, usize>::with_capacity(scan_points.len());
    loop {
        for v in &scan_points {
            for d in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let x = v.point.x as i32 + d.0;
                let y = v.point.y as i32 + d.1;

                if x < 0 || y < 0 || x as usize > w - 1 || y as usize > h - 1 {
                    continue;
                }

                let p = Point::new(x as usize, y as usize);

                if grid[p.y][p.x] != next_elevation {
                    continue;
                }

                match m.get(&p) {
                    None => {
                        let idx = buf.len();
                        buf.push(PointPaths {
                            point: p,
                            path_count: v.path_count,
                        });
                        m.insert(p, idx);
                    }
                    Some(idx) => buf[*idx].path_count += v.path_count,
                }
            }
        }

        scan_points.clear();
        (buf, scan_points) = (scan_points, buf);
        m.clear();

        if next_elevation == b'0' {
            break;
        }
        next_elevation -= 1;
    }

    let mut sum = 0;
    for v in scan_points {
        sum += v.path_count;
    }

    println!("{}", sum);
    Ok(())
}
