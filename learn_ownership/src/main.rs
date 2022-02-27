// fn main() {
//     {
//         // sは、ここでは有効ではない。まだ宣言されていない
//         let s = "hello"; // sは、ここから有効になる
//         println!(" s is {}", s);
//         // sで作業をする
//     } // このスコープは終わり。もうsは有効ではない
//       // println!(" s is {}", s); // <- error

//     // ヒープにメモリを確保する型
//     let mut s = String::from("hello");
//     s.push_str(", world!");

//     println!("{}", s);
//     // ひとたび、メモリを所有している変数がスコープを抜けたら、 メモリは自動的に返還されます

//     let s1 = String::from("hello");
//     let _s2 = s1;

//     //コンパイラは最初の変数をも無効化するので、shallow copyと呼ばれる代わりに、 ムーブとして知られている
//     // println!("{}", s1); //<-error

//     // コピーしたい場合はcloneを使う
//     let c1 = String::from("rust");
//     let c2 = c1.clone(); // ヒープデータが実際にコピーされています

//     println!("c1 = {}, c2 = {}", c1, c2)
// }

// 型はCopyして持続性がある。
// fn main() {
//     let s = String::from("hello"); // sがスコープに入る

//     takes_ownership(s); // sの値が関数にムーブされ...
//                         // ... ここではもう有効ではない
//                         // println!("{}", s); <- error

//     let x = 5; // xがスコープに入る

//     makes_copy(x); // xも関数にムーブされるが、
//                    // i32はCopyなので、この後にxを使っても
//                    // 大丈夫
// } // ここでxがスコープを抜け、sもスコープを抜ける。ただし、sの値はムーブされているので、何も特別なことは起こらない。
//   //

// fn takes_ownership(some_string: String) {
//     // some_stringがスコープに入る。
//     println!("{}", some_string);
// } // ここでsome_stringがスコープを抜け、`drop`が呼ばれる。後ろ盾してたメモリが解放される。
//   //

// fn makes_copy(some_integer: i32) {
//     // some_integerがスコープに入る
//     println!("{}", some_integer);
// } // ここでsome_integerがスコープを抜ける。何も特別なことはない。

// 戻り値とスコープ
// fn main() {
//     let s1 = gives_ownership(); // gives_ownershipは、戻り値をs1に
//                                 // ムーブする

//     let s2 = String::from("hello"); // s2がスコープに入る

//     let s3 = takes_and_gives_back(s2); // s2はtakes_and_gives_backにムーブされ
//                                        // 戻り値もs3にムーブされる
// } // ここで、s3はスコープを抜け、ドロップされる。s2もスコープを抜けるが、ムーブされているので、
//   // 何も起きない。s1もスコープを抜け、ドロップされる。

// fn gives_ownership() -> String {
//     // gives_ownershipは、戻り値を
//     // 呼び出した関数にムーブする

//     let some_string = String::from("hello"); // some_stringがスコープに入る

//     some_string // some_stringが返され、呼び出し元関数に
//                 // ムーブされる
// }

// // takes_and_gives_backは、Stringを一つ受け取り、返す。
// fn takes_and_gives_back(a_string: String) -> String {
//     // a_stringがスコープに入る。

//     a_string // a_stringが返され、呼び出し元関数にムーブされる
// }

// 複数の戻り値を返す
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    //'{}'の長さは、{}です
    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len()メソッドは、Stringの長さを返します

    (s, length)
}
