use advent_of_code::prelude::*;

const INPUT: &str = include_str!("../../input/2020_08.txt");

fn main() {
    let program = INPUT
        .lines()
        .enumerate()
        .map(|(ln, line)| {
            line.parse()
                .or_exit_with(format!("couldn't parse instruction on line '{}'", ln + 1))
        })
        .collect();

    let solution = match advent_of_code::part() {
        advent_of_code::Part::One => {
            let mut computer = Computer::default();
            let (result, _) = computer.run(&program);
            result
        }
        advent_of_code::Part::Two => program
            .patches()
            .find_map(|patch| {
                let mut computer = Computer::with_patch(patch);
                match computer.run(&program) {
                    (result, true) => Some(result),
                    _ => None,
                }
            })
            .or_exit_with("didn't find correct patch"),
    };

    println!("{}", solution);
}
struct Program(Vec<Instruction>);

struct Patch {
    index: usize,
    patched_instruction: Instruction,
}

impl Patch {
    fn for_instruction(instruction: &Instruction, index: usize) -> Option<Patch> {
        let patched_instruction = match instruction {
            NoOp(n) => Jump(*n),
            Jump(n) => NoOp(*n),
            _ => return None,
        };

        Some(Patch {
            index,
            patched_instruction,
        })
    }
}

impl Program {
    fn patches(&self) -> impl Iterator<Item = Patch> + '_ {
        self.0
            .iter()
            .enumerate()
            .filter_map(|(index, instruction)| Patch::for_instruction(instruction, index))
    }
}

#[derive(Default)]
struct Computer {
    acc: i32,
    ip: usize,
    patch: Option<Patch>,
}

impl Computer {
    fn with_patch(patch: Patch) -> Self {
        Computer {
            patch: Some(patch),
            ..Computer::default()
        }
    }

    fn run(&mut self, program: &Program) -> (i32, bool) {
        let mut executed = Vec::with_capacity(program.0.len());
        loop {
            if executed.contains(&self.ip) {
                return (self.acc, false);
            }
            executed.push(self.ip);

            let instruction = match self.patch.as_ref() {
                Some(patch) if patch.index == self.ip => Some(&patch.patched_instruction),
                _ => program.0.get(self.ip),
            };

            match instruction {
                Some(Acc(n)) => {
                    self.acc += n;
                    self.ip += 1;
                }
                Some(Jump(offset)) => {
                    let next_ip = self.ip as i32 + offset;
                    if next_ip < 0 {
                        return (self.acc, true);
                    }
                    self.ip = next_ip as usize;
                }
                Some(NoOp(_)) => {
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
    Acc(i32),
    Jump(i32),
    NoOp(i32),
}
use Instruction::*;

impl std::str::FromStr for Instruction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (opcode, param) = split_once(s, " ").ok_or("invalid instruction format")?;
        let param = param.parse().map_err(|_| "invalid parameter")?;

        match opcode {
            "nop" => Ok(NoOp(param)),
            "jmp" => Ok(Jump(param)),
            "acc" => Ok(Acc(param)),
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
