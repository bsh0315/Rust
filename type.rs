use std::mem::size_of;
fn main() {
    
    //'a'와 같은 영어는 1byte by 아스키 코드
    //'한', 'ㅇ'과 같은 한글은 3byte
    let string1 = "hello";
    let string2 = "몰라";

    // 반드시 같은 자료형끼리만 연산해야함.
    // 만약 i32, i16과 같이 서로 다른 크기의 자료형들을 연산하면 오류발생   
    // 정수의 기본 자료형은 i32임.
    let num1 = 10;
    let num2 = 20_i8;
    let num3 = num1 + num2;
   
    // 실수의 기본 자료형은 f64임.
    // as 키워드를 사용해서 형변환 가능함.
    let float1: f32 = 10.4;
    let float2 = 20.3;
    let result1 = float1 as i32 + float2 as i32; //결과는 30 
    let result2 = float1 + float2; //결과는 30.6999 by 단정도 오차

    println!("float1(10.4) as i32 + float2(20.3) as i32 = {}", result1);
    println!("float1(10.4) + float2(20.3) = {:.1}", result2);
    println!("num1({}) + num2({}) = num3({})",num1, num2, num3);
    println!("string1 size : {}, string1 length : {}", string1.len(), string1.chars().count());
    println!("string2 size : {}, string2 length : {}", string2.len(), string2.chars().count());

}
