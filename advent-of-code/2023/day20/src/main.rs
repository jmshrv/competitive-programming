use std::{
    collections::{HashMap, VecDeque},
    io,
};

#[derive(Debug, PartialEq, Eq)]
enum ModuleType {
    FlipFlop(bool),
    Conjunction(HashMap<String, bool>),
    Broadcaster,
}

#[derive(Debug)]
struct Module {
    name: String,
    module_type: ModuleType,
    destinations: Vec<String>,
}

fn parse_module(line: &str) -> (&str, Module) {
    let (module_type_str, destinations_str) =
        line.split_once(" -> ").expect("Failed to split line!");

    let module_type = match module_type_str {
        "broadcaster" => ModuleType::Broadcaster,
        _ if module_type_str.starts_with('%') => ModuleType::FlipFlop(false),
        _ if module_type_str.starts_with('&') => ModuleType::Conjunction(HashMap::new()),
        _ => panic!("Invalid module type {module_type_str}!"),
    };

    let module_name = if module_type_str == "broadcaster" {
        "broadcaster"
    } else {
        &module_type_str[1..]
    };

    let destinations = destinations_str
        .split(", ")
        .map(|str| str.to_owned())
        .collect::<Vec<_>>();

    let module = Module {
        name: module_name.to_string(),
        module_type,
        destinations,
    };

    (module_name, module)
}

fn wire_up_conjunctions(modules: &mut HashMap<&str, Module>) {
    let mut connected_conjunctions = HashMap::new();

    let conjunctions = modules
        .iter()
        .filter(|(_, module)| match module.module_type {
            ModuleType::Conjunction(_) => true,
            _ => false,
        })
        .collect::<Vec<_>>();

    for (conjunction_name, conjunction) in conjunctions {
        let connections = modules
            .iter()
            .filter(|(module_name, module)| {
                module.destinations.contains(&conjunction_name.to_string())
            })
            .collect::<Vec<_>>();

        let connected_conjunction = ModuleType::Conjunction(
            connections
                .iter()
                .map(|(module_name, _)| (module_name.to_string(), false))
                .collect(),
        );

        // modules
        //     .get_mut(conjunction_name)
        //     .expect("Failed to get conjunction!")
        //     .module_type = connected_conjunction;

        connected_conjunctions.insert(conjunction_name.to_string(), connected_conjunction);

        // conjunction.module_type = connected_conjunction;
    }

    for (conjunction_name, connected_conjunction) in connected_conjunctions {
        modules
            .get_mut(conjunction_name.as_str())
            .expect("Failed to get conjunction!")
            .module_type = connected_conjunction;
    }
}

fn send_pulse(
    pulse: bool,
    sent_from: &str,
    module: &Module,
) -> (ModuleType, Vec<(String, String, bool)>) {
    let mut new_pulses = vec![];

    let new_module_type = match &module.module_type {
        ModuleType::FlipFlop(is_on) => {
            if pulse {
                ModuleType::FlipFlop(*is_on)
            } else {
                for destination in &module.destinations {
                    new_pulses.push((module.name.clone(), destination.clone(), !is_on));
                }

                ModuleType::FlipFlop(!is_on)
            }
        }
        ModuleType::Conjunction(ref connections) => {
            let mut new_connections = connections.clone();
            new_connections
                .entry(sent_from.to_string())
                .and_modify(|old_pulse| *old_pulse = pulse);

            let all_high = new_connections.values().all(|is_high| *is_high);

            for destination in &module.destinations {
                new_pulses.push((module.name.clone(), destination.clone(), !all_high));
            }

            ModuleType::Conjunction(new_connections)
        }
        ModuleType::Broadcaster => {
            for destination in &module.destinations {
                new_pulses.push((module.name.clone(), destination.clone(), pulse));
            }

            ModuleType::Broadcaster
        }
    };

    (new_module_type, new_pulses)
}

fn press_button(modules: &mut HashMap<&str, Module>) -> (u64, u64) {
    let (mut low_pulses, mut high_pulses) = (0, 0);

    let mut queue = VecDeque::new();

    let button = Module {
        name: "button".to_string(),
        module_type: ModuleType::Broadcaster,
        destinations: vec!["broadcaster".to_string()],
    };

    for destination in send_pulse(false, "", &button).1 {
        queue.push_back(destination);
    }

    while let Some((sent_from, destination, pulse)) = queue.pop_front() {
        if pulse {
            high_pulses += 1;
        } else {
            low_pulses += 1;
        }

        if let Some(module) = modules.get(destination.as_str()) {
            let (new_module_type, destinations) = send_pulse(pulse, &sent_from, module);

            modules
                .get_mut(destination.as_str())
                .expect("Failed to get destination!")
                .module_type = new_module_type;

            for destination in destinations {
                {
                    queue.push_back(destination);
                }
            }
        }
    }

    (low_pulses, high_pulses)
}

fn main() {
    // let mut input = io::stdin()
    //     .lines()
    //     .filter_map(|res| res.ok())
    //     .map(|line| parse_module(&line))
    //     .collect::<HashMap<_, _>>();

    let input = io::stdin()
        .lines()
        .filter_map(|res| res.ok())
        .collect::<Vec<_>>();

    let mut modules = input
        .iter()
        .map(|line| parse_module(&line))
        .collect::<HashMap<_, _>>();

    wire_up_conjunctions(&mut modules);

    let (mut part1_low_pulses, mut part1_high_pulses) = (0, 0);

    for _ in 0..1000 {
        let (new_low_pulses, new_high_pulses) = press_button(&mut modules);
        part1_low_pulses += new_low_pulses;
        part1_high_pulses += new_high_pulses;
    }

    let part1_answer = part1_low_pulses * part1_high_pulses;

    println!("{part1_answer}");
}
