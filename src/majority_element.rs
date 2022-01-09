use crate::Solution;

impl Solution {
    /// 多数元素
    /// 给定一个大小为 n 的数组，找到其中的多数元素。多数元素是指在数组中出现次数 大于n/2的元素。
    ///
    /// 你可以假设数组是非空的，并且给定的数组总是存在多数元素。
    ///
    /// 链接：https://leetcode-cn.com/problems/majority-element/
    ///
    /// 尝试设计时间复杂度为 O(n)、空间复杂度为 O(1) 的算法解决此问题。
    ///
    /// 使用常量级别的空间复杂度，也就是 空间复杂度为 O(1) ，使用了投票算法
    ///
    /// 由于众数占据 超过 数组一半的位置。所以如果是真的有众数的话，用他去碰撞其他的杂数，
    /// 目的就是消灭杂数。最后留下的一定是众数
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut candidate = 0;
        let mut count = 0;
        nums.iter().for_each(|e| {
            if count == 0 {
                candidate = *e;
            }
            count += if *e == candidate { 1 } else { -1 }
        });
        candidate
    }
}
#[cfg(test)]
mod test {
    use crate::Solution;
    #[test]
    pub fn test_majority_element() {
        assert_eq!(3, Solution::majority_element(vec![3, 2, 3]));
        assert_eq!(2, Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]));
    }
}
