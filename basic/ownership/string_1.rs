// Rust 소유권 및 참조 개념
// String : 데이터를 직접 소유할 수 있고, 수정할 수 있는 타입
// &String : String의 내용을 빌려오는 참조. 소유권 x 및 읽기 전용 타입.

fn main() {
//String = Sized type, 구조체 자체는 고정된 크기를 가짐.(힙에 저장)
//&str = dynamic type

    let my_name = "David".to_string(); //&str을 Stirng으로 변환
    let other_name = String::from("David2"); //위와 동일한 역할
    // growable + shrinkable
    
    let mut  my_other_name = "David3".to_string();
    my_other_name.push('!');
    println!("{}",my_other_name);
    my_other_name.push_str("Hello, world!");
    
    // name_ref는 my_ohter_name의 내용을 참조
    let name_ref: &String = &my_other_name;

    // 참조한 내용물을 출력함
    println!("{}", name_ref);
    
}
