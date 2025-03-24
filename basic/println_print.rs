fn main() {
    // print!는 println!과 다르게 한 줄씩 출력되는 것이 아님.
    // C에서의 printf와 동일함.
    // \n, \t, \\ 와 같은 것들을 C에서와 동일하게 사용가능
    //  \b, \a같은 것은 안됨.
    print!("This");
    print!("This");
    print!("This\nis\thello\\ asf\n");
    print!(r#"c:\thisdirve\new_drive"#); //r#은 raw text를 의미 \기능 상실시켜줌.
    println!("
        Let me tell you
        what the fuck
        shup up. idiot");
    
    let my_variable = &9;
    println!("{:p}", my_variable); // 주소 출력
    let my_value = 10;
    println!("{:X}", my_value); // 대문자 16진수 출력
    println!("{:x}", my_value); // 소문자 16진수 출력
    println!("{:b}", my_variable); // 이진수 출력
    println!("{:?}", my_variable); // 디버그 할 때 사용(후에 자세히 알아보기
    let value = 32;
    let float = 32.1231455;
    println!("{:>10}", value); // 오른쪽 정렬
    println!("{:<10}", value); // 왼쪽 정렬
    println!("{:^10}", value); // 가운데 정렬
    println!("{:<10}", value); // 왼쪽 정렬
    println!("{:.2}", float); // 소수점 2번째 짜리 까지 출력
    println!("{:>10.2}", float); // 소수점 2번째 짜리 까지 출력 및 오른쪽 정렬
                            
    
}
