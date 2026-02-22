struct Solution;

fn main() {
    let condi1 = Solution::search_insert(vec![1, 2, 3, 4, 5], 4);
    let condi2 = Solution::search_insert(vec![1, 2, 3, 5, 6], 4);
    println!("{} {}", condi1, condi2);
}

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        for i in 0..nums.len() {
            if target <= nums[i] {
                return i as i32;
            }
        }
        return nums.len() as i32;
    }
}
