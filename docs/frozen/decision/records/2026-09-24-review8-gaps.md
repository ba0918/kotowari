# 壁打ちの記録: 8回目の外部レビューで見つかった IR の形の仕様の穴

## Context

8回目の外部のレビュー（GPT、2026-09-24。範囲は `ir.rs` の前半と、kotowari が mds に渡す IR のスキーマ）で、3つが見つかった。algorithm の`要求`と`決定表`が文を持ったときの扱い（[TBL-core-011](../../ir/core/ir-items.md#TBL-core-011)）、"### REQ-001:" のように名前の無い見出し（[REQ-core-043](../../ir/core/ir-items.md#REQ-core-043)）、同じ行が2つあるときにどちらの値を読むか（[REQ-core-045](../../ir/core/ir-items.md#REQ-core-045)）である。どれも利用者が「ほぼ答えが決まっているものは推奨で決めてよい」とした範囲で決める。

Position: A1〜A3 を決めた。承認済み（2026-09-24）。

## Agreements

- A1 algorithm の`要求`は文を持たなくてよく、持ってもよい。`決定表`は表を持ち、文を持ってもよい。今の振る舞いを TBL-core-011 に書く
  - why: TBL-core-011 は「algorithm は持たない」「なし（表を持つ）」と書いていたが、このリポジトリの IR には表の意味を説明する文を持つ決定表が既にあり（[TBL-schema-011](../../ir/schema/block-rules.md#TBL-schema-011)、[TBL-core-010](../../ir/core/ir-document.md#TBL-core-010)）、スキーマもそれを通している。説明の文を禁じる理由は無い
  - rejected: 文を持つ algorithm の要求と決定表を誤りにする。今ある IR の説明の文が書けなくなる
  - decided_by: 利用者（推奨を採用）

- A2 "### ID:" の後に名前の無い見出しは "### ID: 名前" の形でないので、unknown_heading を出し、その `ID` を定義に数えない
  - why: 今は名前が空のまま有効な`項目`として読み、何も知らせていなかった。コロンの無い見出しは既に unknown_heading で、形に合わない見出しの `ID` は定義に数えない（[A110（records）](./records.md#A110)）ので、同じ扱いに揃える
  - decided_by: 利用者（推奨を採用）

- A3 見出しの下に同じ知っている行が2つ以上あるときは、1つ目の行の値を読み、2つ目以降は duplicate_field を出すだけで読まない。今の振る舞いを IR に書く
  - why: どちらを読むかで、後の検査（algorithm の定義の有無、文の有無など）の`指摘`が変わるのに、IR が決めていなかった。書いた順に読む1つ目が素直である
  - decided_by: 利用者（推奨を採用）
