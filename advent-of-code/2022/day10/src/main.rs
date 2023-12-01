use std::{collections::HashSet, fmt::Debug, io, str::FromStr};

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Noop,
    Addx(i32),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (instruction, value) = s.split_once(' ').unwrap_or((s, ""));

        match instruction {
            "noop" => Ok(Instruction::Noop),
            "addx" => Ok(Instruction::Addx(
                value
                    .parse()
                    .expect(&format!("Failed to parse {} as int", value)),
            )),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Registers {
    cycles: usize,
    pc: usize,
    x: i32,
}

impl Registers {
    fn new() -> Self {
        Self {
            cycles: 0,
            pc: 0,
            x: 1,
        }
    }
}

#[derive(Debug)]
struct CPU {
    program: Vec<Instruction>,
    registers: Registers,
    is_in_add_step: bool,
}

impl CPU {
    fn new(program: Vec<Instruction>) -> Self {
        Self {
            program: program,
            registers: Registers::new(),
            is_in_add_step: false,
        }
    }

    fn addx(&mut self, v: i32) -> bool {
        if self.is_in_add_step {
            self.registers.x += v;
            self.is_in_add_step = false;
            return true;
        } else {
            self.is_in_add_step = true;
            return false;
        }
    }

    fn execute(&mut self) -> Option<Registers> {
        if let Some(instruction) = &self.program.get(self.registers.pc) {
            let should_step = match instruction {
                Instruction::Noop => true,
                Instruction::Addx(v) => self.addx(*v),
            };

            self.registers.cycles += 1;

            if should_step {
                self.registers.pc += 1;
            }

            return Some(self.registers);
        }

        None
    }
}

impl Iterator for CPU {
    type Item = Registers;
    fn next(&mut self) -> Option<Self::Item> {
        match self.execute() {
            Some(_) => Some(self.registers),
            None => None,
        }
    }
}

fn main() {
    let input: Vec<Instruction> = io::stdin()
        .lines()
        .map(|line_res| {
            let line = line_res.unwrap();
            line.parse()
                .expect(&format!("Failed to parse instruction {}", line))
        })
        .collect();

    let cpu_1 = CPU::new(input.clone());

    let signal_cycles = HashSet::from([19, 59, 99, 139, 179, 219]);

    // cpu.into_iter().for_each(|registers| {
    //     if signal_cycles.contains(&registers.cycles) {
    //         println!("SIGNAL {}", (registers.cycles + 1) as i32 * registers.x);
    //     }
    // });

    let answer_1: i32 = cpu_1
        .into_iter()
        .filter(|registers| signal_cycles.contains(&registers.cycles))
        .map(|registers| (registers.cycles + 1) as i32 * registers.x)
        .sum();

    println!("{}", answer_1);

    let cpu_2 = CPU::new(input);

    let mut lit_pixels = [false; 240];

    lit_pixels[0] = true; // Needed to fill in for first instruction

    cpu_2.into_iter().for_each(|registers| {
        let diff = (registers.cycles % 40) as i32 - registers.x;
        if diff >= -1 && diff <= 1 {
            lit_pixels[registers.cycles] = true;
        }
    });

    lit_pixels.chunks(40).for_each(|line| {
        for pixel in line {
            if *pixel {
                print!("█");
            } else {
                print!(" ");
            }
        }
        println!();
    });
}
