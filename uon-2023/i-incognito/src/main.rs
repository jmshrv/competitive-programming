use std::{
    collections::{HashMap, HashSet},
    io,
};

fn main() {
    let lines = io::stdin()
        .lines()
        .skip(1)
        .map(|line_res| line_res.unwrap())
        .collect::<Vec<_>>();

    let mut i = 0;
    while i < lines.len() {
        let disguise_count = lines[i].parse::<usize>().unwrap();
        i += 1;

        let mut disguises: HashMap<String, HashSet<String>> = HashMap::new();

        for j in i..i + disguise_count {
            let disguise = &lines[j];
            let split_disguise = disguise.split_once(" ").unwrap();

            if disguises.contains_key(split_disguise.1) {
                disguises
                    .get_mut(split_disguise.1)
                    .unwrap()
                    .insert(split_disguise.0.to_string());
            } else {
                disguises.insert(
                    split_disguise.1.to_string(),
                    HashSet::from([split_disguise.0.to_string(), "nothing".to_string()]),
                );
            }
        }

        let mut count = disguises
            .values()
            .map(|disguise| disguise.len())
            .product::<usize>()
            - disguises.len()
            + 1;

        if disguises.len() == 1 {
            count -= 1;
        }

        println!("{count}");

        i = i + disguise_count;
    }
}
