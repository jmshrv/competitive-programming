use std::io;

use itertools::Itertools;

struct Computer<'a> {
    register_a: u64,
    register_b: u64,
    register_c: u64,

    pc: usize,

    program: &'a [u8],

    output: Vec<u8>,
}

impl Computer<'_> {
    fn new(register_a: u64, register_b: u64, register_c: u64, program: &[u8]) -> Computer<'_> {
        let program_len = program.len();

        Computer {
            register_a,
            register_b,
            register_c,
            pc: 0,
            program,
            output: Vec::with_capacity(program_len),
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

fn solve_part_two(
    register_a_goal: u64,
    register_b: u64,
    register_c: u64,
    program: &[u8],
    i: usize,
) -> Option<u64> {
    for k in 0..8 {
        let a = register_a_goal << 3 | k;

        let mut computer = Computer::new(a, register_b, register_c, program);

        computer.run();

        if computer.output == program[i..] {
            if i == 0 {
                return Some(a);
            }

            if let Some(result) = solve_part_two(a, register_b, register_c, program, i - 1) {
                return Some(result);
            }
        }
    }

    None
}

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();

    let (registers_str, program_str) = input.split_once("\n\n").unwrap();

    let (part_one_register_a, register_b, register_c) = parse_registers(registers_str);
    let program = parse_program(program_str);

    let mut part_one_computer =
        Computer::new(part_one_register_a, register_b, register_c, &program);

    part_one_computer.run();

    let part_one = part_one_computer
        .output
        .iter()
        .map(|byte| char::from_digit(*byte as u32, 10).unwrap())
        .intersperse(',')
        .collect::<String>();

    println!("{part_one}");

    let part_two = solve_part_two(0, register_b, register_c, &program, program.len() - 1).unwrap();

    println!("{part_two}");
}
