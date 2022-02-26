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
fn main() {
    let s = String::from("hello"); // sがスコープに入る

    takes_ownership(s); // sの値が関数にムーブされ...
                        // ... ここではもう有効ではない
                        // println!("{}", s); <- error

    let x = 5; // xがスコープに入る

    makes_copy(x); // xも関数にムーブされるが、
                   // i32はCopyなので、この後にxを使っても
                   // 大丈夫
} // ここでxがスコープを抜け、sもスコープを抜ける。ただし、sの値はムーブされているので、何も特別なことは起こらない。
  //

fn takes_ownership(some_string: String) {
    // some_stringがスコープに入る。
    println!("{}", some_string);
} // ここでsome_stringがスコープを抜け、`drop`が呼ばれる。後ろ盾してたメモリが解放される。
  //

fn makes_copy(some_integer: i32) {
    // some_integerがスコープに入る
    println!("{}", some_integer);
} // ここでsome_integerがスコープを抜ける。何も特別なことはない。
