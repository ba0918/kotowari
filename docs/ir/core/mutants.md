# 変異の結果の検査

"kotowari mutants" が`変異の結果`から`見逃し`と時間切れを`指摘`にし、集計を出すところを扱う。結果のファイルの読み取りは mutants-input.md で、`等価の一覧`の読み方は equivalents.md で扱う。

## 要求

### REQ-core-139: 見逃しの指摘

- 種類: event_driven
- 出典: docs/decision/records/2026-09-17-mutation-tests.md#A2, docs/decision/records/2026-09-17-mutation-tests.md#A7, docs/decision/records/2026-09-17-mutation-tests.md#A11, docs/decision/records/2026-09-17-mutation-tests.md#A31, docs/decision/records/2026-09-17-mutation-tests.md#A37, docs/decision/records/2026-09-17-mutation-tests.md#A40, docs/decision/records/2026-09-17-mutation-tests.md#A50
- 検証: unit

"kotowari mutants" で、`変異の結果`の結果が「見逃した」で、`等価の一覧`のどの1件にも一致しないとき、kotowari は "path" をその`変異の結果`のファイル、"line" をその行、detail を変更の説明にして mutant_survived の`誤り`を出す。結果が「捕まえた」か「ビルド不能」の`変異の結果`には`指摘`を出さず、`等価の一覧`との一致も見ない。同じ内容の`変異の結果`が2件以上あっても畳まず、1件ごとに`指摘`を出す。

### REQ-core-140: 時間切れの指摘

- 種類: event_driven
- 出典: docs/decision/records/2026-09-17-mutation-tests.md#A11, docs/decision/records/2026-09-17-mutation-tests.md#A31, docs/decision/records/2026-09-17-mutation-tests.md#A37, docs/decision/records/2026-09-17-mutation-tests.md#A40, docs/decision/records/2026-09-17-mutation-tests.md#A50
- 検証: unit

"kotowari mutants" で、`変異の結果`の結果が「時間切れ」のとき、kotowari は "path" をその`変異の結果`のファイル、"line" をその行、detail を変更の説明にして mutant_timeout の`注意`を出す。`等価の一覧`との一致は見ない。

### REQ-core-145: 変異の集計

- 種類: algorithm
- 出典: docs/decision/records/2026-09-17-mutation-tests.md#A28, docs/decision/records/2026-09-17-mutation-tests.md#A35, docs/decision/records/2026-09-17-mutation-tests.md#A38, docs/decision/records/2026-09-17-mutation-tests.md#A50, docs/decision/records/2026-09-17-mutation-tests.md#A55
- 定義: TBL-core-025, PROP-core-005
- 検証: unit

### REQ-core-146: 集計の文字の出力

- 種類: event_driven
- 出典: docs/decision/records/2026-09-17-mutation-tests.md#A28, docs/decision/records/2026-09-17-mutation-tests.md#A35, docs/decision/records/2026-09-17-mutation-tests.md#A38
- 検証: unit

"kotowari mutants" で "--format" が "text" のとき、kotowari は`指摘`の行の後の最後の1行に "mutants: caught=数 survived=数 timeout=数 unviable=数 equivalent=数" の形で集計を出す。`指摘`が0件でも出す。

### REQ-core-147: 読む範囲

- 種類: ubiquitous
- 出典: docs/decision/records/2026-09-17-mutation-tests.md#A8, docs/decision/records/2026-09-17-mutation-tests.md#A35, docs/decision/records/2026-09-17-mutation-tests.md#A47, docs/decision/records/2026-09-17-mutation-tests.md#A55
- 検証: unit

kotowari は常に、"kotowari mutants" で`設定ファイル`、結果のファイル、`等価の一覧`、`変異の結果`が指すファイル、`等価の一覧`の1件の "file" が指すファイルだけを読み、`IR`と`テストのファイル`と`判断の記録`と ADR を読まず、"kotowari check" の検査を行わず、"ir"、"decisions.records"、"decisions.adr" の指す先が無くても`停止`しない。

### REQ-core-150: 道具に固有の語を使わない

