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

4ファイル構成のTUIカレンダーアプリ。ratatui + crossterm。

- `src/app.rs` — `App`構造体(状態保持) + `fetch_holidays()`(holidays-jp APIからJSON取得、失敗時は空HashMap)
- `src/ui.rs` — `draw()`でLayout::verticalで3分割(タイトル/カレンダー/フッター)して描画
- `src/main.rs` — `main()`でサブコマンド分岐(引数なし→TUI、`update`→セルフアップデート) + `handle_events()`(crossterm pollベースのイベントループ) + `run_tui()`(Terminal初期化→ループ→復元)
- `src/update.rs` — `run_update()`(self_updateクレートでGitHub Releasesから最新バイナリを取得・置換)

ファイル構成を変更したらREADME・CLAUDE.mdも更新し、`/doc-check`スキルで整合性を検証する。

## Testing

TDDで進める。Red-Green-Refactorサイクルを守る。

`app.rs`にロジックテスト6件、`ui.rs`に描画テスト1件。各モジュール末尾の`#[cfg(test)] mod tests`に配置。`app_with_holidays()`ヘルパーでモックデータ付きAppを生成。TestBackendで描画結果を検証する際、全角文字は空セルが挟まるので空白除去してからassertする。

CIと同じローカルチェックは`/check`スキルで実行。

## CI / Release

- `/release`スキルでバージョンアップ→tag→pushを実行（内部でcargo-releaseを使用）
- `v*`タグのpushでGitHub Actionsがクロスビルド→GitHub Releaseを自動作成
- GitHub Actionsのワークフローには必ず`timeout-minutes`を指定する
- ワークフロー変更時はactionlintでlintする

## Style

- 日本語の全角と半角の間にスペースを入れない（例: ○「TUIアプリ」 ×「TUI アプリ」）
- コミットメッセージはConventional Commits、英語、一行で簡潔に
