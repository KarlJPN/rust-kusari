# 進捗状況

## 現在のステップ

Step 2 完了 — Step 3 に進む準備ができている

## 完了済みステップ

### フェーズ1: ミニ言語のコアを作る

- [x] Step 1: 基本型・変数・関数
- [x] Step 2: 文字列・スライス・位置追跡
- [ ] Step 3: TokenKind と Token を定義する
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

- Step 1 完了、Step 2 完了
- Step 2 小テスト（String/&str）全問正解
- 学んだこと:
  - `String`（自分のノート、所有している）と `&str`（指差し、借りている、読むだけ）
  - `&` で借りると所有権が移らず、何回でも使える
  - `for c in input.chars()` で文字列を1文字ずつ取り出して繰り返す
  - `if ... else ...` で条件分岐
  - `\n` は改行を表す文字
  - 位置追跡: `let mut line` と `let mut column` で「何行目の何文字目か」を数える
  - `for` の中身は上から順に実行される（println! → if/else の順番が重要）

## 次回やるべき最小タスク

1. Step 3 に進む: `enum TokenKind` と `struct Token` を定義する
2. `match` による分岐を学ぶ
3. `Position` をトークンに載せる
