# 見出しの下の行の規則

この文書は、見出しや前置部の下に並ぶ行を、フィールド行、文、箇条書き、表、コードブロックのどれとして読むかを扱う。

## 要求

### REQ-schema-028: 一覧の行の読み分け

- 種類: algorithm
- 出典: docs/decision/records/2026-09-21-mds-spec.md#A10
- 定義: TBL-schema-007
- 検証: property

### REQ-schema-029: フィールド行の値の制約

- 種類: ubiquitous
- 出典: docs/decision/records/2026-09-21-mds-spec.md#A10, docs/decision/records/2026-09-21-mds-spec.md#A29
- 検証: unit

mds は常に、`フィールド行`の値に正規表現と許可リストを課し、区切り文字を宣言したときは区切った要素ごとに課す。

### REQ-schema-054: 文と箇条書きの値の制約

- 種類: ubiquitous
- 出典: docs/decision/records/2026-09-21-mds-spec.md#A31, docs/decision/records/2026-09-21-mds-spec.md#A32
- 検証: unit

mds は常に、`文`と`箇条書き`の行に正規表現を課し、`文`には許可リストも課す。`箇条書き`の照合は元のマーカーの行のマーカーを除いた部分にだけ行い、`継続段落`には行わない。

### REQ-schema-030: 継続段落はその行の一部

- 種類: state_driven
- 出典: docs/decision/records/2026-09-21-mds-spec.md#A11, docs/decision/records/2026-09-23-ir-engine-gaps.md#A12, docs/decision/records/2026-09-23-ir-engine-gaps.md#A13
- 検証: unit

`読み方`が "paragraph" である間、mds は`継続段落`を直前の一覧の行の一部として読み、`文`には数えず、`閉じた世界`でも`指摘`にしない。`読み方`が "line" の間は`継続段落`を作らない（TBL-schema-011）。

### REQ-schema-031: 箇条書きの入れ子

- 種類: event_driven
- 出典: docs/decision/records/2026-09-21-mds-spec.md#A2, docs/decision/records/2026-09-21-mds-spec.md#A10, docs/decision/records/2026-09-21-mds-spec.md#A34, docs/decision/records/2026-09-21-mds-spec.md#A67
- 検証: unit

`箇条書き`に子の一覧があるとき、mds は`スキーマ`が宣言した子の規則に照らし、宣言が無ければ子の行を`指摘`にする。子の規則には`フィールド行`と`箇条書き`を宣言でき、子の`箇条書き`はさらに子の規則を持てる。子の行の読み分けは TBL-schema-007 と同じで、特定の名前を特別扱いしない。子の`箇条書き`に`抽出`を宣言した`スキーマ`は`停止`にする。

### REQ-schema-032: 文の数え方

- 種類: state_driven
- 出典: docs/decision/records/2026-09-21-mds-spec.md#A30, docs/decision/records/2026-09-23-ir-engine-gaps.md#A12, docs/decision/records/2026-09-23-ir-engine-gaps.md#A13, docs/decision/records/2026-09-23-ir-engine-gaps.md#A33
- 検証: unit

`読み方`が "paragraph" である間、mds は空行で区切った段落を1つの`文`として数え、引用、水平線、HTML、画像だけの行は`文`に数えず、`閉じた世界`でも`指摘`にしない。`読み方`が "line" の間の数え方は TBL-schema-011 のとおりである。

### REQ-schema-058: 読み方ごとの行の読み分け

- 種類: algorithm
- 出典: docs/decision/records/2026-09-23-ir-engine-gaps.md#A13, docs/decision/records/2026-09-23-ir-engine-gaps.md#A20, docs/decision/records/2026-09-23-ir-engine-gaps.md#A21, docs/decision/records/2026-09-23-ir-engine-gaps.md#A22, docs/decision/records/2026-09-23-ir-engine-gaps.md#A26, docs/decision/records/2026-09-23-ir-engine-gaps.md#A1, docs/decision/records/2026-09-23-ir-engine-gaps.md#A2, docs/decision/records/2026-09-23-ir-engine-gaps.md#A5
- 定義: TBL-schema-011
- 検証: unit

### REQ-schema-033: 表の検査

