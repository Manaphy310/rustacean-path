// data_types.rs - Rustの基本的なデータ型を学ぶ

/// データ型を学ぶための関数
pub fn run() {
    println!("\n=== データ型 ===\n");

    // 整数型
    println!("--- 整数型 ---");
    let decimal = 98_222; // 10進数（_は読みやすさのため）
    let hex = 0xff; // 16進数
    let octal = 0o77; // 8進数
    let binary = 0b1111_0000; // 2進数
    println!("10進数: {}", decimal);
    println!("16進数: {} (10進数で{})", hex, hex);
    println!("8進数: {} (10進数で{})", octal, octal);
    println!("2進数: {} (10進数で{})", binary, binary);

    // 明示的な整数型
    let small: i8 = 127; // -128 to 127
    let unsigned: u32 = 1000; // 0 to 4,294,967,295
    println!("i8型: {}", small);
    println!("u32型: {}", unsigned);

    // 浮動小数点型
    println!("\n--- 浮動小数点型 ---");
    let x = 2.0; // f64（デフォルト）
    let y: f32 = 3.0; // f32
    println!("f64型: {}", x);
    println!("f32型: {}", y);

    // 算術演算
    println!("\n--- 算術演算 ---");
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;
    println!("加算: {}", sum);
    println!("減算: {}", difference);
    println!("乗算: {}", product);
    println!("除算: {}", quotient);
    println!("剰余: {}", remainder);

    // ブール型
    println!("\n--- ブール型 ---");
    let t = true;
    let f: bool = false;
    println!("真: {}", t);
    println!("偽: {}", f);

    // 文字型（Unicodeスカラー値）
    println!("\n--- 文字型 ---");
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("英字: {}", c);
    println!("Unicode: {}", z);
    println!("絵文字: {}", heart_eyed_cat);

    // タプル型
    println!("\n--- タプル型 ---");
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; // 分割代入
    println!("タプル全体: {:?}", tup);
    println!("分割代入: x={}, y={}, z={}", x, y, z);
    println!("インデックスアクセス: tup.0={}, tup.1={}", tup.0, tup.1);

    // 配列型（固定長）
    println!("\n--- 配列型 ---");
    let arr = [1, 2, 3, 4, 5];
    let first = arr[0];
    let second = arr[1];
    println!("配列: {:?}", arr);
    println!("最初の要素: {}", first);
    println!("2番目の要素: {}", second);

    // 同じ値で初期化
    let arr2 = [3; 5]; // [3, 3, 3, 3, 3]と同じ
    println!("初期化された配列: {:?}", arr2);

    // 文字列スライス型
    println!("\n--- 文字列型 ---");
    let s = "Hello, Rust!"; // &str（文字列スライス）
    println!("文字列スライス: {}", s);
    println!("文字列長: {}", s.len());

    println!("\n");
}
