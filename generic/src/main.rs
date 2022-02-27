fn main() {
    // スタティックメソッド - ある型そのものに紐付き、演算子 :: で呼び出せます。
    // インスタンスメソッド - ある型のインスタンスに紐付き、演算子 . で呼び出せます。

    // スタティックメソッドでStringインスタンスを作成する。
    let s = String::from("Hello world!");

    // インスタンスを使ってメソッド呼び出す。
    println!("{} is {} characters long.", s, s.len());
}

// 一つの struct はフィールドの集合です。
// フィールド とはデータ構造とキーワードを紐付ける値です。その値はプリミティブ型かデータ構造を指定可能です。
struct Creature {
    name: String,
    arms: i32,
    legs: i32,
    birth: String,
}
