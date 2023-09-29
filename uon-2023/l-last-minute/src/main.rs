use std::io;

fn main() {
    let problem_str = io::stdin().lines().next().unwrap().unwrap();

    let problem_vec = problem_str.split(" ").collect::<Vec<_>>();

    let mut a_uniq = problem_vec[0].parse::<u64>().unwrap();
    let mut b_uniq = problem_vec[1].parse::<u64>().unwrap();
    let a_reuse = problem_vec[2].parse::<u64>().unwrap();
    let b_reuse = problem_vec[3].parse::<u64>().unwrap();

    let reusable_perms = a_reuse * b_reuse;

    println!("{}", a_uniq + b_uniq + reusable_perms);
}
