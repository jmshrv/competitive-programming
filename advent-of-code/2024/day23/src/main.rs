use std::{
    collections::{HashMap, HashSet},
    io,
};

use itertools::Itertools;

struct Graph<'a> {
    nodes: HashMap<&'a str, HashSet<&'a str>>,
}

impl<'a> Graph<'a> {
    fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    fn add(&mut self, node_a: &'a str, node_b: &'a str) {
        self.nodes.entry(node_a).or_default().insert(node_b);
        self.nodes.entry(node_b).or_default().insert(node_a);
    }
}

fn triangles<'a>(graph: &'a Graph) -> impl Iterator<Item = [&'a str; 3]> {
    graph.nodes.iter().flat_map(|(vertex, neighbours)| {
        neighbours.iter().flat_map(|neighbour| {
            neighbours
                .intersection(&graph.nodes[neighbour])
                .map(|final_neighbour| {
                    let mut triangle = [*vertex, *neighbour, *final_neighbour];
                    triangle.sort_unstable();
                    triangle
                })
        })
    })
}

fn bron_kerbosch<'a>(
    graph: &'a Graph,
    r: &HashSet<&'a str>,
    p: &mut HashSet<&'a str>,
    x: &mut HashSet<&'a str>,
    max_clique: &mut HashSet<&'a str>,
) {
    if p.is_empty() && x.is_empty() && r.len() > max_clique.len() {
        *max_clique = r.clone();
    }

    for vertex in p.clone().iter() {
        let mut new_r = r.clone();
        new_r.insert(vertex);

        let neighbours = graph.nodes.get(vertex).unwrap();

        let mut new_p = p.intersection(neighbours).copied().collect();
        let mut new_x = x.intersection(neighbours).copied().collect();

        bron_kerbosch(graph, &new_r, &mut new_p, &mut new_x, max_clique);

        p.remove(vertex);
        x.insert(vertex);
    }
}

fn main() {
    let input = io::stdin().lines().map(Result::unwrap).collect::<Vec<_>>();

    let mut graph = Graph::new();

    for line in &input {
        let (node_a, node_b) = line.split_once('-').unwrap();

        graph.add(node_a, node_b);
    }

    let part_one = triangles(&graph)
        .unique()
        .filter(|triangle| triangle.iter().any(|vertex| vertex.starts_with('t')))
        .count();

    println!("{part_one}");

    let mut max_clique = HashSet::new();
    let mut p = graph.nodes.keys().copied().collect();

    bron_kerbosch(
        &graph,
        &HashSet::new(),
        &mut p,
        &mut HashSet::new(),
        &mut max_clique,
    );

    let part_two = max_clique
        .iter()
        .copied()
        .sorted()
        .intersperse(",")
        .collect::<String>();

    println!("{part_two}");
}
