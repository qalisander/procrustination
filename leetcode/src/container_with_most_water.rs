// https://leetcode.com/problems/container-with-most-water/

struct Solution {}

// Greedy solution for O(n)
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        use std::cmp::Ordering;
        let len = height.len();
        if len == 0 {
            return 0;
        }

        let mut max_area = 0;
        let mut p1 = 0;
        let mut p2 = len - 1;
        while p1 < p2 {
            max_area = max_area.max(height[p2].min(height[p1]) * (p2 - p1) as i32);
            match height[p2].cmp(&height[p1]) {
                Ordering::Less => {
                    p2 -= 1;
                }
                Ordering::Equal => {
                    p2 -= 1;
                    p1 += 1;
                }
                Ordering::Greater => {
                    p1 += 1;
                }
            }
        }
        max_area
    }
}

#[cfg(test)]
mod test {
    use crate::container_with_most_water::Solution;

    #[test]
    fn check1() {
        let arg = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        let res = Solution::max_area(arg);
        let expected = 49;
        assert_eq!(expected, res)
    }
}
