use crate::Solution;

impl Solution {
    /// https://leetcode-cn.com/problems/gray-code/
    /// https://pic.leetcode-cn.com/1641610544-emTgEh-grayCode.png
    /// https://baike.baidu.com/item/%E6%A0%BC%E9%9B%B7%E7%A0%81
    /// 当了解到格雷码有个公式 G(n) = n xor (n>>1),也就是官解上的第二种解法
    /// 格雷编码
    pub fn gray_code(n: i32) -> Vec<i32> {
        // G(n) = n xor (n>>1)
        (0..1 << n).map(|e| e ^ (e >> 1)).collect()
    }
}
#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    pub fn test_gray_code() {
        let vec = Solution::gray_code(2);
        assert_eq!(vec, vec![0, 1, 3, 2]);
    }
}
