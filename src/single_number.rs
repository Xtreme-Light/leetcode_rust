use crate::Solution;

impl Solution {
    /// https://leetcode-cn.com/problems/single-number/
    /// 你的算法应该具有线性时间复杂度。 你可以不使用额外空间来实现吗？
    /// 这个题目简直是隐藏起来考察异或运算，如果你了解异或，那么就会解决的非常的优雅，线性时间，不消耗额外空间，不然的话
    /// 可能还是需要不少的辅助空间的。
    ///
    /// 任何数和 00 做异或运算，结果仍然是原来的数，即 a⊕0=a。
    /// 任何数和其自身做异或运算，结果是 0，即 a⊕a=0。
    /// 异或运算满足交换律和结合律，即a⊕b⊕a=b⊕a⊕a=b⊕(a⊕a)=b⊕0=b。
    ///
    /// 交换律和结合律一起配合，就会知道，相同的数，做异或，结果为0。而数和0异或，是他本身，那么这个题目可以直接将所有的数异或，
    /// 然后看结果，对应的结果，就是那个唯一一个的单个数
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.into_iter().reduce(|a, b| a ^ b).unwrap()
    }
}
#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    pub fn test_single_number() {
        assert_eq!(1, Solution::single_number(vec![2, 2, 1]));
    }
}
