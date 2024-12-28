use itertools::join;
use std::error::Error;

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
}

impl Computer {
    fn new(reg: [u64; 3], prg: Vec<u8>) -> Computer {
        Computer {
            output: Vec::<u8>::new(),
            iptr_step: 2,
            iptr: 0,
            prg,
            reg: reg,
        }
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

    fn run(&mut self) {
        while self.iptr < self.prg.len() {
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
                5 => self.output.push((self.combo() % 8) as u8),
                6 => self.reg[B] = self.divide(),
                7 => self.reg[C] = self.divide(),
                _ => panic!("not a valid instruction"),
            }
            self.iptr += self.iptr_step as usize;
            self.iptr_step = 2;
        }
    }
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
    let mut reg = [0 as u64; 3];
    input[..idx].split("\n").map(|x| &x[9..]).for_each(|x| {
        let (name, v) = x.split_once(": ").expect("todo");
        match name.bytes().nth(0).expect("todo") {
            b'A' => reg[A] = v.parse::<u64>().expect("todo"),
            b'B' => reg[B] = v.parse::<u64>().expect("todo"),
            b'C' => reg[C] = v.parse::<u64>().expect("todo"),
            _ => panic!("unknown register"),
        }
    });
    let mut c = Computer::new(reg, prg);

    c.run();
    println!("{}", join(c.output, ","));

    Ok(())
}
