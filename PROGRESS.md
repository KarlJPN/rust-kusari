# 進捗状況

## セッション運用メモ

- 進行ルールは `AGENTS.md` を参照する
- 学習計画の全体像は `docs/plan.md` を参照する

## 現在のステップ

Step 3 途中 — enum / match / struct の復習中。定義部分は書き直し完了、main関数の中身がまだ

## 完了済みステップ

### フェーズ1: ミニ言語のコアを作る

- [x] Step 1: 基本型・変数・関数
- [x] Step 2: 文字列・スライス・位置追跡
- [~] Step 3: TokenKind と Token を定義する（途中）
- [ ] Step 4: Vec でトークン列を持つ
- [ ] Step 5: struct と impl でパーサを作る
- [ ] Step 6: AST を作る
- [ ] Step 7: 参照・借用・可変参照
- [ ] Step 8: Result / Option / ? とカスタムエラー型
- [ ] Step 9: 評価器を作る + Value enum + Display
- [ ] Step 10: HashMap で変数環境を持つ
- [ ] Step 11: 制御構文を追加する
- [ ] Step 12: print 文を追加する
- [ ] Step 13: モジュール分割
- [ ] Step 14: テスト体系化
- [ ] Step 15: REPL とファイル実行

### フェーズ2: 実用CLIを作る

- [ ] Step 1: CLI引数を受け取る
- [ ] Step 2: ファイルを読み込む
- [ ] Step 3: 実データに対して言語を使う
- [ ] Step 4: エラー設計を改善する
- [ ] Step 5: テストとリファクタリング

## 前回のセッション終了時のメモ

- Step 3 途中（復習から再開した）
- 今回やったこと:
  - コードを白紙にして1から書き直す形で復習
  - `enum TokenKind` に `Minus` と `Identifier(String)` を追加して4バリアントに拡張
  - `struct Position` と `struct Token` の定義を書き直し
  - struct の定義（設計図）と値の作成（実際に使う）の違いを再確認
  - `Identifier` が `String` を持つ理由：Token が文字列を所有する必要があるため
- まだ不安なこと:
  - struct の定義と値の作成の書き方が混同しやすい（`=` や `::` を使いそうになった）
  - `usize` という型名がすぐ出てこない
- 理解が定着してきたこと:
  - enum のバリアント定義（値を持つ / 持たない）
  - struct のフィールド定義（`フィールド名: 型,`）

## 次回やるべき最小タスク

1. main関数の中身を書く: `Token` の値を作る（`let token = Token { ... }`の形）
2. `println!` で位置情報を表示する
3. `match` で4つのバリアント全部に対して表示を分ける
4. テスト（`#[test]`, `assert_eq!`）を導入する
