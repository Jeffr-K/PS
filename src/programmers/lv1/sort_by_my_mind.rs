// 없는 숫자 더하기
struct Solution;

impl Solution {
    pub fn solution(numbers: Vec<i32>) -> i32 {
        let mut v = vec![false; 10];

        for &num in numbers.iter() {
            v[num as usize] = true;
        }

        v.iter()
            .enumerate()
            .filter(|&(_, &present)| !present)
            .map(|(i, _)| i as i32)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(Solution::solution(vec![1,2,3,4,6,7,8,0]), 14);
        assert_eq!(Solution::solution(vec![5,8,4,0,6,7,9]), 6);
    }
}
