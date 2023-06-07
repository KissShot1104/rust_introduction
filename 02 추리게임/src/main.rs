use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

	let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

	loop {
		let mut guess = String::new();

		println!("숫자를 입력해주세요 >>");

		io::stdin()
		.read_line(&mut guess)
		.expect("입력을 실패했습니다.");

		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,

		};

		match guess.cmp(&secret_number) {
			Ordering::Greater => println!("큽니다"),
			Ordering::Less => println!("작습니다"),

			Ordering::Equal => {
				println!("정답입니다.");
				break;
			},
		}

	}



}