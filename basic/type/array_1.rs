// Collection types
// Array

// &str
// 원소의 개수가 같은 배열 끼리만 비교가능함. 
fn main() {
    let array = ["One", "Two"]; // [&str; 2]
    let array2 = ["One" ,"Five"]; // [&str; 2]
    let array3 = ["One", "Two" ,"Five"]; // [&str; 3]
    println!("Is array the same as array2? {}", array == array2); //여기서는 false가 출력됨.
    
    let array4 = [0; 640]; //640의 크기의 배열에 0을 채워넣음.
    println!("{:?}", array);
    
    let mut array5 = [0; 640]; // 가변선언, 이를 통해 요소변경 가능.
    let array6 = ["1월", "2월"]; //indexing
    println!("{}, {}", array[0], array[1]);
    

    //array.asldfjas;kdlf(); 
    //이를 통한 오류메시지를 통해서 배열의 type확인 가능.
}
