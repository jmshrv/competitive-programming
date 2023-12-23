use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::io;
use std::ops::RangeInclusive;

use itertools::Itertools;
use nom::character::complete::{char, u64};
use nom::IResult;

type Block = ((u64, u64, u64), (u64, u64, u64));

trait BlockTrait {
    fn xrange(&self) -> RangeInclusive<u64>;

    fn yrange(&self) -> RangeInclusive<u64>;
}

impl BlockTrait for Block {
    fn xrange(&self) -> RangeInclusive<u64> {
        self.0 .0.min(self.1 .0)..=self.0 .0.max(self.1 .0)
    }

    fn yrange(&self) -> RangeInclusive<u64> {
        self.0 .1.min(self.1 .1)..=self.0 .1.max(self.1 .1)
    }
}

fn parse_line(line: &str) -> IResult<&str, Block> {
    let (remaining, x1) = u64(line)?;
    let (remaining, _) = char(',')(remaining)?;
    let (remaining, y1) = u64(remaining)?;
    let (remaining, _) = char(',')(remaining)?;
    let (remaining, z1) = u64(remaining)?;
    let (remaining, _) = char('~')(remaining)?;
    let (remaining, x2) = u64(remaining)?;
    let (remaining, _) = char(',')(remaining)?;
    let (remaining, y2) = u64(remaining)?;
    let (remaining, _) = char(',')(remaining)?;
    let (remaining, z2) = u64(remaining)?;

    let result = ((x1, y1, z1), (x2, y2, z2));

    Ok((remaining, result))
}

fn supports(grouped_input: &BTreeMap<u64, Vec<&Block>>, block: &Block) -> Vec<Block> {
    let mut block_z = block.0 .2.max(block.1 .2);

    let mut supported_blocks = vec![];

    for (level, blocks) in grouped_input {
        if level <= block_z {
            continue;
        }

        for other_block in blocks {
            let intersect_x = block
                .xrange()
                .filter(|x| other_block.xrange().contains(&x))
                .collect::<Vec<_>>();

            if !intersect_x.is_empty() {
                supported_blocks.push(**other_block);
            }
        }
    }

    supported_blocks
}

fn main() {
    let input = io::stdin()
        .lines()
        .filter_map(|res| res.ok())
        .map(|line| parse_line(&line).expect("Failed to parse line!").1)
        .collect::<Vec<_>>();

    let mut grouped_input = BTreeMap::new();

    let levels = input
        .iter()
        .flat_map(|block| block.0 .2.min(block.1 .2)..=block.0 .2.max(block.1 .2))
        .collect::<BTreeSet<_>>();

    for level in levels {
        let blocks = input.iter().filter(|block| {
            (block.0 .2.min(block.1 .2)..=block.0 .2.max(block.1 .2)).contains(&level)
        });

        grouped_input.insert(level, blocks.collect::<Vec<_>>());
    }

    println!("{:?}", grouped_input);

    let mut part1_answer = 0;

    for (level, blocks) in grouped_input {
        for block in blocks {
            for supported_block in supports(&grouped_input, block) {}
        }
    }
}
