
/**
* 문제: 베스트 앨범
* 링크: https://school.programmers.co.kr/learn/courses/30/lessons/42579
* 난이도: Level 3
* 유형: 해시
* 설명:
    * 스트리밍 사이트에서 장르 별로 가장 많이 재생된 노래를 두 개씩 모아 베스트 앨범을 출시하려 합니다. 노래는 고유 번호로 구분하며, 노래를 수록하는 기준은 다음과 같습니다.
        * 1. 속한 노래가 많이 재생된 장르를 먼저 수록합니다.
        * 2. 장르 내에서 많이 재생된 노래를 먼저 수록합니다.
        * 3. 장르 내에서 재생 횟수가 같은 노래 중에서는 고유 번호가 낮은 노래를 먼저 수록합니다.
    * 노래의 장르를 나타내는 문자열 배열 genres 와 노래별 재생 횟수를 나타내는 정수 배열 plays 가 주어질 때, 베스트 앨범에 들어갈 노래의 고유 번호를 순서대로 return 하도록 solution 함수를 완성하세요.
* 제한사항
    * genres[i]는 고유번호가 i인 노래의 장르입니다.
    * plays[i]는 고유번호가 i인 노래가 재생된 횟수입니다.
    * genres 와 plays 의 길이는 같으며, 이는 1 이상 10,000 이하입니다.
    * 장르 종류는 100개 미만입니다.
    * 장르에 속한 곡이 하나라면, 하나의 곡만 선택합니다.
    * 모든 장르는 재생된 횟수가 다릅니다.
*/
struct Solution;

impl Solution {
    pub fn solution(genres: Vec<&str>, plays: Vec<i32>) -> Vec<i32> {
        use std::collections::*;

        let mut answer = Vec::with_capacity(genres.len());  // 미리 capacity 할당
        let mut hashmap: HashMap<&str, Vec<(i32, i32)>> = HashMap::new();

        // 모든 곡을 해시맵에 추가
        for i in 0..genres.len() {
            hashmap.entry(genres[i])
                .or_insert(Vec::new())
                .push((i as i32, plays[i]));
        }

        // 장르별 재생 횟수 내림차순 정렬
        for (_, v) in hashmap.iter_mut() {
            v.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));  // 재생 횟수가 같으면 고유번호 오름차순
        }

        // 장르별 총 재생횟수 계산하고 정렬
        let mut sorted_genres: Vec<(&str, i32)> = hashmap
            .iter()
            .map(|(k, v)| (*k, v.iter().map(|(_, p)| p).sum()))
            .collect();



        // 장르 총 재생횟수로 내림차순 정렬
        sorted_genres.sort_by(|a: &(&str, i32), b: &(&str, i32)| b.1.cmp(&a.1));

        // 정렬된 장르 순서대로 상위 2개씩 결과에 추가
        for (genre, _) in sorted_genres {
            if let Some(songs) = hashmap.get(genre) {
                // 장르에 속한 곡이 하나면 하나만, 아니면 최대 2개까지 추가
                answer.push(songs[0].0);
                if songs.len() > 1 {
                    answer.push(songs[1].0);
                }
            }
        }

        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let genres = vec!["classic", "pop", "classic", "classic", "pop"];
        let plays = vec![500, 600, 150, 800, 2500];
        assert_eq!(Solution::solution(genres, plays), vec![]);
    }
}
