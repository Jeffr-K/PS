mod leetcode;

fn main() {
    let greet = "Hello, world!";
    println!("{}", greet);
    let split_greet = &greet.split(",").collect::<Vec<&str>>();
    println!("{:?}", split_greet);

    /// TODO: 단어별로 문자열 쪼개기
    let chars = &greet.split("").collect::<Vec<&str>>();
    println!("{:?}", chars);

    /// TODO: 단어별로 앞뒤 공백 없앤 채로 문자열 쪼개기
    let chars_without_whitespace: &Vec<&str> = &greet
        .split_terminator("")
        .filter(|s| !s.is_empty())
        .collect();
    println!("{:?}", *chars_without_whitespace);

    let chars_without_whitespace_or: &Vec<&str> = &greet
        .split_terminator("")
        .skip(1)
        .take_while(|s| !s.is_empty())
        .collect();
    println!("{:?}", *chars_without_whitespace_or);

    /// TODO: 문자열에서 인덱스를 활용해 문자 추출
    let extract_char_at_index = &greet
        .chars()
        .nth(3)
        .unwrap();
    println!("{:?}", extract_char_at_index);

    /// TODO: 추출한 문자가 숫자인지 확인
    let is_digit = &extract_char_at_index.is_digit(10);
    let is_alphabetic = &extract_char_at_index.is_alphabetic();
    println!("digit? {:?} alphabet? {:?}", is_digit, is_alphabetic);

    /// TODO: 문자열 자체를 모두 소문자로 만들 수 있는지 확인(ToLowercase)
    let lowercase_greet = &greet.to_lowercase();
    println!("{:?}", lowercase_greet);

    /// TODO: 문자열 자체를 모두 대문자로 만들 수 있는지 확인(ToUppercase)
    let uppercase_greet = &greet.to_uppercase();
    println!("{:?}", uppercase_greet);

    /// TODO: 빈 문자열 제거한 벡터에서 모든 문자열을 Upper case 로 변경하기
    let uppercase_chars_without_whitespace: &Vec<String> = &greet
        .split_terminator("")
        .filter(|s| !s.is_empty())
        .map(|s| s.to_uppercase())
        .collect();
    println!("{:?}", *uppercase_chars_without_whitespace);

    /// TODO: 빈 문자열 제거한 벡터에서 모든 문자열을 Lower case 로 변경하기
    let lowercase_chars_without_whitespace: &Vec<String> = &greet
        .split_terminator("")
        .filter(|s| !s.is_empty())
        .map(|s| s.to_lowercase())
        .collect();
    println!("{:?}", *lowercase_chars_without_whitespace);
}
