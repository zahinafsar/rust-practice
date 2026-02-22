struct Solution;

fn main() {
    let res = Solution::length_of_last_word("Hello there, I am Zahin".to_string());
    println!("{}", res);
}

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let res = s.trim().split(' ').last().unwrap().len();
        return res as i32;
    }
}
