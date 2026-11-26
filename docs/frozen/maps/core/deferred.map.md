# deferred の対応表

元: `docs/ir/core/deferred.md`（日本語は `deferred.ja.md`）。要求を後回しにする宣言と、後回しに関する注意（deferred_with_test、depends_on_deferred）を扱う。まだ作らない決定を「提案」にすることに置き換わり、提案の決定を指すマーカーには何も出さない（`docs/decisions/core/confirmations.md` の C-core-confirm-7、C-core-confirm-8）。後回しへの依存の注意をやめることは、同じ決定の見送った案に以前の決定として書いてある。

この話題は新しい形で役目が無くなるので、書き直した文書は作らない（`docs/decisions/core/form-switch.md` の「新しい版で一度に切り替え、古い形を読む仕組みは作らない」）。「新しい ID」の列に ID があるのは、`docs/decisions/core/` にすでにある確認することの行が同じ振る舞いを持つときだけ。「決定の見出し」の列は、ガイドのマーカーを付け替えるときの、いちばん当たる置き換え先の決定（`docs/decisions/core/` の見出し）。

## 元の ID の表

| 元の ID | 新しい ID | 決定の見出し | 理由 |
|---|---|---|---|
| REQ-core-208 | - | 状態は実装する側が変え、まだ作らない決定は「提案」のまま置く | 消える。後回しの宣言は持たず、まだ作らない決定は「提案」にする（C-core-confirm-7）。見出しは `docs/decisions/core/confirmations.md` のもの |
| REQ-core-209 | - | 状態は実装する側が変え、まだ作らない決定は「提案」のまま置く | 消える。後回しの宣言は持たず、まだ作らない決定は「提案」にする（C-core-confirm-7）。見出しは `docs/decisions/core/confirmations.md` のもの |
| REQ-core-210 | - | 状態は実装する側が変え、まだ作らない決定は「提案」のまま置く | 消える。後回しの宣言は持たず、まだ作らない決定は「提案」にする（C-core-confirm-7）。見出しは `docs/decisions/core/confirmations.md` のもの |
| REQ-core-211 | - | 状態は実装する側が変え、まだ作らない決定は「提案」のまま置く | 消える。提案の決定を指すマーカーには何も出さない（C-core-confirm-8）。deferred_with_test は無くなる。見出しは `docs/decisions/core/confirmations.md` のもの |
| REQ-core-212 | - | 状態は実装する側が変え、まだ作らない決定は「提案」のまま置く | 消える。後回しへの依存の注意は持たない（`confirmations.md` の「状態は実装する側が変え、まだ作らない決定は「提案」のまま置く」 の見送った案に、以前の決定として書いてある）。depends_on_deferred は無くなる |
| REQ-core-213 | - | 状態は実装する側が変え、まだ作らない決定は「提案」のまま置く | 消える。skill の扱いは C-core-confirm-10、C-core-confirm-11 の（読んで確認）の行に置き換わる。見出しは `docs/decisions/core/confirmations.md` のもの |
| EX-core-384 | - | 状態は実装する側が変え、まだ作らない決定は「提案」のまま置く | 消える。後回しの宣言の場面。見出しは `docs/decisions/core/confirmations.md` のもの |
| EX-core-385 | - | 状態は実装する側が変え、まだ作らない決定は「提案」のまま置く | 消える。後回しの宣言の場面。見出しは `docs/decisions/core/confirmations.md` のもの |
| EX-core-386 | - | 状態は実装する側が変え、まだ作らない決定は「提案」のまま置く | 消える。後回しの宣言の場面。見出しは `docs/decisions/core/confirmations.md` のもの |
| EX-core-387 | - | 状態は実装する側が変え、まだ作らない決定は「提案」のまま置く | 消える。後回しの宣言の場面。見出しは `docs/decisions/core/confirmations.md` のもの |
| EX-core-388 | - | 状態は実装する側が変え、まだ作らない決定は「提案」のまま置く | 消える。後回しの宣言の場面。見出しは `docs/decisions/core/confirmations.md` のもの |
| EX-core-389 | - | 状態は実装する側が変え、まだ作らない決定は「提案」のまま置く | 消える。後回しの宣言の場面。見出しは `docs/decisions/core/confirmations.md` のもの |
| EX-core-390 | - | 状態は実装する側が変え、まだ作らない決定は「提案」のまま置く | 消える。後回しの宣言の場面。見出しは `docs/decisions/core/confirmations.md` のもの |
| EX-core-391 | - | 状態は実装する側が変え、まだ作らない決定は「提案」のまま置く | 消える。後回しの宣言の場面。見出しは `docs/decisions/core/confirmations.md` のもの |
| EX-core-392 | - | 状態は実装する側が変え、まだ作らない決定は「提案」のまま置く | 消える。後回しの注意の場面。見出しは `docs/decisions/core/confirmations.md` のもの |
| EX-core-393 | - | 状態は実装する側が変え、まだ作らない決定は「提案」のまま置く | 消える。後回しの注意の場面。見出しは `docs/decisions/core/confirmations.md` のもの |
| EX-core-394 | - | 状態は実装する側が変え、まだ作らない決定は「提案」のまま置く | 消える。後回しの注意の場面。見出しは `docs/decisions/core/confirmations.md` のもの |
| EX-core-395 | - | 状態は実装する側が変え、まだ作らない決定は「提案」のまま置く | 消える。後回しの注意の場面。見出しは `docs/decisions/core/confirmations.md` のもの |
| EX-core-396 | - | 状態は実装する側が変え、まだ作らない決定は「提案」のまま置く | 消える。後回しの注意の場面。見出しは `docs/decisions/core/confirmations.md` のもの |
| EX-core-397 | - | 状態は実装する側が変え、まだ作らない決定は「提案」のまま置く | 消える。後回しの注意の場面。見出しは `docs/decisions/core/confirmations.md` のもの |

