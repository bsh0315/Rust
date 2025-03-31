// const 및 static
// const는 전역상수, static은 전역변수
// static은 사용하면 안됨.
const  a:i32 = 10;
static mut b:i32 = 20;


fn main(){
    println!("const a = {}", a);
    println!("static b = {}",unsafe {b});
    unsafe { b= 30; }
    println!("static b = {}", unsafe {b});
}
