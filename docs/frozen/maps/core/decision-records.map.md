# decision-records の対応表

元: `docs/ir/core/decision-records.md`（日本語は `decision-records.ja.md`）。判断の記録と ADR の持ち方を扱う。判断の記録も ADR も持たなくなり、今あるものは凍結した木へ移る。

この話題は新しい形で役目が無くなるので、書き直した文書は作らない（`docs/decisions/core/form-switch.md` の「新しい版で一度に切り替え、古い形を読む仕組みは作らない」）。「新しい ID」の列に ID があるのは、`docs/decisions/core/` にすでにある確認することの行が同じ振る舞いを持つときだけ。「決定の見出し」の列は、ガイドのマーカーを付け替えるときの、いちばん当たる置き換え先の決定（`docs/decisions/core/` の見出し）。

## 元の ID の表

| 元の ID | 新しい ID | 決定の見出し | 理由 |
|---|---|---|---|
| REQ-core-092 | - | 仕様は、話題ごとの、今有効な決定の文書だけで持つ | 消える。判断の記録も ADR も持たない（`decision-documents.md` の「仕様は、話題ごとの、今有効な決定の文書だけで持つ」、`form-switch.md` の「ADR はやめ、今あるものは凍結する」） |
| REQ-core-093 | - | 事実が変わったら書き直し、決定を覆すときは古い決定を見送った案に書く | 消える。壁打ちごとの記録は書かず、決定の文書を直接書き直す（`decision-documents.md` の「仕様は、話題ごとの、今有効な決定の文書だけで持つ」、`decision-documents.md` の「事実が変わったら書き直し、決定を覆すときは古い決定を見送った案に書く」） |
| REQ-core-094 | - | ADR はやめ、今あるものは凍結する | 消える。ADR をやめる。見出しは `docs/decisions/core/form-switch.md` のもの |
| REQ-core-095 | - | ADR はやめ、今あるものは凍結する | 消える。ADR をやめる。見出しは `docs/decisions/core/form-switch.md` のもの |
| REQ-core-096 | - | ADR はやめ、今あるものは凍結する | 消える。ADR をやめる。見出しは `docs/decisions/core/form-switch.md` のもの |
| REQ-core-097 | - | ADR はやめ、今あるものは凍結する | 消える。今ある ADR は消さずに凍結した木へ移す（`form-switch.md` の「ADR はやめ、今あるものは凍結する」、`decision-documents.md` の「決定の文書は docs/decisions に置き、古い形の文書は凍結した木へ移す」） |
| REQ-core-103 | - | ADR はやめ、今あるものは凍結する | 消える。ADR をやめる。見出しは `docs/decisions/core/form-switch.md` のもの |

## テストごとの表

| テストのファイル | テストの関数名 | 今のマーカーの ID | 付け替えたあとの ID | 理由 |
|---|---|---|---|---|
| tests/native_paths.rs | ir_and_decision_source_walkers_still_read_original_byte_named_paths | REQ-core-313, REQ-core-092, EX-core-485 | - | この話題の分は消える（マーカーから外す）。ほかの話題の ID も指す: REQ-core-313、EX-core-485（library-api。対応表あり）。テストを残すかどうかはその話題の対応表で決まる |

## ガイドのマーカー

この話題の ID を指すガイドのマーカーは無い。

## 消えない振る舞い（利用者に聞くこと）

無い。この話題の振る舞いで、決定の文書にも意味があり、`docs/decisions/core/` のどこにも決まっていないものは見つからなかった。
