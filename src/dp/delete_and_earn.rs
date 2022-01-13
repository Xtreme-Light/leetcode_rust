use crate::Solution;
use std::collections::HashMap;

impl Solution {
    /// day3
    /// https://leetcode-cn.com/problems/delete-and-earn/
    ///
    /// 给你一个整数数组 nums ，你可以对它进行一些操作。
    ///
    /// 每次操作中，选择任意一个 nums[i] ，删除它并获得 nums[i] 的点数。之后，你必须删除 所有 等于 nums[i] - 1 和 nums[i] + 1 的元素。
    ///
    /// 开始你拥有 0 个点数。返回你能通过这些操作获得的最大点数。
    ///
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        let n = *nums.iter().max().unwrap() as usize;
        // 记录各个数字出现的次数
        let mut arr = vec![0; n + 1];
        for i in 0..nums.len() {
            arr[nums[i] as usize] += 1;
        }
        let mut pick = 0;
        let mut ban = 0;
        for i in 0..arr.len() {
            let pre_pick = pick;
            pick = pick.max(ban + arr[i] * i);
            ban = pre_pick;
        }
        pick as i32
    }
}
#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    pub fn test_delete_and_earn() {
        assert_eq!(Solution::delete_and_earn(vec![3, 4, 2]), 6);
        assert_eq!(Solution::delete_and_earn(vec![2, 2, 3, 3, 3, 4]), 9);
    }
}
