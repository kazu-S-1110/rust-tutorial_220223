// Rustには Result と呼ばれるジェネリックな列挙型が組み込まれており、失敗する可能性のある値を返せます。 これは言語がエラーを処理する際の慣用的な方法です。
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

fn do_something(i: i32) -> Result<String, String> {
    if i == 43 {
        Ok(String::from("しっかり43の値が入ってます！"))
    } else {
        Err(String::from("正しい値じゃないよ"))
    }
}

fn main() {
    let result = do_something(43);

    match result {
        Ok(v) => println!("Right! {}", v),
        Err(e) => println!("Error: {}", e),
    }
}
