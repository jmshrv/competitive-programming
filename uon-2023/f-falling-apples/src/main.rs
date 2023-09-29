use std::io;

fn main() {
    let mut lines = io::stdin()
        .lines()
        .skip(1)
        .map(|line_res| line_res.unwrap())
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut apple_indicies: Vec<Vec<usize>> = vec![];
    let mut slots: Vec<usize> = vec![];

    for _ in 0..lines[0].len() {
        slots.push(lines.len() - 1);
    }

    for i in 0..lines.len() {
        apple_indicies.push(vec![]);
        for j in 0..lines.first().unwrap().len() {
            if lines[i][j] == 'a' {
                apple_indicies[i].push(j);
            }

            if lines[i][j] == '#' {
                slots[j] = i - 1;
            }
        }
    }

    println!("{:?}", slots);

    for i in (0..apple_indicies.len()).rev() {
        for j in apple_indicies[i].iter() {
            if i == lines.len() {
                continue;
            }

            // for lines_i in (i + 1)..lines.len() {
            //     if lines[lines_i][*j] == '.' {
            //         lines[lines_i][*j] = 'a';
            //         lines[lines_i - 1][*j] = '.';
            //     } else {
            //         break;
            //     }
            // }

            lines[i][*j] = '.';
            lines[slots[*j]][*j] = 'a';
            if slots[*j] > 0 {
                slots[*j] = slots[*j] - 1;
            }

            println!("{:?}", slots);

            for line in lines.iter() {
                for char in line {
                    print!("{char}");
                }
                println!();
            }
            println!();
        }
    }

    for line in lines {
        for char in line {
            print!("{char}");
        }
        println!();
    }
}
