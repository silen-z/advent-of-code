use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = File::open("input/2019_23.txt")?;
    let reader = BufReader::new(input_file);

    let instructions: Vec<Instruction> = reader
        .lines()
        .map(|line| line.unwrap().parse())
        .collect::<Result<_, _>>()?;

    let mut cpu = match advent_of_code::part() {
        advent_of_code::Part::One => Computer::part1(instructions),
        advent_of_code::Part::Two => Computer::part2(instructions),
    };

    cpu.run();

    println!("{}", cpu.reg_b);

    Ok(())
}

struct Computer {
    reg_a: u32,
    reg_b: u32,
    instructions: Vec<Instruction>,
}

impl Computer {
    fn part1(instructions: Vec<Instruction>) -> Self {
        Computer {
            reg_a: 0,
            reg_b: 0,
            instructions,
        }
    }

    fn part2(instructions: Vec<Instruction>) -> Self {
        Computer {
            reg_a: 1,
            reg_b: 0,
            instructions,
        }
    }

    fn run(&mut self) {
        let mut ip = 0;
        while let Some(instruction) = self.instructions.get(ip).cloned() {
            match instruction {
                Instruction::Halve(ref reg) => {
                    self.set(reg, self.get(reg) / 2);
                    ip += 1;
                }
                Instruction::Triple(ref reg) => {
                    self.set(reg, self.get(reg) * 3);
                    ip += 1;
                }
                Instruction::Increment(ref reg) => {
                    self.set(reg, self.get(reg) + 1);
                    ip += 1;
                }
                Instruction::Jump(offset) => {
                    ip = ((ip as isize) + offset) as usize;
                }
                Instruction::JumpIfEven(ref reg, offset) => {
                    if self.get(reg) % 2 == 0 {
                        ip = ((ip as isize) + offset) as usize;
                    } else {
                        ip += 1;
                    }
                }
                Instruction::JumpIfOne(ref reg, offset) => {
                    if self.get(reg) == 1 {
                        ip = ((ip as isize) + offset) as usize;
                    } else {
                        ip += 1;
                    }
                }
            }
        }
    }

    fn get(&self, reg: &Register) -> u32 {
        match reg {
            Register::A => self.reg_a,
            Register::B => self.reg_b,
        }
    }

    fn set(&mut self, reg: &Register, value: u32) {
        match reg {
            Register::A => self.reg_a = value,
            Register::B => self.reg_b = value,
        }
    }
}

#[derive(Clone)]
enum Register {
    A,
    B,
}

impl std::str::FromStr for Register {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "a" => Ok(Register::A),
            "b" => Ok(Register::B),
            _ => Err(InstructionParseError::InvalidRegister),
        }
    }
}

#[derive(Clone)]
enum Instruction {
    Halve(Register),
    Triple(Register),
    Increment(Register),
    Jump(isize),
    JumpIfEven(Register, isize),
    JumpIfOne(Register, isize),
}

impl std::str::FromStr for Instruction {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[0..3] {
            "hlf" => Ok(Instruction::Halve(s[4..].parse()?)),
            "tpl" => Ok(Instruction::Triple(s[4..].parse()?)),
            "inc" => Ok(Instruction::Increment(s[4..].parse()?)),
            "jmp" => Ok(Instruction::Jump(
                s[4..]
                    .parse()
                    .map_err(|_| InstructionParseError::InvalidJumpOffest)?,
            )),
            "jie" => Ok(Instruction::JumpIfEven(
                s[4..5].parse()?,
                s[7..]
                    .parse()
                    .map_err(|_| InstructionParseError::InvalidJumpOffest)?,
            )),
            "jio" => Ok(Instruction::JumpIfOne(
                s[4..5].parse()?,
                s[7..]
                    .parse()
                    .map_err(|_| InstructionParseError::InvalidJumpOffest)?,
            )),
            _ => Err(InstructionParseError::InvalidInstruction),
        }
    }
}
#[derive(Debug)]
enum InstructionParseError {
    InvalidInstruction,
    InvalidJumpOffest,
    InvalidRegister,
}

impl std::fmt::Display for InstructionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "couldn't parse instruction: {:?}", self)
    }
}

impl std::error::Error for InstructionParseError {}
