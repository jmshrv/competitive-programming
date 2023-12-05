use std::{io, ops::Range};

use rayon::iter::{IntoParallelIterator, ParallelIterator};

/// A range for Advent of Code day 5 - not to be confused with Rust's Range type
#[derive(Debug)]
struct AoCRange {
    destination_start: i64,
    source_start: i64,
    range_length: i64,
}

impl AoCRange {
    fn new(destination_start: i64, source_start: i64, range_length: i64) -> Self {
        Self {
            destination_start,
            source_start,
            range_length,
        }
    }

    /// Calculates the destination from the given source. If the source is out-of-range, returns None.
    fn destination(&self, source: i64) -> Option<i64> {
        let source_diff = source - self.source_start;

        if source_diff >= 0 && source_diff < self.range_length {
            Some(self.destination_start + source_diff)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Category {
    ranges: Vec<AoCRange>,
    child_category: Option<Box<Category>>,
}

impl Category {
    fn new(ranges: Vec<AoCRange>, child_category: Option<Category>) -> Self {
        Self {
            ranges,
            child_category: child_category.map(Box::new),
        }
    }

    fn recurse(&self, seed: i64) -> i64 {
        if let Some(child_category) = &self.child_category {
            child_category.recurse(self.destination(seed))
        } else {
            self.destination(seed)
        }
    }
    fn destination(&self, source: i64) -> i64 {
        self.ranges
            .iter()
            .find_map(|range| range.destination(source))
            .unwrap_or(source)
    }
}

fn parse_seeds(line: &str) -> Vec<i64> {
    line.strip_prefix("seeds: ")
        .expect("Failed to find seeds: prefex")
        .split_ascii_whitespace()
        .filter_map(|seed_str| seed_str.parse::<i64>().ok())
        .collect::<Vec<_>>()
}

fn parse_seed_ranges(line: &str) -> Vec<Range<i64>> {
    parse_seeds(line)
        .chunks(2)
        .map(|seed_range_pair| seed_range_pair[0]..seed_range_pair[0] + seed_range_pair[1])
        // .map(|range| range.collect::<Vec<_>>())
        // .flatten()
        // .collect::<HashSet<_>>()
        // .iter()
        // .map(|seed| *seed)
        .collect::<Vec<_>>()
}

/// Parses the input into the root category. Note that the seeds section must be skipped.
fn parse_category(input: &[String]) -> Category {
    // let (category, remaining) = input.split_once("\n\n").unwrap_or((input, ""));
    let newline_index_res = input.iter().enumerate().find(|line| line.1.is_empty());

    let empty_slice = &["".to_string()][..];

    let (category, remaining) = if let Some(newline_index) = newline_index_res {
        input.split_at(newline_index.0 + 1)
    } else {
        (input, empty_slice)
    };

    let category_no_title = category.iter().skip(1);

    let mut ranges = vec![];

    for line in category_no_title {
        if line.is_empty() {
            continue;
        }

        let numbers = line
            .split_ascii_whitespace()
            .map(|num| num.parse::<i64>().expect("Failed to parse number"))
            .collect::<Vec<_>>();

        if numbers.len() != 3 {
            panic!("Got {} numbers in a line, expected 3", numbers.len());
        }

        let range = AoCRange::new(numbers[0], numbers[1], numbers[2]);
        ranges.push(range);
    }

    if remaining.is_empty() {
        return Category::new(ranges, None);
    }

    Category::new(ranges, Some(parse_category(remaining)))
}

fn main() {
    let input = io::stdin()
        .lines()
        .filter_map(|res| res.ok())
        .collect::<Vec<_>>();

    let seeds_line = input.first().expect("No first line?");
    let seeds = parse_seeds(&seeds_line);
    let category = parse_category(&input[2..]);

    let part1_answer = seeds
        .iter()
        .map(|seed| category.recurse(*seed))
        .min()
        .expect("No seeds?");

    println!("{part1_answer}");

    let seed_ranges = parse_seed_ranges(&seeds_line);

    let part2_answer = seed_ranges
        .iter()
        .map(|seed_range| {
            seed_range
                .clone()
                .into_par_iter()
                .map(|seed| category.recurse(seed))
                .min()
                .expect("No seed in range?")
        })
        .min()
        .expect("Still no seeds?");

    println!("{part2_answer}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aoc_range_destination() {
        let aoc_range = AoCRange::new(50, 98, 2);

        assert_eq!(aoc_range.destination(98), Some(50));
        assert_eq!(aoc_range.destination(99), Some(51));
        assert_eq!(aoc_range.destination(0), None);
    }

    #[test]
    fn test_category_recurse() {
        let chain = Category::new(
            vec![AoCRange::new(50, 98, 2), AoCRange::new(52, 50, 48)],
            Some(Category::new(
                vec![
                    AoCRange::new(0, 15, 37),
                    AoCRange::new(37, 52, 2),
                    AoCRange::new(39, 0, 15),
                ],
                None,
            )),
        );

        assert_eq!(chain.recurse(79), 81);
        assert_eq!(chain.recurse(14), 53);
        assert_eq!(chain.recurse(55), 57);
        assert_eq!(chain.recurse(13), 52);
    }
}
