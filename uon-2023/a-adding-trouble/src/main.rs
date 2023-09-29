use std::io;

fn main() {
    io::stdin()
        .lines()
        .map(|line| {
            line.unwrap()
                .split(" ")
                .map(|num_str| num_str.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|line_iter| {
            let res = line_iter[0] + line_iter[1];
            let expected = line_iter[2];
            if res == expected {
                "correct!"
            } else {
                "wrong!"
            }
        })
        .for_each(|res| println!("{res}"));
}
