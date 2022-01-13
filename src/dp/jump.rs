use crate::Solution;
impl Solution {
    /// https://leetcode-cn.com/problems/jump-game-ii/
    /// 给你一个非负整数数组 nums ，你最初位于数组的第一个位置。
    /// 数组中的每个元素代表你在该位置可以跳跃的最大长度。
    /// 
    /// 你的目标是使用最少的跳跃次数到达数组的最后一个位置。
    /// 
    /// 假设你总是可以到达数组的最后一个位置。
    /// 

    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut end = 0;
        let mut max_position = 0;
        let mut steps = 0;
        for (i, item) in nums.iter().enumerate().take(nums.len()-1){
            max_position = std::cmp::max(max_position, i + (*item as usize));
            if i == end {
                end = max_position;
                steps += 1;
            }     
        }
        steps
        
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    pub fn test_jump() {
        assert_eq!(Solution::jump(vec![2,3,0,1,4]), 2);
        assert_eq!(Solution::jump(vec![2,3,1,1,4]), 2);

    }

}