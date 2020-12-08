use advent_of_code::prelude::*;

const INPUT: &str = include_str!("../../input/2020_08.txt");

fn main() {
    let program = INPUT
        .lines()
        .map(|line| line.parse().or_exit_with("couldn't parse instruction"))
        .collect();

    let solution = match advent_of_code::part() {
        advent_of_code::Part::One => Computer::new(program).run().0,
        advent_of_code::Part::Two => program
            .patches()
            .find_map(|patched| match Computer::new(patched).run() {
                (result, true) => Some(result),
                _ => None,
            })
            .or_exit_with("didn't find correct patch"),
    };

    println!("{}", solution);
}
struct Program(Vec<Instruction>);

impl Program {
    fn patches(self) -> impl Iterator<Item = Program> {
        (0..self.0.len()).filter_map(move |line| self.create_patch(line))
    }

    fn create_patch(&self, line: usize) -> Option<Program> {
        let patch = match self.0.get(line) {
            Some(Instruction::NoOp(n)) => Instruction::Jump(*n),
            Some(Instruction::Jump(n)) => Instruction::NoOp(*n),
            _ => return None,
        };

        let mut patched = self.0.clone();
        patched[line] = patch;

        Some(Program(patched))
    }
}

struct Computer {
    acc: isize,
    ip: isize,
    program: Program,
    exectued: Vec<usize>,
}

impl Computer {
    fn new(program: Program) -> Self {
        Computer {
            acc: 0,
            ip: 0,
            program,
            exectued: Vec::new(),
        }
    }

    fn run(&mut self) -> (isize, bool) {
        loop {
            let ip = self.ip as usize;

            if self.exectued.contains(&ip) {
                return (self.acc, false);
            }

            self.exectued.push(ip);

            match self.program.0.get(ip) {
                Some(Instruction::Acc(n)) => {
                    self.acc += n;
                    self.ip += 1;
                }
                Some(Instruction::Jump(jmp)) => {
                    self.ip += jmp;
                }
                Some(Instruction::NoOp(_)) => {
                    self.ip += 1;
                }
                None => {
                    return (self.acc, true);
                }
            }
        }
    }
}

#[derive(Clone)]
enum Instruction {
    Acc(isize),
    Jump(isize),
    NoOp(isize),
}

impl std::str::FromStr for Instruction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (opcode, param) = split_once(s, " ").ok_or("invalid instruction format")?;
        let param = param.parse().map_err(|_| "invalid parameter")?;

        match opcode {
            "nop" => Ok(Instruction::NoOp(param)),
            "jmp" => Ok(Instruction::Jump(param)),
            "acc" => Ok(Instruction::Acc(param)),
            _ => Err("unknown instruction"),
        }
    }
}

impl std::iter::FromIterator<Instruction> for Program {
    fn from_iter<I: IntoIterator<Item = Instruction>>(iter: I) -> Self {
        let instructions = iter.into_iter().collect();
        Program(instructions)
    }
}
