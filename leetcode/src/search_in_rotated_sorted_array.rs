// https://leetcode.com/problems/search-in-rotated-sorted-array/description/?envType=featured-list&envId=top-100-liked-questions?envType=featured-list&envId=top-100-liked-questions
struct Solution;
/// nums = [4, 5, 6, 7, 0, 1, 2], target = 0
/// return index of target number or -1 if is not found

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut pivot = 0;
        for i in 1..nums.len() {
            if nums[i] < nums[i - 1] {
                pivot = i;
            }
        }

        match nums[..pivot]
            .binary_search(&target)
            .or(nums[pivot..].binary_search(&target).map(|x| x + pivot))
        {
            Ok(index) => index as i32,
            Err(_) => -1,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::search_in_rotated_sorted_array::Solution;

    #[test]
    pub fn check0() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 0;
        let expected = 4;
        let ans = Solution::search(nums, target);
        assert_eq!(expected, ans)
    }

    #[test]
    pub fn check1() {
        let nums = vec![];
        let target = 0;
        let expected = -1;
        let ans = Solution::search(nums, target);
        assert_eq!(expected, ans)
    }
}
