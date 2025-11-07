// control_flow.rs - 制御フロー（if文、ループ）を学ぶ

/// 制御フローを学ぶための関数
pub fn run() {
    println!("\n=== 制御フロー ===\n");

    // if式
    println!("--- if式 ---");
    let number = 7;

    if number < 5 {
        println!("{}は5より小さい", number);
    } else if number < 10 {
        println!("{}は5以上10未満", number);
    } else {
        println!("{}は10以上", number);
    }

    // if式は値を返せる
    println!("\n--- if式で値を返す ---");
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("条件式の結果: {}", number);

    // loop - 無限ループ
    println!("\n--- loop（無限ループ） ---");
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // breakで値を返せる
        }
    };
    println!("loopの結果: {}", result);

    // while - 条件付きループ
    println!("\n--- while（条件付きループ） ---");
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("発射！");

    // for - コレクションのループ
    println!("\n--- for（コレクションのループ） ---");
    let arr = [10, 20, 30, 40, 50];
    println!("配列の各要素:");
    for element in arr {
        println!("  値: {}", element);
    }

    // for - 範囲のループ
    println!("\n--- for（範囲のループ） ---");
    println!("1から5まで:");
    for number in 1..6 {
        println!("  {}", number);
    }

    // 降順ループ
    println!("\n--- for（降順ループ） ---");
    println!("3から1まで（カウントダウン）:");
    for number in (1..4).rev() {
        println!("  {}!", number);
    }
    println!("発射！");

    // match式 - 強力なパターンマッチング
    println!("\n--- match式 ---");
    let number = 13;
    match number {
        1 => println!("1です！"),
        2 | 3 | 5 | 7 | 11 => println!("{}は素数です", number),
        13..=19 => println!("{}は10代の数です", number),
        _ => println!("{}はそれ以外の数です", number),
    }

    // matchで値を返す
    println!("\n--- matchで値を返す ---");
    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1,
    };
    println!("boolean {} は binary {}", boolean, binary);

    // continueとbreak
    println!("\n--- continueとbreak ---");
    println!("偶数のみ表示（1-10）:");
    for i in 1..=10 {
        if i % 2 != 0 {
            continue; // 奇数はスキップ
        }
        if i > 8 {
            break; // 8より大きければループを抜ける
        }
        println!("  {}", i);
    }

    println!("\n");
}
