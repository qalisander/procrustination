// https://leetcode.com/problems/unique-paths/description/?envType=featured-list&envId=top-100-liked-questions?envType=featured-list&envId=top-100-liked-questions

// There is a robot on an m x n grid. The robot is initially located at the top-left corner (i.e., grid[0][0]).
// The robot tries to move to the bottom-right corner (i.e., grid[m - 1][n - 1]).
// The robot can only move either down or right at any point in time.
//
// Given the two integers m and n, return the number of possible unique paths that the robot can take to reach the bottom-right corner.
struct Solution;

// Complexity
// time O(nm)
// memory O(nm)

// 0  1  1  1  1  1  1
// 1  2  3  4  5  6  7
// 1  3  6  10 15 21 28

impl Solution {
    // m > 1, n > 1
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        if m < 1 || n < 1 {
            panic!("Invalid input");
        }

        let (m, n) = (m as usize, n as usize);
        let mut unique_paths_count = vec![vec![-1_i32; n]; m];

        for i in 0..m {
            for j in 0..n {
                unique_paths_count[i][j] = if i == 0 || j == 0 {
                    1
                } else {
                    unique_paths_count[i - 1][j] + unique_paths_count[i][j - 1]
                }
            }
        }

        unique_paths_count[m - 1][n - 1]
    }
}

#[cfg(test)]
mod test {
    use crate::unique_paths::Solution;

    #[test]
    pub fn check1() {
        let (m, n) = (3, 7);
        let expected = 28;
        let unique_paths = Solution::unique_paths(m, n);
        assert_eq!(expected, unique_paths);
    }
}
