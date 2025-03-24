// mutability
// shadowing : 같은 이름을 다시 쓰는 것

// mmutable by default = 
// mut 

fn double(input : i32) -> i32 {
    input * 2
}

fn triple(input : i32) -> i32 {
    input * 3
}

fn main() {
    // mut 
    let mut my_number = 10;
    my_number = 9;

    //shadowing
    let my_variable = 10;
    let my_variable = "My variable";
    println!("{}\n", my_variable);
    
    //shadowing 사용의 예
    let x = 9;
    let x = double(x);
    let x = triple(x);
    println!("{}\n", x);

    let my_value = 9;
    println!("{}", my_value);
    {
        let my_value = "some string";
        println!("{}", my_value);
    } // shadowing된 my_value의 scope는 {}부분 까지임. 
    println!("{}", my_value); //여기서 부터는 다시 값이 9가됨.
}

