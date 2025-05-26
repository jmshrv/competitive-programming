use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut anagrams: HashMap<String, Vec<String>> = HashMap::new();

        for str in strs {
            let mut sorted_str_vec = str.chars().collect::<Vec<_>>();
            sorted_str_vec.sort_unstable();
            let sorted_str = sorted_str_vec.iter().collect::<String>();

            anagrams.entry(sorted_str).or_default().push(str);
        }

        anagrams.into_values().collect()
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::group_anagrams(vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string()
        ])
    )
}
