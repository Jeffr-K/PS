struct Solution;

/// Reverse String
/// [344. Reverse String](https://leetcode.com/problems/reverse-string/)
/// * Description
///   :Write a function that reverses a string. The input string is given as an array of characters s.
/// * Constraints
///   :1 <= s.length <= 10^5
///   :s[i] is a printable ascii character.
/// * Time complexity
/// * Space complexity
impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let mut start = 0;
        let mut end = s.len() - 1;

        while start < end {
            let temp = s[start];
            s[start] = s[end];
            s[end] = temp;
            start += 1;
            end -= 1;
        }
    }

    pub fn reverse_string_by_std(s: &mut Vec<char>) {
        // reverse() 는 in-place 로 동작하므로 현재 문제에 대해 완벽하게 유효함.
        s.reverse();
    }

    pub fn reverse_string_by_slice_pattern(s: &mut Vec<char>) {
        s[..].reverse();
    }

    pub fn reverse_string_by_swap(s: &mut Vec<char>) {
        let n = s.len();
        for i in 0..n/2 {
            s.swap(i, n-i-1);
        }
    }

    pub fn reverse_string_by_iterator_and_window(s: &mut Vec<char>) {
        // TODO: chunks_mut 메서드를 사용할 때 발생하는 가변 대여(mutable borrow) 문제
        let len = s.len();
        s
            .chunks_mut(len)
            .for_each(|chunk| chunk.reverse());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_string() {
        let mut input = vec!['h', 'e', 'l', 'l', 'o'];
        let expected = vec!['o', 'l', 'l', 'e', 'h'];

        Solution::reverse_string(&mut input);
        Solution::reverse_string_by_std(&mut input);
        Solution::reverse_string_by_slice_pattern(&mut input);
        Solution::reverse_string_by_swap(&mut input);
        Solution::reverse_string_by_iterator_and_window(&mut input);

        assert_eq!(input, expected);
    }
}
