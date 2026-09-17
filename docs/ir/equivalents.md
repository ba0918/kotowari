# 等価の一覧

`等価の一覧`の置き場と形、`変異の結果`との一致の取り方、一覧の1件への`指摘`を扱う。

## 要求

### REQ-141: 一覧の1件との一致

- 種類: ubiquitous
- 出典: docs/decision/records/2026-09-17-mutation-tests.md#A9, docs/decision/records/2026-09-17-mutation-tests.md#A15, docs/decision/records/2026-09-17-mutation-tests.md#A33, docs/decision/records/2026-09-17-mutation-tests.md#A37, docs/decision/records/2026-09-17-mutation-tests.md#A48, docs/decision/records/2026-09-17-mutation-tests.md#A51, docs/decision/records/2026-09-17-mutation-tests.md#A52, docs/decision/records/2026-09-17-mutation-tests.md#A34, docs/decision/records/2026-09-17-mutation-tests.md#A44
- 検証: unit

kotowari は常に、結果が「見逃した」の`変異の結果`と形の正しい`等価の一覧`の1件を、REQ-110 の正規化を掛けた "file" が`変異の結果`のファイルと同じ文字列で、"change" が変更の説明と同じ文字列で、"text" が`変異の結果`のファイルの今の内容のその行の文面と、どちらも前後の半角空白とタブを除いて同じ文字列のときに一致とする。行は TBL-010 のとおりに区切り、行の終わりの "\r\n" の "\r" は文面に含めない。`変異の結果`のファイルが無い、読めない、UTF-8 でない、または行がそのファイルの行数を超えるとき、kotowari は`停止`せず、その`変異の結果`はどの1件にも一致しない。同じ文面の行が複数あるファイルでは、1件がそのどの行の`変異の結果`にも一致する。

### REQ-142: 文面の無くなった1件

- 種類: event_driven
- 出典: docs/decision/records/2026-09-17-mutation-tests.md#A20, docs/decision/records/2026-09-17-mutation-tests.md#A31, docs/decision/records/2026-09-17-mutation-tests.md#A40, docs/decision/records/2026-09-17-mutation-tests.md#A48, docs/decision/records/2026-09-17-mutation-tests.md#A52, docs/decision/records/2026-09-17-mutation-tests.md#A53, docs/decision/records/2026-09-17-mutation-tests.md#A57
- 検証: unit

"kotowari mutants" で、形の正しい`等価の一覧`の1件の "text" と、どちらも前後の半角空白とタブを除いて同じ文面の行が "file" のファイルに1つも無いとき（ファイルが無い、読めない、UTF-8 でないときを含む）、kotowari は "path" を`等価の一覧`のファイル、"line" を null、detail を一覧に書かれたままの "file" と "change" を ": " でつないだ文字列にして equivalent_stale の`注意`を1件ごとに出す。結果のファイルにその`変異`が現れるかは見ない。形の誤った1件には出さない。

### REQ-143: 形の誤った1件

- 種類: event_driven
- 出典: docs/decision/records/2026-09-17-mutation-tests.md#A16, docs/decision/records/2026-09-17-mutation-tests.md#A17, docs/decision/records/2026-09-17-mutation-tests.md#A31, docs/decision/records/2026-09-17-mutation-tests.md#A34, docs/decision/records/2026-09-17-mutation-tests.md#A37, docs/decision/records/2026-09-17-mutation-tests.md#A40, docs/decision/records/2026-09-17-mutation-tests.md#A44, docs/decision/records/2026-09-17-mutation-tests.md#A51, docs/decision/records/2026-09-17-mutation-tests.md#A53, docs/decision/records/2026-09-17-mutation-tests.md#A57
- 検証: unit

"kotowari mutants" で、`等価の一覧`の1件が鍵と値の組でないとき、"file"、"change"、"text"、"class"、"why" のいずれかの鍵が無いとき、この5つ以外の鍵を持つとき、値が文字列でないとき、"why" が前後の半角空白とタブを除いて空のとき、"class" が "equivalent" でないとき、または "file" が絶対パスか ".." の要素を含むとき、kotowari は "path" を`等価の一覧`のファイル、"line" を null、detail を一覧に書かれたままの "file" と "change" を ": " でつないだ文字列にして equivalent_invalid の`誤り`を1件ごとに出し、その1件をどの`変異の結果`とも一致させない。detail の "file" と "change" は、無いか文字列でなければ空の文字列にする。同じ内容の1件が2つ以上あること自体は検査しない。

