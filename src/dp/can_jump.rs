/// dag 4
use crate::Solution;
impl Solution {
    /// https://leetcode-cn.com/problems/jump-game/
    /// 给定一个非负整数数组 nums ，你最初位于数组的 第一个下标 。
    /// 
    /// 数组中的每个元素代表你在该位置可以跳跃的最大长度。
    /// 判断你是否能够到达最后一个下标。
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut rightmost = 0;
        for (index,item) in nums.iter().enumerate() {
            if index <= rightmost {
                rightmost = std::cmp::max(rightmost, index +(*item as usize));
                if rightmost >= n-1 {
                    return true;
                }
            }
        }
        false
    }
}


#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    pub fn test_can_jump() {
        assert!(Solution::can_jump(vec![2,3,1,1,4]));
        assert!(!Solution::can_jump(vec![3,2,1,0,4]));

    }

}
