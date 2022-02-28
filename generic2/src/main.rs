// Result はとてもよく使うので、Rust にはそれを扱うための強力な演算子 ? が用意されています。 以下の2つのコードは等価です。

// do_something_that_might_fail()?

// match do_something_that_might_fail() {
//     Ok(v) => v,
//     Err(e) => return Err(e),
// }

fn do_something(i: i32) -> Result<String, String> {
    if i == 43 {
        Ok(String::from("しっかり43の値が入ってます！"))
    } else {
        Err(String::from("正しい値じゃないよ"))
    }
}

fn main() -> Result<(), String> {
    // let result = do_something(43);

    // match result {
    //     Ok(v) => println!("Right! {}", v),
    //     Err(e) => println!("Error: {}", e),
    // }

    let v = do_something(43)?;
    println!("{}", v);
    Ok(())
}
