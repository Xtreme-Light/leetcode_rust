use crate::Solution;
/// https://leetcode-cn.com/problems/palindrome-number/
/// 回文
impl Solution {
    /// 这样虽然只有一行代码，但是效率和内存占用都比较高
    pub fn is_palindrome(x: i32) -> bool {
        x.to_string() == x.to_string().chars().rev().collect::<String>()
    }
    pub fn is_palindrome_2(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        if x < 10 {
            return true;
        }
        let _temp = x.to_string();
        let bytes = _temp.as_bytes();
        let (mut start, mut end) = (0, bytes.len() - 1);
        while start < end {
            if bytes[start] != bytes[end] {
                return false;
            }
            start += 1;
            end -= 1;
        }

        true
    }
}
#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    pub fn test_is_palindrome() {
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome(121), Solution::is_palindrome_2(121));
    }
}
