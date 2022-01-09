use crate::Solution;

impl Solution {
    /// 编写一个高效的算法来搜索 m x n 矩阵 matrix 中的一个目标值 target 。该矩阵具有以下特性：
    /// https://leetcode-cn.com/problems/search-a-2d-matrix-ii/solution/
    /// 每行的元素从左到右升序排列。
    /// 每列的元素从上到下升序排列。
    ///
    /// 每一行去做二分查找是不错的选择
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let lines = matrix.len();
        for i in 0..lines {
            let line = &matrix[i];
            let width = line.len();
            // 进行二分查找
            if target > *line.get(width - 1).unwrap() {
                continue;
            }
            let mut start = 0;
            let mut end = width - 1;
            while start <= end {
                let mid = (end - start) / 2 + start;
                let num = line[mid];
                if target == num {
                    return true;
                } else if target < num {
                    end = mid - 1;
                } else {
                    start = mid + 1;
                }
            }
        }
        false
    }
    /// 第二种解法，这种解法是基于题目提供的matrix矩阵特点来做到快速处理的
    /// 观察矩阵，可以发现，从右上角出发进行比较，
    /// 如果目标元素比矩阵元素小，那么左移查找，
    /// 如果目标元素比矩阵元素大，那么下移查找
    /// 有点类似于二分查找树，左节点比节点小，右节点比节点大
    pub fn search_matrix_2(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.len() == 0 || matrix[0].len() == 0 {
            return false;
        }

        // 行
        let mut j = 0;
        let height = matrix.len();
        // 列
        let mut width = matrix[j].len() - 1;
        while j < height {
            if matrix[j][width] == target {
                return true;
            } else if matrix[j][width] > target {
                if width == 0 {
                    return false;
                }
                width -= 1;
            } else {
                j += 1;
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    pub fn test_search_matrix() {
        assert_eq!(
            Solution::search_matrix(
                vec![
                    vec![1, 4, 7, 11, 15],
                    vec![2, 5, 8, 12, 19],
                    vec![3, 6, 9, 16, 22],
                    vec![10, 13, 14, 17, 24],
                    vec![18, 21, 23, 26, 30]
                ],
                5
            ),
            true
        );
        assert_eq!(
            Solution::search_matrix_2(
                vec![
                    vec![1, 4, 7, 11, 15],
                    vec![2, 5, 8, 12, 19],
                    vec![3, 6, 9, 16, 22],
                    vec![10, 13, 14, 17, 24],
                    vec![18, 21, 23, 26, 30]
                ],
                5
            ),
            true
        );
        assert_eq!(
            Solution::search_matrix(
                vec![
                    vec![1, 4, 7, 11, 15],
                    vec![2, 5, 8, 12, 19],
                    vec![3, 6, 9, 16, 22],
                    vec![10, 13, 14, 17, 24],
                    vec![18, 21, 23, 26, 30]
                ],
                20
            ),
            false
        );
    }
    #[test]
    pub fn test() {
        let a = 3 / 2;
        println!("result {}", a);
    }
}