- 種類: ubiquitous
- 出典: docs/decision/records/2026-09-21-mds-spec.md#A13, docs/decision/records/2026-09-23-ir-engine-gaps.md#A16
- 検証: unit

mds は常に、`表`のヘッダのセル列を宣言したときだけヘッダと列数を照合し、宣言しないときは`表`の有無と`出現回数`だけを見る。列数の照合では、セルが`文書`のヘッダ行より少ないデータ行を`指摘`にする。セルが`文書`のヘッダ行より多いデータ行は、ヘッダのセル列の宣言に依らず、余ったセルを捨てて`指摘`にしない。

### REQ-schema-059: 表の選び方

- 種類: event_driven
- 出典: docs/decision/records/2026-09-23-ir-engine-gaps.md#A27, docs/decision/records/2026-09-23-ir-engine-gaps.md#A28, docs/decision/records/2026-09-23-ir-engine-gaps.md#A12
- 検証: unit

`表`の規則が "header" と一緒に "select: first" を宣言したとき、mds はその規則を置いた`ノード`の中で、ヘッダが宣言と合う最初の`表`だけをその規則の`表`として検査と`抽出`に使い、ほかの`表`はヘッダが合っても合わなくても宣言していない`表`として扱う。"select" を書かない`表`の規則は、ヘッダの合わない`表`を形の違反の`指摘`にする。"header" なしで "select" を書いた`スキーマ`は`停止`にする。

### REQ-schema-034: コードブロックの検査

- 種類: ubiquitous
- 出典: docs/decision/records/2026-09-21-mds-spec.md#A12, docs/decision/records/2026-09-21-mds-spec.md#A35
- 検証: unit

mds は常に、`コードブロック`の言語を宣言したときだけ言語を照合し、行ごとの正規表現を宣言したときは、行頭の空白を除いた空でない行だけを照合する。

### REQ-schema-041: フィールド行の並び順

- 種類: event_driven
- 出典: docs/decision/records/2026-09-21-mds-spec.md#A10, docs/decision/records/2026-09-21-mds-spec.md#A36
- 検証: unit

`フィールド行`の一覧に並び順の強制を宣言したとき、mds は`スキーマ`に書いた順で現れない`フィールド行`を`指摘`にする。宣言しないときの並びは順不同である。

## 決定表

### TBL-schema-007: 一覧の行の読み分け

- 出典: docs/decision/records/2026-09-21-mds-spec.md#A10, docs/decision/records/2026-09-21-mds-spec.md#A33

| 順 | 行の形 | 読み方 |
|---|---|---|
| 1 | マーカーに「名前と値」が続き、名前が`スキーマ`の宣言と一致する | `フィールド行` |
| 2 | マーカーに続くが、1 に当たらない | `箇条書き` |
| 3 | 数字と区切りの点で始まる | どの規則種別にも属さず、`閉じた世界`では`指摘` |

### TBL-schema-011: 読み方ごとの行の読み分け

- 出典: docs/decision/records/2026-09-23-ir-engine-gaps.md#A1, docs/decision/records/2026-09-23-ir-engine-gaps.md#A2, docs/decision/records/2026-09-23-ir-engine-gaps.md#A5, docs/decision/records/2026-09-23-ir-engine-gaps.md#A13, docs/decision/records/2026-09-23-ir-engine-gaps.md#A21, docs/decision/records/2026-09-23-ir-engine-gaps.md#A22, docs/decision/records/2026-09-23-ir-engine-gaps.md#A26, docs/decision/records/2026-09-21-mds-spec.md#A11, docs/decision/records/2026-09-21-mds-spec.md#A30, docs/decision/records/2026-09-23-ir-engine-gaps.md#A29, docs/decision/records/2026-09-23-ir-engine-gaps.md#A35, docs/decision/records/2026-09-23-ir-engine-gaps.md#A33, docs/decision/records/2026-09-23-ir-engine-gaps.md#A38, docs/decision/records/2026-09-23-ir-engine-gaps.md#A12

見出しと`前置部`の下の行を、`読み方`ごとにどう読むかを決める。どちらの`読み方`でも、一覧の行そのものの読み分けは TBL-schema-007、子の一覧は REQ-schema-031 のとおりである。

