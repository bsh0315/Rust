//만약 함수에 인자로 넘겨져도 소유권을 유지하고 싶은 경우

fn print_string(s: &String) {
    println!("{}", s);

}


fn main() {
    let string = String::from("Hello, Rust!");

    // string의 참조만 전달함으로써 소유권을 유지함.
    print_string(&string);
    print_string(&string);
    print_string(&string);
    print_string(&string);
}
