// scala와 비슷하게 입력값 -> 반환값 의 형태로 함수를 작성할 수 있음
// 만약 반환값이 empty tuple인 경우에는 (one : i32, two: i32)라고만 쓰면 됨.
//

fn give_number(one: i32, two: i32) -> i32 {
    let mult =  one * two;
    println!("{}", mult);
    mult
}

fn print_number(one: i16, two: i16) -> i16 {
    let multiplied_by_ten = {
        let first_number = 10;
        first_number * one * two
    };
    multiplied_by_ten 
}

fn main() {
    let value =  give_number(7,8);
    println!("{}", value);
    println!("{}", print_number(4,5));
}
