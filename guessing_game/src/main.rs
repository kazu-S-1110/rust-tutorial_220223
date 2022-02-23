use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    //範囲は下限値を含み、上限値を含まないため

    println!("The secret number is : {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    //let foo = 5; // immutable
    //let mut bar = 5; // mutable

    // ::new行にある::という記法は、newがString型の関連関数であることを表しています。 関連関数とは、String型の特定のオブジェクトよりも型(この場合はString)に対して 実装された関数のことであり、静的(スタティック)メソッドと呼ばれる言語もあります。

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    // 仮に、プログラムの冒頭でuse std::ioとしていなければ、この関数呼び出しは、std::io::stdinと記述していたでしょう。

    println!("You guessed: {}", guess);

    /* 別の例(PythonのFormatに似てる)
    let x = 5;
    let y = 10;

    println!("x = {} and y = {}", x, y); */
}
