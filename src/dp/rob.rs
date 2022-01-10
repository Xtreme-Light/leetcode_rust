use crate::Solution;

impl Solution {
    /// day 3
    /// 你是一个专业的小偷，计划偷窃沿街的房屋。每间房内都藏有一定的现金，
    /// 影响你偷窃的唯一制约因素就是相邻的房屋装有相互连通的防盗系统，如果两间相邻的房屋在同一晚上被小偷闯入，系统会自动报警。
    ///
    /// 给定一个代表每个房屋存放金额的非负整数数组，计算你 不触动警报装置的情况下 ，一夜之内能够偷窃到的最高金额。
    ///
    /// https://leetcode-cn.com/problems/house-robber/
    /// 看起来fold函数真是解决动态规划问题的神器
    pub fn rob(nums: Vec<i32>) -> i32 {
        nums.iter()
            .fold((0, 0), |acc, i| (acc.0.max(acc.1 + i), acc.0))
            .0
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    pub fn test_rob() {
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
    }
}
