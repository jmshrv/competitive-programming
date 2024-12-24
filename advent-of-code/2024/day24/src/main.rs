use std::{
    collections::{HashMap, HashSet},
    io,
    str::FromStr,
};

use itertools::Itertools;

enum Operator {
    And,
    Or,
    Xor,
}

impl Operator {
    fn evaluate(&self, lhs: bool, rhs: bool) -> bool {
        match self {
            Operator::And => lhs && rhs,
            Operator::Or => lhs || rhs,
            Operator::Xor => lhs ^ rhs,
        }
    }
}

impl FromStr for Operator {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(Self::And),
            "OR" => Ok(Self::Or),
            "XOR" => Ok(Self::Xor),
            _ => Err(()),
        }
    }
}

struct Gate<'a> {
    lhs: &'a str,
    rhs: &'a str,
    out: &'a str,
    operator: Operator,
}

impl<'a> Gate<'a> {
    fn run(&self, states: &mut HashMap<&'a str, bool>) -> bool {
        if let (Some(lhs), Some(rhs)) = (states.get(self.lhs), states.get(self.rhs)) {
            states.insert(self.out, self.operator.evaluate(*lhs, *rhs));

            return true;
        }

        false
    }
}

// Should technically be TryFrom but this is AoC so no if it breaks may as well panic :)
impl<'a> TryFrom<&'a str> for Gate<'a> {
    type Error = ();
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let mut split = value.split(' ');

        // rust fans be like "it doesn't allocate tho"
        let lhs = split.next().ok_or(())?;
        let operator_str = split.next().ok_or(())?;
        let rhs = split.next().ok_or(())?;
        let out = split.nth(1).ok_or(())?;

        let operator = operator_str.parse()?;

        Ok(Self {
            lhs,
            rhs,
            out,
            operator,
        })
    }
}

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();

    let (initial_states_str, gates_str) = input.split_once("\n\n").unwrap();

    let mut states = initial_states_str
        .lines()
        .map(|line| line.split_once(": ").unwrap())
        .map(|(wire, state_str)| (wire, state_str.parse::<u8>().unwrap() != 0))
        .collect::<HashMap<_, _>>();

    let gates = gates_str
        .lines()
        .map(|line| Gate::try_from(line).unwrap())
        .collect::<Vec<_>>();

    let z_states = gates
        .iter()
        .map(|gate| gate.out)
        .filter(|out| out.starts_with('z'))
        .collect::<HashSet<_>>();

    while !z_states.iter().all(|state| states.contains_key(state)) {
        for gate in &gates {
            gate.run(&mut states);
        }
    }

    let part_one = states
        .iter()
        .filter(|(state_name, _)| state_name.starts_with('z'))
        .sorted_by(|(state_name_a, _), (state_name_b, _)| {
            let state_name_a_int = state_name_a
                .strip_prefix('z')
                .unwrap()
                .parse::<u8>()
                .unwrap();
            let state_name_b_int = state_name_b
                .strip_prefix('z')
                .unwrap()
                .parse::<u8>()
                .unwrap();

            state_name_b_int.cmp(&state_name_a_int) // Sort in reverse order
        })
        .map(|(_, value)| value)
        .fold(0, |acc, value| (acc << 1) | *value as u64);

    println!("{part_one}");
}
