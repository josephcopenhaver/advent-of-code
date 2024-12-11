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
            for i in 0..4 {
                let p = match i {
                    0 => {
                        if !(v.point.x > 0 && grid[v.point.y][v.point.x - 1] == next_elevation) {
                            continue;
                        }
                        Point::new(v.point.x - 1, v.point.y)
                    }
                    1 => {
                        if !(v.point.x < w - 1 && grid[v.point.y][v.point.x + 1] == next_elevation)
                        {
                            continue;
                        }
                        Point::new(v.point.x + 1, v.point.y)
                    }
                    2 => {
                        if !(v.point.y > 0 && grid[v.point.y - 1][v.point.x] == next_elevation) {
                            continue;
                        }
                        Point::new(v.point.x, v.point.y - 1)
                    }
                    3 => {
                        if !(v.point.y < h - 1 && grid[v.point.y + 1][v.point.x] == next_elevation)
                        {
                            continue;
                        }
                        Point::new(v.point.x, v.point.y + 1)
                    }
                    _ => panic!("unreachable"),
                };

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
