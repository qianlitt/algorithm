struct Solution;

impl Solution {
    // 暴力解
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() {
            // 枚举 i
            for j in i + 1..nums.len() {
                // 枚举 i 右边的 j
                if nums[i] + nums[j] == target {
                    // 满足要求
                    return vec![i as i32, j as i32]; // 返回两个数的下标
                }
            }
        }
        vec![] // 返回空数组
    }

    // 哈希表
    pub fn two_sum_hashmap(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // map: <数组的值, 元素下标>
        let mut map = std::collections::HashMap::new();
        for (i, &value) in nums.iter().enumerate() {
            if let Some(&search_result) = map.get(&(target - value)) {
                // 已找到，返回下标(search_result, i)
                return vec![search_result as i32, i as i32];
            }
            map.insert(value, i); // 将当前元素存入哈希表
        }

        vec![] // 返回空数组
    }
}

fn main() {
    // You can optionally experiment here.
    let nums = vec![1, 2, 3, 4, 5];
    // 迭代器
    for it in nums.iter() {
        print!("{}", it); // 12345
    }

    let mut map = std::collections::HashMap::new();
    for (index, &value) in nums.iter().enumerate() {
        if let Some(&i) = map.get(&(value)) {
            print!("{}", i);
        }
        map.insert(value, index);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = Solution::two_sum(nums, target);
        assert!(result == vec![0, 1] || result == vec![1, 0]);
    }

    #[test]
    fn example2() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let result = Solution::two_sum(nums, target);
        assert!(result == vec![1, 2] || result == vec![2, 1]);
    }

    #[test]
    fn example3() {
        let nums = vec![3, 3];
        let target = 6;
        let result = Solution::two_sum(nums, target);
        assert!(result == vec![0, 1] || result == vec![1, 0]);
    }

    // HashMap
    #[test]
    fn hash_example1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = Solution::two_sum_hashmap(nums, target);
        assert!(result == vec![0, 1] || result == vec![1, 0]);
    }

    #[test]
    fn hash_example2() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let result = Solution::two_sum_hashmap(nums, target);
        assert!(result == vec![1, 2] || result == vec![2, 1]);
    }

    #[test]
    fn hash_example3() {
        let nums = vec![3, 3];
        let target = 6;
        let result = Solution::two_sum_hashmap(nums, target);
        assert!(result == vec![0, 1] || result == vec![1, 0]);
    }
}
