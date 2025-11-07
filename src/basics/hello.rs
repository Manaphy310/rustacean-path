// hello.rs - Hello Worldとprintln!マクロの使い方

/// Hello Worldを学ぶための関数
pub fn run() {
    println!("\n=== Hello World ===\n");

    // 基本的な出力
    println!("Hello, World!");
    println!("こんにちは、Rust!");

    // フォーマット付き出力
    println!("\n--- フォーマット付き出力 ---");
    let name = "Rustacean";
    let age = 1;
    println!("私の名前は{}です", name);
    println!("{}歳です", age);
    println!("名前: {}, 年齢: {}歳", name, age);

    // 名前付きプレースホルダー
    println!("\n--- 名前付きプレースホルダー ---");
    println!(
        "{language}は{adjective}プログラミング言語です",
        language = "Rust",
        adjective = "安全で高速な"
    );

    // デバッグ出力 - {:?} を使うと構造体やタプルなどを出力できる
    println!("\n--- デバッグ出力 ---");
    let tuple = (1, "hello", true);
    println!("タプル: {:?}", tuple);

    // 見やすいデバッグ出力 - {:#?} を使うと整形される
    let array = [1, 2, 3, 4, 5];
    println!("配列: {:#?}", array);

    println!("\n");
}
