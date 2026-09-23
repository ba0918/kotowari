# 判断の記録の補足の行の検査

判断の記録の番号の行と補足の行の読み方（すべての判断の記録に適用）と、"## Context" の見出しを持つ判断の記録での補足の行の有無と名前の検査を扱う。superseded_by のリンクの検査は revision-link.md が扱う。

## Requirements

### REQ-core-129: 形の検査の対象

- kind: ubiquitous
- source: docs/decision/records/2026-09-17-record-form.md#A1, docs/decision/records/2026-09-17-record-form.md#A22, docs/decision/records/2026-09-17-record-form.md#A26, docs/decision/records/2026-09-17-record-form.md#A27, docs/decision/records/2026-09-17-record-form.md#A39, docs/decision/records/2026-09-17-record-form.md#A41, docs/decision/records/2026-09-17-decision-log.md#A6, docs/decision/records/records.md#A134
- verification: unit

kotowari は常に、"## " の後を前後の空白を除いて "Context" と完全一致で比べた見出しを持つ`判断の記録`だけに REQ-core-130 と REQ-core-131 を適用し、持たない`判断の記録`には適用しない。REQ-core-132 は "## Context" の有無にかかわらず、すべての`判断の記録`に適用する。`決定の節`の見出しを持たないファイルは`判断の記録`でなく、どちらの検査も受けない。

### REQ-core-130: 必須の補足の行

- kind: algorithm
- source: docs/decision/records/2026-09-17-record-form.md#A2, docs/decision/records/2026-09-17-record-form.md#A6, docs/decision/records/2026-09-17-record-form.md#A7, docs/decision/records/2026-09-17-record-form.md#A13, docs/decision/records/2026-09-17-record-form.md#A32, docs/decision/records/2026-09-17-decision-log.md#A1, docs/decision/records/2026-09-17-decision-log.md#A12
- definition: TBL-core-022
- verification: unit

### REQ-core-131: 知らない名前の補足の行

- kind: event_driven
- source: docs/decision/records/2026-09-17-record-form.md#A1, docs/decision/records/2026-09-17-record-form.md#A6, docs/decision/records/2026-09-17-record-form.md#A7, docs/decision/records/2026-09-17-record-form.md#A12, docs/decision/records/2026-09-17-record-form.md#A14, docs/decision/records/2026-09-17-record-form.md#A27
- verification: unit

TBL-core-022 の節にある`番号の行`の`補足の行`の名前が TBL-core-022 の認める名前のどれでもないとき、kotowari は "line" をその行にし detail をその名前にして record_field_unknown の`誤り`を出す。

### REQ-core-133: 補足の行の名前と値

- kind: ubiquitous
- source: docs/decision/records/2026-09-17-record-form.md#A23, docs/decision/records/2026-09-17-record-form.md#A27, docs/decision/records/2026-09-17-record-form.md#A33, docs/decision/records/2026-09-17-record-form.md#A37, docs/decision/records/2026-09-17-decision-log.md#A2, docs/decision/records/2026-09-17-record-form.md#A1, docs/decision/records/2026-09-17-record-form.md#A12, docs/decision/records/2026-09-17-record-form.md#A35
- verification: unit

kotowari は常に、`補足の行`の名前を "- " の直後から最初の ":" までの文字、値を最初の ":" の後から行末までの前後の空白を除いた文字として読む。名前は1文字以上で空白と ":" を含まず、当たらない行は`補足の行`でない。値が空の`補足の行`は TBL-core-022 の判定で無いものとして数え、名前の検査（REQ-core-131）は値が空でも受ける。値が空の superseded_by の行は TBL-core-023 の判定の対象にしない。"## Context" を持たない`判断の記録`では、値が空の superseded_by の行はどの`指摘`も受けない（REQ-core-132 のほかの判定はすべての`判断の記録`で受ける）。名前の形に当たらない行は`除外`で、その名前は有るものと数えない。値が "not recorded" の行は有るものとして数える。

