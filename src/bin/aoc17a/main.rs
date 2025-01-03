use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

use itertools::Itertools;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Instruction {
    DivA,        // adv
    XorLoadB,    // bxl
    StoreB,      // bst
    JumpNotZero, // jnz
    BXorC,       // bxc
    Output,      // out
    DivB,        // bdv
    DivC,        // cdv
}

impl Instruction {
    fn from_int(i: u8) -> Self {
        match i {
            0 => Instruction::DivA,
            1 => Instruction::XorLoadB,
            2 => Instruction::StoreB,
            3 => Instruction::JumpNotZero,
            4 => Instruction::BXorC,
            5 => Instruction::Output,
            6 => Instruction::DivB,
            7 => Instruction::DivC,
            _ => panic!("Invalid operation"),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Register {
    A,
    B,
    C,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Operand {
    Register(Register),
    Value(u8),
}

impl Operand {
    fn resolve(&self, memory: Memory) -> isize {
        match self {
            Operand::Register(Register::A) => memory.a,
            Operand::Register(Register::B) => memory.b,
            Operand::Register(Register::C) => memory.c,
            Operand::Value(v) => *v as isize,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Operation {
    instruction: Instruction,
    operand: Operand,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Memory {
    a: isize,
    b: isize,
    c: isize,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Machine {
    memory: Memory,
    pc: usize,
}

impl Machine {
    fn new(memory: Memory) -> Self {
        Machine {
            memory,
            pc: 0,
        }
    }

    fn run(&mut self, operations: Vec<Operation>) -> Vec<u8> {
        let mut outputs = Vec::new();
        while self.pc < operations.len() {
            let operation = &operations[self.pc];
            let operand = operation.operand.resolve(self.memory);
            self.pc += 1;
            match operation.instruction {
                Instruction::DivA => {
                    let numerator = self.memory.a;
                    let denominator = 1 << operand;
                    self.memory.a = numerator / denominator;
                }
                Instruction::XorLoadB => {
                    self.memory.b ^= operand;
                }
                Instruction::StoreB => {
                    self.memory.b = operand & 0b111;
                }
                Instruction::JumpNotZero => {
                    if self.memory.a != 0 {
                        assert!(operand % 2 == 0);
                        self.pc = operand as usize / 2;
                    }
                }
                Instruction::BXorC => {
                    self.memory.b ^= self.memory.c;
                }
                Instruction::Output => {
                    outputs.push((self.memory.b & 0b111) as u8);
                }
                Instruction::DivB => {
                    let numerator = self.memory.a;
                    let denominator = 1 << operand;
                    self.memory.b = numerator / denominator;
                }
                Instruction::DivC => {
                    let numerator = self.memory.a;
                    let denominator = 1 << operand;
                    self.memory.c = numerator / denominator;
                }
            }
        }
        outputs
    }
}

fn read_input() -> (Memory, Vec<Operation>) {
    let file = File::open("inputs/17.txt");

    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let br = BufReader::new(file);
    let mut lines = br.lines();

    let mut memory = Memory { a: 0, b: 0, c: 0 };

    memory.a = lines
        .next()
        .unwrap()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .parse()
        .unwrap();
    memory.b = lines
        .next()
        .unwrap()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .parse()
        .unwrap();
    memory.c = lines
        .next()
        .unwrap()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .parse()
        .unwrap();
    let mut lines = lines.skip(1);
    let operations = lines
        .next()
        .unwrap()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split(",")
        .map(|s| s.parse::<u8>().unwrap())
        .chunks(2)
        .into_iter()
        .map(|mut chunk| {
            let instruction = Instruction::from_int(chunk.next().unwrap());
            let operand = chunk.next().unwrap();
            if operand > 7 {
                panic!("Invalid operand");
            }
            let operand = match instruction {
                Instruction::XorLoadB | Instruction::JumpNotZero | Instruction::BXorC => {
                    Operand::Value(operand)
                }
                Instruction::DivA
                | Instruction::StoreB
                | Instruction::Output
                | Instruction::DivB
                | Instruction::DivC => match operand {
                    0..=3 => Operand::Value(operand),
                    4 => Operand::Register(Register::A),
                    5 => Operand::Register(Register::B),
                    6 => Operand::Register(Register::C),
                    _ => panic!("Invalid register"),
                },
            };
            Operation {
                instruction,
                operand,
            }
        })
        .collect();

    (memory, operations)
}

fn main() {
    let now = Instant::now();

    let (memory, operations) = read_input();
    println!("{:?}", memory);
    println!("{:?}", operations);

    let mut machine = Machine::new(memory);
    let outputs = machine.run(operations);
    println!("{}", outputs.iter().join(","));

    println!("{}s", now.elapsed().as_secs_f64());
}
