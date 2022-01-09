use crate::Solution;

impl Solution {
    /// https://leetcode-cn.com/problems/min-cost-climbing-stairs/
    /// 给你一个整数数组 cost ，其中 cost[i] 是从楼梯第 i 个台阶向上爬需要支付的费用。一旦你支付此费用，即可选择向上爬一个或者两个台阶。
    ///
    /// 你可以选择从下标为 0 或下标为 1 的台阶开始爬楼梯。
    ///
    /// 请你计算并返回达到楼梯顶部的最低花费。
    /// 输入：cost = [1,100,1,1,1,100,1,1,100,1]
    /// 输出：6
    /// 解释：你将从下标为 0 的台阶开始。
    /// - 支付 1 ，向上爬两个台阶，到达下标为 2 的台阶。
    /// - 支付 1 ，向上爬两个台阶，到达下标为 4 的台阶。
    /// - 支付 1 ，向上爬两个台阶，到达下标为 6 的台阶。
    /// - 支付 1 ，向上爬一个台阶，到达下标为 7 的台阶。
    /// - 支付 1 ，向上爬两个台阶，到达下标为 9 的台阶。
    /// - 支付 1 ，向上爬一个台阶，到达楼梯顶部。
    /// 总花费为 6 。
    ///
    ///
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let length = cost.len();
        let mut pre = 0;
        let mut cur = 0;
        for i in 2..length + 1 {
            let next = std::cmp::min(cur + cost[i - 1], pre + cost[i - 2]);
            pre = cur;
            cur = next;
        }
        cur
    }
    pub fn min_cost_climbing_stairs_2(cost: Vec<i32>) -> i32 {
        cost.iter()
            .fold((0, 0), |acc, x| (acc.1.min(acc.0 + x), acc.0 + x))
            .0
    }
}
#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    pub fn test_min_cost_climbing_stairs() {
        assert_eq!(15, Solution::min_cost_climbing_stairs(vec![10, 15, 20]));
        assert_eq!(
            6,
            Solution::min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1])
        );
    }
    #[test]
    pub fn test_min_cost_climbing_stairs_2() {
        assert_eq!(15, Solution::min_cost_climbing_stairs_2(vec![10, 15, 20]));
        assert_eq!(
            6,
            Solution::min_cost_climbing_stairs_2(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1])
        );
    }
}
