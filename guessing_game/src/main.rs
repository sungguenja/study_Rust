use std::io; // std이라는 라이브러리에서 io 라이브러리를 가져옴

fn main() {     // fn 함수 명명
    println!("숫자를 맞혀봅시다");

    println!("정답이라고 생각하는 숫자를 입력하세요.");

    let mut guess = String::new();  // let 불변변수 선언 let mut 가변변수 선언
    // guess라는 가변변수에 string타입의 빈 값을 선언
    io::stdin().read_line(&mut guess).expect("입력한 값을 읽지 못했습니다.");
    // 위 두줄은 한줄로 작성해도 괜찮다
    // 1번줄을 쓰지 않았다면 std::io::stdin
    
    println!("입력한 값: {}", guess);
}