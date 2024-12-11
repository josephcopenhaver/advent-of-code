use std::error::Error;

const INPUT: &str = include_str!("../../../input.txt");

fn main() -> Result<(), Box<dyn Error>> {
    let mut sum = 0;
    for v in INPUT.lines() {
        let (dst, params) = v.split_once(": ").expect("not valid input");
        let dst = dst.parse::<i64>()?;
        let mut params = params.split(" ");
        let first = params
            .next()
            .expect("no first param after ': ' sequence")
            .parse::<i64>()?;

        let mut totals = vec![first];
        let mut buf = Vec::new();
        for v in params {
            let num_chars = v.len();
            let v = v.parse::<i64>()?;

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
            (totals, buf) = (buf, totals);
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
