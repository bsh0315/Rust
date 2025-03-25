// & immutable reference / shared reference
// &mut mutable reference / unique reference
// &mut를 통해서 참조를 통해서도 값을 바꿀 수 있음.
// &와 &mut는 같이 사용할 수 없음.

fn main() {
    let mut my_number = 9;
    let num_ref = &mut my_number;
    let mut my_number2 = 90;
    let num_ref2 = &my_number2;

    println!("Number is {}", my_number2);
    // 불변 참조일 경우에는 원본으로 접근 가능

    // println!("Number is {}", my_number);
    // 해당 print는 오류 발생
    // 가변참조 사용중에는 원본 변수로 직접 접근을 못함.
    // 소유권이 이동한 것은 아님.

    *num_ref = 10;
    println!("Number is {}", num_ref);
}
