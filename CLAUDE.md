# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```sh
cargo build --release        # ビルド
cargo run --release          # 実行
cargo test                   # テスト実行
cargo fmt                    # フォーマット
cargo clippy -- -W clippy::all  # lint
```

## Architecture

単一ファイル構成(`src/main.rs`)のTUIカレンダーアプリ。ratatui + crossterm。

- `App`構造体が状態を保持(year, month, holidays, today, should_quit)
- `fetch_holidays()` → holidays-jp APIからJSON取得。失敗時は空HashMapで継続
- `draw()` → Layout::verticalで3分割(タイトル/カレンダー/フッター)して描画
- `handle_events()` → crossterm pollベースのイベントループ
- `main()` → 祝日取得 → Terminal初期化 → ループ → Terminal復元

## Testing

TDDで進める。Red-Green-Refactorサイクルを守る。

`#[cfg(test)] mod tests`がmain.rs末尾にある。`app_with_holidays()`ヘルパーでモックデータ付きAppを生成。TestBackendで描画結果を検証する際、全角文字は空セルが挟まるので空白除去してからassertする。

CIと同じローカルチェックは`/check`スキルで実行。

## CI / Release

- `/release`スキルでバージョンアップ→tag→pushを実行（内部でcargo-releaseを使用）
- `v*`タグのpushでGitHub Actionsがクロスビルド→GitHub Releaseを自動作成
- GitHub Actionsのワークフローには必ず`timeout-minutes`を指定する
- ワークフロー変更時はactionlintでlintする

## Style

- 日本語の全角と半角の間にスペースを入れない（例: ○「TUIアプリ」 ×「TUI アプリ」）
- コミットメッセージはConventional Commits、英語、一行で簡潔に
