struct Solution {}


impl Solution {
    pub fn my_atoi(str: String) -> i32 {
        let num_str = str.trim_start().to_string();
        let mut res: String = String::new();
        let mut flag: Option<char> = None;
        for ch in num_str.chars() {
            if (ch == '+' || ch == '-') && flag == None {
                flag = Some(ch);
            } else if ch >= '0' && ch <= '9' {
                if flag == None {
                    flag = Some('+');
                }
                res.push(ch);
            } else {
                break;
            }
        }
        res = res.trim_start_matches('0').to_string();
        if res.len() > 10 {
            res = res[0..=10].to_string();
        }
        res.insert(0, flag.unwrap_or('+'));
        let number = res.parse::<i64>().unwrap_or(0);
        if number > i32::max_value() as i64 {
            return i32::max_value();
        } else if number < i32::min_value() as i64 {
            return i32::min_value();
        }
        return number as i32;
    }
}

fn main() {
    let test_str = "   -42".to_string();
    assert_eq!(-42, Solution::my_atoi(test_str));
    let test_str = "4193 with words".to_string();
    assert_eq!(4193, Solution::my_atoi(test_str));
    let test_str = "words and 987".to_string();
    assert_eq!(0, Solution::my_atoi(test_str));
    let test_str = "-91283472332".to_string();
    assert_eq!(-2147483648, Solution::my_atoi(test_str));
    let test_str = "20000000000000000000".to_string();
    assert_eq!(2147483647, Solution::my_atoi(test_str));
    let test_str = "-5-".to_string();
    assert_eq!(-5, Solution::my_atoi(test_str));
    let test_str = "1234567890123456789012345678901234567890".to_string();
    assert_eq!(2147483647, Solution::my_atoi(test_str));
    let test_str = "  0000000000012345678".to_string();
    assert_eq!(12345678, Solution::my_atoi(test_str));
}
