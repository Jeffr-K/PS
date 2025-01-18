struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.is_empty() {
            return "".to_string();
        }

        let chars: Vec<char> = s.chars().collect();
        let mut start = 0;
        let mut max_len = 1;

        for i in 0..chars.len() {
            let len1 = Self::expand_around_center(&chars, i as i32, i as i32);
            let len2 = Self::expand_around_center(&chars, i as i32, (i + 1) as i32);
            let len = len1.max(len2);

            if len > max_len {
                start = i - (len - 1) / 2;
                max_len = len;
            }
        }

        chars[start..start + max_len].iter().collect()
    }

    fn expand_around_center(chars: &[char], mut left: i32, mut right: i32) -> usize {
        while left >= 0 && right < chars.len() as i32 && chars[left as usize] == chars[right as usize] {
            left -= 1;
            right += 1;
        }

        (right - left - 1) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_palindrome() {
        assert_eq!(Solution::longest_palindrome("babad".to_string()), "bab".to_string());
        assert_eq!(Solution::longest_palindrome("cbbd".to_string()), "bb".to_string());
        assert_eq!(Solution::longest_palindrome("a".to_string()), "a".to_string());
        assert_eq!(Solution::longest_palindrome("ac".to_string()), "a".to_string());
        assert_eq!(Solution::longest_palindrome("".to_string()), "".to_string());
    }
}
