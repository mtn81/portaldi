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

# フォーマット
cargo fmt

# フォーマット/リントチェック (cargo fmt --check + cargo clippy -- -D warnings)
make check

# 全パッケージのテスト (ワークスペース外のテストパッケージ含む)
make test-all

# 全パッケージのクリーン
make clean-all
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

- コミットする前に必ず `make check` と `make test-all` を実行する
