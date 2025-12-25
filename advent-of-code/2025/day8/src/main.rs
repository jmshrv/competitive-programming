use std::{collections::HashSet, env, io};

use itertools::Itertools;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct JunctionBox {
    x: i64,
    y: i64,
    z: i64,
}

fn distance(from: &JunctionBox, to: &JunctionBox) -> i64 {
    // No need to do a sqrt, we're just comparing to find the shortest, don't care that it's
    // actually distance^2
    (from.x - to.x).pow(2) + (from.y - to.y).pow(2) + (from.z - to.z).pow(2)
}

fn run(junction_boxes: &[JunctionBox], connection_count: usize, part2: bool) -> usize {
    let shortest_boxes = junction_boxes
        .iter()
        .tuple_combinations()
        .sorted_unstable_by(|(aa, ab), (ba, bb)| distance(aa, ab).cmp(&distance(ba, bb)))
        .take(if part2 { usize::MAX } else { connection_count })
        .collect::<Vec<_>>();

    let mut circuits: Vec<HashSet<JunctionBox>> = Vec::new();

    for (box_a, box_b) in shortest_boxes {
        connect_boxes(&mut circuits, box_a, box_b);

        if part2 && circuits.len() == 1 && circuits[0].len() == junction_boxes.len() {
            return (box_a.x * box_b.x) as usize;
        }
    }

    circuits
        .iter()
        .map(|circuit| circuit.len())
        .sorted_unstable()
        .rev()
        .take(3)
        .product()
}

fn connect_boxes(
    circuits: &mut Vec<HashSet<JunctionBox>>,
    box_a: &JunctionBox,
    box_b: &JunctionBox,
) {
    let circuit_a_idx = circuits.iter().position(|c| c.contains(box_a));
    let circuit_b_idx = circuits.iter().position(|c| c.contains(box_b));

    match (circuit_a_idx, circuit_b_idx) {
        (Some(a), Some(b)) if a != b => {
            // Merge circuits! Remove one and add its contents to the other
            let circuit_b = circuits.remove(b);
            circuits[if a > b { a - 1 } else { a }].extend(circuit_b);
        }
        (Some(a), None) => {
            circuits[a].insert(*box_b);
        }
        (None, Some(b)) => {
            circuits[b].insert(*box_a);
        }
        (Some(_), Some(_)) => { /* same circuit, nothing to do */ }
        (None, None) => {
            circuits.push(HashSet::from([*box_a, *box_b]));
        }
    }
}

fn main() {
    let input = io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            let (x_str, y_str, z_str) = line.split(',').collect_tuple().unwrap();

            JunctionBox {
                x: x_str.parse().unwrap(),
                y: y_str.parse().unwrap(),
                z: z_str.parse().unwrap(),
            }
        })
        .collect::<Vec<_>>();

    // who needs clap when you can do it this way
    let (_, connection_count_str) = env::args().collect_tuple().unwrap();
    let connection_count = connection_count_str.parse::<usize>().unwrap();

    let part1_answer = run(&input, connection_count, false);
    println!("{part1_answer}");

    let part2_answer = run(&input, connection_count, true);
    println!("{part2_answer}");
}
