fn main() {
    {
        // sは、ここでは有効ではない。まだ宣言されていない
        let s = "hello"; // sは、ここから有効になる
        println!(" s is {}", s);
        // sで作業をする
    } // このスコープは終わり。もうsは有効ではない
      // println!(" s is {}", s); // <- error

    // ヒープにメモリを確保する型
    let mut s = String::from("hello");
    s.push_str(", world!");

    println!("{}", s);
    // ひとたび、メモリを所有している変数がスコープを抜けたら、 メモリは自動的に返還されます

    let s1 = String::from("hello");
    let s2 = s1;

    //コンパイラは最初の変数をも無効化するので、shallow copyと呼ばれる代わりに、 ムーブとして知られている
    // println!("{}", s1); //<-error
}
