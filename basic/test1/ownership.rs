//소유권 및 Clone
fn modify_string(mut s: String) -> String{
    s.push_str(" is modified");
    s
}


fn main() {
    let mut s = String::from("hello");
    let s2 = s.clone();
    let mod_s = modify_string(s);
    
    println!("s2 : {}", s2);
    println!("mod_s : {}", mod_s);
}