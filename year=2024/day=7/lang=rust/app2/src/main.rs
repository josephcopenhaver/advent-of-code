use std::error::Error;

const INPUT: &str = include_str!("../../../input.txt");

fn main() -> Result<(), Box<dyn Error>> {
    let mut sum = 0;
    for v in INPUT.lines() {
        let (dst, params) = v.split_once(": ").expect("not valid input");
        let dst = dst.parse::<i64>()?;
        let mut params = params.split(" ");
        let first = params.next().expect("").parse::<i64>()?;

        let mut totals = vec![first, 1];
        let mut buf = Vec::new();
        for v in params {
            let num_chars = v.len();
            let v = v.parse::<i64>()?;
            buf.clear();

            for a in &totals {
                let n = *a * v;
                if n <= dst {
                    buf.push(n);
                }
                let n = *a + v;
                if n <= dst {
                    buf.push(n);
                }
                let n = *a * 10_i64.pow(num_chars as u32) + v;
                if n <= dst {
                    buf.push(n);
                }
            }

            totals.clear();
            for v in &buf {
                totals.push(*v);
            }
        }

        for v in totals {
            if v == dst {
                sum += dst;
                break;
            }
        }
    }

    println!("{}", sum);
    Ok(())
}
