use crate::Solution;

impl Solution {
    /// https://leetcode-cn.com/problems/fibonacci-number/
    /// 斐波那契数列
    pub fn fib(n: i32) -> i32 {
        if n < 2 {
            return n;
        }
        let mut _temp_1 = 1;
        let mut _temp_2 = 0;
        let mut _temp = 0;
        for _index in 2..(n + 1) {
            _temp = _temp_1 + _temp_2;
            _temp_2 = _temp_1;
            _temp_1 = _temp;
        }
        _temp
    }
    pub fn fib_2(n: i32) -> i32 {
        (0..n).fold((0, 1), |(a, b), _| (b, a + b)).0
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    pub fn test_fib() {
        let fib = Solution::fib(2);
        assert_eq!(1, fib);
        let fib = Solution::fib(3);
        assert_eq!(2, fib);
        let fib = Solution::fib(4);
        assert_eq!(3, fib);
    }
    #[test]
    pub fn test_fib_2() {
        let fib = Solution::fib_2(2);
        assert_eq!(1, fib);
        let fib = Solution::fib_2(3);
        assert_eq!(2, fib);
        let fib = Solution::fib_2(4);
        assert_eq!(3, fib);
    }
}
