use std::error::Error;

// assumes the start start of reg B and C are always 0
//
// could have used https://en.wikipedia.org/wiki/Z3_Theorem_Prover
// - https://github.com/Z3Prover/z3/
//
// ultimately there was a pattern when processing numbers of sufficient
// quantity to produce output similar to the end state of the program

const INPUT: &str = include_str!("../../../input.txt");

const A: usize = 0;
const B: usize = 1;
const C: usize = 2;

struct Computer {
    iptr: usize,
    output: Vec<u8>,
    prg: Vec<u8>,
    reg: [u64; 3],
    iptr_step: u8,
    offset: u8,
}

impl Computer {
    fn new(reg: [u64; 3], prg: Vec<u8>) -> Computer {
        Computer {
            offset: 0,
            output: Vec::<u8>::new(),
            iptr_step: 2,
            iptr: 0,
            prg,
            reg: reg,
        }
    }

    fn reset_for_find(&mut self) {
        self.output.clear();
        self.iptr = 0;
        self.reg[B] = 0;
        self.reg[C] = 0;
    }

    fn combo(&self) -> u64 {
        match self.prg[self.iptr + 1] {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.reg[A],
            5 => self.reg[B],
            6 => self.reg[C],
            7 => panic!("not a valid combo value id: 7"),
            _ => panic!("not a valid combo value id: greater than 7"),
        }
    }

    fn literal(&self) -> u8 {
        self.prg[self.iptr + 1]
    }

    fn divide(&self) -> u64 {
        self.reg[A] / (2_u64.pow(self.combo() as u32))
    }

    fn run(&mut self) -> bool {
        self.iptr_step = 2;
        while self.iptr < self.prg.len() {
            // TODO: how about infinite cycles?
            match self.prg[self.iptr] {
                0 => self.reg[A] = self.divide(),
                1 => self.reg[B] = self.reg[B] ^ (self.literal() as u64),
                2 => self.reg[B] = self.combo() % 8,
                3 => {
                    if self.reg[A] != 0 {
                        self.iptr = self.literal() as usize;
                        self.iptr_step = 0;
                    }
                }
                4 => self.reg[B] = self.reg[B] ^ self.reg[C],
                5 => {
                    if self.output.len() >= self.prg.len() {
                        return false;
                    }
                    let v = (self.combo() % 8) as u8;
                    if self.offset > 0
                        && self.prg[self.prg.len() - self.offset as usize + self.output.len()] != v
                    {
                        return false;
                    }
                    self.output.push(v);
                }
                6 => self.reg[B] = self.divide(),
                7 => self.reg[C] = self.divide(),
                _ => panic!("not a valid instruction"),
            }
            self.iptr += self.iptr_step as usize;
            self.iptr_step = 2;
        }

        true
    }
}

// find is a recursive depth first search with minimal stack size usage
//
// ideally this would be a closure around the pointer types, but not sure how
// to make a recursive closure in rust without type shenanigans
fn find(c: &mut Computer, out: &mut u64, a: u64, i: u8) -> bool {
    c.reset_for_find();
    c.offset = i;
    c.reg[A] = a;

    if !c.run() {
        return false;
    }

    if c.prg == c.output {
        *out = a;
        return true;
    }

    if i == 0 || c.output.as_slice() == &c.prg.as_slice()[(c.prg.len() - i as usize)..] {
        for n in 0..8 {
            if find(c, out, 8 * a + n, i + 1) {
                return true;
            }
        }
    }

    false
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = INPUT.trim_end();
    let idx = input.find("\n\n").expect("failed to find program start");
    let prg = input[(idx + 11)..]
        .split(",")
        .map(|c| {
            c.parse::<u8>()
                .expect("failed to parse a program instruction")
        })
        .collect::<Vec<u8>>();
    let mut c = Computer::new([0; 3], prg);

    let mut answer = 0 as u64;
    if !find(&mut c, &mut answer, 0, 0) {
        panic!("failed to find an answer")
    }

    println!("{}", answer);

    Ok(())
}
