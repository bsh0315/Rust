// 패턴 매칭(scala에서의 방식과 거의 동일함.)

fn main(){
    let x = 4;
    match x{
        1 => println!("Value is {}", x),
        2 => println!("Value is {}", x),
        3 => println!("Value is {}", x),
        4 => println!("Value is {}", x),
        5 => println!("Value is {}", x),
        _ => println!("Fucking invald value."),
        // rust에서는 모든 경우의 수를 모두 커버 하지않으면,
        // 컴파일에러가 발생함.
    }
}