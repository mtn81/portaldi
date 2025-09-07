# Claude Code Configuration

このファイルはClaude Codeがプロジェクトを理解するための設定ファイルです。

## プロジェクト概要
Rust製のDIライブラリ

## 開発環境
- 言語: Rust
- パッケージマネージャー: Cargo

## よく使うコマンド
```bash
# ビルド
cargo build

# テスト実行
cargo test

# リント
cargo clippy

# フォーマット
cargo fmt

# 実行
cargo run
```

## プロジェクト構造
```
packages     
├── index    クレートのAPIを公開とドキュメント
├── core     DIコンテナなどのコアになる部品の実装     
├── macros   proc-macroの実装 
└── tests    各種設定を変えながらのテスト(独立したパッケージにする必要があるので、cargoのワークスペースからは除外)
```

## 注意事項
- コミットする前に必ず `cargo clippy` と `cargo fmt` を実行する
- テストは `cargo test` で実行する