## テストごとの表

| テストのファイル | テストの関数名 | 今のマーカーの ID | 付け替えたあとの ID | 理由 |
|---|---|---|---|---|
| tests/step15_guides.rs | req_203_a_document_level_declaration_leads_the_fingerprint_of_its_requirements_only | REQ-core-203, REQ-core-209 | - | この話題の分は消える（マーカーから外す）。ほかの話題の ID も指す: REQ-core-203（guides。対応表あり）。テストを残すかどうかはその話題の対応表で決まる |
| tests/step15_guides.rs | req_203_a_requirement_level_declaration_is_part_of_the_body_it_fingerprints | REQ-core-203, REQ-core-208 | - | この話題の分は消える（マーカーから外す）。ほかの話題の ID も指す: REQ-core-203（guides。対応表あり）。テストを残すかどうかはその話題の対応表で決まる |
| tests/step16_deferred.rs | ex_386_a_document_level_declaration_alone_is_not_a_scope | REQ-core-209, EX-core-386 | - | 古い形のコードと一緒に消す |
| tests/step16_deferred.rs | ex_387_a_second_document_level_declaration_is_a_duplicate_field | REQ-core-209, TBL-core-008, EX-core-387 | - | この話題の分は消える（マーカーから外す）。ほかの話題の ID も指す: TBL-core-008（findings。対応表あり）。テストを残すかどうかはその話題の対応表で決まる |
| tests/step16_deferred.rs | ex_388_a_declaration_in_the_glossary_is_still_an_unknown_field | REQ-core-209, EX-core-388 | - | 古い形のコードと一緒に消す |
| tests/step16_deferred.rs | req_209_a_declaration_in_the_flags_is_still_an_unknown_field | REQ-core-209 | - | 古い形のコードと一緒に消す |
| tests/step16_deferred.rs | ex_390_a_declaration_whose_decision_is_missing_is_a_source_invalid_on_its_line | REQ-core-210, REQ-core-115, REQ-core-046, TBL-core-011, EX-core-390 | - | この話題の分は消える（マーカーから外す）。ほかの話題の ID も指す: REQ-core-115（sources。同じ依頼で対応表を書いた）、REQ-core-046、TBL-core-011（ir-items。同じ依頼で対応表を書いた）。テストを残すかどうかはその話題の対応表で決まる |
| tests/step16_deferred.rs | ex_391_form_errors_and_duplicate_ids_remain_under_a_declaration | REQ-core-208, EX-core-391 | - | 古い形のコードと一緒に消す |
| tests/step16_deferred.rs | req_208_both_declarations_on_one_requirement_raise_nothing | REQ-core-208 | - | 古い形のコードと一緒に消す |
| tests/step16_deferred.rs | req_209_a_declaration_in_a_document_without_requirements_is_still_checked | REQ-core-209, REQ-core-210 | - | 古い形のコードと一緒に消す |
| tests/step16_deferred.rs | req_210_an_empty_declaration_is_a_missing_source_on_its_line | REQ-core-210, TBL-core-008, TBL-core-019 | - | この話題の分は消える（マーカーから外す）。ほかの話題の ID も指す: TBL-core-008（findings。対応表あり）、TBL-core-019（finding-order。対応表あり）。テストを残すかどうかはその話題の対応表で決まる |
| tests/step16_deferred.rs | ex_384_a_requirement_level_declaration_removes_the_missing_test | REQ-core-208, REQ-core-210, REQ-core-085, EX-core-384 | - | この話題の分は消える（マーカーから外す）。ほかの話題の ID も指す: REQ-core-085（coverage。対応表あり）。テストを残すかどうかはその話題の対応表で決まる |
| tests/step16_deferred.rs | ex_385_a_document_level_declaration_defers_every_requirement_of_the_document | REQ-core-208, REQ-core-209, REQ-core-137, EX-core-385 | - | この話題の分は消える（マーカーから外す）。ほかの話題の ID も指す: REQ-core-137（coverage。対応表あり）。テストを残すかどうかはその話題の対応表で決まる |
| tests/step16_deferred.rs | ex_389_an_empty_declaration_still_defers_the_requirement | REQ-core-208, REQ-core-210, EX-core-389 | - | 古い形のコードと一緒に消す |
| tests/step16_deferred.rs | req_208_the_first_of_two_requirements_with_one_id_decides_whether_it_is_deferred | REQ-core-208, REQ-core-085 | - | この話題の分は消える（マーカーから外す）。ほかの話題の ID も指す: REQ-core-085（coverage。対応表あり）。テストを残すかどうかはその話題の対応表で決まる |
| tests/step16_deferred.rs | ex_392_a_mark_on_a_deferred_requirement_is_a_notice_on_its_heading | REQ-core-211, REQ-core-031, TBL-core-009, TBL-core-019, EX-core-392 | - | この話題の分は消える（マーカーから外す）。ほかの話題の ID も指す: REQ-core-031、TBL-core-009（findings。対応表あり）、TBL-core-019（finding-order。対応表あり）。テストを残すかどうかはその話題の対応表で決まる |
| tests/step16_deferred.rs | ex_393_a_mark_on_a_deferred_scenario_is_a_notice_on_its_tag_line | REQ-core-211, TBL-core-019, EX-core-393 | - | この話題の分は消える（マーカーから外す）。ほかの話題の ID も指す: TBL-core-019（finding-order。対応表あり）。テストを残すかどうかはその話題の対応表で決まる |
| tests/step16_deferred.rs | req_211_a_mark_from_a_language_without_a_query_counts | REQ-core-211 | - | 古い形のコードと一緒に消す |
| tests/step16_deferred.rs | req_211_two_marks_on_one_id_are_one_notice | REQ-core-211 | - | 古い形のコードと一緒に消す |
| tests/step16_deferred.rs | req_211_the_notice_goes_to_the_first_of_two_requirements_with_one_id | REQ-core-211 | - | 古い形のコードと一緒に消す |
| tests/step16_deferred.rs | ex_394_a_statement_of_a_requirement_that_is_not_deferred_pointing_at_one_is_a_notice | REQ-core-212, REQ-core-031, TBL-core-009, TBL-core-019, EX-core-394 | - | この話題の分は消える（マーカーから外す）。ほかの話題の ID も指す: REQ-core-031、TBL-core-009（findings。対応表あり）、TBL-core-019（finding-order。対応表あり）。テストを残すかどうかはその話題の対応表で決まる |
| tests/step16_deferred.rs | ex_395_a_scenario_about_deferred_and_other_requirements_is_a_notice_and_needs_a_test | REQ-core-212, REQ-core-137, EX-core-395 | - | この話題の分は消える（マーカーから外す）。ほかの話題の ID も指す: REQ-core-137（coverage。対応表あり）。テストを残すかどうかはその話題の対応表で決まる |
| tests/step16_deferred.rs | ex_396_references_from_deferred_requirements_are_not_notices | REQ-core-212, EX-core-396 | - | 古い形のコードと一緒に消す |
| tests/step16_deferred.rs | ex_397_each_reference_in_a_step_is_one_notice | REQ-core-212, EX-core-397 | - | 古い形のコードと一緒に消す |
| tests/step16_deferred.rs | req_212_a_definition_line_pointing_at_a_deferred_requirement_is_a_notice | REQ-core-212 | - | 古い形のコードと一緒に消す |
| tests/step16_deferred.rs | req_212_a_property_statement_pointing_at_a_deferred_requirement_is_a_notice | REQ-core-212 | - | 古い形のコードと一緒に消す |
| tests/step16_deferred.rs | req_212_references_to_a_deferred_scenario_and_from_the_flags_are_not_notices | REQ-core-212 | - | 古い形のコードと一緒に消す |

