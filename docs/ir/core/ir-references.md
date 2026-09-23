# シナリオのタグと ID の参照

シナリオのタグの検査、ID の参照切れ、kotowari が見ない形の性質を扱う。

## Requirements

### REQ-core-052: 知らないタグ

- kind: event_driven
- source: docs/decision/records/records.md#A27, docs/decision/records/ir-form.md#検査の種類, docs/decision/records/records.md#A109, docs/decision/records/records.md#A143
- verification: unit

gherkin のブロックの中のタグの行（`シナリオ`に結び付くかを問わない）に "@id"、"@about"、"@source" 以外のタグがあるとき、またはタグの行に "@" で始まらない語があるとき、kotowari はその名前か語を detail にして unknown_tag の`誤り`を出す。

### REQ-core-053: 無いタグ

- kind: event_driven
- source: docs/decision/records/records.md#A42, docs/decision/records/ir-form.md#検査の種類, docs/decision/records/records.md#A97
- verification: unit

`シナリオ`に "@id" か "@about" のタグが無いとき、kotowari は無いタグの名前を detail にして missing_tag の`誤り`を出す。値が空のタグ（"=" の後に何も無い）は、無いタグとして扱う。

### REQ-core-054: 参照切れ

- kind: event_driven
- source: docs/decision/records/records.md#A21, docs/decision/records/records.md#A39, docs/decision/records/records.md#A52, docs/decision/records/records.md#A28, docs/decision/records/ir-form.md#検査の種類, docs/decision/records/records.md#A67, docs/decision/records/records.md#A119, docs/decision/records/records.md#A130, docs/decision/records/records.md#A145, docs/decision/records/records.md#A152, docs/decision/records/2026-09-23-ir-english-tokens.md#A2, docs/decision/records/2026-09-24-multi-language-tests.md#A15
- verification: unit

"- definition:" の行、"@about" のタグ、`問題の記録`の "- related:" の行、`要求`と`性質`の`文`と gherkin のステップの行の中で、二重引用符の外でバッククォートで囲んだ`ID`、`印`（`問い合わせのある言語`でどの`テスト`の`直前のコメントの塊`にも無いものを除く）のいずれかが存在しない`ID`を指すとき、または "- definition:"、"@about"、"- related:" の値が`ID`の形でないとき、kotowari は出現ごとに1件の unresolved_reference の`誤り`を出す。

### REQ-core-055: EARS の型を見ない

- kind: prohibition
- source: docs/decision/records/records.md#A42, docs/decision/records/2026-09-17-check-reach.md#A3, docs/decision/records/2026-09-17-check-reach.md#A4
- verification: unit

kotowari は、`要求`の`文`が EARS の型に沿うかを検査してはならない。

### REQ-core-056: 矛盾の読みの数を見ない

- kind: prohibition
- source: docs/decision/records/records.md#A28, docs/decision/records/2026-09-17-check-reach.md#A3, docs/decision/records/2026-09-17-check-reach.md#A4
- verification: unit

kotowari は、`問題の記録`の矛盾の読みが2つ以上あるかを検査してはならない。

### REQ-core-113: gherkin のブロックの中の行

- kind: event_driven
- source: docs/decision/records/records.md#A109, docs/decision/records/records.md#A132, docs/decision/records/records.md#A133, docs/decision/records/records.md#A155, docs/decision/records/2026-09-24-review3-gaps.md#A1
- verification: unit

gherkin の`コードブロック`の中の行を行頭の空白を除いて見て、タグの行（"@" で始まる）、"Scenario:" の行、ステップの行（"Given"、"When"、"Then"、"And"、"But" に半角空白1つ以上が続く行）、"#" で始まる注釈、空行のいずれでもない行（"Feature:"、"Background:"、"Scenario Outline:"、"Examples:"、データ表の行を含む）があるとき、kotowari は行の文字を detail にして invalid_gherkin_line の`誤り`を出す。同じブロックの中でそれより前に "Scenario:" の行が無いステップの行、および直後が "Scenario:" でないタグの行も、同じ`誤り`を出す。ただし空行と注釈を挟まずに続くステップの行は、最初のステップの行の`誤り`に含める。前に "Scenario:" の行があるステップの行は、間に空行、注釈、誤りの行があってもそのシナリオのステップになる。タグの行は "Scenario:" の直前の行だけを結び付け、間にほかの行があれば結び付けない。

### REQ-core-114: ID の定義と形に合わない @id

- kind: event_driven
- source: docs/decision/records/records.md#A110, docs/decision/records/records.md#A139, docs/decision/records/records.md#A151
- verification: unit

"@id" の値が EX の`ID`の形でないとき、kotowari は値を detail、タグの行を "line" にして invalid_id の`誤り`を出し、"@id" の missing_tag は出さず（"@about" が無いときの missing_tag は出す）、その`シナリオ`の missing_source の detail は "Scenario:" の行の文字にする。存在する`ID`の集合には、形に合う見出しの`ID`と形に合う "@id" の値だけを数える。

### REQ-core-124: ID の形

