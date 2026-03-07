# 進捗状況

## 現在のステップ

Step 3 途中 — enum / match / struct の基本を学習済み。テスト導入が残っている

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

- Step 3 途中（自己評価: 3/5）
- 学んだこと:
  - `enum`: 種類を全部決めておく仕組み。値を持てるバリアント (`Number(i64)`) と持たないバリアント (`Plus`) がある
  - `match`: enum の種類ごとに処理を分ける構文。全パターン必須。`Number(n)` の `n` で値を取り出せる
  - `struct`: 複数の情報を1つにまとめる箱。ドット `.` で中身を取り出す
  - `usize`: 0以上の正の整数型
  - struct の中に別の struct や enum を型として入れられる
- まだ不安なこと:
  - struct の入れ子が複雑になると混乱しやすい
  - 一つずつなら理解できるが全体像が掴みにくい

## 次回やるべき最小タスク

1. Step 3 の続き: TokenKind に Minus と Identifier を追加する
2. テスト（`#[test]`, `assert_eq!`）を導入する
3. 復習: enum / match / struct を使ったコードを読み直す
