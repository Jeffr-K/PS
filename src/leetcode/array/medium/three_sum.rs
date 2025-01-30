struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        vec![vec![]]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_sum() {
        assert_eq!(
            Solution::three_sum(vec![-1,0,1,2,-1,-4]),
            vec![vec![-1,-1,2], vec![-1,0,1]]
        );
        assert_eq!(
            Solution::three_sum(vec![]),
            vec![]
        );
        assert_eq!(
            Solution::three_sum(vec![0]),
            vec![]
        );
    }
}
