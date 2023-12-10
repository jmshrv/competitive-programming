use std::{
    collections::{HashMap, HashSet, VecDeque},
    io,
    ops::{Add, Sub},
};

use image::{ImageBuffer, Rgb, RgbImage};

// impl<T> Add for (T, T)
// where
//     T: Add<Output = T>,
// {
//     type Output = (T, T);

//     fn add(self, rhs: Self) -> Self::Output {
//         (self.0 + rhs.0, self.1 + rhs.1)
//     }
// }

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Index(usize, usize);

impl Add<(usize, usize)> for Index {
    type Output = Index;

    fn add(self, rhs: (usize, usize)) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Add<(i32, i32)> for Index {
    type Output = Index;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Self(self.0 + rhs.0 as usize, self.1 + rhs.1 as usize)
    }
}

impl Sub<(usize, usize)> for Index {
    type Output = Index;

    fn sub(self, rhs: (usize, usize)) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Index {
    fn neighbours(&self) -> [Index; 8] {
        // This is likely to overflow and therefore breaks debug builds, but it's AoC and I'm tired
        [
            *self + (-1, -1),
            *self + (-1, 0),
            *self + (-1, 1),
            *self + (0, -1),
            *self + (0, 1),
            *self + (1, -1),
            *self + (1, 0),
            *self + (1, 1),
        ]
    }
}

fn find_start(input: &Vec<String>) -> Option<Index> {
    for (column_index, line) in input.iter().enumerate() {
        for (row_index, char) in line.char_indices() {
            if char == 'S' {
                return Some(Index(column_index, row_index));
            }
        }
    }

    None
}

/// Steps through the map at the given index. Returns a list of next steps.
fn next_steps(input: &Vec<String>, index: Index) -> Option<Vec<Index>> {
    let pipe = input.get(index.0)?.chars().nth(index.1)?;

    match pipe {
        '|' => Some(vec![index + (1, 0), index - (1, 0)]),
        '-' => Some(vec![index + (0, 1), index - (0, 1)]),
        'L' => Some(vec![index - (1, 0), index + (0, 1)]),
        'J' => Some(vec![index - (1, 0), index - (0, 1)]),
        '7' => Some(vec![index + (1, 0), index - (0, 1)]),
        'F' => Some(vec![index + (1, 0), index + (0, 1)]),
        '.' => None,
        'S' => Some(vec![index + (0, 1), index - (0, 1)]), // TODO
        _ => panic!("Invalid pipe!"),
    }
}

fn traverse(input: &Vec<String>, start: Index) -> HashMap<Index, usize> {
    // let next_steps_opt = next_steps(input, start);

    // if let Some(next_steps) = next_steps_opt {
    //     // let unvisited = next_steps
    //     //     .iter()
    //     //     .filter(|step| !visited.contains_key(step))
    //     //     .collect::<Vec<_>>();

    //     for step in next_steps {
    //         if let Some(visited_step_length) = visited.get(&step) {
    //             if *visited_step_length > travel_length {
    //                 visited.insert(step, travel_length);
    //             }
    //         } else {
    //             visited.insert(step, travel_length);
    //         }

    //         println!("{:?}", step);

    //         if step == Index(1, 1) {
    //             return;
    //         }

    //         traverse(input, step, visited, travel_length + 1);
    //     }
    // }

    let mut explored = HashMap::from([(start, 0)]);
    let mut queue = VecDeque::from([(start, 0)]);

    while let Some((index, length)) = queue.pop_front() {
        let next_length = length + 1;

        for next in next_steps(input, index).unwrap_or(vec![]) {
            if !explored.contains_key(&next) {
                explored.insert(next, next_length);
                queue.push_back((next, next_length));
            }
        }
    }

    explored

    // let next_steps_opt = next_steps(input, start);

    // if let Some(next_steps) = next_steps_opt {
    //     for step in next_steps {
    //         if visited.contains_key(&step) {
    //             continue;
    //         }

    //         visited.insert(step, travel_length);

    //         traverse(input, step, visited, travel_length + 1);
    //     }
    // }
}

// fn flood_fill(input: &Vec<String>, index: Index) -> Option<Vec<Index>> {
//     for neighbour in input {

//     }
// }

// fn enclosed(input: &Vec<String>, visited: &HashMap<Index, usize>) -> usize {
//     let mut all_inside = HashSet::new();

//     for pipe in visited.keys() {
//         for neighbour in pipe.neighbours() {
//             if let Some(row) = input.get(neighbour.0) {
//                 if let Some(pipe) = row.chars().nth(neighbour.1) {
//                     if pipe == '.' {
//                         if let Some(inside) = flood_fill(input, neighbour) {
//                             for index in inside {
//                                 all_inside.insert(index);
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//     }

//     todo!();
// }

fn main() {
    let input = io::stdin()
        .lines()
        .filter_map(|res| res.ok())
        .collect::<Vec<_>>();

    let start = find_start(&input).expect("Failed to find start!");

    let visited = traverse(&input, start);

    let part1_answer = visited.values().max().expect("Nothing visited?");

    println!("{part1_answer}");

    // let part2_answer = enclosed(&input, &visited);

    // let mut img: RgbImage = ImageBuffer::new(
    //     input.first().expect("No first line?").chars().count() as u32 * 3,
    //     input.len() as u32 * 3,
    // );

    // img.pixels_mut().for_each(|pix| pix.0 = [127, 127, 127]);

    // for (column_index, line) in input.iter().enumerate() {
    //     for (row_index, char) in line.char_indices() {
    //         let image_row_index = row_index as u32 * 3 + 1;
    //         let image_column_index = column_index as u32 * 3 + 1;

    //         let green = Rgb([0, 255, 0]);
    //         let fill = Rgb([255, 255, 255]);

    //         match char {
    //             '|' => {
    //                 img.put_pixel(image_row_index, image_column_index - 1, fill);
    //                 img.put_pixel(image_row_index, image_column_index, fill);
    //                 img.put_pixel(image_row_index, image_column_index + 1, fill);
    //             }
    //             '-' => {
    //                 img.put_pixel(image_row_index - 1, image_column_index, fill);
    //                 img.put_pixel(image_row_index, image_column_index, fill);
    //                 img.put_pixel(image_row_index + 1, image_column_index, fill);
    //             }
    //             'L' => {
    //                 img.put_pixel(image_row_index, image_column_index - 1, fill);
    //                 img.put_pixel(image_row_index, image_column_index, fill);
    //                 img.put_pixel(image_row_index + 1, image_column_index, fill);
    //             }
    //             'J' => {
    //                 img.put_pixel(image_row_index, image_column_index - 1, fill);
    //                 img.put_pixel(image_row_index, image_column_index, fill);
    //                 img.put_pixel(image_row_index - 1, image_column_index, fill);
    //             }
    //             '7' => {
    //                 img.put_pixel(image_row_index - 1, image_column_index, fill);
    //                 img.put_pixel(image_row_index, image_column_index, fill);
    //                 img.put_pixel(image_row_index, image_column_index + 1, fill);
    //             }
    //             'F' => {
    //                 img.put_pixel(image_row_index, image_column_index, fill);
    //                 img.put_pixel(image_row_index + 1, image_column_index, fill);
    //                 img.put_pixel(image_row_index, image_column_index + 1, fill);
    //             }
    //             // '.' => {
    //             //     for i in (image_column_index - 1)..=(image_column_index + 1) {
    //             //         for j in (image_row_index - 1)..=(image_row_index + 1) {
    //             //             img.put_pixel(j, i, Rgb([0, 0, 0]));
    //             //         }
    //             //     }
    //             // }
    //             '.' => {}
    //             'S' => {
    //                 img.put_pixel(image_row_index - 1, image_column_index, green);
    //                 img.put_pixel(image_row_index, image_column_index, green);
    //                 img.put_pixel(image_row_index + 1, image_column_index, green);
    //             } // TODO
    //             // 'S' => {
    //             //     img.put_pixel(image_row_index, image_column_index, green);
    //             //     img.put_pixel(image_row_index + 1, image_column_index, green);
    //             //     img.put_pixel(image_row_index, image_column_index + 1, green);
    //             // }
    //             _ => panic!("Invalid pipe {}!", char),
    //         }
    //     }
    // }

    // img.save("map.png").expect("Failed to output PNG!");

    // let filled_image = image::open("map_filled.png").expect("Failed to open filled map!");
    // let filled_image_rgb = filled_image
    //     .as_rgb8()
    //     .expect("Failed to convert into RGB image!");

    // let mut part2_answer = 0;

    // for (column_index, line) in input.iter().enumerate() {
    //     for (row_index, _) in line.char_indices() {
    //         let image_row_index = row_index as u32 * 3;
    //         let image_column_index = column_index as u32 * 3;

    //         let mut valid = true;

    //         for i in image_row_index..=image_row_index + 2 {
    //             for j in image_column_index..=image_column_index + 2 {
    //                 if filled_image_rgb[(i, j)] != Rgb([255, 0, 0]) {
    //                     valid = false;
    //                 }
    //             }
    //         }

    //         if valid {
    //             part2_answer += 1;
    //         }
    //     }
    // }

    // Part 2 taken from Colm, sorry :(

    let mut part2_answer = 0;
    // let (min_x, min_y, max_x, max_y) =
    let min_x = visited.keys().min_by(|x, y| x.0.cmp(&y.0)).unwrap().0;
    let min_y = visited.keys().min_by(|x, y| x.1.cmp(&y.1)).unwrap().1;

    let max_x = visited.keys().max_by(|x, y| x.0.cmp(&y.0)).unwrap().0;
    let max_y = visited.keys().max_by(|x, y| x.1.cmp(&y.1)).unwrap().1;

    for y in min_y + 1..=max_y - 1 {
        let mut parity = false;
        for x in min_x..=max_x {
            // I realise now I've been indexing by y,x this whole time and haven't realised
            if visited.contains_key(&Index(y, x)) {
                let pipe = input[y].chars().nth(x).unwrap();

                if matches!(pipe, '|' | 'L' | 'J') {
                    parity = !parity
                }
            } else if parity {
                part2_answer += 1
            }
        }
    }

    println!("{part2_answer}");
}
