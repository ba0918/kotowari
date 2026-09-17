# 要求とテストの対応

要求に印のあるテストがあるか、テストに印があるかの検査を扱う。

## 要求

### REQ-085: テストのない要求

- 種類: event_driven
- 出典: docs/decision/records/records.md#A21, docs/decision/records/records.md#A39, docs/decision/records/ir-form.md#検査の種類, docs/decision/records/records.md#A120
- 検証: unit

検証が "review" 以外の`要求`で、その`ID`を含む`印`が1つも無いとき、kotowari は requirement_without_test の`誤り`を出す。"- 検証:" の行が無い`要求`には verification_missing だけを出し、requirement_without_test は出さない。

### REQ-086: 印の無いテスト

- 種類: event_driven
- 出典: docs/decision/records/records.md#A21, docs/decision/records/records.md#A24, docs/decision/records/ir-form.md#検査の種類
- 検証: unit

`問い合わせのある言語`の`テスト`に`印`が無いとき、kotowari はその関数の名前を detail にして test_without_id の`誤り`を出す。

### REQ-087: 問い合わせの無い言語の対応

- 種類: event_driven
- 出典: docs/decision/records/records.md#A24, docs/decision/records/records.md#A39, docs/decision/adr/0002-tree-sitter.md#結果, docs/decision/records/records.md#A51, docs/decision/records/ir-form.md#検査の種類
- 検証: unit

`問い合わせの無い言語`の`テストのファイル`を読むとき、kotowari は拾った`印`を requirement_without_test を消す側に数え、test_without_id を出さない。

### REQ-088: IR に文書が無いとき

- 種類: event_driven
- 出典: docs/decision/records/records.md#A41, docs/decision/records/records.md#A51
- 検証: unit

`IR`に文書が無いとき、kotowari は`IR`の検査の`指摘`を0件にし、`テスト`との対応の検査を行う。

## 具体例

```gherkin
@id=EX-019 @about=REQ-088 @source=docs/decision/records/records.md#A51,docs/decision/records/records.md#A29,docs/decision/records/records.md#A26
Scenario: IR が空でも印の無いテストは挙がる
  Given `IR`の置き場に文書が無い
  And "#[test]" の付いた関数が1つあり、`印`が無い
  When "kotowari check" を実行する
  Then test_without_id の誤りが1件出る
  And requirement_without_test の誤りは出ない
  And 終了コードは 1 である
```
