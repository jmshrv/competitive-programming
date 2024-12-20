use std::io;

fn is_design_possible(design: &str, inventory: &[&str]) -> bool {
    if design.is_empty() {
        return true;
    }

    inventory
        .iter()
        .filter(|inventory_design| design.starts_with(**inventory_design))
        .any(|valid_design| {
            is_design_possible(design.strip_prefix(valid_design).unwrap(), inventory)
        })
}

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();

    let (inventory_str, designs_str) = input.split_once("\n\n").unwrap();

    let inventory = inventory_str.split(", ").collect::<Vec<_>>();
    let designs = designs_str.lines().collect::<Vec<_>>();

    let part_one = designs
        .iter()
        .filter(|design| is_design_possible(design, &inventory))
        .count();

    println!("{part_one}");
}
