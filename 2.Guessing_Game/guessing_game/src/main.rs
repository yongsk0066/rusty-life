extern crate rand;
// 외부 의존성이 있음을 표시

use std::io;
// 표준 라이브러리 std에서 io를 스코프로 가져온다.
use rand::Rng;
use std::cmp::Ordering;
// rand 라이브러리에서 Rng를 스코프로 가져온다.

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    // rand 라이브러리에서 thread_rng()를 호출한다.
    // 시드를정하고 정수를 생성한다.
    // 기본은 i32로 추론된다.

    println!("The secret number is: {}", secret_number);

    loop {
        // while true같은 역할

        println!("Please input your guess.");

        let mut guess = String::new();
        // mut을 통해 가변변수를 선언한다.
        // String::new 의 결과값(String인스턴스)이 guess에 대입된다.
        // :: 는 String의 연관함수임을 의미한다. (정적메소드)

        // 빈 String 을 생성하고 이를 guess에 저장한다.

        io::stdin()
            .read_line(&mut guess)
            // & 는 참조자(reference)임을 나타낸다.
            // 참조자또한 기본이 불변이기 때문에 &guess가 아니라 &mut guess로 적어준다.
            .expect("Failed to read line");
        // read_line은 io::Result를 반환한다.
        // Result는 열거형(enums)로 Ok, Err를 가진다.
        // Result는 expect 메소드를 가지고 있다.
        // Err를 받을 경우 프로그램을 종료하고 에러메시지를 출력한다.
        // Ok를 받을 경우 결과값을 반환한다.

        let guess: u32 = match guess.trim().parse() {
            // parse는 Result를 돌려준다.
            Ok(num) => num,
            Err(_) => continue,
        };
        // guess를 생성했었지만, rust에서는 shadowing이 허용된다.
        // trim()은 문자열의 양쪽 공백을 제거한다.
        // parse()는 String을 u32로 변환한다.

        println!("You guessed: {}", guess);
        // 중괄호(format string)으로 순서대로 넘겨받는 값을 출력한다.

        match guess.cmp(&secret_number) {
            // match를 통해 사용자가 입력한 값과 정답을 비교한다.
            // match는  패턴 매칭
            // cmp 메소드는 Ordering(enums)을 반환한다.
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
