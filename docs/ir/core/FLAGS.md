# 問題の記録

IR の読み取りをスキーマに置き換える工程で、まだ埋まっていない決めごとと、置き換えの前の記述が残っている箇所を並べる。

## 問題の記録

### FLAG-core-001: 指摘の対応表の行がまだ無い

- 種類: gap
- 関係: REQ-core-171, TBL-core-029
- 出典: docs/decision/records/2026-09-22-ir-engine.md#A10

TBL-core-029 は対応表が持つ5つの列と、名前が同じで意味が違う3件を定めるだけで、写し先そのものを並べた表はまだ書かれていない。その表は "crates/kotowari-markdown-schema/src/finding.rs" の種類の列挙を読んで作る。壁打ちでは列だけを決め、1行ずつの中身を決めていない。

### FLAG-core-002: 新しい停止の理由の文言が決まっていない

- 種類: gap
- 関係: REQ-core-175, TBL-core-018, TBL-core-020
- 出典: docs/decision/records/2026-09-22-ir-engine.md#A9

REQ-core-175 は停止の理由を1つ足すと定めるが、TBL-core-018 の標準エラーの1行目の文言も、TBL-core-020 の詳細も決まっていない。

### FLAG-core-003: 埋め込むスキーマの抽出の宣言が新しい形になっていない

- 種類: gap
- 関係: REQ-core-168, REQ-schema-048
- 出典: docs/decision/records/2026-09-22-ir-engine.md#A51

".mds/schemas/" の3つの YAML の抽出の宣言 22 件（うち導かれる値を使うのは9件）は書式の並びのままで、REQ-schema-048 の入れ子の形になっていない。書き換えないと置き換えた後の "kotowari check" が通らない。

### FLAG-core-004: docs/ir/schema の文書にスキーマの宣言が残っている

- 種類: contradiction
- 関係: REQ-core-168
- 出典: docs/decision/records/2026-09-22-ir-engine.md#A8

REQ-core-168 は IR の文書がスキーマを宣言しないと定めるが、"docs/ir/schema/" の 10 文書は先頭の frontmatter に宣言を持つ。

### FLAG-core-005: スキルの references に新しい3種類が無い

- 種類: gap
- 関係: REQ-core-174, REQ-core-125
- 出典: docs/decision/records/2026-09-22-ir-engine.md#A39

"skills/kotowari/references/findings.md" の表に unknown_line、unknown_code_block、glossary_title_invalid の行が無く、同じ references の "ir-form.md" と、出典の先である "docs/decision/records/ir-form.md" の除外の列挙には、"## " の見出しの直下で最初の "### " より前の行が残っている。REQ-core-125 が references の1列目とコードが出す種類の集合の一致をテストで固定しているので、コードが3種類を出すようになったときに同じ工程で直す。

### FLAG-core-006: 自前の読み取りを名指しする確かめ方が残っている

- 種類: contradiction
- 関係: REQ-core-169, REQ-core-089, REQ-core-041, REQ-core-120
- 出典: docs/decision/records/2026-09-22-ir-engine.md#A1

REQ-core-089、REQ-core-041、REQ-core-120 の確かめ方は "crates/kotowari-core/src/ir.rs" の parse_document、scope_lines、check_documents、read_utf8_file を名指しする。REQ-core-169 で自前の読み取りをやめると、これらの関数は残らないか役割が変わる。