- kind: ubiquitous
- source: docs/decision/records/records.md#A52, docs/decision/records/2026-09-16-ir-tree.md#A6, docs/decision/records/2026-09-16-ir-tree.md#A11, docs/decision/records/ir-form.md#ID, docs/decision/records/2026-09-22-id-namespace.md#A1, docs/decision/records/2026-09-22-id-namespace.md#A2
- verification: unit

`ID`は常に、"REQ-"、"TBL-"、"PROP-"、"EX-"、"FLAG-" のいずれかに、省いてよい名前と "-" を続け、その後に3桁以上の数字を置いた形で、4桁以上のときは先頭が "0" でない。名前は小文字の英字で始まり、2文字目からは小文字の英数字と "-" だけからなる。`項目`の見出しやタグの形に書く "nnn" はこの数字を表す。この形に合わない見出しは REQ-core-043、合わない "@id" は REQ-core-114 のとおりに扱い、`印`の中の合わない`ID`は`印`の検査のとおりに扱う。

### REQ-core-167: ID の名前と置き場の一致

- kind: event_driven
- source: docs/decision/records/2026-09-22-id-namespace.md#A3, docs/decision/records/2026-09-22-id-namespace.md#A5
- verification: unit

見出しの`ID`か "@id" の値に名前があり、その名前が文書の置き場からの相対パスの第1階層と異なるとき、kotowari は`ID`を detail にして id_domain_mismatch の`誤り`を出す。文書が IR の置き場の直下にあって第1階層を持たないときも、名前があれば同じ`誤り`を出す。

## Examples

```gherkin
@id=EX-core-283 @about=REQ-core-113 @source=docs/decision/records/2026-09-24-review3-gaps.md#A1
Scenario: シナリオの中の空行と注釈はステップを切らない
  Given gherkin のブロックに、"Scenario:" の行の後に空行と注釈を挟んで3つのステップの行を持つ`シナリオ`がある
  When "kotowari check" を実行する
  Then invalid_gherkin_line は出ず、`シナリオ`は3つのステップを持つ

@id=EX-core-009 @about=REQ-core-052 @source=docs/decision/records/records.md#A27,docs/decision/records/ir-form.md#検査の種類
Scenario: やめたタグは誤りになる
  Given `シナリオ`に "@requirement=REQ-001" のタグがある
  When "kotowari check" を実行する
  Then unknown_tag の誤りが1件出る

@id=EX-core-010 @about=REQ-core-054 @source=docs/decision/records/records.md#A52,docs/decision/records/records.md#A39,docs/decision/records/ir-form.md#検査の種類
Scenario: 無い要求を指す about は参照切れになる
  Given `シナリオ`の "@about" が "REQ-999" を指し、"REQ-999" はどこにも無い
  When "kotowari check" を実行する
  Then detail が "REQ-999" の unresolved_reference の誤りが出る

@id=EX-core-028 @about=REQ-core-124 @source=docs/decision/records/2026-09-16-ir-tree.md#A6,docs/decision/records/2026-09-16-ir-tree.md#A11
Scenario: 4桁の ID は通り、先頭が 0 の4桁と2桁以下は通らない
  Given 見出しが "### REQ-1000: 名前"、"### REQ-0001: 名前"、"### REQ-1: 名前" の3つある
  When "kotowari check" を実行する
  Then "REQ-1000" は`要求`として読まれ、"REQ-0001" と "REQ-1" の見出しに unknown_heading の誤りが出る

@id=EX-core-029 @about=REQ-core-124 @source=docs/decision/records/2026-09-16-ir-tree.md#A6,docs/decision/records/2026-09-16-ir-tree.md#A11
Scenario: タグと印の中の ID も同じ形で見る
  Given "@id=EX-1000" のタグと "@id=EX-0001" のタグがあり、テストに "@kotowari[REQ-1000]" の`印`がある
  When "kotowari check" を実行する
  Then "EX-1000" は`シナリオ`の`ID`になり、"EX-0001" のタグに invalid_id の誤りが出て、`印`の "REQ-1000" は`要求`の`ID`として照合される

@id=EX-core-043 @about=REQ-core-124 @source=docs/decision/records/2026-09-22-id-namespace.md#A1,docs/decision/records/2026-09-22-id-namespace.md#A2
Scenario: 名前のある ID と名前の無い ID は両方とも通る
  Given "core/a.md" に見出しが "### REQ-core-001: 名前" と "### REQ-002: 名前" と "### REQ-Core-003: 名前" の3つある
  When "kotowari check" を実行する
  Then "REQ-core-001" と "REQ-002" は`要求`として読まれ、"REQ-Core-003" の見出しに unknown_heading の誤りが出る

@id=EX-core-044 @about=REQ-core-167 @source=docs/decision/records/2026-09-22-id-namespace.md#A3,docs/decision/records/2026-09-22-id-namespace.md#A5
Scenario: 名前が置き場の第1階層と違えば誤りになる
  Given "core/a.md" に "### REQ-schema-001: 名前" の見出しがあり、"b.md" に "### REQ-core-002: 名前" の見出しがある
  When "kotowari check" を実行する
  Then detail が "REQ-schema-001" と "REQ-core-002" の id_domain_mismatch の誤りが2件出る
```
