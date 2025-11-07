# 🦀 Rustacean Path

Rust初学者向けの学習プログラムです。基本文法をモジュール化された構造で段階的に学べます。

## 📚 学習内容

このプロジェクトでは、以下のRustの基本概念を学ぶことができます：

1. **Hello World** (`src/basics/hello.rs`)
   - `println!` マクロの使い方
   - フォーマット付き出力
   - デバッグ出力

2. **変数と可変性** (`src/basics/variables.rs`)
   - 不変変数と可変変数（`mut`）
   - シャドーイング
   - 定数（`const`）
   - スコープ

3. **データ型** (`src/basics/data_types.rs`)
   - 整数型、浮動小数点型
   - ブール型、文字型
   - タプル型、配列型
   - 文字列スライス

4. **制御フロー** (`src/basics/control_flow.rs`)
   - `if` 式
   - `loop`、`while`、`for` ループ
   - `match` 式（パターンマッチング）
   - `break`、`continue`

5. **関数** (`src/basics/functions.rs`)
   - 関数の定義と呼び出し
   - 引数と戻り値
   - 式と文の違い

## 🚀 実行方法

### 前提条件

Rustがインストールされている必要があります。インストールされていない場合は、以下のコマンドでインストールしてください：

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### プログラムの実行

```bash
# ビルドして実行
cargo run

# リリースビルドで実行（最適化あり）
cargo run --release

# ビルドのみ
cargo build
```

## 📁 プロジェクト構造

```
rustacean-path/
├── Cargo.toml              # プロジェクト設定ファイル
├── README.md               # このファイル
└── src/
    ├── main.rs             # エントリーポイント
    └── basics/             # 基本文法モジュール
        ├── mod.rs          # モジュール定義
        ├── hello.rs        # Hello World
        ├── variables.rs    # 変数と可変性
        ├── data_types.rs   # データ型
        ├── control_flow.rs # 制御フロー
        └── functions.rs    # 関数
```

## 🎓 学習の進め方

1. まず `cargo run` でプログラムを実行して、各セクションの出力を確認しましょう
2. 各モジュールのソースコード（`src/basics/` 内のファイル）を読んで、コメントと実装を理解しましょう
3. コードを自分で変更して、動作を実験してみましょう
4. エラーメッセージを恐れずに、いろいろ試してみましょう

## 📖 次のステップ

基本文法を習得したら、以下のトピックに進むことをおすすめします：

- **所有権（Ownership）** - Rustの最も重要な概念
- **構造体とenum** - 独自のデータ型を定義する
- **エラーハンドリング** - `Result` と `Option` 型
- **コレクション** - `Vec`、`HashMap`、`String` など
- **モジュールシステム** - より大きなプログラムの構造化

## 🔗 参考リソース

- [The Rust Programming Language（日本語版）](https://doc.rust-jp.rs/book-ja/)
- [Rust By Example（日本語版）](https://doc.rust-jp.rs/rust-by-example-ja/)
- [公式ドキュメント](https://www.rust-lang.org/learn)

Happy Coding! 🦀
