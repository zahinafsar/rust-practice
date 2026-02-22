struct Solution;

fn main() {
    let res1 = Solution::plus_one(vec![1, 2, 3]);
    assert_eq!(res1, [1, 2, 4]);

    let res2 = Solution::plus_one(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
    assert_eq!(res2, [9, 8, 7, 6, 5, 4, 3, 2, 1, 1]);
}

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let digits_in_string: String = digits.iter().map(|x| x.to_string()).collect();
        let digits_in_number: i128 = digits_in_string.parse().unwrap();
        return (digits_in_number + 1)
            .to_string()
            .chars()
            .map(|x| x.to_digit(10).unwrap() as i32)
            .collect();
    }
}
