// functions.rs - 関数の定義と使い方を学ぶ

/// 関数を学ぶための関数
pub fn run() {
    println!("\n=== 関数 ===\n");

    // 関数の呼び出し
    println!("--- 基本的な関数 ---");
    say_hello();

    // 引数を持つ関数
    println!("\n--- 引数を持つ関数 ---");
    greet("太郎");
    greet("花子");

    // 複数の引数
    println!("\n--- 複数の引数 ---");
    print_labeled_measurement(5, 'h');

    // 戻り値を持つ関数
    println!("\n--- 戻り値を持つ関数 ---");
    let x = five();
    println!("five()の戻り値: {}", x);

    let result = plus_one(5);
    println!("plus_one(5)の戻り値: {}", result);

    // 計算をする関数
    println!("\n--- 計算をする関数 ---");
    let sum = add(10, 20);
    println!("10 + 20 = {}", sum);

    let product = multiply(7, 6);
    println!("7 × 6 = {}", product);

    // 式と文の違い
    println!("\n--- 式と文の違い ---");
    let y = {
        let x = 3;
        x + 1 // セミコロンなし = 式（値を返す）
    };
    println!("ブロック式の値: {}", y);

    // 早期リターン
    println!("\n--- 早期リターン ---");
    println!("check_number(0): {}", check_number(0));
    println!("check_number(5): {}", check_number(5));
    println!("check_number(-3): {}", check_number(-3));

    println!("\n");
}

// 引数なし、戻り値なし
fn say_hello() {
    println!("こんにちは！");
}

// 引数あり、戻り値なし
fn greet(name: &str) {
    println!("{}さん、こんにちは！", name);
}

// 複数の引数（型注釈が必要）
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("測定値: {}{}", value, unit_label);
}

// 戻り値を持つ関数（->で戻り値の型を指定）
fn five() -> i32 {
    5 // セミコロンなし = この値を返す
}

fn plus_one(x: i32) -> i32 {
    x + 1 // セミコロンなし = 式として値を返す
}

// 計算を行う関数
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

// 条件分岐を持つ関数
fn check_number(n: i32) -> &'static str {
    if n == 0 {
        return "ゼロです"; // 早期リターン
    }

    if n > 0 {
        "正の数です"
    } else {
        "負の数です"
    }
}
