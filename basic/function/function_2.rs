// String, &String, &mut String
// 씨발 뭔 자료형이 이리많냐 헷갈림.

// String(CBV)
// 매개변수를 통해서 소유권을 넘겨받을 때는 
// mut string과 같이 변수 이름 앞에 mut를 붙여야함.
// 또한, 한번 소유권을 넘겨준 이상 원본은 무의미 해짐을 반드시 기억!!

// &String(불변참조)
// 원본 데이터에 대한 불변참조임.
// 읽기전용으로 열기가능함.

// &mut String(가변참조)
// 원본 데이터를 수정할 수 있음.


fn add_is_great(country_name: &mut String) {
    country_name.push_str(" is great!");
    println!("&mut, it says: {}", country_name);
}

fn modify_string(mut string: String){
    string.push_str("추가된 문자열이다 병신아");
    println!("String, it says {}", string);
}

fn modify_string2(string: &String){
    println!("&String, it says {}", string);
}

fn main() {
    let mut my_country = String::from("캐나다");
    add_is_great(&mut my_country);
    add_is_great(&mut my_country);
    println!("&mut, it says: {}",my_country);

    let string = String::from("hello");
    modify_string(string);
    //modify_string(string); 
    // println!("{}", string); 이거는 오루발생!!

    let string2 = String::from("what the fuck");
    modify_string2(&string2); 
    modify_string2(&string2); 
    println!("{}", string2);
    
}
