# Claude Code Configuration

このファイルはClaude Codeがプロジェクトを理解するための設定ファイルです。

## プロジェクト概要

Rust製のDIライブラリ

## 開発環境

- 言語: Rust (edition 2024, MSRV 1.85.0, toolchain は stable)
- パッケージマネージャー: Cargo (ワークスペース構成)
- `make test-all` には [wasmtime](https://wasmtime.dev/) が必要 (wasm32-wasip1 ターゲットは rust-toolchain.toml で導入される)
- `make lint` には [cargo-machete](https://github.com/bnjbvr/cargo-machete) が必要 (`cargo install cargo-machete --locked`)
- `make coverage` には [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov) と llvm-tools が必要 (`cargo install cargo-llvm-cov --locked` と `rustup component add llvm-tools-preview`)
- `make udeps-all` には nightly ツールチェーン (wasm32-wasip1 ターゲット込み) と [cargo-udeps](https://github.com/est31/cargo-udeps) が必要 (`rustup toolchain install nightly --target wasm32-wasip1` と `cargo install cargo-udeps --locked`)

## よく使うコマンド

```bash
# ビルド
cargo build

# フォーマット
cargo fmt

# フォーマット/リント/未使用依存のチェック (cargo fmt --check + cargo clippy -- -D warnings + cargo machete)
make lint

# フォーマット/リント/未使用依存の自動修正 (cargo machete --fix + cargo fmt + cargo clippy --fix)
make lint-fix

# 全パッケージのテスト (ワークスペース外のテストパッケージ含む)
make test-all

# ワークスペースのテストカバレッジを計測 (表を表示し、HTML を target/llvm-cov/html に出力。cargo-llvm-cov が必要)
make coverage

# 全パッケージの未使用依存をコンパイルして厳密にチェック (nightly と cargo-udeps が必要)
make udeps-all

# 全パッケージのクリーン
make clean-all
```

## プロジェクト構造

```
packages
├── index    クレートのAPIを公開とドキュメント (portaldi)
├── core     DIコンテナなどのコアになる部品の実装 (portaldi-core)
├── macros   proc-macroの実装 (portaldi-macros)
└── tests    各種設定を変えながらのテスト(独立したパッケージにする必要があるので、cargoのワークスペースからは除外)
```

## 開発時の規約

- コミットする前に必ず `make lint` と `make test-all` を実行する (CIも同じ内容を実行する。Claude Code では `.claude/hooks/check-before-commit.sh` が `git commit` 前に自動実行する)
- ソースコード (`.rs`、`Cargo.toml` など) のコメントは英語で書く
- コメントは必要最小限にする。コードを読めば分かる内容 (処理の言い換え、アサーションやエラーメッセージの繰り返しなど) は書かず、コードからは読み取れない理由や背景だけを書く
- rustのバージョンが上がった場合などに、trybuildのstderrの出力がズレたらTRYBUILD=overwriteで再生成する
- ドキュメントは packages/index/src/docs/ 配下に配置されている。機能を変更したらドキュメントと README.md にも反映する
- `packages/tests/` にテストパッケージを追加したら、Makefile の `test-all` / `udeps-all` / `clean-all` にも追記し、`Cargo.toml` に空の `[workspace]` を書く (親ディレクトリのワークスペースに誤って所属しないようにするため)
- マクロが生成するコードだけが使う依存 (`async-trait`、`futures` など) を cargo-machete が未使用と誤検知する場合は、そのパッケージの `[package.metadata.cargo-machete]` の `ignored` に追加する
- バージョンは手動で変更しない。GitHub Release 時に CI がルート `Cargo.toml` の `#tag:release-version` 行を書き換えて crates.io に公開する
- `portaldi` クレートでは、再エクスポートとドキュメントのみ定義する
- `portaldi-core` クレートでは、コアな機能 (trait、struct, function など)を実装する
- `portaldi-macros` クレートでは、手続き型マクロを実装する (`.claude/rules/impl_macro.md` を参照)

## レビュー

`@claude` などでレビューを依頼されたときは、`review-guidelines` スキルの観点に従う。
レビュー結果は必ず日本語で記述する。
