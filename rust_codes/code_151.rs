struct Solution {}

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_whitespace().into_iter().rev().collect::<Vec<&str>>().join(" ")
    }
}

fn main() {
    let source = "the sky is blue".to_string();
    assert_eq!("blue is sky the".to_string(), Solution::reverse_words(source));
    let source = "  hello world!  ".to_string();
    assert_eq!("world! hello".to_string(), Solution::reverse_words(source));
    let source = "a good   example".to_string();
    assert_eq!("example good a".to_string(), Solution::reverse_words(source));
}
