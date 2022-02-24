fn main() {
    let number = 3;

    // 明示し、必ずifには条件式として、論理値を与えなければなりません
    if number < 5 {
        println!("condition was true"); // 条件は真でした
    } else {
        println!("condition was false"); // 条件は偽でした
    }

    // ifは式なので、let文の右辺に持ってくることができます
    let condition = true;
    // ifとelseアームは互換性のない値の型になり、 コンパイラがプログラム内で問題の見つかった箇所をスバリ指摘してくれます
    let number = if condition { 5 } else { 6 };

    // numberの値は、{}です
    println!("The value of number is: {}", number);
}
