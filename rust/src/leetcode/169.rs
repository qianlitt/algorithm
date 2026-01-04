struct Solution;

impl Solution {
    // 严格众数问题：找出序列器出现次数>n/2的元素
    // Boyer-Moore 投票算法
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut major = nums[0];
        let mut count = 0;

        for &i in &nums {
            if count == 0 {
                count = 1;
                major = i;
            } else {
                count += if i == major { 1 } else { -1 };
            }
        }
        major
    }
    pub fn majority_element_foreach(nums: Vec<i32>) -> i32 {
        let mut major = nums[0];
        let mut count = 0;

        nums.iter().for_each(|&i| {
            if count == 0 {
                count = 1;
                major = i;
            } else {
                count += if i == major { 1 } else { -1 };
            }
        });
        major
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
        let nums = vec![3, 2, 3];
        let result = 3;

        assert_eq!(result, Solution::majority_element(nums.clone()));
        assert_eq!(result, Solution::majority_element_foreach(nums.clone()));
    }
    #[test]
    fn example2() {
        let nums = vec![2, 2, 1, 1, 1, 2, 2];
        let result = 2;

        assert_eq!(result, Solution::majority_element(nums.clone()));
        assert_eq!(result, Solution::majority_element_foreach(nums.clone()));
    }
}
