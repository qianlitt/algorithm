struct Solution;

impl Solution {
    pub fn xor_operation(n: i32, start: i32) -> i32 {
        let mut ans = start;
        let mut i = 1;
        while i < n {
            ans ^= start + 2 * i;
            i += 1;
        }
        ans
    }
    // 灵神！！！
    // 1. XOR：相同出0,不同出1；
    //      阿贝尔群；
    //      偶数个1异或=0,奇数个1异或=1
    // 2. 若start是偶数，则nums[i]都是偶数；反之都是奇数
    pub fn xor_operation_1(n: i32, start: i32) -> i32 {
        let xor_n = |n| match n % 4 {
            0 => n,
            1 => 1,
            2 => n + 1,
            _ => 0,
        };
        let a = start / 2;
        let b = start & n & 1; // 当且仅当 start, n 的最低位都为1时，b 才为1；否则为0。即 start, n 都为奇数时，b 才为1。
        // ans = [F(a-1) xor F(a+n-1)] * 2 + b
        (xor_n(a - 1) ^ xor_n(a + n - 1)) * 2 + b
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(8, Solution::xor_operation(5, 0));
        assert_eq!(8, Solution::xor_operation(4, 3));
        assert_eq!(7, Solution::xor_operation(1, 7));
        assert_eq!(2, Solution::xor_operation(10, 5));
    }
    #[test]
    fn example2() {
        assert_eq!(8, Solution::xor_operation_1(5, 0));
        assert_eq!(8, Solution::xor_operation_1(4, 3));
        assert_eq!(7, Solution::xor_operation_1(1, 7));
        assert_eq!(2, Solution::xor_operation_1(10, 5));
    }
}
