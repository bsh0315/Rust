// uninitialized variable, 러스트에서는 절대 사용 불가
// control flow

// possibly uninitialized = maybe doewn't have a value yet

fn loop_then_return(mut counter: i32) -> i32 {
    loop{
        counter +=1;
        if counter % 50 == 0 {
            break;
        }

    }
    counter
}


fn main() {
    let mut my_number = {
        // 복잡한 것들
        let x = 9;
        x+9 // 블록의 맨 마지막에 ';'가 없는 것은 반환값이기 때문임.
    };

    println!("{}", my_number);

    {
        // 복잡한 것들
        let x = loop_then_return(43);
        my_number = x;
    };

    println!("{}", my_number);


}
