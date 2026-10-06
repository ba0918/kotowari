# 整合のフェーズの指摘のうち再評価するものを決める

## Context

整合のフェーズの指摘を cycle の指摘のループに乗せた（[A31](./2026-10-06-changes-rethink.md#A31)〜[A34](./2026-10-06-changes-rethink.md#A34)）後の直し全体を、整合のフェーズとして IR と読み合わせた。
IR の [REQ-core-363](../../ir/core/consistency-phase.md#REQ-core-363) は、毎回の実行が前の回までのフェーズの指摘「ごとに」still_present か no_longer_visible を返すと書く。
スキルは人の判断を待つ指摘（human_judgment）を再評価に渡さない。
これは IR より狭い読みで、具体化ではなく食い違いにあたるので、どちらが正しいかをここで決める。

Position: 整合のフェーズの1回の実行で決めた（2026-10-06）。

## Agreements

- A1 毎回の実行が still_present か no_longer_visible を返すのは、前の回までのフェーズの指摘のうち、開いていて直しを待つもの（cycle が渡す見える指摘）に限る。人の判断を待つ指摘は再評価せず、既知の指摘として渡して上げ直させない。IR の REQ-core-363 をこの形に改める。
  - why: IR どうしが矛盾する。[REQ-core-361](../../ir/core/consistency-phase.md#REQ-core-361) は human_judgment の指摘を最後の報告まで開いたままにし、フェーズが人に戻して止まる道を設けないとする。REQ-core-363 の「指摘ごとに」をそのまま当てると、その指摘は開いたままなので still_present が返り、REQ-core-363 自身の止め方（2回続けて still_present）で cycle の進まないときの終わり方（agent/skills/kotowari-cycle/SKILL.md の Endings 3）に入り、人に戻して止まる。no_longer_visible を返せば閉じてしまい、開いたまま残す決まりに反する。どちらの返し方でも REQ-core-361 と両立しないので、REQ-core-363 の書き方が誤り
  - rejected: human_judgment の指摘も毎回再評価する（上のとおり止まるか閉じるかのどちらかになる）
  - decided_by: 整合のフェーズ（根拠: REQ-core-361 と REQ-core-363 の how_to_verify、[A32](./2026-10-06-changes-rethink.md#A32)、agent/skills/kotowari-cycle/SKILL.md の Judgment stays here の突き合わせの規則と Endings 3。直しはコミット 657a371）
- A2 フェーズの新しい指摘のうち human_judgment のものは、oracle.measured を not_applicable とする。
  - why: agent/skills/kotowari-review/references/finding-schema.md の表は human_judgment に not_applicable を求め、agent/skills/kotowari-cycle/references/consistency-phase.md は新しい指摘をすべて fails_now としていた。human_judgment の指摘は根拠と実測で決められなかったものなので、決めた実測が無い。REQ-core-363 はレビューの指摘と同じ形とする
  - decided_by: 整合のフェーズ（根拠: 上の二つのファイルの該当行。直しはコミット 50851ec）
