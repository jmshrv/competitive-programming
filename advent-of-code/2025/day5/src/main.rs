use std::{collections::HashSet, io, ops::RangeInclusive};

fn merge_ranges(ranges: Vec<RangeInclusive<u64>>) -> HashSet<RangeInclusive<u64>> {
    let mut merged_ranges = ranges.clone().into_iter().collect::<HashSet<_>>();

    let mut overlaps_exist = true;

    while overlaps_exist {
        overlaps_exist = false;

        for range in merged_ranges.clone() {
            // We have the awkward and_then because we can't take a reference while messing with the
            // HashSet later on
            let maybe_intersect = merged_ranges
                .iter()
                .find(|merged_range| {
                    if range.start() == merged_range.start() && range.end() == merged_range.end() {
                        return false;
                    }

                    (range.start() >= merged_range.start() && range.start() <= merged_range.end())
                        || (range.end() >= merged_range.start()
                            && range.end() <= merged_range.end())
                })
                .and_then(|intersect| Some(intersect.clone()));

            if let Some(intersect) = maybe_intersect {
                overlaps_exist = true;

                merged_ranges.remove(&range);
                merged_ranges.remove(&intersect);

                let new_start = range.start().min(intersect.start());
                let new_end = range.end().max(intersect.end());

                let new_range = *new_start..=*new_end;
                merged_ranges.insert(new_range);
            } else {
                merged_ranges.insert(range.clone());
            }
        }
    }

    merged_ranges
}

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();

    let (ranges_str, ingredients_str) = input.split_once("\n\n").unwrap();

    let ranges = ranges_str
        .lines()
        .map(|range| {
            let (from_str, to_str) = range.split_once('-').unwrap();

            let from = from_str.parse::<u64>().unwrap();
            let to = to_str.parse::<u64>().unwrap();

            from..=to
        })
        .collect::<Vec<_>>();

    let merged_ranges = merge_ranges(ranges);

    // let calculated = merged_ranges
    //     .iter()
    //     .flat_map(|range| range.clone().into_iter())
    //     .sorted()
    //     .collect::<Vec<_>>();

    // println!("{:?}", calculated);

    let ingredients = ingredients_str
        .lines()
        .map(str::parse::<u64>)
        .map(Result::unwrap)
        .collect::<Vec<_>>();

    let part1_answer = ingredients
        .iter()
        .filter(|ingredient| {
            merged_ranges
                .iter()
                .find(|range| range.contains(ingredient))
                .is_some()
        })
        .count();

    println!("{part1_answer}");

    let part2_answer: usize = merged_ranges.into_iter().map(|range| range.count()).sum();

    println!("{part2_answer}");
}
