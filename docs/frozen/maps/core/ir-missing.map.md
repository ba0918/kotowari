# ir-missing の対応表

元: `docs/ir/core/ir-missing.md`（日本語は `ir-missing.ja.md`）。項目に必須の行と表が無いときの検査と、gherkin の外の "Scenario:" の行を扱う。決定の形に置き換わる（必須の節の欠けは C-core-docs-6）。

この話題は新しい形で役目が無くなるので、書き直した文書は作らない（`docs/decisions/core/form-switch.md` の「新しい版で一度に切り替え、古い形を読む仕組みは作らない」）。「新しい ID」の列に ID があるのは、`docs/decisions/core/` にすでにある確認することの行が同じ振る舞いを持つときだけ。「決定の見出し」の列は、ガイドのマーカーを付け替えるときの、いちばん当たる置き換え先の決定（`docs/decisions/core/` の見出し）。

## 元の ID の表

| 元の ID | 新しい ID | 決定の見出し | 理由 |
|---|---|---|---|
| REQ-core-098 | - | 1つの決定は、問題・決めたこと・見送った案・確認することの文章で書く | 消える。項目の必須の行と gherkin の外の "Scenario:" の行は無くなる。見出しは `docs/decisions/core/decision-documents.md` のもの |
| REQ-core-099 | - | 1つの決定は、問題・決めたこと・見送った案・確認することの文章で書く | 消える。決定表は「決めたこと」の中の表で、専用の節も ID も持たない。見出しは `docs/decisions/core/decision-documents.md` のもの |
| REQ-core-100 | - | 1つの決定は、問題・決めたこと・見送った案・確認することの文章で書く | 消える。項目の必須の行と gherkin の外の "Scenario:" の行は無くなる。見出しは `docs/decisions/core/decision-documents.md` のもの |

## テストごとの表

| テストのファイル | テストの関数名 | 今のマーカーの ID | 付け替えたあとの ID | 理由 |
|---|---|---|---|---|
| crates/kotowari-core/tests/ir.rs | req_098_missing_field | REQ-core-098 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_098_review_requirement_without_how_to_verify_is_a_missing_field | REQ-core-098, TBL-core-008, EX-core-259 | - | この話題の分は消える（マーカーから外す）。ほかの話題の ID も指す: TBL-core-008（findings。対応表あり）、EX-core-259（status。対応表あり）。テストを残すかどうかはその話題の対応表で決まる |
| crates/kotowari-core/tests/ir.rs | req_099_missing_table | REQ-core-099 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_100_scenario_outside_gherkin_is_ignored | REQ-core-100 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_098_required_lines_are_told_apart_from_empty_values | REQ-core-047, REQ-core-048, REQ-core-049, REQ-core-050, REQ-core-051, REQ-core-098 | - | この話題の分は消える（マーカーから外す）。ほかの話題の ID も指す: REQ-core-047、REQ-core-048、REQ-core-049、REQ-core-050、REQ-core-051（ir-items。同じ依頼で対応表を書いた）。テストを残すかどうかはその話題の対応表で決まる |
| crates/kotowari-core/tests/ir.rs | req_098_flag_without_relation_line_produces_missing_field | REQ-core-098 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_098_tbl_with_valid_source_no_missing_source | REQ-core-098 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_100_scenario_line_under_heading_is_excluded_from_statement | REQ-core-100 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_098_empty_verification_value_is_invalid_not_missing | REQ-core-098 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_098_empty_kind_value_is_unknown_kind_not_missing_field | REQ-core-098 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_098_empty_definition_value_on_algorithm_is_without_definition | REQ-core-098 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_098_empty_value_of_unknown_field_still_reports_unknown_field | REQ-core-098 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_098_flag_entirely_missing_kind_line_produces_missing_field | REQ-core-098 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_098_flag_relation_line_with_only_commas_does_not_report_missing_field | REQ-core-098 | - | 古い形のコードと一緒に消す |
| tests/step12_status.rs | req_165_review_requirement_without_how_to_verify_is_not_complete | REQ-core-165, REQ-core-098, EX-core-259 | - | この話題の分は消える（マーカーから外す）。ほかの話題の ID も指す: REQ-core-165、EX-core-259（status。対応表あり）。テストを残すかどうかはその話題の対応表で決まる |
| tests/step3_sources_terms.rs | req_059_missing_source_for_item_scenario_and_term | REQ-core-059, REQ-core-098 | - | この話題の分は消える（マーカーから外す）。ほかの話題の ID も指す: REQ-core-059（sources。同じ依頼で対応表を書いた）。テストを残すかどうかはその話題の対応表で決まる |

## ガイドのマーカー

次のガイドのマーカーが、この話題の ID を指している。付け替え先は元の ID の表の「決定の見出し」の列。見出しの列が「-」の ID や、消える指摘を説明する節は、ガイドの節ごと書き直すか消すことになる。

- `docs/guides/findings.ja.md` の 136 行: REQ-core-098
- `docs/guides/findings.ja.md` の 157 行: REQ-core-099
- `docs/guides/commands/status.md` の 204 行: REQ-core-098
- `docs/guides/commands/status.ja.md` の 204 行: REQ-core-098
- `docs/guides/findings.md` の 136 行: REQ-core-098
- `docs/guides/findings.md` の 157 行: REQ-core-099

## 消えない振る舞い（利用者に聞くこと）

無い。この話題の振る舞いで、決定の文書にも意味があり、`docs/decisions/core/` のどこにも決まっていないものは見つからなかった。
