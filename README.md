# Tiny Exp

Rust製の小さな式言語インタープリター。WebAssemblyで動作し、ブラウザで式のトークン・AST・評価結果を表示します。

[ブラウザで試す](https://oishimugicha.github.io/tiny-exp/)

## 実行例

```text
1 + 2 * 3                       → 7
(1 + 2) * 3                     → 9
if 1 < 2 then 42 else 0          → 42
if true then 1 else true + 2     → 1
```

## 言語仕様

値は64ビット符号付き整数（`i64`）と真偽値（`true` / `false`）。加減乗算、比較、条件分岐に対応します。入力は式1つで、Unicodeの空白・改行を使用できます。変数とコメントは未対応です。

```text
expr       = if_expr | comparison
if_expr    = "if" expr "then" expr "else" expr
comparison = sum [ "<" sum ]
sum        = product { ("+" | "-") product }
product    = atom { "*" atom }
atom       = [ "-" ] digits | "true" | "false" | "(" expr ")"
digits     = digit { digit }
digit      = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
```

`{ ... }` は繰り返し、`[ ... ]` は省略可能を表します。

- 演算の優先順位は `*`、`+ -`、`<` の順。算術演算は左結合です。比較の連鎖（`1 < 2 < 3`）は構文エラーになります。
- 負号は整数リテラルにのみ使用でき、`- 2` や `1--2` も有効です。`-(1)` や `--2` は構文エラーになります。整数の先頭のゼロは許可します。
- 整数の範囲は `-9223372036854775808` から `9223372036854775807`。リテラル・演算結果の範囲超過はエラーになります。
- 算術演算と比較の引数は整数のみ。型の不一致は実行時エラーになります。
- ifの条件は真偽値で、選択された分岐だけを評価します。両分岐の型は異なっていても構いませんが、どちらも構文として有効である必要があります。
- 演算の引数にif式を置く場合は、`1 + (if true then 2 else 3)` のように括弧が必要です。
- 空入力、不完全な式、余分なトークン、未知の単語・文字はエラーになります。

## ローカル開発

必要環境: Rust（edition 2024対応）、wasm-pack 0.15.0、Python 3（HTTPサーバー用）。

### セットアップ

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-pack --version 0.15.0 --locked
```

### ビルド・起動

リポジトリのルートで実行します。

```sh
wasm-pack build --target web --locked
python3 -m http.server 8000 --bind 127.0.0.1
```

アクセス先: http://localhost:8000 。Rustの変更後はWasmを再ビルドします。

### テスト

```sh
cargo test --locked
cargo test --release --locked
```

CIはPRでテストとWasmビルドを実行し、mainへのpushまたはmainを指定した手動実行でGitHub Pagesに公開します。
