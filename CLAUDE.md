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
make lint

# フォーマット/リントの自動修正 (cargo fmt + cargo clippy --fix)
make lint-fix

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

- コミットする前に必ず `make lint` と `make test-all` を実行する
- rustのバージョンが上がった場合などに、trybuildのstderrの出力がズレたらTRYBUILD=overwriteで再生成する
- ドキュメントは、packages/index/src/docs/ 配下に配置されている。

## PRレビュー時の観点

`@claude` でレビューを依頼されたときは、以下の観点でフィードバックする:

- コード品質とベストプラクティス
- 潜在的なバグや問題
- パフォーマンスの考慮点
- セキュリティ上の懸念
- テストカバレッジ
- ドキュメントに内容が反映されているか

レビュー結果は必ず日本語で記述し、建設的で役立つ内容を心がける。