### REQ-148: 一覧の置き場

- 種類: event_driven
- 出典: docs/decision/records/2026-09-17-mutation-tests.md#A16, docs/decision/records/2026-09-17-mutation-tests.md#A34, docs/decision/records/2026-09-17-mutation-tests.md#A36, docs/decision/records/2026-09-17-mutation-tests.md#A44, docs/decision/records/2026-09-17-mutation-tests.md#A45, docs/decision/records/2026-09-17-mutation-tests.md#A49, docs/decision/records/2026-09-17-mutation-tests.md#A55
- 検証: unit

設定に "mutants.equivalents" の鍵が無いとき、または指す先が空（0バイトか注釈だけ）のとき、kotowari は`等価の一覧`を0件として "kotowari mutants" を続ける。鍵の指す先が無いか読めないとき、kotowari は読めないファイルを理由に`停止`する。指す先が UTF-8 でないとき、kotowari は UTF-8 でないファイルを理由に`停止`する。指す先が YAML として読めないとき、または最上位が並びでないとき、kotowari は設定の誤りを理由に`停止`し、詳細に`等価の一覧`のファイルの相対パスを出す。"kotowari check" は鍵の値を REQ-014 のとおりに検査するだけで、指す先を読まず、有無も見ない。

## 具体例

```gherkin
@id=EX-211 @about=REQ-139,REQ-141,REQ-145 @source=docs/decision/records/2026-09-17-mutation-tests.md#A9,docs/decision/records/2026-09-17-mutation-tests.md#A11,docs/decision/records/2026-09-17-mutation-tests.md#A15,docs/decision/records/2026-09-17-mutation-tests.md#A31,docs/decision/records/2026-09-17-mutation-tests.md#A37,docs/decision/records/2026-09-17-mutation-tests.md#A38,docs/decision/records/2026-09-17-mutation-tests.md#A52
Scenario: 一覧に載った見逃しは指摘にならず equivalent に数える
  Given "src/a.rs" の3行目が "    if a == b {" で、結果のファイルに "src/a.rs" の3行目の変更の説明が "replace == with != in f" の`見逃し`がある
  And `等価の一覧`に "file" が "src/a.rs"、"change" が "replace == with != in f"、"text" が "if a == b {"、"class" が "equivalent"、"why" が空でない1件がある
  When "kotowari mutants --tool cargo-mutants outcomes.json" を実行する
  Then mutant_survived の誤りは出ず、"mutants" の "survived" は 0、"equivalent" は 1 になる
  And 終了コードは 0 である

@id=EX-212 @about=REQ-141 @source=docs/decision/records/2026-09-17-mutation-tests.md#A11,docs/decision/records/2026-09-17-mutation-tests.md#A15,docs/decision/records/2026-09-17-mutation-tests.md#A31
Scenario: 行が動いただけなら一致したまま
  Given EX-211 の`等価の一覧`があり、"src/a.rs" の "    if a == b {" の行が7行目に動き、結果のファイルの`見逃し`の行も 7 である
  When "kotowari mutants --tool cargo-mutants outcomes.json" を実行する
  Then mutant_survived の誤りは出ない

@id=EX-213 @about=REQ-141,REQ-142 @source=docs/decision/records/2026-09-17-mutation-tests.md#A11,docs/decision/records/2026-09-17-mutation-tests.md#A15,docs/decision/records/2026-09-17-mutation-tests.md#A20,docs/decision/records/2026-09-17-mutation-tests.md#A31,docs/decision/records/2026-09-17-mutation-tests.md#A40
Scenario: 行を書き換えると一致せず、1件は古くなる
  Given EX-211 の`等価の一覧`があり、"src/a.rs" の3行目が "    if a == c {" に変わり、"if a == b {" の行はどこにも無く、結果のファイルに3行目の同じ変更の説明の`見逃し`がある
  When "kotowari mutants --tool cargo-mutants outcomes.json" を実行する
  Then "src/a.rs" の3行目に mutant_survived の誤りが出る
  And "path" が`等価の一覧`のファイルで "line" が null、detail が "src/a.rs: replace == with != in f" の equivalent_stale の注意が出る

@id=EX-214 @about=REQ-141 @source=docs/decision/records/2026-09-17-mutation-tests.md#A11,docs/decision/records/2026-09-17-mutation-tests.md#A31,docs/decision/records/2026-09-17-mutation-tests.md#A33,docs/decision/records/2026-09-17-mutation-tests.md#A40
Scenario: 行がファイルの行数を超える見逃しは一致せずに指摘になる
  Given "src/a.rs" が5行で、結果のファイルに "src/a.rs" の9行目の`見逃し`がある
  When "kotowari mutants --tool cargo-mutants outcomes.json" を実行する
  Then "line" が 9 の mutant_survived の誤りが出る
  And 終了コードは 1 である

@id=EX-215 @about=REQ-143 @source=docs/decision/records/2026-09-17-mutation-tests.md#A11,docs/decision/records/2026-09-17-mutation-tests.md#A16,docs/decision/records/2026-09-17-mutation-tests.md#A31,docs/decision/records/2026-09-17-mutation-tests.md#A34,docs/decision/records/2026-09-17-mutation-tests.md#A40,docs/decision/records/2026-09-17-mutation-tests.md#A53
Scenario: 理由が空白だけの1件は誤りで、見逃しを外さない
  Given EX-211 の場面で、`等価の一覧`の1件の "why" が半角空白だけである
  When "kotowari mutants --tool cargo-mutants outcomes.json" を実行する
  Then detail が "src/a.rs: replace == with != in f" の equivalent_invalid の誤りが出る
  And "src/a.rs" の3行目に mutant_survived の誤りが出る
  And equivalent_stale の注意は出ない

@id=EX-216 @about=REQ-143 @source=docs/decision/records/2026-09-17-mutation-tests.md#A17,docs/decision/records/2026-09-17-mutation-tests.md#A31,docs/decision/records/2026-09-17-mutation-tests.md#A34
Scenario: 等価でない分類は書けない
  Given `等価の一覧`の1件の "class" が "untested" である
  When "kotowari mutants --tool cargo-mutants outcomes.json" を実行する
  Then equivalent_invalid の誤りが出る

@id=EX-217 @about=REQ-148 @source=docs/decision/records/2026-09-17-mutation-tests.md#A34,docs/decision/records/2026-09-17-mutation-tests.md#A36
Scenario: 一覧の指す先が無いと停止する
  Given 設定の "mutants.equivalents" が "docs/equivalents.yaml" で、そのファイルが無い
  When "kotowari mutants --tool cargo-mutants outcomes.json" を実行する
  Then 終了コードは 2 である
  And 標準エラーの1行目は "unreadable file: " で始まる

@id=EX-220 @about=REQ-143 @source=docs/decision/records/2026-09-17-mutation-tests.md#A31,docs/decision/records/2026-09-17-mutation-tests.md#A44
Scenario: file の無い1件は detail の前半が空になる
  Given `等価の一覧`の1件に "file" の鍵が無く、"change" が "replace f with ()" である
  When "kotowari mutants --tool cargo-mutants outcomes.json" を実行する
  Then detail が ": replace f with ()" の equivalent_invalid の誤りが出る

@id=EX-221 @about=REQ-148 @source=docs/decision/records/2026-09-17-mutation-tests.md#A45
Scenario: check は一覧の指す先が無くても止まらない
  Given 設定の "mutants.equivalents" が "docs/equivalents.yaml" で、そのファイルが無い
  When "kotowari check" を実行する
  Then 終了コードは 2 でない

@id=EX-232 @about=REQ-141 @source=docs/decision/records/2026-09-17-mutation-tests.md#A11,docs/decision/records/2026-09-17-mutation-tests.md#A15,docs/decision/records/2026-09-17-mutation-tests.md#A31,docs/decision/records/2026-09-17-mutation-tests.md#A38
Scenario: 同じ文面の行が2つあれば1件がどちらにも効く
  Given "src/a.rs" の3行目と8行目がどちらも "    if a == b {" で、結果のファイルに両方の行の同じ変更の説明の`見逃し`があり、EX-211 の`等価の一覧`がある
  When "kotowari mutants --tool cargo-mutants outcomes.json" を実行する
  Then mutant_survived の誤りは出ず、"mutants" の "equivalent" は 2 になる

@id=EX-233 @about=REQ-141 @source=docs/decision/records/2026-09-17-mutation-tests.md#A11,docs/decision/records/2026-09-17-mutation-tests.md#A15,docs/decision/records/2026-09-17-mutation-tests.md#A31,docs/decision/records/2026-09-17-mutation-tests.md#A37
Scenario: file の違う1件は一致しない
  Given EX-211 の場面で、`等価の一覧`の1件の "file" が "src/b.rs" である
  When "kotowari mutants --tool cargo-mutants outcomes.json" を実行する
  Then "src/a.rs" の3行目に mutant_survived の誤りが出る

@id=EX-234 @about=REQ-141 @source=docs/decision/records/2026-09-17-mutation-tests.md#A15,docs/decision/records/2026-09-17-mutation-tests.md#A51,docs/decision/records/2026-09-17-mutation-tests.md#A52
Scenario: 書き方の違うパスとタブの字下げでも一致する
  Given EX-211 の場面で、`等価の一覧`の1件の "file" が "./src/a.rs" で、"src/a.rs" の3行目の字下げがタブで行の終わりが "\r\n" である
  When "kotowari mutants --tool cargo-mutants outcomes.json" を実行する
  Then mutant_survived の誤りは出ない

@id=EX-235 @about=REQ-141,REQ-142 @source=docs/decision/records/2026-09-17-mutation-tests.md#A11,docs/decision/records/2026-09-17-mutation-tests.md#A20,docs/decision/records/2026-09-17-mutation-tests.md#A31,docs/decision/records/2026-09-17-mutation-tests.md#A48
Scenario: UTF-8 でないソースでは停止せず、一致しない側に倒れる
  Given EX-211 の場面で、"src/a.rs" が UTF-8 でない
  When "kotowari mutants --tool cargo-mutants outcomes.json" を実行する
  Then 終了コードは 1 で、mutant_survived の誤りと equivalent_stale の注意が出る

@id=EX-236 @about=REQ-143 @source=docs/decision/records/2026-09-17-mutation-tests.md#A31,docs/decision/records/2026-09-17-mutation-tests.md#A51
Scenario: 基準のディレクトリの外を指す1件は誤りになる
  Given `等価の一覧`の1件の "file" が "../x/src/a.rs" である
  When "kotowari mutants --tool cargo-mutants outcomes.json" を実行する
  Then equivalent_invalid の誤りが出る

@id=EX-237 @about=REQ-143 @source=docs/decision/records/2026-09-17-mutation-tests.md#A31,docs/decision/records/2026-09-17-mutation-tests.md#A34,docs/decision/records/2026-09-17-mutation-tests.md#A53
Scenario: 形の誤った同じ内容の1件が2つあれば誤りも2件出る
  Given `等価の一覧`に、"why" が空で内容の同じ1件が2つある
  When "kotowari mutants --tool cargo-mutants outcomes.json" を実行する
  Then equivalent_invalid の誤りが2件出る

@id=EX-238 @about=REQ-148 @source=docs/decision/records/2026-09-17-mutation-tests.md#A16,docs/decision/records/2026-09-17-mutation-tests.md#A36,docs/decision/records/2026-09-17-mutation-tests.md#A49
Scenario: 空の一覧は0件として続ける
  Given 設定の "mutants.equivalents" の指す先が0バイトで、結果のファイルに "summary" が "CaughtMutant" の1件がある
  When "kotowari mutants --tool cargo-mutants outcomes.json" を実行する
  Then 終了コードは 0 である

@id=EX-239 @about=REQ-148 @source=docs/decision/records/2026-09-17-mutation-tests.md#A34,docs/decision/records/2026-09-17-mutation-tests.md#A44,docs/decision/records/2026-09-17-mutation-tests.md#A49
Scenario: 最上位が並びでない一覧は一覧のパスを出して停止する
  Given 設定の "mutants.equivalents" が "docs/equivalents.yaml" で、その中身が "file: src/a.rs" の1行である
  When "kotowari mutants --tool cargo-mutants outcomes.json" を実行する
  Then 終了コードは 2 である
  And 標準エラーの1行目は "config error: docs/equivalents.yaml" で始まる

@id=EX-243 @about=REQ-142 @source=docs/decision/records/2026-09-17-mutation-tests.md#A20,docs/decision/records/2026-09-17-mutation-tests.md#A31,docs/decision/records/2026-09-17-mutation-tests.md#A40,docs/decision/records/2026-09-17-mutation-tests.md#A57
Scenario: 古い1件の detail は書かれたままのパスで出る
  Given `等価の一覧`に "file" が "./src/a.rs"、"change" が "replace f with ()" の形の正しい1件があり、その "text" の行は "src/a.rs" に無い
  When "kotowari mutants --tool cargo-mutants outcomes.json" を実行する
  Then detail が "./src/a.rs: replace f with ()" の equivalent_stale の注意が出る
```
