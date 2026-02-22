struct Solution;

fn main() {
    let result = Solution::str_str("zahinissad".to_string(), "hi".to_string());
    println!("{}", result);
}

impl Solution {
    pub fn str_str(h: String, n: String) -> i32 {
        let mut hp = 0;
        loop {
            if hp == h.len() {
                break;
            } else {
                let mut np: usize = 0;
                loop {
                    if np == n.len() {
                        return hp.try_into().unwrap();
                    } else {
                        if h.chars().nth(hp + np) != n.chars().nth(np) {
                            break;
                        }
                    }
                    np += 1;
                }
            }
            hp += 1;
        }
        return -1;
    }
}