| 行の形 | "paragraph" | "line" |
|---|---|---|
| 1つ以上の "#" の直後に空白が続く行 | 見出し | 見出し |
| それ以外の "#" で始まる行 | CommonMark のとおり | `文` |
| `文`の次の、"===" か "---" だけの行 | 前の行とあわせて1つの見出し | 前の行も、その行も`文` |
| 空行で区切らずに続く、一覧でも`表`でもない複数の行 | まとめて1つの`文` | 1行ずつ`文` |
| 一覧の行の直後に空行なしで続く、一覧の行でない行 | 直前の一覧の行の一部（`フィールド行`なら値に入る） | `文` |
| 一覧の行の後に空行を挟み、字下げして続く、一覧の行でない行 | `継続段落` | `文` |
| 一覧の行の後に字下げして続く一覧の行 | 一覧の行 | 一覧の行 |
| 引用、水平線、HTML、画像の行 | `文`に数えず、`指摘`にもしない | `文` |
| 一覧の行の後でない所で、空行の後に4つ以上の空白で字下げした行 | 字下げの`コードブロック` | `文` |
| "```" か "~~~" のフェンスで囲んだブロック | `コードブロック` | `コードブロック` |
| 区切りの行を持つ、縦棒で始まる行の並び | `表` | `表` |
| 区切りの行を持たず`表`にならない、縦棒で始まる行 | `文` | `文` |

## 性質

### PROP-schema-006: マーカーの種類は読み分けを変えない

- 出典: docs/decision/records/2026-09-21-mds-spec.md#A10

一覧のマーカーが "-"、"*"、"+" のどれであっても、同じ行は同じ`規則種別`として読まれる。

## 具体例

