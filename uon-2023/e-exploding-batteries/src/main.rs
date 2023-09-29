use std::io;

fn main() {
    // log(2) + 2/4
    io::stdin()
        .lines()
        .map(|line_res| line_res.unwrap())
        .map(|line| line.parse::<f64>().unwrap())
        .filter(|amperage| *amperage != 0.0)
        .map(|amperage| amperage.log2() + amperage / 4.0)
        .map(|res_float| res_float.round() as u64)
        .for_each(|res| println!("{res}"));
}
