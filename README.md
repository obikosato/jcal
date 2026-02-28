# jcal - 日本の祝日付きTUIカレンダー

ターミナルで`cal`コマンドのようにカレンダーを表示しつつ、日本の祝日をハイライト表示するTUIアプリケーション。

![demo](demo/demo.gif)

祝日データは[holidays-jp](https://holidays-jp.github.io/) APIから取得。オフライン時は祝日なしで動作します。

## 必要環境

- Rust 1.70+

## ビルド・実行

```sh
# ビルド
cargo build --release

# 実行
cargo run --release

# または直接バイナリを実行
./target/release/jcal
```

## 開発

```sh
# フォーマット
cargo fmt

# lint
cargo clippy -- -W clippy::all

# フォーマットチェック（CI向け）
cargo fmt -- --check

# ビルド確認
cargo build
```

## 技術スタック

ratatui + crosstermでTUI、chronoで日付処理、reqwestで祝日API取得。詳細は`Cargo.toml`を参照。

## ファイル構成

```txt
jcal/
├── Cargo.toml
├── README.md
├── demo/
│   ├── demo.tape   # VHS録画スクリプト
│   └── demo.gif    # 生成されたデモGIF
└── src/
    └── main.rs
```
