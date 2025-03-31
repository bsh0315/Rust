// 불변참조 및 가변 참조를 활용한 예제
fn modify_string(s: &mut String) {
    // 가변 참조를 사용하여 문자열에 " is modified."를 추가합니다.
    s.push_str(" is modified.");
    // s를 불변 참조(&String)로 바인딩합니다.
    let s2: &String = s;
    // 불변 참조를 통해 수정된 문자열을 출력합니다.
    println!("{}", s2);
}

fn main() {
    // 가변 변수 s를 생성합니다.
    let mut s = String::from("hello");
    // s의 가변 참조를 함수에 전달하여 내부 데이터를 수정합니다.
    modify_string(&mut s);
    // 함수 호출이 끝나면서 가변 참조는 소멸되었으므로,
    // 원본 변수 s에 대해 직접 접근하여 출력할 수 있습니다.
    println!("from main: {}", s);
}
