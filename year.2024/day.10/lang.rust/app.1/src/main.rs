use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::rc::Rc;

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
    let num_peaks = INPUT.trim_end().chars().filter(|c| *c == '9').count() + 1;
    let mut grid = vec![vec![0 as u8; w]; h];

    struct PointPeaks<'a> {
        point: Point,
        peaks: Rc<Cow<'a, HashSet<Point>>>,
    }
    let mut scan_points = Vec::<PointPeaks>::new();

    let mut next_elevation = b'9';
    for (y, v) in INPUT.lines().enumerate() {
        for (x, v) in v.as_bytes().iter().enumerate() {
            let v = *v;
            grid[y][x] = v;
            if v == next_elevation {
                let p = Point::new(x, y);
                let mut set = HashSet::<Point>::with_capacity(num_peaks);
                set.insert(p);
                scan_points.push(PointPeaks {
                    point: p,
                    peaks: Rc::new(Cow::Owned(set)),
                });
            }
        }
    }
    next_elevation -= 1;

    let mut buf = Vec::<PointPeaks>::with_capacity(scan_points.len());
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
                        buf.push(PointPeaks {
                            point: p,
                            peaks: v.peaks.clone(),
                        });
                        m.insert(p, idx);
                    }
                    Some(idx) => {
                        let mut cow_dst = Rc::make_mut(&mut buf[*idx].peaks).to_owned();
                        let dst = cow_dst.to_mut();
                        for v in v.peaks.iter() {
                            dst.insert(*v);
                        }
                        buf[*idx].peaks = Rc::new(cow_dst);
                    }
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
        sum += v.peaks.len();
    }

    println!("{}", sum);
    Ok(())
}
