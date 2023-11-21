// https://leetcode.com/problems/longest-palindromic-substring/
struct Solution;

// l - s.len()
// runtime: O(l)
// memory: O(l)
impl Solution {
    /// Variation of Manacher's algorithm
    //  if point i is inside 'central' palindrome
    //      if mirrored point has palindrome inside 'central' palindrome
    //          => i has the same palindrome radius
    //      if mirrored point has palindrome outside 'central' palindrome
    //          => i has palindrome radius till previous(center) palindrome(center) border
    //      if mirrored point has palindrome end where 'central' palindrome ends
    //          => make i as a center of the next central palindrome
    //  if point ends at the border or outside 'central' palindrome
    //      => make i as a center of the next central palindrome
    pub fn longest_palindrome(s: String) -> String {
        use std::cmp::Ordering;
        // string with char '#' interleaved between, at the front and in the end
        let chars = s.chars().fold(vec!['#'], |mut acc, ch| {
            acc.push(ch);
            acc.push('#');
            acc
        });
        let len = chars.len();
        let mut max_palindrome = &chars[0..1];
        let mut radius = vec![0usize; len];
        let mut center = 0;
        for i in 0..chars.len() {
            let delta = i - center;
            if delta < radius[center] {
                let mirrored = center - delta;
                match (mirrored - radius[mirrored]).cmp(&(center - radius[center])) {
                    // mirrored palindrome outside of central palindrome
                    Ordering::Less => {
                        radius[i] = radius[center] - delta;
                        continue;
                    }
                    // mirrored palindrome inside of central palindrome
                    Ordering::Greater => {
                        radius[i] = radius[mirrored];
                        continue;
                    }
                    Ordering::Equal => {}
                }
            }
            center = i;
            while i >= radius[i] + 1
                && i + radius[i] + 1 < len
                && chars[i - radius[i] - 1] == chars[i + radius[i] + 1]
            {
                radius[i] += 1;
            }
            let palindrome = &chars[i - radius[i]..=i + radius[i]];
            if palindrome.len() > max_palindrome.len() {
                max_palindrome = palindrome;
            }
        }
        max_palindrome.iter().filter(|&&ch| ch != '#').collect()
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

    #[test]
    fn check4() {
        let input = "babadada".to_owned();
        let expected = "adada".to_owned();
        let output = Solution::longest_palindrome(input);
        assert_eq!(expected, output);
    }

    #[test]
    fn check5() {
        let input = "222020221".to_owned();
        let expected = "2202022".to_owned();
        let output = Solution::longest_palindrome(input);
        assert_eq!(expected, output);
    }
}
