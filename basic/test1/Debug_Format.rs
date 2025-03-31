// Debug Format
#[derive(Debug)]
struct Person {
    name: String,
    age : u32,
}

fn main(){
    let mut person = Person{
        name: String::from("엄준식"),
        age : 30,
    };
    println!("Person : {:?}", person);

    
}