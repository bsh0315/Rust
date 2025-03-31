// Copy 타입 test
// 대입(=)에서 소유권 이동은 Copy 트레이트를 구현하지 않은 타입에서 발생함.
//ex) String 

fn main() {
    let num1: i32 = 100;
    let num2: i32 = num1; // 소유권 이동이 발생되지 않음.
    //let num2: i32 = num1.clone();

    println!("num1 = {}, num2 = {}", num1, num2);
    
}