### REQ-core-134: 補足の行の数を見ない

- kind: prohibition
- source: docs/decision/records/2026-09-17-record-form.md#A15
- verification: unit

kotowari は、同じ名前の`補足の行`が1つの`番号の行`の下に2つ以上あることを`指摘`してはならない。

### REQ-core-135: 判断の記録で読まない行

- kind: ubiquitous
- source: docs/decision/records/2026-09-17-record-form.md#A14, docs/decision/records/2026-09-17-record-form.md#A24, docs/decision/records/2026-09-17-record-form.md#A27, docs/decision/records/2026-09-17-record-form.md#A34, docs/decision/records/2026-09-17-record-form.md#A35, docs/decision/records/records.md#A115, docs/decision/records/2026-09-17-record-form.md#A45
- verification: unit

kotowari は常に、`判断の記録`の TBL-core-022 の表に無い節（Revisions と "## Context" を含む）にある`番号の行`の形の行と`補足の行`の形の行、節の最初の`番号の行`より前にある`補足の行`の形の行、`番号の行`でも`補足の行`でも見出しでもない行（箇条でない本文の行と、最初の "## " の見出しより前の行を含む）、`コードブロック`の中（gherkin を含む。中の "## " の見出しも数えない。閉じられずに文書が終わるときは文書の終わりまでが中で、`指摘`は出さない）を、`除外`として読まない。見出しは節の切り分けと "## Context" の判定のために読み、`指摘`の対象にしない。節の一覧は TBL-core-022 の表の「節」の列で、REQ-core-130 の適用範囲とは別にすべての`判断の記録`で使う。

### REQ-core-136: 記録の読み取りは1つの関数

- kind: ubiquitous
- source: docs/decision/records/2026-09-17-record-form.md#A10, docs/decision/records/2026-09-17-record-form.md#A11, docs/decision/records/2026-09-17-record-form.md#A16
- verification: review
- how_to_verify: 判断の記録を読む関数が1つで（`crates/kotowari-core/src/sources.rs` の parse_records_file、または記録の読み取りのモジュール）、形の検査と出典の判定がその返す構造だけを読むことを確認。`rg "lines\(\)" crates/kotowari-core/src/sources.rs` で、記録のファイルの行を直接読む箇所が読み取り関数の外に無いことを確認

kotowari は常に、`判断の記録`の読み取り（節、`番号の行`、`補足の行`、リンク）を1つの関数で行い、形の検査と`出典`の判定はその関数が返す構造だけを読む。

## Decision tables

### TBL-core-022: 節ごとの必須の補足の行

- source: docs/decision/records/2026-09-17-record-form.md#A2, docs/decision/records/2026-09-17-record-form.md#A6, docs/decision/records/2026-09-17-record-form.md#A7, docs/decision/records/2026-09-17-record-form.md#A12, docs/decision/records/2026-09-17-record-form.md#A13, docs/decision/records/2026-09-17-record-form.md#A14, docs/decision/records/2026-09-17-record-form.md#A27, docs/decision/records/2026-09-17-record-form.md#A32, docs/decision/records/2026-09-17-record-form.md#A33, docs/decision/records/2026-09-17-decision-log.md#A1, docs/decision/records/records.md#A115

認める名前は "why"、"rejected"、"decided_by"、"superseded_by"、"decides"、"related" の6つ。`番号の行`の判定を`補足の行`より先に行い、`番号の行`は`補足の行`と見ない。節の`番号の行`に必須の名前の`補足の行`が無いとき、"line" をその`番号の行`にし detail をその名前にして record_field_missing の`誤り`を出す。表に無い節の行は読まない。節は "## " の見出しで始まり次の "## " の見出しで終わり、"### " の見出しは節を終えない。

| 節 | 必須の名前 |
|---|---|
| Agreements | why |
| Prohibitions | why |
| Delegated | why |
| Rejected | why |
| Undecided | decides |
| Superseded | superseded_by |

## Examples

