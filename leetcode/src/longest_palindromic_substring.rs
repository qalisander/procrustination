// https://leetcode.com/problems/longest-palindromic-substring/?envType=featured-list&envId=top-100-liked-questions?envType=featured-list&envId=top-100-liked-questions

struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let chars: Vec<_> = s.chars().collect();
        if chars.is_empty() { panic!("Str is empty") }

        // there is at least single
        let mut longest: &[char] = &chars[0..1];
        for start in 0..chars.len() {
            for end in (start..chars.len()).rev() {
                let mut i = start;
                let mut j = end;
                loop {
                    if i >= j {
                        if longest.len() < chars[start..=end].len()
                        {
                            longest = &chars[start..=end];
                        }
                        break;
                    } else if chars[i] == chars[j] {
                        i += 1;
                        j -= 1;
                    } else {
                        break;
                    }
                }
            }
        }

        longest.iter().collect()
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
}