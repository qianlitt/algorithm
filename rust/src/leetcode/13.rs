use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let roman = HashMap::from([
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]);
        let chars: Vec<char> = s.chars().collect();

        let mut ans = 0;
        for ch in 0..chars.len() - 1 {
            let x = roman[&chars[ch]];
            let y = roman[&chars[ch + 1]];

            if x < y {
                ans -= x;
            } else {
                ans += x;
            }
        }

        ans + roman[&chars[chars.len() - 1]] // 加上最后一个
    }

    pub fn roman_to_int_rev(s: String) -> i32 {
        let roman = HashMap::from([
            (b'I', 1),
            (b'V', 5),
            (b'X', 10),
            (b'L', 50),
            (b'C', 100),
            (b'D', 500),
            (b'M', 1000),
        ]);
        let chars = s.as_bytes();

        let mut ans = 0;
        let mut prev = roman[&chars[chars.len() - 1]];
        // 从后往前遍历
        for i in (0..chars.len()).rev() {
            let current = roman[&chars[i]];

            if current < prev {
                ans -= current;
            } else {
                ans += current;
            }
            prev = current;
        }
        ans
    }
}

fn main() {
    // You can optionally experiment here.
    let roman = HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]);
    println!("{:?}\n", roman.get(&'I')); // Some(1)
    println!("{:?}\n", roman[&'I']); // 1

    for i in 0..4 {
        print!("{} ", i); // 0 1 2 3
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let s = "III".to_string();
        let ans = 3;

        assert_eq!(ans, Solution::roman_to_int(s.clone()));
        assert_eq!(ans, Solution::roman_to_int_rev(s));
    }
    #[test]
    fn example2() {
        let s = "IV".to_string();
        let ans = 4;

        assert_eq!(ans, Solution::roman_to_int(s.clone()));
        assert_eq!(ans, Solution::roman_to_int_rev(s));
    }
    #[test]
    fn example3() {
        let s = "IX".to_string();
        let ans = 9;

        assert_eq!(ans, Solution::roman_to_int(s.clone()));
        assert_eq!(ans, Solution::roman_to_int_rev(s));
    }
    #[test]
    fn example4() {
        let s = "LVIII".to_string();
        let ans = 58;

        assert_eq!(ans, Solution::roman_to_int(s.clone()));
        assert_eq!(ans, Solution::roman_to_int_rev(s));
    }
    #[test]
    fn example5() {
        let s = "MCMXCIV".to_string();
        let ans = 1994;

        assert_eq!(ans, Solution::roman_to_int(s.clone()));
        assert_eq!(ans, Solution::roman_to_int_rev(s));
    }
}
