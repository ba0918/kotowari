# ガイドの印

`ガイド`の置き場と読み方、`ガイドの印`の形、`指紋`の取り方、`指紋`の照合で出す`指摘`を扱う。`指紋`を出す "kotowari list" の鍵は list.md の `TBL-core-026`、"guides" の群の出し方は output.md と status.md が扱う。

## Requirements

### REQ-core-198: ガイドを読む

- kind: ubiquitous
- source: docs/decision/records/2026-09-24-doc-marks.md#A1, docs/decision/records/2026-09-24-doc-marks.md#A4, docs/decision/records/2026-09-24-doc-marks.md#A16, docs/decision/records/2026-09-24-doc-marks.md#A22, docs/decision/records/2026-09-24-doc-marks.md#A24, docs/decision/records/2026-09-24-doc-marks.md#A29
- verification: unit

kotowari は常に、"kotowari check" と "kotowari status" で設定の "guides.files" の glob に当たるファイルを`ガイド`として読み、"guides.files" が空の一覧のときは`ガイド`を1つも読まない。glob の読み方、走査、`除外`、読めないファイルと UTF-8 でないファイルと先の無いシンボリックリンクでの`停止`は、"tests.files" と`テストのファイル`のとおりにする（REQ-core-019、REQ-core-079、REQ-core-018、`TBL-core-001`）。何にも当たらない glob は`誤り`にしない。"guides.files" の glob が`IR`や`判断の記録`の置き場のファイルに当たっても`停止`せず、そのファイルを`ガイド`としても読む。

### REQ-core-199: ガイドとテストの置き場の重なり

- kind: event_driven
- source: docs/decision/records/2026-09-24-doc-marks.md#A15, docs/decision/records/2026-09-24-doc-marks.md#A28, docs/decision/records/2026-09-24-doc-marks.md#A36
- verification: unit

"kotowari check" で、1つのファイルが "guides.files" の glob と "tests.files" の glob の両方に当たるとき、kotowari は設定の誤りを理由に`停止`し、詳細に、重なったファイルのうちパスのバイト順で最初の1つの`基準のディレクトリ`からの相対パスに ": matched by both guides.files and tests.files" を続けた文字列を出す。

### REQ-core-200: ガイドの印を読む場所

- kind: ubiquitous
- source: docs/decision/records/2026-09-24-doc-marks.md#A5, docs/decision/records/2026-09-24-doc-marks.md#A9, docs/decision/records/2026-09-24-doc-marks.md#A25, docs/decision/records/2026-09-24-doc-marks.md#A30
- verification: unit

kotowari は常に、`ガイド`を CommonMark として読み、HTML のコメントの "<!--" から "-->" までの中にある "@kotowari[" で始まる並びだけを`ガイドの印`として読む。CommonMark のコードブロック（字下げの形とフェンスの形。`コードブロック`より広い）とコードスパンの中、および "-->" の後の文字（"<!--" で始まる HTML ブロックの同じ行の残りを含む）にある "@kotowari[" は読まない（`除外`）。1つのコメントの中と1行の中の`ガイドの印`はすべて拾う。

### REQ-core-201: ガイドの印の形

- kind: algorithm
- source: docs/decision/records/2026-09-24-doc-marks.md#A5, docs/decision/records/2026-09-24-doc-marks.md#A7, docs/decision/records/2026-09-24-doc-marks.md#A10
- definition: TBL-core-036
- verification: unit

### REQ-core-202: 形の誤ったガイドの印

- kind: event_driven
- source: docs/decision/records/2026-09-24-doc-marks.md#A10, docs/decision/records/2026-09-24-doc-marks.md#A23, docs/decision/records/2026-09-24-doc-marks.md#A31, docs/decision/records/2026-09-24-doc-marks.md#A32
- verification: unit

`ガイドの印`の中が空か区切りだけのとき、同じ行に閉じ括弧が無いとき、区切った1件に ":" が無いとき、":" の前が `ID` の形でない（空を含む）とき、または ":" の後が `TBL-core-036` の`指紋`の形でないとき、kotowari は "path" を`ガイド`、"line" をその`ガイドの印`の始まりの行、detail をその行の文字にして invalid_marker の`誤り`を1つの`ガイドの印`につき1件出し、その`ガイドの印`のどの1件も照合しない。

### REQ-core-203: 指紋の取り方

- kind: ubiquitous
- source: docs/decision/records/2026-09-24-doc-marks.md#A6, docs/decision/records/2026-09-24-doc-marks.md#A7, docs/decision/records/2026-09-24-doc-marks.md#A26, docs/decision/records/2026-09-24-doc-marks.md#A27, docs/decision/records/2026-09-24-doc-marks.md#A33
- verification: unit

