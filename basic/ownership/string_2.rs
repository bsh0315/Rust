// 소유권 연습문제
// 소유권 이전, 그냥 '='을 사용하면 됨.
// ex) let s2 = s1;

fn modify_string(mut s: String)-> String {
    s.push_str(" -modified");
    println!("{}", s);
    s

}


fn main() {
    let string = String::from("Hello, Rust!");
    let modified1 = modify_string(string);

    println!("{}", modified1);
    //만약 modify_string(string);만 하고 string을 출력하면 안됨 
    //modify_string(string)을 통해서 이미 소유권이 함수로 넘어갔기 때문임.
    //때문에 함수로부터의 소유권을 다시 반환받을 때
    //필요한 새로운 변수가 필요함.

    let string2 = String::from("Hello, Rust!");
    let modified2 = modify_string(string2.clone());
    // clone은 원본 String의 복사본을 만들어줌(원본과는 연관 x)
    println!("{}", string2);

}
