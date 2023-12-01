use std::collections::HashMap;
use std::fs;

fn will_contain_gold_bag(bag: &str, rules: Vec<&str>, bags: HashMap<&str, Vec<&str>>) -> bool {
    let mut has_contained_bag = false;
    for rule in &rules {
        let mut rule_string = rule.to_string();
        if rule_string.split_whitespace().last().unwrap() == "bag" {
            rule_string.push('s');
        }
        if rule_string.contains("shiny gold") {
            has_contained_bag = true;
            return has_contained_bag;
        } else if rule_string.contains("no other bags") == false {
            let new_bag_rule = bags
                .get(&rule_string[2..])
                .expect(&format!(
                    "Something went wrong! Rule was {}, hashmap was {:?}",
                    &rule_string[2..],
                    bags
                ))
                .clone();
            if will_contain_gold_bag(&rule_string[2..], new_bag_rule, bags.clone()) {
                has_contained_bag = true;
		return has_contained_bag;
            }
        }
    }
    return has_contained_bag;
}

fn calculate_total_bags(rules: Vec<&str>, bags: HashMap<&str, Vec<&str>>) -> usize {
    let mut count = 0;
    for rule in &rules {
        let mut rule_string = rule.to_string();
        if rule_string.split_whitespace().last().unwrap() == "bag" {
            rule_string.push('s');
        }
        if rule_string.contains("no other bags") == false {
            let amount_of_bags = rule_string.chars().next().unwrap().to_digit(10).unwrap() as usize;
            let new_bag_rule = bags
                .get(&rule_string[2..])
                .expect(&format!(
                    "Something went wrong! Rule was {}, hashmap was {:?}",
                    &rule_string[2..],
                    bags
                ))
                .clone();
            count += amount_of_bags;
            count += calculate_total_bags(new_bag_rule, bags.clone()) * amount_of_bags;
        }
    }
    return count;
}
fn main() {
    let input: String = fs::read_to_string("src/input.txt")
        .expect("Failed to read input")
        .replace(".", "");

    let mut bags: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines() {
        let line_split: Vec<&str> = line.split(" contain ").collect();
        let rules: Vec<&str> = line_split[1].split(", ").collect();
        bags.insert(line_split[0], rules);
    }

    let mut count = 0;
    let mut iters = 0;
    for (bag, rules) in bags.clone() {
        iters += 1;
        println!("Part one iter {}", iters);
        if will_contain_gold_bag(bag, rules, bags.clone()) {
            count += 1
        }
    }
    println!("Part one answer: {}", count);

    let gold_bag = bags.get("shiny gold bags").unwrap();
    println!(
        "Part two answer: {}",
        calculate_total_bags(gold_bag.clone(), bags.clone())
    );
}