```gherkin
@id=EX-schema-011 @about=REQ-schema-028 @source=docs/decision/records/2026-09-21-mds-spec.md#A10
Scenario: 宣言していない名前の行は箇条書きとして読む
  Given `フィールド行`の名前を宣言した`スキーマ`がある
  When 宣言していない名前の「名前と値」の行を持つ`文書`を検査する
  Then その行は`箇条書き`として読まれる

@id=EX-schema-012 @about=REQ-schema-033 @source=docs/decision/records/2026-09-21-mds-spec.md#A13
Scenario: ヘッダを宣言しない表はどのヘッダでも通る
  Given ヘッダのセル列を宣言しない`表`の規則を持つ`スキーマ`がある
  When 任意のヘッダを持つ`表`の`文書`で "mds check" を実行する
  Then ヘッダの`指摘`は出ない

@id=EX-schema-031 @about=REQ-schema-058,TBL-schema-011 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A1,docs/decision/records/2026-09-23-ir-engine-gaps.md#A13
Scenario: 行で読むとフィールド行の直後の行は文になる
  Given "reading: line" を宣言し、`項目`に`フィールド行`と`文`の`抽出`を宣言した`スキーマ`がある
  And `フィールド行`の次の行に空行を挟まずに一覧でない行を持つ`項目`の`文書`がある
  When "mds values --format json" を実行する
  Then `フィールド行`の値はその行の値だけで、次の行を含まない
  And 次の行は`文`の要素として出る

@id=EX-schema-032 @about=REQ-schema-058,TBL-schema-011 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A2,docs/decision/records/2026-09-23-ir-engine-gaps.md#A5,docs/decision/records/2026-09-23-ir-engine-gaps.md#A13
Scenario: 行で読むと空行の後の字下げした行と引用の行は文になる
  Given "reading: line" を宣言し、`文`を宣言しない`項目`を持つ`スキーマ`がある
  And その`項目`に、一覧の行の後に空行を挟んで字下げした行と、引用の行と、HTML の行と、画像だけの行を持つ`文書`がある
  When "mds check --format json" を実行する
  Then 4つの行のそれぞれに、宣言していない行の種別が`文`の`指摘`が1件ずつ出る

@id=EX-schema-033 @about=REQ-schema-058,TBL-schema-011 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A21,docs/decision/records/2026-09-23-ir-engine-gaps.md#A22
Scenario: 行で読むと下線の見出しと字下げのコードブロックは作らない
  Given "reading: line" を宣言し、`項目`の`文`に行番号の`導かれる値`を宣言した`スキーマ`がある
  And `項目`に、`文`の次の "---" だけの行と、空行の後に4つの空白で字下げした行を持つ`文書`がある
  When "mds values --format json" を実行する
  Then "---" の行とその前の行と字下げした行は、どれも`文`の要素として出る
  And 見出しと`コードブロック`の`指摘`は出ない

@id=EX-schema-034 @about=REQ-schema-058,TBL-schema-011 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A26
Scenario: 表にならない縦棒の行はどちらの読み方でも文になる
  Given `文`を宣言しない`項目`を持つ`スキーマ`を、"reading: paragraph" と "reading: line" の2通り用意する
  And その`項目`に、区切りの行を持たない "| a | b |" の行を持つ`文書`がある
  When それぞれの`スキーマ`で "mds check --format json" を実行する
  Then どちらでも、その行に宣言していない行の種別が`文`の`指摘`が1件出る

@id=EX-schema-035 @about=REQ-schema-030,REQ-schema-032 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A12,docs/decision/records/2026-09-21-mds-spec.md#A11,docs/decision/records/2026-09-21-mds-spec.md#A30
Scenario: 段落で読むと続く行は一覧の行に入り、引用の行は文に数えない
  Given "reading" を書かず、`文`を宣言しない`項目`を持つ`スキーマ`がある
  And その`項目`に、`フィールド行`の次に空行を挟まず続く行と、空行を挟んで字下げした`継続段落`と、引用の行を持つ`文書`がある
  When "mds check --format json" を実行する
  Then `指摘`は1件も出ない

@id=EX-schema-036 @about=REQ-schema-059 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A27,docs/decision/records/2026-09-23-ir-engine-gaps.md#A28
Scenario: select first はヘッダの合う最初の表だけを使う
  Given `前置部`の`表`に "header" と "select: first" と`抽出`を宣言した`スキーマ`がある
  And ヘッダの合わない`表`、ヘッダの合う`表`、ヘッダの合う2つ目の`表`を、それぞれ空行を挟んでこの順に`前置部`に持つ`文書`がある
  When "mds check --format json" と "mds values --format json" を実行する
  Then `抽出`の値は2つ目に現れた`表`のデータ行だけになる
  And 1つ目と3つ目の`表`には、宣言していない行の種別が`表`の`指摘`が出て、ヘッダの形の違反の`指摘`は出ない

@id=EX-schema-037 @about=REQ-schema-059 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A27,docs/decision/records/2026-09-23-ir-engine-gaps.md#A28
Scenario: select を書かない表の規則はヘッダの合わない表を指摘にする
  Given `前置部`の`表`に "header" だけを宣言した`スキーマ`と、"header" を書かずに "select: first" を宣言した`スキーマ`がある
  And ヘッダの合わない`表`を`前置部`に持つ`文書`がある
  When それぞれの`スキーマ`で "mds check" を実行する
  Then 前者ではヘッダの形の違反の`指摘`が出る
  And 後者では終了コードは 2 である

@id=EX-schema-038 @about=REQ-schema-033 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A16
Scenario: ヘッダより多いセルは捨て、少ないセルは指摘にする
  Given 3つのセルのヘッダを宣言した`表`の規則を持つ`スキーマ`がある
  And 4つのセルのデータ行と、2つのセルのデータ行を持つ`表`の`文書`がある
  When "mds check --format json" と "mds values --format json" を実行する
  Then 4つのセルの行には`指摘`が出ず、その行の`抽出`の値は先頭の3つのセルだけを持つ
  And 2つのセルの行には`指摘`が1件出る

@id=EX-schema-039 @about=REQ-schema-058,TBL-schema-011 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A13,docs/decision/records/2026-09-23-ir-engine-gaps.md#A35
Scenario: 行で読んでも字下げした一覧の行は一覧の行のまま
  Given "reading: line" を宣言し、`項目`に`フィールド行`だけを宣言した`スキーマ`がある
  And `フィールド行`の次の行に、字下げした "  - b" の行を持つ`項目`の`文書`がある
  When "mds check --format json" を実行する
  Then その行には、宣言していない行の種別が`箇条書き`の`指摘`が出て、種別が`文`の`指摘`は出ない
```
