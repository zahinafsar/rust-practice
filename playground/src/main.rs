struct Solution;

fn main() {
    let result = Solution::is_valid("]".to_string());
    println!("{}", result);
}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut temp = Vec::new();

        for ch in s.chars() {
            if ch == '(' || ch == '{' || ch == '[' {
                temp.push(ch);
            } else {
                let pop = temp.pop();
                if pop.is_some() {
                    if (ch == ')' && pop != Some('('))
                        || (ch == '}' && pop != Some('{'))
                        || (ch == ']' && pop != Some('['))
                    {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }

        return temp.is_empty();
    }
}
