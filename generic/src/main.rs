#![allow(dead_code)] // この行でコンパイラのwarningsメッセージを止めます。
struct Location(i32, i32);

enum PoisonType {
    Acidic,
    Painful,
    Lethal,
}
enum Size {
    Big,
    Small,
}
enum Weapon {
    Claw(i32, Size),
    Poison(PoisonType),
    None,
}
fn main() {
    // 構造体のインスタンス化
    let rust_crab = Creature {
        species: Species::Crab,
        name: String::from("RustCrab"),
        arms: 2,
        legs: 4,
        birth: String::from("today"),
        weapon: Weapon::Claw(2, Size::Small),
    };
    let ferris = Creature {
        species: Species::Crab,
        name: String::from("ferris"),
        arms: 2,
        legs: 4,
        birth: String::from("1 year ago"),
        weapon: Weapon::Poison(PoisonType::Lethal),
    };

    match ferris.species {
        Species::Crab => match ferris.weapon {
            Weapon::Claw(num_claws, ref size) => {
                //参照にしないとあかんってエラーが出た。
                let size_description = match size {
                    Size::Big => "big",
                    Size::Small => "small",
                };
                println!(
                    "ferris is a crab with {} {} claws",
                    num_claws, size_description
                )
            }
            _ => println!("ferris is a crab with some other weapon"),
        },
        _ => println!("ferris is some other animal"),
    }

    match rust_crab.species {
        Species::Crab => match ferris.weapon {
            Weapon::Claw(num_claws, size) => {
                let size_description = match size {
                    Size::Big => "big",
                    Size::Small => "small",
                };
                println!(
                    "ferris is a crab with {} {} claws",
                    num_claws, size_description
                )
            }
            _ => println!("ferris is a crab with some other weapon"),
        },
        _ => println!("ferris is some other animal"),
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
    weapon: Weapon,
}

enum Species {
    Crab,
    Octopus,
    Fish,
    Clam,
}