kotowari は常に、`項目`と`シナリオ`の`指紋`を、次の行の並びを "\n" でつないだ文字列（最後の行の後には "\n" を付けない）の UTF-8 のバイト列の SHA-256 を16進の小文字で書いた先頭の8文字にする。`項目`は `TBL-core-027` の "body" の行から "- source:" の行を除いた並び、`シナリオ`はステップの行だけの並びである。行の終わりの文字（"\r\n" の "\r" を含む）は行に入れない。見出しの名前、`シナリオ`のタグの行と "Scenario:" の行は`指紋`に入らない。

### REQ-core-204: 古いガイドの印

- kind: event_driven
- source: docs/decision/records/2026-09-24-doc-marks.md#A2, docs/decision/records/2026-09-24-doc-marks.md#A8, docs/decision/records/2026-09-24-doc-marks.md#A11, docs/decision/records/2026-09-24-doc-marks.md#A14, docs/decision/records/2026-09-24-doc-marks.md#A20, docs/decision/records/2026-09-24-doc-marks.md#A12, docs/decision/records/2026-09-24-doc-marks.md#A32, docs/decision/records/2026-09-24-doc-marks.md#A34, docs/decision/records/2026-09-24-doc-marks.md#A31, docs/decision/records/2026-09-24-doc-marks.md#A27
- verification: unit

形の正しい`ガイドの印`の1件について、その `ID` の`項目`か`シナリオ`が`IR`に無いとき、またはその `ID` のどの`項目`と`シナリオ`の`指紋`も1件に書かれた`指紋`と同じでないとき、kotowari は "path" を`ガイド`、"line" をその`ガイドの印`の始まりの行、detail を「`ID`、1件に書かれた`指紋`、今の`指紋`」を1つの半角空白で区切った文字列にして guide_stale の`注意`を1件ごとに出す。今の`指紋`は、`IR`に無いときは "-"、同じ `ID` が2か所以上にあるときは REQ-core-032 の1つ目の`指紋`である。

### REQ-core-205: ガイドを書く場面

- kind: ubiquitous
- source: docs/decision/records/2026-09-24-doc-marks.md#A12, docs/decision/records/2026-09-24-doc-marks.md#A13, docs/decision/records/2026-09-24-doc-marks.md#A18, docs/decision/records/2026-09-24-doc-marks.md#A2, docs/decision/records/2026-09-24-doc-marks.md#A14, docs/decision/records/2026-09-24-doc-marks.md#A37
- verification: review
- how_to_verify: "agent/skills/kotowari/" の skill を読み、`ガイド`を書く場面と見直す場面があること、その場面が`ガイドの印`を置く細かさの目安（1つの`ガイドの印`に `ID` が数個まで）、`ガイドの印`をそれが受け持つ節の見出しの隣に1つ置く決まり、節の内容に対応する`項目`を "kotowari query" で確かめて`ガイドの印`に足す手順、guide_stale の detail の今の`指紋`を`ガイドの印`に書き写す前に節を見直す手順を持つことを確かめる

kotowari の skill は常に、`ガイド`を書く場面と見直す場面を持つ。

## Decision tables

### TBL-core-036: ガイドの印の構文

- source: docs/decision/records/2026-09-24-doc-marks.md#A5, docs/decision/records/2026-09-24-doc-marks.md#A7, docs/decision/records/2026-09-24-doc-marks.md#A10, docs/decision/records/2026-09-24-doc-marks.md#A23, docs/decision/records/2026-09-24-doc-marks.md#A31