- 種類: prohibition
- 出典: docs/decision/records/2026-09-17-mutation-tests.md#A12, docs/decision/records/2026-09-17-mutation-tests.md#A13, docs/decision/records/2026-09-17-mutation-tests.md#A31, docs/decision/records/2026-09-17-mutation-tests.md#A54
- 検証: review
- 確かめ方: 指摘の種類名（mutant_survived、mutant_timeout、equivalent_stale、equivalent_invalid）と detail を組み立てる src/mutants.rs と src/lib.rs に、道具の結果の値の綴り（CaughtMutant、MissedMutant）が現れないことを rg -n 'CaughtMutant|MissedMutant' src/mutants.rs src/lib.rs が何も出さないことで確認。道具の値を写す src/cargo_mutants.rs は対象外

kotowari は、`指摘`の種類と detail の形に、変異テストの道具に固有の語を使ってはならない。detail の中の変更の説明は道具が出した文のままで、この対象にしない。

## 決定表

### TBL-core-025: "kotowari mutants" の JSON

- 出典: docs/decision/records/2026-09-17-mutation-tests.md#A35, docs/decision/records/2026-09-17-mutation-tests.md#A38, docs/decision/records/2026-09-17-mutation-tests.md#A40, docs/decision/records/2026-09-17-mutation-tests.md#A55

"kotowari mutants" の JSON の最上位は findings、counts、mutants の3つの鍵だけを持つ。

| 階層 | 鍵 | 中身 |
|---|---|---|
| 最上位 | findings | 指摘の一覧。鍵は TBL-core-006、並べ方は TBL-core-007 |
| 最上位 | counts | 種類ごとの指摘の数。1件も無い種類は持たない |
| 最上位 | mutants | 下の5つの鍵を持つオブジェクト |
| "mutants" の中 | caught | 結果が「捕まえた」の変異の結果の数 |
| "mutants" の中 | survived | 結果が「見逃した」で、等価の一覧のどの1件にも一致しない変異の結果の数 |
| "mutants" の中 | timeout | 結果が「時間切れ」の変異の結果の数 |
| "mutants" の中 | unviable | 結果が「ビルド不能」の変異の結果の数 |
| "mutants" の中 | equivalent | 結果が「見逃した」で、等価の一覧の1件以上に一致する変異の結果の数 |

## 性質

### PROP-core-005: 集計の合計

- 出典: docs/decision/records/2026-09-17-mutation-tests.md#A11, docs/decision/records/2026-09-17-mutation-tests.md#A31, docs/decision/records/2026-09-17-mutation-tests.md#A38, docs/decision/records/2026-09-17-mutation-tests.md#A40, docs/decision/records/2026-09-17-mutation-tests.md#A50

"mutants" の5つの値の合計は`変異の結果`の数に等しく、"survived" は mutant_survived の`指摘`の数に、"timeout" は mutant_timeout の`指摘`の数に等しい。

## 具体例