## ガイドのマーカー

次のガイドのマーカーが、この話題の ID を指している。付け替え先は元の ID の表の「決定の見出し」の列。見出しの列が「-」の ID や、消える指摘を説明する節は、ガイドの節ごと書き直すか消すことになる。

- `docs/guides/deferred.md` の 5 行: REQ-core-208
- `docs/guides/deferred.md` の 16 行: REQ-core-208, REQ-core-209, REQ-core-210
- `docs/guides/deferred.md` の 58 行: REQ-core-208
- `docs/guides/deferred.md` の 134 行: REQ-core-211, REQ-core-212
- `docs/guides/deferred.md` の 162 行: REQ-core-209, REQ-core-210
- `docs/guides/deferred.ja.md` の 5 行: REQ-core-208
- `docs/guides/deferred.ja.md` の 16 行: REQ-core-208, REQ-core-209, REQ-core-210
- `docs/guides/deferred.ja.md` の 58 行: REQ-core-208
- `docs/guides/deferred.ja.md` の 134 行: REQ-core-211, REQ-core-212
- `docs/guides/deferred.ja.md` の 162 行: REQ-core-209, REQ-core-210
- `docs/guides/findings.ja.md` の 177 行: REQ-core-210
- `docs/guides/findings.ja.md` の 316 行: REQ-core-211, REQ-core-212
- `docs/guides/findings.md` の 177 行: REQ-core-210
- `docs/guides/findings.md` の 316 行: REQ-core-211, REQ-core-212

## 消えない振る舞い（利用者に聞くこと）

無い。この話題の振る舞いで、決定の文書にも意味があり、`docs/decisions/core/` のどこにも決まっていないものは見つからなかった。
