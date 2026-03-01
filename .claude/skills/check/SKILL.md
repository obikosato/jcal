---
name: check
description: CIと同じチェックをローカルで実行する
---

CIと同じチェックをローカルで実行してください。すべてパスしたら結果を報告してください。

1. `cargo fmt -- --check`
2. `cargo clippy -- -W clippy::all`
3. `cargo test`
4. `docker run --rm -v "$(pwd):/repo" -w /repo rhysd/actionlint:latest -color`
