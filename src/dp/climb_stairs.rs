use crate::Solution;

impl Solution {
    ///假设你正在爬楼梯。需要 n 阶你才能到达楼顶。
    ///
    /// 每次你可以爬 1 或 2 个台阶。你有多少种不同的方法可以爬到楼顶呢？
    ///
    /// 注意：给定 n 是一个正整数。
    pub fn climb_stairs(n: i32) -> i32 {
        if n < 3 {
            return n;
        }
        let mut _temp_1 = 1;
        let mut _temp_2 = 2;
        let mut sum = 0;
        let mut i = 2;
        while i < n {
            sum = _temp_1 + _temp_2;
            _temp_1 = _temp_2;
            _temp_2 = sum;
            i += 1;
        }
        sum
    }
    pub fn climb_stairs_2(n: i32) -> i32 {
        (1..n).fold((1, 2), |(a, b), _| (b, a + b)).0
    }
}
// https://leetcode-cn.com/problems/climbing-stairs/
#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    pub fn test_climb_stairs() {
        assert_eq!(Solution::climb_stairs(2), 2);
        assert_eq!(Solution::climb_stairs(3), 3);
    }
    #[test]
    pub fn test_climb_stairs_2() {
        assert_eq!(Solution::climb_stairs_2(2), 2);
        assert_eq!(Solution::climb_stairs_2(3), 3);
    }
}
