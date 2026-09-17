# スキルの references と本体の一致

スキル kotowari の references（"skills/kotowari/references/" の文書）に写した本体の値が本体と一致することの検査を扱う。この検査は kotowari の振る舞いではなく、このリポジトリのテストが行う。突き合わせる相手は本体のコードが持つ値であり、IR の表ではない。

## 要求

### REQ-125: 指摘の種類の一致

- 種類: ubiquitous
- 出典: docs/decision/records/2026-09-17-check-reach.md#A6, docs/decision/records/2026-09-17-check-reach.md#A7
- 検証: unit

このリポジトリのテストは常に、"skills/kotowari/references/findings.md" のヘッダの1列目が「種類」の表について、ヘッダと区切りの行を除いた1列目の集合が、本体のコードが出す`指摘`の種類の集合と等しいことを確かめる。

### REQ-126: 既定の一致

- 種類: ubiquitous
- 出典: docs/decision/records/2026-09-17-check-reach.md#A6, docs/decision/records/2026-09-17-check-reach.md#A7, docs/decision/records/2026-09-17-check-reach.md#A11, docs/decision/records/2026-09-17-check-reach.md#A27
- 検証: unit

このリポジトリのテストは常に、"skills/kotowari/references/config.md" の setup の手順1にある YAML のコードブロックに `TBL-004` の鍵がすべて書かれていて、そのブロックを`設定ファイル`として本体のコードで読んだ結果が本体のコードが持つ設定の既定の値と等しいことを確かめる。

### REQ-127: 停止の文言の一致

- 種類: ubiquitous
- 出典: docs/decision/records/2026-09-17-check-reach.md#A6, docs/decision/records/2026-09-17-check-reach.md#A7, docs/decision/records/2026-09-17-check-reach.md#A12
- 検証: unit

このリポジトリのテストは常に、"skills/kotowari/references/findings.md" のヘッダの1列目が「文言」の表について、ヘッダと区切りの行を除いた1列目の集合が、本体のコードが`停止`の理由として標準エラーの1行目に出す文言の集合と等しいことを確かめる。

## 具体例

```gherkin
@id=EX-036 @about=REQ-125 @source=docs/decision/records/2026-09-17-check-reach.md#A7,docs/decision/records/records.md#A154
Scenario: references の表から種類が1つ欠けるとテストが落ちる
  Given "skills/kotowari/references/findings.md" の「種類」の表から "duplicate_term" の行が欠けている
  When このリポジトリのテストを実行する
  Then 指摘の種類の一致のテストが失敗する

@id=EX-037 @about=REQ-126 @source=docs/decision/records/2026-09-17-check-reach.md#A11,docs/decision/records/2026-09-16-notice.md#A5
Scenario: setup の YAML の既定が本体とずれるとテストが落ちる
  Given "skills/kotowari/references/config.md" の手順1の YAML のコードブロックの "limits.lines" が 120 で、本体の既定が 200
  When このリポジトリのテストを実行する
  Then 既定の一致のテストが失敗する

@id=EX-043 @about=REQ-126 @source=docs/decision/records/2026-09-17-check-reach.md#A27
Scenario: setup の YAML から鍵が落ちるとテストが落ちる
  Given "skills/kotowari/references/config.md" の手順1の YAML のコードブロックに "limits.requirements" の鍵が無く、本体の既定は 10
  When このリポジトリのテストを実行する
  Then 既定の一致のテストが失敗する

@id=EX-039 @about=REQ-127 @source=docs/decision/records/2026-09-17-check-reach.md#A12
Scenario: references の停止の表の文言が本体とずれるとテストが落ちる
  Given "skills/kotowari/references/findings.md" の「文言」の表の1行目が "config error" ではなく "configuration error"
  When このリポジトリのテストを実行する
  Then 停止の文言の一致のテストが失敗する
```
