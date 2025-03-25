// 예외적으로 &와 &mut을 같이 쓸 수 있는경우.

fn main() {
    let mut number = 10;
    let number_change = &mut number;
    *number_change += 10;
    let number_ref = &number;
    println!("Number is {}", number_ref);

}