```gherkin
@id=EX-core-204 @about=REQ-core-138,REQ-core-139,TBL-core-024 @source=docs/decision/records/2026-09-17-mutation-tests.md#A2,docs/decision/records/2026-09-17-mutation-tests.md#A7,docs/decision/records/2026-09-17-mutation-tests.md#A11,docs/decision/records/2026-09-17-mutation-tests.md#A31,docs/decision/records/2026-09-17-mutation-tests.md#A40,docs/decision/records/2026-09-17-mutation-tests.md#A42,docs/decision/records/2026-09-17-mutation-tests.md#A50,docs/decision/records/2026-09-17-mutation-tests.md#A16
Scenario: 見逃しは誤りになる
  Given 結果のファイルの "outcomes" に、"summary" が "Success" の基準の実行と、"file" が "src/a.rs"、"span.start" の "line" が 3 で "column" が 5、"name" が "src/a.rs:3:5: replace f with ()"、"summary" が "MissedMutant" の1件がある
  And `等価の一覧`の鍵が設定に無い
  When "kotowari mutants --tool cargo-mutants outcomes.json" を実行する
  Then "path" が "src/a.rs"、"line" が 3、detail が "replace f with ()" の mutant_survived の誤りが出る
  And 終了コードは 1 である

@id=EX-core-205 @about=REQ-core-139,REQ-core-145 @source=docs/decision/records/2026-09-17-mutation-tests.md#A11,docs/decision/records/2026-09-17-mutation-tests.md#A35,docs/decision/records/2026-09-17-mutation-tests.md#A38
Scenario: 捕まえた変異とビルド不能の変異には何も出ない
  Given 結果のファイルに "summary" が "CaughtMutant" の1件と "Unviable" の1件がある
  When "kotowari mutants --tool cargo-mutants outcomes.json" を実行する
  Then "findings" は空で、"mutants" の "caught" は 1、"unviable" は 1 になる
  And 終了コードは 0 である

@id=EX-core-206 @about=REQ-core-140 @source=docs/decision/records/2026-09-17-mutation-tests.md#A11,docs/decision/records/2026-09-17-mutation-tests.md#A31,docs/decision/records/2026-09-17-mutation-tests.md#A37,docs/decision/records/2026-09-17-mutation-tests.md#A40
Scenario: 時間切れは注意で、一覧に書いても外れない
  Given 結果のファイルに "src/a.rs" の3行目の "summary" が "Timeout" の1件があり、`等価の一覧`に "file" と "change" と "text" がその`変異`に合い、"class" が "equivalent" で "why" が空でない1件がある
  When "kotowari mutants --tool cargo-mutants outcomes.json" を実行する
  Then "path" が "src/a.rs" で "line" が 3 の mutant_timeout の注意が出る
  And 終了コードは 0 である

@id=EX-core-209 @about=REQ-core-145,REQ-core-146,PROP-core-005 @source=docs/decision/records/2026-09-17-mutation-tests.md#A35,docs/decision/records/2026-09-17-mutation-tests.md#A38
Scenario: 変異が0件の結果は終了コード0で集計はすべて0
  Given 結果のファイルの "outcomes" に基準の実行の1件だけがある
  When "kotowari mutants --tool cargo-mutants --format text outcomes.json" を実行する
  Then 標準出力は "mutants: caught=0 survived=0 timeout=0 unviable=0 equivalent=0" の1行だけである
  And 終了コードは 0 である

@id=EX-core-210 @about=REQ-core-147 @source=docs/decision/records/2026-09-17-mutation-tests.md#A8,docs/decision/records/2026-09-17-mutation-tests.md#A35,docs/decision/records/2026-09-17-mutation-tests.md#A55
Scenario: IR の置き場が無くても結果を読める
  Given 設定の "ir" の指す先が無く、結果のファイルに "summary" が "CaughtMutant" の1件がある
  When "kotowari mutants --tool cargo-mutants outcomes.json" を実行する
  Then 終了コードは 0 である

@id=EX-core-229 @about=REQ-core-139,PROP-core-005 @source=docs/decision/records/2026-09-17-mutation-tests.md#A11,docs/decision/records/2026-09-17-mutation-tests.md#A31,docs/decision/records/2026-09-17-mutation-tests.md#A38,docs/decision/records/2026-09-17-mutation-tests.md#A50
Scenario: 同じ変異が2件あれば指摘も2件出る
  Given 結果のファイルに、ファイルと行と "name" が同じで "summary" が "MissedMutant" の1件が2つある
  When "kotowari mutants --tool cargo-mutants outcomes.json" を実行する
  Then mutant_survived の誤りが2件出て、"mutants" の "survived" は 2 になる

@id=EX-core-230 @about=REQ-core-146 @source=docs/decision/records/2026-09-17-mutation-tests.md#A11,docs/decision/records/2026-09-17-mutation-tests.md#A31,docs/decision/records/2026-09-17-mutation-tests.md#A35,docs/decision/records/2026-09-17-mutation-tests.md#A38,docs/decision/records/2026-09-17-mutation-tests.md#A40
Scenario: 集計は指摘の行の後の最後の1行に出る
  Given 結果のファイルに "src/a.rs" の3行目の変更の説明が "replace f with ()" の`見逃し`が1件だけある
  When "kotowari mutants --tool cargo-mutants --format text outcomes.json" を実行する
  Then 標準出力の1行目は "src/a.rs:3 [error] mutant_survived replace f with ()" である
  And 標準出力の最後の行は "mutants: caught=0 survived=1 timeout=0 unviable=0 equivalent=0" である

@id=EX-core-231 @about=REQ-core-147,TBL-core-025 @source=docs/decision/records/2026-09-17-mutation-tests.md#A8,docs/decision/records/2026-09-17-mutation-tests.md#A35,docs/decision/records/2026-09-17-mutation-tests.md#A55
Scenario: 印の無いテストがあっても mutants は指摘せず、JSON は3つの鍵だけを持つ
  Given "#[test]" の付いた関数に`印`が無く、結果のファイルに "summary" が "CaughtMutant" の1件がある
  When "kotowari mutants --tool cargo-mutants outcomes.json" を実行する
  Then "findings" は空で、JSON の最上位の鍵は "findings"、"counts"、"mutants" の3つだけである
```
