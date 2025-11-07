// variables.rs - 変数と可変性について学ぶ

/// 変数の基本を学ぶための関数
pub fn run() {
    println!("\n=== 変数と可変性 ===\n");

    // 不変変数（デフォルト）
    println!("--- 不変変数 ---");
    let x = 5;
    println!("xの値: {}", x);
    // x = 6; // エラー！不変変数は再代入できない

    // 可変変数
    println!("\n--- 可変変数 (mut) ---");
    let mut y = 10;
    println!("yの初期値: {}", y);
    y = 20; // mut を付けると再代入できる
    println!("yの更新後: {}", y);

    // シャドーイング（同じ変数名を再宣言）
    println!("\n--- シャドーイング ---");
    let z = 5;
    println!("zの最初の値: {}", z);

    let z = z + 10; // 新しい変数として再定義
    println!("zをシャドーイングした後: {}", z);

    let z = z * 2; // さらに再定義
    println!("zを再度シャドーイング: {}", z);

    // シャドーイングは型を変更できる
    println!("\n--- シャドーイングで型変更 ---");
    let spaces = "   "; // 文字列型
    println!("spaces（文字列）: '{}'", spaces);

    let spaces = spaces.len(); // 数値型に変更
    println!("spaces（数値）: {}", spaces);

    // 定数
    println!("\n--- 定数 ---");
    const MAX_POINTS: u32 = 100_000; // 定数は型注釈が必須
    println!("定数 MAX_POINTS: {}", MAX_POINTS);
    println!("※ 定数は常に不変で、プログラム全体で有効");

    // スコープ
    println!("\n--- スコープ ---");
    let outer = 42;
    println!("外側のスコープ: outer = {}", outer);
    {
        let inner = 100;
        println!("内側のスコープ: inner = {}", inner);
        println!("内側からouterにアクセス: {}", outer);
    }
    // println!("{}", inner); // エラー！innerはスコープ外
    println!("内側のスコープを抜けた後: outer = {}", outer);

    println!("\n");
}
