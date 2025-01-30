/// 문제: 가장 긴 팰린드롬
/// 링크: https://school.programmers.co.kr/learn/courses/30/lessons/12904
struct Solution;

impl Solution {
    pub fn solution(height: &[i32]) -> i32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(Solution::solution(&[1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(Solution::solution(&[1, 1]), 1);
        assert_eq!(Solution::solution(&[4, 3, 2, 1, 4]), 16);
        assert_eq!(Solution::solution(&[1, 2, 1]), 2);
    }
}
