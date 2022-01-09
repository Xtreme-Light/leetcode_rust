use crate::Solution;

impl Solution {
    /// https://leetcode-cn.com/problems/n-th-tribonacci-number/
    /// 泰波那契序列Tn定义如下：
    ///
    /// T0 = 0, T1 = 1, T2 = 1, 且在 n >= 0的条件下 Tn+3 = Tn + Tn+1 + Tn+2
    ///
    /// 给你整数 n，请返回第 n 个泰波那契数 Tn 的值。
    pub fn tribonacci(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return 1;
        }
        if n == 2 {
            return 1;
        }
        let mut _temp_0 = 0;
        let mut _temp_1 = 1;
        let mut _temp_2 = 1;
        let mut _temp = 0;
        for _index in 3..(n + 1) {
            _temp = _temp_0 + _temp_1 + _temp_2;
            _temp_0 = _temp_1;
            _temp_1 = _temp_2;
            _temp_2 = _temp;
        }
        _temp
    }
    pub fn tribonacci_2(n: i32) -> i32 {
        (0..n).fold((0, 1, 1), |(a, b, c), _| (b, c, a + b + c)).0
    }
}
#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    pub fn test_tribonacci() {
        let fib = Solution::tribonacci(4);
        assert_eq!(4, fib);
        let fib = Solution::tribonacci(25);
        assert_eq!(1389537, fib);
    }

    #[test]
    pub fn test_tribonacci_2() {
        let fib = Solution::tribonacci_2(4);
        assert_eq!(4, fib);
        let fib = Solution::tribonacci_2(25);
        assert_eq!(1389537, fib);
    }
}
