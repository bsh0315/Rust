// It's trivial to copy the bytes. 
// Ownership and copy types

fn prints_number(number: i32){
    println!("{}", number);
}

fn prints_string(input: String){
    println!("{}", input);
}

// copy : copy types
// 단순히 메모리의 비트들을 복사함.

// clone : non-copy types
// clone은 c++에서의 '깊은 복사' 개념과 같음
// 복사를 수행할 때 완전히 독립적으로 복사본을 만듬.
// 원본과는 관계가 없음.

fn main() {
    let my_number = 8;
    prints_number(my_number);
    prints_number(my_number);

    let my_country = "Austra".to_string();
    prints_string(my_country.clone()); //clone을 통해서 독립적인 데이터 생성.
    prints_string(my_country);

}
