// assumes nodes are always a number between 0 and 99 inclusive
// 99 in binary is 0b0110_0011 (note that MSB is unused in the byte)
// so each node needs 7 bits to indicate presence and another 7 bits
// to indicate directional pairing with another node

use std::convert::Into;
use std::error::Error;

const INPUT: &str = include_str!("../../../input.txt");

const MAX_NODE_VAL: usize = 0b0110_0011;
const SHIFT: usize = 7;
const MAX_BIT_COUNT: usize = (MAX_NODE_VAL << SHIFT) | MAX_NODE_VAL;
const MAX_BYTE_COUNT: usize = (MAX_BIT_COUNT + 7) / 8;

struct Bitmap([u8; MAX_BYTE_COUNT]);

// Bitmap is quick and dirty
// it has zero bounds checking
// and can end in up to 7 unused bits
//
// Ideally a full implementation would have
// these aspects covered.
impl Bitmap {
    fn new() -> Bitmap {
        Bitmap([0 as u8; MAX_BYTE_COUNT])
    }

    fn set<T: Into<usize>>(&mut self, idx: T) {
        let idx = idx.into();

        self.0[idx / 8] |= 1 << (idx % 8);
    }

    // fn clear<T: Into<usize>>(&mut self, idx: T) {
    //     let idx = idx.into();
    //
    //     self.0[idx / 8] &= !(1 << (idx % 8));
    // }

    fn is_set<T: Into<usize>>(&self, idx: T) -> bool {
        let idx = idx.into();

        (self.0[idx / 8] & (1 << (idx % 8))) != 0
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut bm = Bitmap::new();

    let mut it = INPUT.lines();

    // load directed edges
    while let Some(v) = it.next() {
        if v.len() == 0 {
            break;
        }

        let (left, right) = v.split_once("|").unwrap();
        let left = left.parse::<u8>()?;
        let right = right.parse::<u8>()?;
        // enumerates the directed edges that are not valid
        let directed_edge = ((right as u16) << SHIFT) | (left as u16);
        bm.set(directed_edge);
    }

    // compute sum over remaining record lines
    let mut sum = 0;
    for v in it {
        let seq: Vec<u8> = v
            .split(",")
            .map(|v| v.parse::<u8>())
            .collect::<Result<_, _>>()?;
        if !valid_order(&seq, &bm) {
            continue;
        }

        // return value of the middle record
        // sequence is guaranteed to be an odd length
        sum += seq[seq.len() / 2] as i32;
    }

    println!("{}", sum);
    Ok(())
}

fn valid_order(seq: &Vec<u8>, bm: &Bitmap) -> bool {
    for left_idx in 0..seq.len() {
        for right_idx in (left_idx + 1)..seq.len() {
            let directed_edge = ((seq[left_idx] as u16) << SHIFT) | (seq[right_idx] as u16);
            if bm.is_set(directed_edge) {
                // a rule exists that says the order should be the other way around
                // so a rule violation is detected, not a valid seq
                return false;
            }
        }
    }

    true
}
