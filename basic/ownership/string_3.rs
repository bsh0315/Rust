// OWNERSHIP = 소유권

// & = reference
fn return_it() -> &String {
    let name = String::from("백승헌");
    &name // return &String
          // 컴파일 오류가 발생함!!
          // name의 lifetime은 현재 함수에만 한정됨.
          // 함수가 끝나면 해당 데이터를 없음.
}
fn main() {
    let country = String::from("백승헌");
    let ref_one = &country;
    let ref_two = &country;

    println!("Name is : {}", ref_one);
    

}

