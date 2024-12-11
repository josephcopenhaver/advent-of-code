use std::error::Error;

const INPUT: &str = include_str!("../../../input.txt");

fn is_inc(a: i32, b: i32) -> bool {
    b > a && b - a <= 3
}

fn is_dec(a: i32, b: i32) -> bool {
    a > b && a - b <= 3
}

fn inc_idx(x: &[i32]) -> (usize, bool) {
    for i in 0..(x.len() - 1) {
        if !is_inc(x[i], x[i + 1]) {
            return (i, false);
        }
    }

    (x.len(), true)
}

fn dec_idx(x: &[i32]) -> (usize, bool) {
    for i in 0..(x.len() - 1) {
        if !is_dec(x[i], x[i + 1]) {
            return (i, false);
        }
    }

    (x.len(), true)
}

fn mono_inc(x: &[i32]) -> bool {
    let (_, b) = inc_idx(x);
    b
}

fn mono_dec(x: &[i32]) -> bool {
    let (_, b) = dec_idx(x);
    b
}

fn main() -> Result<(), Box<dyn Error>> {
    let size = INPUT
        .lines()
        .map(|v| v.chars().filter(|&v| v == ' ').count())
        .max()
        .expect("no input")
        + 1;

    let mut buf: Vec<i32> = Vec::<i32>::with_capacity(size);

    let mut sum = 0;
    for v in INPUT.lines() {
        buf.clear();
        for v in v.split(" ") {
            buf.push(v.parse::<i32>()?);
        }

        // the next few lines are overly verbose and not very generic because I wanted to experiment
        // with supporting early short circuiting.
        //
        // perhaps in the future I'll rewrite it to be less bespoke-looking and use more generality
        // rather than explicit flow-control
        // but it short circuits quickly and firmly scales... don't repeat this
        // in the future for small problem-sets that do not require the optimization
        //
        // I found it interesting to think through though.
        //
        // the overhead of parallelizing this code is likely not worth it as well

        let (idx, ok) = inc_idx(&buf);
        if ok {
            sum += 1;
            continue;
        }
        if idx == buf.len() - 2 {
            // issue occurred at the very end of the sequence
            // it's monotonic given a tolerance of 1 if the last element is dropped
            sum += 1;
            continue;
        }
        if (idx == 0 || is_inc(buf[idx - 1], buf[idx + 1])) && mono_inc(&buf[idx + 1..]) {
            // removing idx would make a monotonic sequence
            sum += 1;
            continue;
        }
        if is_inc(buf[idx], buf[idx + 2]) && mono_inc(&buf[idx + 2..]) {
            // removing idx+1 would make a monotonic sequence
            sum += 1;
            continue;
        }

        let (idx, ok) = dec_idx(&buf);
        if ok {
            sum += 1;
            continue;
        }
        if idx == buf.len() - 2 {
            // issue occurred at the very end of the sequence
            // it's monotonic given a tolerance of 1 if the last element is dropped
            sum += 1;
            continue;
        }
        if (idx == 0 || is_dec(buf[idx - 1], buf[idx + 1])) && mono_dec(&buf[idx + 1..]) {
            // removing idx would make a monotonic sequence
            sum += 1;
            continue;
        }
        if is_dec(buf[idx], buf[idx + 2]) && mono_dec(&buf[idx + 2..]) {
            // removing idx+1 would make a monotonic sequence
            sum += 1;
            continue;
        }
    }

    println!("{}", sum);
    Ok(())
}
