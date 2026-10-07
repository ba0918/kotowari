# 新しい依頼の開始と同じ依頼の再開を分ける

## Context

工程の skill の校正で、iterate が cycle の再開表を使う形になった。
既存の指摘ファイルがあるブランチでは、iterate が `first_review_head` を空にし、実装へ進む計画の行を適用外にするため、再開表は整合のフェーズを選ぶ。
新しい依頼の実装を飛ばさず、同じ依頼の再開では実装を繰り返さない入口を明記する。

Position: 整合のフェーズで決めた（2026-10-07）。

## Agreements

- A1 iterate の新しい依頼は判断の後に実装から始め、同じ依頼の再開と「run more」は計画の未実行ステップの行を除いた cycle の再開表で続ける
  - why: [校正の決定 A13](./2026-10-06-premise-and-slimming.md#A13) は既存の振る舞いを失う変更を求めていない。新しい依頼を同じブランチで扱うことと、同じ依頼を途中から続けることを分ければ、依頼の実装を省略せず、再開で重複実行もしない。規則は既存の置換表の1行に置き、再開表を複製しない。工程の入口の規則なので IR は変更しない。
  - decided_by: 整合のフェーズ（根拠: `git diff b89cf15..8db572d -- agent/skills/kotowari-iterate/SKILL.md` と [iterate の The loop](../../../agent/skills/kotowari-iterate/SKILL.md#the-loop) 86〜89、109〜115 行、[cycle の Resuming and running more](../../../agent/skills/kotowari-cycle/SKILL.md#resuming-and-running-more) 193〜203 行を読み合わせた。変更前の、新しい依頼を判断後に実装へ渡す置換が、実装の委譲内容だけの置換に変わっていた。既存の指摘ファイルと空の `first_review_head` を持つ新しい依頼を表に当てると、修正前は step 2、修正後は step 1。同じ依頼の再開は step 2、4、6 の該当行、「run more」は終了したフェーズまたは step 4 に進む。）