| 部分 | 形 |
|---|---|
| 始まり | @kotowari[ |
| 中身 | 1件を "ID:指紋" の形で書き、コンマで区切って並べる。コンマで区切った1件ごとに前後の空白を除き、最初の ":" で `ID` と`指紋`に分け、それぞれの前後の空白を除く。同じ `ID` の1件が2つ以上あっても1件ずつ扱う |
| `指紋` | 16進の小文字（"0" から "9" と "a" から "f"）の8文字 |
| 終わり | 始まりと同じ行の "]" |

## Examples

```gherkin
@id=EX-core-362 @about=REQ-core-198,REQ-core-200,REQ-core-203,REQ-core-204 @source=docs/decision/records/2026-09-24-doc-marks.md#A2,docs/decision/records/2026-09-24-doc-marks.md#A5,docs/decision/records/2026-09-24-doc-marks.md#A6,docs/decision/records/2026-09-24-doc-marks.md#A7,docs/decision/records/2026-09-24-doc-marks.md#A4,docs/decision/records/2026-09-24-doc-marks.md#A17,docs/decision/records/2026-09-24-doc-marks.md#A27
Scenario: 指紋が今の IR と同じなら何も出ない
  Given "guides.files" が "guides/**/*.md" で、"docs/decision/records/r.md" に決定 "A1" があり、"docs/ir/a.md" の "REQ-001" の本文が "- kind: ubiquitous"、"- source: docs/decision/records/r.md#A1"、"- verification: unit"、空の行、"文。" の5行である
  And "guides/a.md" に "<!-- @kotowari[REQ-001:51b1f3da] -->" の行がある
  When "kotowari check --format json" を実行する
  Then "guides/a.md" への`指摘`は出ず、"guides" の "files" は 1、"marks" は 1 である

@id=EX-core-363 @about=REQ-core-204 @source=docs/decision/records/2026-09-24-doc-marks.md#A2,docs/decision/records/2026-09-24-doc-marks.md#A8,docs/decision/records/2026-09-24-doc-marks.md#A14,docs/decision/records/2026-09-24-doc-marks.md#A7,docs/decision/records/2026-09-24-doc-marks.md#A12,docs/decision/records/2026-09-24-doc-marks.md#A32
Scenario: 本文を変えると古い印が注意になり、揃っているかは妨げない
  Given EX-core-362 の`IR`と`ガイド`があり、"REQ-001" の本文の "文。" の行を "別の文。" に変える
  When "kotowari check --format text" を実行する
  Then "guides/a.md" の`ガイドの印`の行に guide_stale の`注意`が1件出て、detail は "REQ-001 51b1f3da " で始まり、その後に "51b1f3da" でない16進の小文字8文字が続く
  And 終了コードは 0 で、ほかに`誤り`と`問題の記録`が無ければ "kotowari status" の "complete" は true である

@id=EX-core-364 @about=REQ-core-203 @source=docs/decision/records/2026-09-24-doc-marks.md#A6,docs/decision/records/2026-09-24-doc-marks.md#A27,docs/decision/records/2026-09-24-doc-marks.md#A17
Scenario: 見出しの名前と出典を変えただけなら古くならない
  Given EX-core-362 の`IR`と`ガイド`があり、"REQ-001" の見出しの名前を変え、"- source:" の行に出典を1つ足す
  When "kotowari check --format json" を実行する
  Then guide_stale の`注意`は出ず、"guides" の "marks" は 1 である

@id=EX-core-365 @about=REQ-core-204 @source=docs/decision/records/2026-09-24-doc-marks.md#A11,docs/decision/records/2026-09-24-doc-marks.md#A14
Scenario: IR から消した項目を指す印は注意になる
  Given "guides/a.md" に "<!-- @kotowari[REQ-009:51b1f3da] -->" の行があり、`IR`に "REQ-009" が無い
  When "kotowari check --format text" を実行する
  Then detail が "REQ-009 51b1f3da -" の guide_stale の`注意`が1件出て、unresolved_reference の`誤り`は出ない

@id=EX-core-366 @about=REQ-core-200 @source=docs/decision/records/2026-09-24-doc-marks.md#A9,docs/decision/records/2026-09-24-doc-marks.md#A17
Scenario: コメントの外とコードの中の並びは読まない
  Given "guides/a.md" に、"```" で始まるコードブロックの中の "<!-- @kotowari[REQ-001] -->" の行と、コメントの外の本文の "@kotowari[REQ-001] の形で書く" の行がある
  When "kotowari check --format json" を実行する
  Then "guides/a.md" への invalid_marker も guide_stale も出ず、"guides" の "marks" は 0 である

@id=EX-core-367 @about=REQ-core-202,TBL-core-036 @source=docs/decision/records/2026-09-24-doc-marks.md#A10,docs/decision/records/2026-09-24-doc-marks.md#A31,docs/decision/records/ir-form.md#出力,docs/decision/records/2026-09-24-doc-marks.md#A32
Scenario: 指紋の無い印と大文字の指紋は形の誤り
  Given "guides/a.md" の3行目が "<!-- @kotowari[REQ-001] -->" で、5行目が "<!-- @kotowari[REQ-001:8C0D7663] -->" である
  When "kotowari check --format text" を実行する
  Then 3行目と5行目に invalid_marker の`誤り`が1件ずつ出て、どちらの行にも guide_stale は出ない
  And 終了コードは 1 である

@id=EX-core-368 @about=REQ-core-199 @source=docs/decision/records/2026-09-24-doc-marks.md#A15,docs/decision/records/2026-09-24-doc-marks.md#A36
Scenario: ガイドとテストの置き場が重なると停止する
  Given "guides.files" が "docs/**/*.md" で、"tests.files" が "**/*" で、"docs/guide.md" がある
  When "kotowari check" を実行する
  Then 終了コードは 2 で、停止の詳細は "docs/guide.md: matched by both guides.files and tests.files" である

@id=EX-core-369 @about=REQ-core-198,REQ-core-206 @source=docs/decision/records/2026-09-24-doc-marks.md#A4,docs/decision/records/2026-09-24-doc-marks.md#A17
Scenario: guides.files が無ければガイドは読まない
  Given `設定ファイル`に "guides" の鍵が無く、"docs/guide.md" に "<!-- @kotowari[REQ-001:00000000] -->" の行がある
  When "kotowari check --format json" を実行する
  Then guide_stale の`注意`は出ず、"guides" の "files" と "marks" はどちらも 0 である

@id=EX-core-370 @about=REQ-core-204 @source=docs/decision/records/2026-09-24-doc-marks.md#A20,docs/decision/records/2026-09-24-doc-marks.md#A17
Scenario: 同じ ID が2か所にあれば、どちらかと一致すれば古くない
  Given "REQ-001" が "docs/ir/a.md" と "docs/ir/b.md" にあり、"docs/ir/a.md" の "REQ-001" の`指紋`は "51b1f3da" でなく、"docs/ir/b.md" の "REQ-001" の`指紋`が "51b1f3da" で、"guides/a.md" に "<!-- @kotowari[REQ-001:51b1f3da] -->" の行がある
  When "kotowari check --format json" を実行する
  Then duplicate_id の`誤り`は出るが、guide_stale の`注意`は出ず、"guides" の "marks" は 1 である

@id=EX-core-371 @about=REQ-core-200,REQ-core-206,TBL-core-036 @source=docs/decision/records/2026-09-24-doc-marks.md#A5,docs/decision/records/2026-09-24-doc-marks.md#A17,docs/decision/records/2026-09-24-doc-marks.md#A2,docs/decision/records/2026-09-24-doc-marks.md#A14
Scenario: 1つの印に並べた2件を数える
  Given EX-core-362 の`IR`に`指紋`が "51b1f3da" でない "EX-001" を足し、"guides/a.md" の`ガイドの印`を "<!-- @kotowari[REQ-001:51b1f3da, EX-001:51b1f3da] -->" にする
  When "kotowari check --format json" を実行する
  Then "guides" の "files" は 1、"marks" は 2 で、guide_stale の`注意`は detail が "EX-001 51b1f3da " で始まる1件だけである

@id=EX-core-372 @about=REQ-core-203,TBL-core-026 @source=docs/decision/records/2026-09-24-doc-marks.md#A6,docs/decision/records/2026-09-24-doc-marks.md#A7,docs/decision/records/2026-09-24-doc-marks.md#A14,docs/decision/records/2026-09-24-doc-marks.md#A27
Scenario: list の1件に指紋が出る
  Given EX-core-362 の`IR`がある
  When "kotowari list --format json" を実行する
  Then "id" が "REQ-001" の1件の "fingerprint" は "51b1f3da" である

@id=EX-core-373 @about=REQ-core-202,TBL-core-036 @source=docs/decision/records/2026-09-24-doc-marks.md#A10,docs/decision/records/2026-09-24-doc-marks.md#A23,docs/decision/records/2026-09-24-doc-marks.md#A31,docs/decision/records/2026-09-24-doc-marks.md#A35
Scenario: 1件だけ形の誤った印はどの1件も照合しない
  Given EX-core-362 の`IR`があり、"guides/a.md" の`ガイドの印`が "<!-- @kotowari[REQ-001:00000000, foo:51b1f3da] -->" である
  When "kotowari check --format json" を実行する
  Then その行に invalid_marker の`誤り`が1件出て、guide_stale の`注意`は出ず、"guides" の "marks" は 0 である

@id=EX-core-374 @about=REQ-core-200,TBL-core-036 @source=docs/decision/records/2026-09-24-doc-marks.md#A23,docs/decision/records/2026-09-24-doc-marks.md#A25,docs/decision/records/2026-09-24-doc-marks.md#A17,docs/decision/records/2026-09-24-doc-marks.md#A35
Scenario: 括弧の内側の空白は許し、コメントの後ろの並びは読まない
  Given EX-core-362 の`IR`があり、"guides/a.md" の3行目が "<!-- @kotowari[ REQ-001 : 51b1f3da ] -->" で、5行目が "<!-- a --> @kotowari[REQ-001:00000000]" である
  When "kotowari check --format json" を実行する
  Then "guides/a.md" への`指摘`は出ず、"guides" の "marks" は 1 である

@id=EX-core-375 @about=REQ-core-203 @source=docs/decision/records/2026-09-24-doc-marks.md#A27
Scenario: シナリオの名前を変えても古くならない
  Given "EX-001" の`シナリオ`を指す`ガイドの印`が今の`指紋`を持ち、"EX-001" の "Scenario:" の行の名前と "@source" のタグだけを変える
  When "kotowari check --format json" を実行する
  Then guide_stale の`注意`は出ない
```
