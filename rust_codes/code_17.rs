struct Solution {}

impl Solution {
    fn get_vec_from_number(number: char) -> Vec<char> {
        match number {
            '2' => vec!['a', 'b', 'c'],
            '3' => vec!['d', 'e', 'f'],
            '4' => vec!['g', 'h', 'i'],
            '5' => vec!['j', 'k', 'l'],
            '6' => vec!['m', 'n', 'o'],
            '7' => vec!['p', 'q', 'r', 's'],
            '8' => vec!['t', 'u', 'v'],
            '9' => vec!['w', 'x', 'y', 'z'],
            _ => vec![]
        }
    }
    fn calculate(digits: &String) -> Vec<String> {
        let mut res: Vec<String> = vec![];
        if !digits.is_empty() {
            let characters = Solution::get_vec_from_number(digits.chars().nth(0).unwrap());
            for character in characters.iter() {
                let calculates = Solution::calculate(&digits[1..].to_string());
                if calculates.is_empty() {
                    res.push(character.to_string());
                } else {
                    for cal in calculates.iter() {
                        res.push(character.to_string() + cal);
                    }
                }
            }
        }
        res
    }

    pub fn letter_combinations(digits: String) -> Vec<String> {
        Solution::calculate(&digits)
    }
}


fn main() {
    let res = vec!["ad".to_string(), "ae".to_string(), "af".to_string(),
                   "bd".to_string(), "be".to_string(), "bf".to_string(), "cd".to_string(), "ce".to_string(), "cf".to_string()];
    let test_str = "23".to_string();
    assert_eq!(res, Solution::letter_combinations(test_str));
}
