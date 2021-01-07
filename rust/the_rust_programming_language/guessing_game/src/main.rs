extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        // Rust の変数は基本的には immutable だが mut を付与することで可変の変数となる
        // :: で静的メソッドの呼び出し
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess) // ＆ で変数の参照
            .expect("Failed to read line");

        // shadowing で同じ変数名を再定義できる
        // trim で 前後の空白（改行含む）を除去して parse で u32 に変換
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // _ は包括値
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too samll!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
