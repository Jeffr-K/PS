struct Solution;

impl Solution {
    pub fn is_leap_year(a: i32, b: i32) -> String {
        let months = vec![31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let days = vec!["FRI", "SAT", "SUN", "MON", "TUE", "WED", "THU"];

        let mut answer = 0;

        for i in 0..a-1 {
            answer += months[i as usize];
        }

        answer += b - 1;

        days[answer as usize % 7].to_string()
    }
}

mod tests {
    #[test]
    fn test_is_leap_year() {
        use super::*;

        assert_eq!(Solution::is_leap_year(5, 24), "TUE".to_string());
    }
}
