// const : 전역상수, 꼭 타입을 지정해줘야함.
const HIGH_SCORE: i32 = 20;

// static : 전역변수, 반드 타입 지정
// static lifetime : 프로그램이 시작하고 끝날 때까지 살아있음.
static LOW_SCORE: i32 = 0;

fn print_high_score() {
    println!("The high score is {}", HIGH_SCORE);
}



fn main() {
    print_high_score();
    //unsafe한 전역변수인 static은 절대 사용하지 않아야됨.
    // unsafe를 만약 사용할 때는 이런식으로 쓰기
    unsafe {LOW_SCORE = 1;} 

}


