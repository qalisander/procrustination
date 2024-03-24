// https://leetcode.com/problems/sort-colors/

struct Solution;
impl Solution {
    pub fn sort_colors(nums: &mut [i32]) {
        let mut counts = [0; 3];
        for num in nums.iter() {
            if 0 > *num || 2 < *num {
                panic!("Not valid number")
            }
            counts[*num as usize] += 1;
        }
        let mut i = 0;
        for (num, mut count) in counts.into_iter().enumerate() {
            while count > 0 {
                nums[i] = num as i32;
                count -= 1;
                i += 1;
            }
        }
    }

    pub fn sort_colors2(nums: &mut [i32]) {
        let mut low = 0;
        let mut mid = 0;
        let mut high = nums.len() - 1;
        while mid <= high {
            match nums[mid] {
                0 => {
                    nums.swap(low, mid);
                    low += 1;
                    mid += 1;
                }
                1 => {
                    mid += 1;
                }
                2 => {
                    nums.swap(mid, high);
                    if high == 0 {
                        return;
                    }
                    high -= 1;
                }
                _ => panic!("Invalid number"),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::sort_colors::Solution;

    #[test]
    fn check1() {
        let mut nums = vec![2, 0, 2, 1, 1, 0];
        let expected = vec![0, 0, 1, 1, 2, 2];
        Solution::sort_colors(&mut nums);
        assert_eq!(expected, nums);
    }

    #[test]
    fn check2() {
        let mut nums = vec![2, 0, 2, 1, 1, 0];
        let expected = vec![0, 0, 1, 1, 2, 2];
        Solution::sort_colors2(&mut nums);
        assert_eq!(expected, nums);
    }
}
