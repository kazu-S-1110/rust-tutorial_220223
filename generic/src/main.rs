#![allow(dead_code)] // この行でコンパイラのwaringsメッセージを止めます。
struct Location(i32, i32);
fn main() {
    // スタティックメソッド - ある型そのものに紐付き、演算子 :: で呼び出せます。
    // インスタンスメソッド - ある型のインスタンスに紐付き、演算子 . で呼び出せます。

    // スタティックメソッドでStringインスタンスを作成する。
    let s = String::from("Hello world!");

    // インスタンスを使ってメソッド呼び出す。
    println!("{} is {} characters long.", s, s.len());

    // 構造体のインスタンス化
    let ferris = Creature {
        species: Species::Crab,
        name: String::from("RustCrab"),
        arms: 2,
        legs: 4,
        birth: String::from("today"),
    };
    println!(
        "This is a {}. They have {} arms, {} legs, and birthday is {}",
        ferris.name, ferris.arms, ferris.legs, ferris.birth
    );

    match ferris.species {
        Species::Crab => println!("{} is a crab", ferris.name),
        Species::Octopus => println!("{} is a octopus", ferris.name),
        Species::Fish => println!("{} is a fish", ferris.name),
        Species::Clam => println!("{} is a clam", ferris.name),
    }

    // タプルライクな構造体
    let loc = Location(32, 43);
    println!("{},{}", loc.0, loc.1)
}

// 一つの struct はフィールドの集合です。
// フィールド とはデータ構造とキーワードを紐付ける値です。その値はプリミティブ型かデータ構造を指定可能です。
struct Creature {
    species: Species,
    name: String,
    arms: i32,
    legs: i32,
    birth: String,
}

enum Species {
    Crab,
    Octopus,
    Fish,
    Clam,
}
