use std::io;

use itertools::iproduct;

fn neighbours(map: &[Vec<char>], posy: usize, posx: usize) -> impl Iterator<Item = char> {
    let ystart = if posy == 0 { 0 } else { posy - 1 };
    let yend = map.len().min(posy + 2); // +2 to fiddily get the +1 in a non-inclusive range

    let xstart = if posx == 0 { 0 } else { posx - 1 };
    let xend = map[posx].len().min(posx + 2);

    iproduct!(ystart..yend, xstart..xend)
        .filter(move |(y, x)| !(*y == posy && *x == posx))
        .map(|(y, x)| map[y][x])
}

fn accessible_indices(map: &[Vec<char>]) -> impl Iterator<Item = (usize, usize)> {
    let xlen = map[0].len();

    iproduct!(0..map.len(), 0..xlen)
        .filter(|(y, x)| map[*y][*x] == '@')
        .filter(|(y, x)| neighbours(map, *y, *x).filter(|&cell| cell == '@').count() < 4)
}

fn cleanup(mut map: Vec<Vec<char>>) -> u32 {
    let mut did_do_clean = true;
    let mut removed_count = 0;
    
    while did_do_clean {
        did_do_clean = false;

        let indices = accessible_indices(&map).collect::<Vec<_>>();
        
        for (y, x) in indices {
            map[y][x] = '.';
            did_do_clean = true; // Since we found rolls to move, there still may be work to do.
            removed_count += 1;
        }

        // If we get here without looping through any indices, we're done and the = false above
        // will break us out
    }

    removed_count
}

fn main() {
    let input = io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let part1_answer = accessible_indices(&input).count();
    println!("{part1_answer}");

    let part2_answer = cleanup(input);
    println!("{part2_answer}");
}
