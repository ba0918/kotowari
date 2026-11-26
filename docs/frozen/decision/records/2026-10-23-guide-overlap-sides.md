# ガイドの重なりの判定に数える対の側

## Context

[全体像の書き出しでテストのファイルの中身を読まない変更](./2026-10-23-overview-test-contents.md)の整合のフェーズを再び回し、[前の回の決定](./2026-10-23-overview-test-contents-consistency.md#A1)の根拠を読み直した。
前の回は、3つの置き場すべてに当たるファイルでは [REQ-core-199](../../ir/core/guides.md#REQ-core-199) の停止が必ず先に起きるとしたが、測ったのは言語が1つの場だけだった。
言語が2つ以上の場では、"guides.files" の glob に当たっても`先頭の言語`の側が無い側（[REQ-core-338](../../ir/core/translation-pairs.md#REQ-core-338)）を、重なりの判定の前にガイドから外していた。そのため、3つすべてに当たるこの側では元データとテストの重なりの文言が出て、ガイドとテストの glob の両方に当たるこの側でも REQ-core-199 で止まらなかった。
逆に、glob に当たらず、当たったファイルと同じ対にあるためにガイドとして読む側（[REQ-core-337](../../ir/core/translation-pairs.md#REQ-core-337)）は、重なりに数えて止まっていたが、IR に書かれていなかった。

Position: 整合のフェーズで決めた（2026-10-23）。

## Agreements

- A1 [REQ-core-199](../../ir/core/guides.md#REQ-core-199) と [REQ-core-280](../../ir/core/overview-data.md#REQ-core-280) のガイドの重なりは、"guides.files" の glob に当たったファイルに、ガイドとして読む側を合わせて判定する。glob に当たれば`先頭の言語`の側が無い側も数え、当たったファイルと同じ対にあるためにガイドとして読む側は glob に当たらなくても数える。コードの判定をこの集合に改め、IR の REQ-core-199 にこの数え方を書き、REQ-core-280 のガイドの判定は REQ-core-199 のとおりにすると書く
  - why: REQ-core-199 と REQ-core-280 は glob に当たるファイルの重なりで止めると書いており、側が無いことでそれを外す決定は無い（[D3](./2026-10-05-localization.md#D3) の「ほかの検査で読まない」は中身を読む検査のことで、置き場の重なりの判定を除くとは書いていない）。glob に当たる側を外していたのは REQ-core-199 との食い違いなので、コードを IR に合わせる。同じ対にあるためにガイドとして読む側は glob に当たらないので IR の文面では止まらないが、ガイドとしてもテストとしても読まれ、ガイドの印がテストの印として数えられる。重なりで止める理由（[A15](./2026-09-24-doc-marks.md#A15)）はこれを黙って許さないことなので、今の停止を残して IR に書く。こうすると前の回が書いた REQ-core-280 の最後の文（3つすべてに当たるファイルは REQ-core-199 で止まる）は、側の有無に関わらず成り立つ
  - decided_by: 整合のフェーズ（根拠: "languages" を [ja, en] にした使い捨ての場で、3つの glob がすべて "shared/**" に当たり "shared/a.en.md" だけがあるとき、直す前の check、status、overview build はどれも終了コード 2、標準エラーの1行目 "config error: shared/a.en.md: matched by both overview.files and tests.files" だった。"guides.files" と "overview.files" だけが当たるときは、check と status が終了コード 1 で止まらなかった。"tests.files" が "guides/*.en.md"、"guides.files" が "guides/a.md" のときと、"tests.files" が "docs/g/x.md"、"guides.files" が "docs/g/*.ja.md" で対の両方の側があるとき（先頭の言語を en にした場）は、直す前から check がそれぞれ "config error: guides/a.en.md: matched by both guides.files and tests.files" と "config error: docs/g/x.md: matched by both guides.files and tests.files" で止まっていた。直した後は "tests/step21_translations.rs" の `req_core_280_a_side_without_its_first_side_matched_by_all_three_stops_on_the_guides_and_tests_overlap`、`req_core_280_a_side_without_its_first_side_matched_by_overview_and_guides_stops`、`req_core_199_a_side_found_by_name_counts_as_a_guide_in_the_overlap` が、"docs/g/*.ja.md" の場面を除く3つの場面を確かめる（その場面は対の向きが逆なだけで、コードは同じ集合で判定する）。判定は "crates/kotowari/src/guides.rs" の `collect` が作る集合で行い、"crates/kotowari/src/acquisition.rs" の `read_guides_and_overview` と `load_overview` が同じ集合を元データの重なりの判定に渡す）

## Rejected

- A2 側を外す前の、glob に当たったファイルだけで判定する（IR の文面のとおり）
  - why: 同じ対にあるためにガイドとして読む側がテストのファイルと重なっても止まらなくなり、ガイドの印がテストの印として黙って数えられる。A15 の理由に反し、今ある停止を失う
- A3 IR の REQ-core-199 と REQ-core-280 を「ガイドとして読むファイル」の重なりに改め、`先頭の言語`の側が無い側を判定から外す
  - why: IR の部分どうしの食い違いも、IR が実現できないことの再現も無く、IR を改める根拠が無い。外すと、glob の重なりという設定の誤りが translation_missing の誤りの陰に隠れる
