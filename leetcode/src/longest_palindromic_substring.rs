// https://leetcode.com/problems/longest-palindromic-substring/?envType=featured-list&envId=top-100-liked-questions?envType=featured-list&envId=top-100-liked-questions

struct Solution;

// l - s.len()
// runtime: O(l^2)
// memory: O(l)
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let chars: Vec<_> = s.chars().collect();
        if chars.len() <= 1 {
            return s;
        }

        let mut max_palindrome = &chars[0..1];
        for i in 0..chars.len() - 1 {
            let odd = Solution::expand_max_palindrome(&chars, i, i);
            if max_palindrome.len() < odd.len() {
                max_palindrome = odd;
            }
            let even = Solution::expand_max_palindrome(&chars, i, i + 1);
            if max_palindrome.len() < even.len() {
                max_palindrome = even;
            }
        }

        max_palindrome.iter().collect()
    }

    // cbbd
    fn expand_max_palindrome(chars: &Vec<char>, left: usize, right: usize) -> &[char] {
        let len = chars.len();
        let mut i = left;
        let mut j = right;
        if chars[i] != chars[j] {
            return &chars[i..j];
        }

        while i >= 1 && j + 1 < len && chars[i - 1] == chars[j + 1] {
            i -= 1;
            j += 1;
        }
        &chars[i..=j]
    }
}

#[cfg(test)]
mod test {
    use crate::longest_palindromic_substring::Solution;

    #[test]
    fn check1() {
        let input = "babad".to_owned();
        let expected = "bab".to_owned();
        let output = Solution::longest_palindrome(input);
        assert_eq!(expected, output);
    }

    #[test]
    fn check2() {
        let input = "cbbd".to_owned();
        let expected = "bb".to_owned();
        let output = Solution::longest_palindrome(input);
        assert_eq!(expected, output);
    }

    #[test]
    fn check3() {
        let input = "bannannaas".to_owned();
        let expected = "annanna".to_owned();
        let output = Solution::longest_palindrome(input);
        assert_eq!(expected, output);
    }
}
