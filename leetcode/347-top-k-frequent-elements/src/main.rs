use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut counts: HashMap<i32, i32> = HashMap::new();

        for num in nums {
            *counts.entry(num).or_insert(0) += 1;
        }

        let mut top_k = counts.into_iter().collect::<Vec<_>>();

        top_k.sort_unstable_by(|(_, a_count), (_, b_count)| b_count.cmp(a_count));

        top_k
            .into_iter()
            .map(|(num, _)| num)
            .take(k as usize)
            .collect()
    }
}

fn main() {
    println!("{:?}", Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2))
}
