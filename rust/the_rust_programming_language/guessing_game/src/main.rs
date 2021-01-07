extern crate rand;

use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    // Rust の変数は基本的には immutable だが mut を付与することで可変の変数となる
    // :: で静的メソッドの呼び出し
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess) // ＆ で変数の参照
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
