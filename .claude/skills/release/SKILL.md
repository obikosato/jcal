---
name: release
description: cargo-releaseで新しいバージョンをリリースする
allowed_commands:
  - "git tag"
  - "git log"
  - "cargo metadata"
  - "cargo release"
---

cargo-releaseを使って新しいバージョンをリリースしてください。
`cargo release`はCargo.tomlのversion更新→コミット→tag作成→pushを一括で行い、
pushされた`v*`タグをトリガーにGitHub Actionsがクロスビルド→GitHub Releaseを自動作成します。

1. `git tag` と `cargo metadata --format-version 1 --no-deps | jq -r '.packages[0].version'` で現在のバージョンを確認
2. 前回タグ以降のコミットを確認して、次のバージョンをユーザーに提案（Semantic Versioning）
3. 確認後、`cargo release <patch|minor|major> --execute` を実行
