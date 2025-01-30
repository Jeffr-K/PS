use std::collections::VecDeque;

struct Solution;

impl Solution {

    pub fn trapping_rain_water_v1(height: &[i32]) -> i32 {
        use std::cmp::max;

        let mut left = 0;
        let mut right = height.len() - 1;
        let mut left_max = height[left];
        let mut right_max = height[right];
        let mut volume = 0;

        while left < right {
            left_max = max(left_max, height[left]);
            right_max = max(right_max, height[right]);

            if left_max < right_max {
                volume += left_max - height[left];
                left += 1;
            } else {
                volume += right_max - height[right];
                right -= 1;
            }
        }

        volume
    }

    pub fn trapping_rain_water_v2(height: &[i32]) -> i32 {
        let mut stack = VecDeque::new();
        let mut volume = 0;

        for i in 0..height.len() {
            // 현재 높이가 스택의 top 에 있는 위치의 높이보다 크다면
            while !stack.is_empty() && height[i] > height[*stack.back().unwrap()] {
                let top = stack.pop_back().unwrap();  // 낮은 높이를 가진 인덱스를 꺼냄

                if stack.is_empty() {  // 스택이 비었다면 물을 담을 수 없음
                    break;
                }

                // 물이 고일 수 있는 너비 계산
                let prev = *stack.back().unwrap();
                let distance = i - prev - 1;  // 현재 위치와 이전 벽 사이의 거리

                // 물이 고일 수 있는 높이 계산
                let bounded_height = height[i].min(height[prev]) - height[top];

                // 부피 = 너비 * 높이
                volume += distance as i32 * bounded_height;
            }

            stack.push_back(i);
        }

        volume
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_with_two_pointer() {
        assert_eq!(Solution::trapping_rain_water_v1(&[0,1,0,2,1,0,1,3,2,1,2,1]), 6);  // 우리가 앞서 본 예제
        assert_eq!(Solution::trapping_rain_water_v1(&[4,2,0,3,2,5]), 9);              // 다른 일반적인 예제
        assert_eq!(Solution::trapping_rain_water_v1(&[1,1]), 0);                      // 물이 고일 수 없는 경우
        assert_eq!(Solution::trapping_rain_water_v1(&[4,3,2,1,4]), 6);                // V자 모양
        assert_eq!(Solution::trapping_rain_water_v1(&[1,2,1]), 0);                    // 물이 고일 수 없는 경우
    }
    #[test]
    fn test_solution_with_stack() {
        assert_eq!(Solution::trapping_rain_water_v2(&[0,1,0,2,1,0,1,3,2,1,2,1]), 6);  // 우리가 앞서 본 예제
        assert_eq!(Solution::trapping_rain_water_v2(&[4,2,0,3,2,5]), 9);              // 다른 일반적인 예제
        assert_eq!(Solution::trapping_rain_water_v2(&[1,1]), 0);                      // 물이 고일 수 없는 경우
        assert_eq!(Solution::trapping_rain_water_v2(&[4,3,2,1,4]), 6);                // V자 모양
        assert_eq!(Solution::trapping_rain_water_v2(&[1,2,1]), 0);                    // 물이 고일 수 없는 경우
    }
}
