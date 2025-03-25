fn main() {
// String
    //  .capacity()     : 바이트단위의 용량 확인
    //  .push('문자')         : 문자 삽입
    //  .push_str('문자열')     : 문자열 삽입
    //  .pop()          : 문자열의 마지막 문자 제거
    //  with_capacity(정수값) : 지정한 용량을 가진 빈 stirng 생성
    //  .replace("바꿀 문자","바꾼 문자")   : 특정문자열을 다른 문자열로 변경함.
    
    let mut my_name = String::from("David");
   
    my_name.push('!');
    println!("My name is {}", my_name);
   
    my_name.push_str(" and I live in Seoul");
    println!("My name is {}", my_name);
   
    let size = my_name.capacity();
    println!("My name is {}", size);
   
    my_name.pop();
    println!("My name is {}", my_name);
    // replace는 결과값을 새롭게 반환함. 따라서 새롭게 저장필요.
    my_name = my_name.replace("and", "hello");
    println!("My name is {}", my_name);
   

    let mut s= String::with_capacity(10);
    s.push_str("WTdHaaaaaaaa");
    println!("{} {} {}", s, s.len(), s.capacity());
}
