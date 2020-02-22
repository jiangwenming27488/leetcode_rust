struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut res: Vec<char> = vec![];
        s.chars().for_each(|character| {
            if res.is_empty() {
                res.push(character);
            } else {
                let last = *res.last().unwrap();
                if character == ']' && last == '[' || (character == '}' && last == '{') || (character == ')' && last == '(') {
                    res.pop();
                } else {
                    res.push(character);
                }
            }
        });
        res.is_empty()
    }
}

fn main() {
    let test_str = "()".to_string();
    assert_eq!(true, Solution::is_valid(test_str));
    let test_str = "()[]{}".to_string();
    assert_eq!(true, Solution::is_valid(test_str));
    let test_str = "(]".to_string();
    assert_eq!(false, Solution::is_valid(test_str));
    let test_str = "([)]".to_string();
    assert_eq!(false, Solution::is_valid(test_str));
    let test_str = "{[]}".to_string();
    assert_eq!(true, Solution::is_valid(test_str));
}
