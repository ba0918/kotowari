# スキルの references と本体の一致

"agent/skills/" の下のスキルの references（"agent/skills/kotowari/references/" と "agent/skills/kotowari-plan/references/" の文書）に写した本体の値や形が本体と一致することの検査を扱う。この検査は kotowari の振る舞いではなく、このリポジトリのテストが行う。突き合わせる相手は本体のコードが持つ値であり、IR の表ではない。

## Requirements

### REQ-core-125: 指摘の種類の一致

- kind: ubiquitous
- source: docs/decision/records/2026-09-17-check-reach.md#A6, docs/decision/records/2026-09-17-check-reach.md#A7, docs/decision/records/2026-09-23-ir-english-tokens.md#A6, docs/decision/records/2026-09-23-skill-distribution.md#A1
- verification: unit

このリポジトリのテストは常に、"agent/skills/kotowari/references/findings.md" のヘッダの1列目が「Kind」の表について、ヘッダと区切りの行を除いた1列目の集合が、本体のコードが出す`指摘`の種類の集合と等しいことを確かめる。

### REQ-core-126: 既定の一致

- kind: ubiquitous
- source: docs/decision/records/2026-09-17-check-reach.md#A6, docs/decision/records/2026-09-17-check-reach.md#A7, docs/decision/records/2026-09-17-check-reach.md#A11, docs/decision/records/2026-09-17-check-reach.md#A27, docs/decision/records/2026-09-17-mutation-tests.md#A59, docs/decision/records/2026-09-23-skill-distribution.md#A1
- verification: unit

このリポジトリのテストは常に、"agent/skills/kotowari/references/config.md" の setup の手順1にある YAML のコードブロックに `TBL-core-004` の既定のある鍵がすべて書かれていて、そのブロックを`設定ファイル`として本体のコードで読んだ結果が本体のコードが持つ設定の既定の値と等しいことを確かめる。

### REQ-core-127: 停止の文言の一致

- kind: ubiquitous
- source: docs/decision/records/2026-09-17-check-reach.md#A6, docs/decision/records/2026-09-17-check-reach.md#A7, docs/decision/records/2026-09-17-check-reach.md#A12, docs/decision/records/2026-09-23-ir-english-tokens.md#A6, docs/decision/records/2026-09-23-skill-distribution.md#A1
- verification: unit

このリポジトリのテストは常に、"agent/skills/kotowari/references/findings.md" のヘッダの1列目が「Message」の表について、ヘッダと区切りの行を除いた1列目の集合が、本体のコードが`停止`の理由として標準エラーの1行目に出す文言の集合と等しいことを確かめる。

### REQ-core-195: 例の計画書の一致

- kind: ubiquitous
- source: docs/decision/records/2026-09-24-plan-schema.md#A13, docs/decision/records/2026-09-23-skill-distribution.md#A1, docs/decision/records/2026-09-24-plan-schema.md#A34
- verification: unit

このリポジトリのテストは常に、"agent/skills/kotowari-plan/references/plan-example.md" を本体のコードで`計画書`として読んだとき、`指摘`が1件も出ないことを確かめる。
