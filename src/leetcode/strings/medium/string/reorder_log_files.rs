struct Solution;

/// Approach
//// #1) '문자 로그', '숫자 로그' 로 분류하여 각 벡터로 담기
//// #2) 문자로그:
////     :각 벡터에서 식별자를 제외한 나머지 문장들을 공백없이 붙여서 정렬하기
////     :swap 함수 사용
//// #3) 숫자로그:
////     :상대적으로 큰 숫자를 기준으로 정렬
//// #4) 문자로그 + 숫자로그 합치기
impl Solution {
    pub fn reorder_log_files(logs: Vec<String>) -> Vec<String> {
        // #1) 로그를 '문자 로그', '숫자 로그' 로 분류
        let mut letter_logs: Vec<String> = Vec::new();
        let mut digit_logs: Vec<String> = Vec::new();

        for log in logs {
            let parts: Vec<&str> = log.split_whitespace().collect();
            if parts[1].chars().next().unwrap().is_digit(10) {
                digit_logs.push(log);
            } else {
                letter_logs.push(log);
            }
        }

        // #2) 문자 로그 정렬
        letter_logs.sort_by(|a, b| {
            let a_parts: Vec<&str> = a.split_whitespace().collect();
            let b_parts: Vec<&str> = b.split_whitespace().collect();

            // 식별자를 제외한 내용 비교
            let a_content = &a[a_parts[0].len()..];
            let b_content = &b[b_parts[0].len()..];

            // 내용이 같으면 식별자로 비교
            match a_content.cmp(b_content) {
                std::cmp::Ordering::Equal => a_parts[0].cmp(b_parts[0]),
                other => other
            }
        });

        // #3) 숫자 로그는 입력 순서 유지

        // #4) 문자로그 + 숫자로그 합치기
        letter_logs.extend(digit_logs);
        letter_logs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reorder_log_files_example1() {
        let example2 = vec![
            String::from("a1 9 2 3 1"),
            String::from("g1 act car"),
            String::from("zo4 4 7"),
            String::from("ab1 off key dog"),
            String::from("a8 act zoo"),
        ];
        let expected = vec![
            String::from("g1 act car"),
            String::from("a8 act zoo"),
            String::from("ab1 off key dog"),
            String::from("a1 9 2 3 1"),
            String::from("zo4 4 7"),
        ];

        assert_eq!(Solution::reorder_log_files(example2), expected);
    }

    #[test]
    fn test_reorder_log_files_example2() {
        let example2 = vec![
            String::from("a1 9 2 3 1"),
            String::from("g1 act car"),
            String::from("zo4 4 7"),
            String::from("ab1 off key dog"),
            String::from("a8 act zoo"),
        ];
        let expected = vec![
            String::from("g1 act car"),
            String::from("a8 act zoo"),
            String::from("ab1 off key dog"),
            String::from("a1 9 2 3 1"),
            String::from("zo4 4 7"),
        ];

        assert_eq!(Solution::reorder_log_files(example2), expected);
    }
}
