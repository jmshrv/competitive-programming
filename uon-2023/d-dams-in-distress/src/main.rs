use std::{collections::BTreeMap, io};

struct Node {
    pub capacity: u32,
    pub water_level: u32,
    pub children: Vec<usize>,
}

impl Node {
    fn new(capacity: u32, water_level: u32) -> Self {
        Self {
            water_level,
            capacity,
            children: vec![],
        }
    }
}

fn main() {
    let lines = io::stdin()
        .lines()
        .skip(1)
        .map(|line_res| line_res.unwrap())
        .collect::<Vec<_>>();

    let graph_indicies = lines
        .iter()
        .map(|line| line.split(" ").collect::<Vec<_>>())
        .map(|split_line| {
            (
                split_line[0].parse::<usize>().unwrap(),
                split_line[1].parse::<u32>().unwrap(),
                split_line[2].parse::<u32>().unwrap(),
            )
        });

    let mut graph: BTreeMap<usize, Node> = BTreeMap::new();

    for index in graph_indicies {
        if index.0 == 0 {
            graph.insert(1, Node::new(index.1, index.2));
            continue;
        }

        if graph.contains_key(&index.0) {
            *graph.get_mut(&index.0).unwrap();
        } else {
            graph.insert(index.0, Node::new(index.1, index.2));
        }
    }
}