```gherkin
@id=EX-core-101 @about=REQ-core-130 @source=docs/decision/records/2026-09-17-record-form.md#A1,docs/decision/records/2026-09-17-record-form.md#A2,docs/decision/records/2026-09-17-record-form.md#A7,docs/decision/records/2026-09-17-record-form.md#A27,docs/decision/records/2026-09-17-record-form.md#A32
Scenario: Context を持つ記録で why の無い決定は誤りになる
  Given "## Context" の見出しを持つ判断の記録の Agreements の節に、"- A1 " で始まる行があり、その下に "- why:" の行が無い
  When "kotowari check" を実行する
  Then "line" が "- A1 " の行で detail が "why" の record_field_missing の誤りが出る

@id=EX-core-102 @about=REQ-core-129 @source=docs/decision/records/2026-09-17-record-form.md#A1,docs/decision/records/2026-09-17-record-form.md#A22,docs/decision/records/2026-09-17-record-form.md#A27
Scenario: Context を持たない記録は形の検査を受けない
  Given "## Context" の見出しを持たない判断の記録の Agreements の節に、"- A1 " で始まる行があり、その下に "- why:" の行が無い
  When "kotowari check" を実行する
  Then その記録を指す record_field_missing と record_field_unknown の誤りは出ない

@id=EX-core-103 @about=REQ-core-133 @source=docs/decision/records/2026-09-17-record-form.md#A1,docs/decision/records/2026-09-17-record-form.md#A7,docs/decision/records/2026-09-17-record-form.md#A23,docs/decision/records/2026-09-17-record-form.md#A27,docs/decision/records/2026-09-17-record-form.md#A32,docs/decision/records/2026-09-17-record-form.md#A33,docs/decision/records/2026-09-17-decision-log.md#A2,docs/decision/records/2026-09-17-record-form.md#A2
Scenario: not recorded は通り、空白だけの値は無いと数える
  Given "## Context" の見出しを持つ判断の記録の Agreements の節に、"- A1 " の行の下に "- why: not recorded" があり、"- A2 " の行の下に "- why:   " の行がある
  When "kotowari check" を実行する
  Then "- A1 " の行を指す record_field_missing は出ず、"- A2 " の行を指す detail が "why" の record_field_missing の誤りが出る

@id=EX-core-104 @about=REQ-core-131 @source=docs/decision/records/2026-09-17-record-form.md#A1,docs/decision/records/2026-09-17-record-form.md#A12,docs/decision/records/2026-09-17-record-form.md#A27,docs/decision/records/2026-09-17-record-form.md#A2
Scenario: 知らない名前の補足の行は誤りになる
  Given "## Context" の見出しを持つ判断の記録の Agreements の節に、"- A1 " の行の下に "- why: x" と "- reason: x" の行がある
  When "kotowari check" を実行する
  Then "line" が "- reason: x" の行で detail が "reason" の record_field_unknown の誤りが出て、"- A1 " の行を指す誤りは出ない

@id=EX-core-105 @about=REQ-core-130 @source=docs/decision/records/2026-09-17-record-form.md#A1,docs/decision/records/2026-09-17-record-form.md#A2,docs/decision/records/2026-09-17-record-form.md#A7,docs/decision/records/2026-09-17-record-form.md#A13,docs/decision/records/2026-09-17-record-form.md#A27,docs/decision/records/2026-09-17-record-form.md#A32
Scenario: Superseded の行に superseded_by が無ければ誤りになる
  Given "## Context" の見出しを持つ判断の記録の Superseded の節に、"- A3 " の行があり、その下に "- why: x" だけがある
  When "kotowari check" を実行する
  Then "line" が "- A3 " の行で detail が "superseded_by" の record_field_missing の誤りが出る

@id=EX-core-110 @about=REQ-core-135 @source=docs/decision/records/2026-09-17-record-form.md#A1,docs/decision/records/2026-09-17-record-form.md#A14,docs/decision/records/2026-09-17-record-form.md#A24,docs/decision/records/2026-09-17-record-form.md#A27,docs/decision/records/2026-09-17-record-form.md#A35,docs/decision/records/2026-09-17-record-form.md#A33,docs/decision/records/2026-09-17-record-form.md#A2
Scenario: 表に無い節の番号の行と親の無い補足の行と別の形の行は読まれない
  Given "## Context" の見出しを持つ判断の記録の Revisions の節に "- A21 は A5 を置き換える" の行があり、Agreements の節の最初の "- A1 " の行より前に "- why: x" の行があり、"- A1 " の行の下に "- why: x" と "- (i) x" と "- why : x" と "（なし）" の行がある
  When "kotowari check" を実行する
  Then どの行を指す誤りも出ない

@id=EX-core-111 @about=REQ-core-134 @source=docs/decision/records/2026-09-17-record-form.md#A1,docs/decision/records/2026-09-17-record-form.md#A12,docs/decision/records/2026-09-17-record-form.md#A15,docs/decision/records/2026-09-17-record-form.md#A27
Scenario: 同じ名前の補足の行が2つあっても通る
  Given "## Context" の見出しを持つ判断の記録の Agreements の節の "- A1 " の行の下に "- why: x" の行が2つある
  When "kotowari check" を実行する
  Then "- A1 " の行とその下の行を指す誤りは出ない

@id=EX-core-113 @about=REQ-core-135 @source=docs/decision/records/2026-09-17-record-form.md#A1,docs/decision/records/2026-09-17-record-form.md#A27,docs/decision/records/2026-09-17-record-form.md#A34,docs/decision/records/2026-09-17-record-form.md#A2
Scenario: コードブロックの中の番号の行は読まれない
  Given "## Context" の見出しを持つ判断の記録 "docs/decision/records/x.md" の Agreements の節に、"- A1 " の行とその下の "- why: x" があり、その後のコードブロックの中に "- A9 x" の行がある
  When "kotowari check" を実行する
  Then "- A9 x" の行を指す誤りは出ず、"- A1 " の行を指す record_field_missing も出ない

@id=EX-core-118 @about=REQ-core-058 @source=docs/decision/records/2026-09-17-record-form.md#A34,docs/decision/records/records.md#A38,docs/decision/records/ir-form.md#検査の種類
Scenario: コードブロックの中の番号は出典の先にならない
  Given "decisions.records" が "docs/decision/records" で、判断の記録 "docs/decision/records/x.md" の Agreements の節のコードブロックの中にだけ "- A9 x" の行があり、IR の要求が出典 "docs/decision/records/x.md#A9" を書いている
  When "kotowari check" を実行する
  Then detail が "docs/decision/records/x.md#A9" の source_invalid の誤りが出る

@id=EX-core-114 @about=REQ-core-130 @source=docs/decision/records/2026-09-17-record-form.md#A1,docs/decision/records/2026-09-17-record-form.md#A5,docs/decision/records/2026-09-17-record-form.md#A27,docs/decision/records/2026-09-17-record-form.md#A33,docs/decision/records/2026-09-17-record-form.md#A2
Scenario: 字下げしていない補足の行も決定に付く
  Given "## Context" の見出しを持つ判断の記録の Agreements の節に、"- A1 " の行の次の行に字下げ無しの "- why: x" がある
  When "kotowari check" を実行する
  Then "- A1 " の行を指す record_field_missing は出ない

@id=EX-core-115 @about=REQ-core-130 @source=docs/decision/records/2026-09-17-record-form.md#A1,docs/decision/records/2026-09-17-record-form.md#A27,docs/decision/records/2026-09-17-record-form.md#A33,docs/decision/records/2026-09-17-record-form.md#A2
Scenario: 本文にコロンを含む決定の行は補足の行と見ない
  Given "## Context" の見出しを持つ判断の記録の Agreements の節に、"- A1 定義を機械的にする: 節にある行" の行とその下の "- why: x" がある
  When "kotowari check" を実行する
  Then "- A1 " の行を指す誤りは出ない
```
