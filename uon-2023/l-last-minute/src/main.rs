use std::io;

fn main() {
    let problem_str = io::stdin().lines().next().unwrap().unwrap();

    let problem_vec = problem_str.split(" ").collect::<Vec<_>>();

    let a_uniq = problem_vec[0].parse::<u64>().unwrap();
    let b_uniq = problem_vec[1].parse::<u64>().unwrap();
    let a_reuse = problem_vec[2].parse::<u64>().unwrap();
    let b_reuse = problem_vec[3].parse::<u64>().unwrap();

    let reusable_perms = a_reuse * b_reuse;

    let a_uniq_b_uniq = u64::min(a_uniq, b_uniq);
    let a_uniq_b_reuse = u64::max(a_uniq, b_reuse);
    let b_uniq_a_reuse = u64::min(b_uniq, a_reuse);

    println!("reusable_perms: {reusable_perms}");
    println!("a_uniq_b_uniq: {a_uniq_b_uniq}");
    println!("a_uniq_b_reuse: {a_uniq_b_reuse}");
    println!("b_uniq_a_reuse: {b_uniq_a_reuse}");

    let mut other = [a_uniq_b_uniq, a_uniq_b_reuse, b_uniq_a_reuse];
    other.sort();
    let other_max = other.last().unwrap();

    println!(
        "{}",
        // reusable_perms + a_uniq_b_uniq + u64::max(a_uniq_b_reuse, b_uniq_a_reuse)
        reusable_perms + other_max
    )
}
