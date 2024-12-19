use std::io;

use itertools::Itertools;

struct Computer {
    register_a: u64,
    register_b: u64,
    register_c: u64,

    pc: usize,

    program: Vec<u8>,

    output: Vec<u8>,
}

impl Computer {
    fn new(register_a: u64, register_b: u64, register_c: u64, program: Vec<u8>) -> Self {
        Self {
            register_a,
            register_b,
            register_c,
            pc: 0,
            program,
            output: Vec::new(),
        }
    }

    fn combo_operand(&self, operand: u8) -> u64 {
        match operand {
            0..=3 => operand as u64,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            _ => panic!("Invalid combo operand {operand}!"),
        }
    }

    fn adv(&mut self, combo_operand: u64) {
        self.register_a /= 2_u64.pow(combo_operand as u32);
    }

    fn bxl(&mut self, literal_operand: u8) {
        self.register_b ^= literal_operand as u64;
    }

    fn bst(&mut self, combo_operand: u64) {
        self.register_b = combo_operand % 8;
    }

    fn jnz(&mut self, literal_operand: u8) {
        if self.register_a == 0 {
            return;
        }

        // Hack here so that I don't have to care when incrementing pc in run :)
        self.pc = literal_operand as usize - 2;
    }

    fn bxc(&mut self) {
        self.register_b ^= self.register_c;
    }

    fn out(&mut self, combo_operand: u64) {
        self.output.push((combo_operand % 8) as u8);
    }

    fn bdv(&mut self, combo_operand: u64) {
        self.register_b = self.register_a / 2_u64.pow(combo_operand as u32);
    }

    fn cdv(&mut self, combo_operand: u64) {
        self.register_c = self.register_a / 2_u64.pow(combo_operand as u32);
    }

    fn run(&mut self) {
        while let (Some(opcode), Some(operand)) =
            (self.program.get(self.pc), self.program.get(self.pc + 1))
        {
            match opcode {
                0 => self.adv(self.combo_operand(*operand)),
                1 => self.bxl(*operand),
                2 => self.bst(self.combo_operand(*operand)),
                3 => self.jnz(*operand),
                4 => self.bxc(),
                5 => self.out(self.combo_operand(*operand)),
                6 => self.bdv(self.combo_operand(*operand)),
                7 => self.cdv(self.combo_operand(*operand)),
                _ => panic!("Invalid opcode {opcode}!"),
            }

            self.pc += 2;
        }
    }
}

fn parse_registers(registers_str: &str) -> (u64, u64, u64) {
    let split_registers = registers_str.lines().collect::<Vec<_>>();

    let register_a = split_registers[0]
        .strip_prefix("Register A: ")
        .unwrap()
        .parse::<u64>()
        .unwrap();

    let register_b = split_registers[1]
        .strip_prefix("Register B: ")
        .unwrap()
        .parse::<u64>()
        .unwrap();

    let register_c = split_registers[2]
        .strip_prefix("Register C: ")
        .unwrap()
        .parse::<u64>()
        .unwrap();

    (register_a, register_b, register_c)
}

fn parse_program(program_str: &str) -> Vec<u8> {
    program_str
        .strip_prefix("Program: ")
        .unwrap()
        .split(',')
        .map(|instruction_str| instruction_str.parse().unwrap())
        .collect()
}

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();

    let (registers_str, program_str) = input.split_once("\n\n").unwrap();

    let (register_a, register_b, register_c) = parse_registers(registers_str);
    let program = parse_program(program_str);

    let mut computer = Computer::new(register_a, register_b, register_c, program);

    computer.run();

    let part_one = computer
        .output
        .iter()
        .map(|byte| char::from_digit(*byte as u32, 10).unwrap())
        .intersperse(',')
        .collect::<String>();

    println!("{part_one}");
}
