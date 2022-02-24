fn main() {
    // Rustの定数の命名規則は、 全て大文字でアンダースコアで単語区切りすることです):
    const MAX_POINTS: u32 = 100_000;

    // シャドーイングは、変数をmutにするのとは違います。なぜなら、letキーワードを使わずに、 誤ってこの変数に再代入を試みようものなら、コンパイルエラーが出るからです。
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);

    let guess: u32 = "43".parse().expect("not a number");

    // スカラー型は、単独の値を表します。Rustには主に4つのスカラー型があります: 整数、浮動小数点数、論理値、最後に文字です

    let t = true;
    let f: bool = false;

    let c: char = 'z';

    // 複合型:2種類の基本的な複合型
    // タプル型
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // 配列型　タプルと異なり、配列の全要素は、 同じ型でなければなりません
    let a = [1, 2, 3, 4, 5];
    let first = a[2]; //3
}
