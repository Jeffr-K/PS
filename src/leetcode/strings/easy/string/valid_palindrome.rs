struct Solution;

/// Valid Palindrome
/// [125. Valid Palindrome](https://leetcode.com/problems/valid-palindrome/description/)
/// * Description
///   :A phrase is a palindrome if, after converting all uppercase letters into lowercase letters and removing all non-alphanumeric characters, it reads the same forward and backward. Alphanumeric characters include letters and numbers.
///   :Given a string s, return true if it is a palindrome, or false otherwise.
/// * Constraints
///   :1 <= s.length <= 2 * 10^5
///   :s consists only of printable ASCII characters.
/// * Time complexity
/// * Space complexity
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut start = 0;
        let mut end = s.len() - 1;
        let chars: Vec<char> = s.chars().collect();

        while start < end {
            if !chars[start].is_ascii_alphanumeric() {
                start += 1;
                continue;
            } else if !chars[end].is_ascii_alphanumeric() {
                end -= 1;
                continue;
            } else {
                if chars[start].to_ascii_lowercase() != chars[end].to_ascii_lowercase() {
                    return false;
                }
                start += 1;
                end -= 1;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert_eq!(Solution::is_palindrome("A man, a plan, a canal: Panama".to_string()), true);
        assert_eq!(Solution::is_palindrome("race a car".to_string()), false);
        assert_eq!(Solution::is_palindrome(" ".to_string()), true);
        assert_eq!(Solution::is_palindrome("0P".to_string()), false);
    }
}
