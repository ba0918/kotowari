# revision-link の対応表

元: `docs/ir/core/revision-link.md`（日本語は `revision-link.ja.md`）。判断の記録の superseded_by のリンクの検査を扱う。改めた決定へのリンクは、覆した古い決定を新しい決定の見送った案に書くことに置き換わる。

この話題は新しい形で役目が無くなるので、書き直した文書は作らない（`docs/decisions/core/form-switch.md` の「新しい版で一度に切り替え、古い形を読む仕組みは作らない」）。「新しい ID」の列に ID があるのは、`docs/decisions/core/` にすでにある確認することの行が同じ振る舞いを持つときだけ。「決定の見出し」の列は、ガイドのマーカーを付け替えるときの、いちばん当たる置き換え先の決定（`docs/decisions/core/` の見出し）。

## 元の ID の表

| 元の ID | 新しい ID | 決定の見出し | 理由 |
|---|---|---|---|
| REQ-core-132 | - | 事実が変わったら書き直し、決定を覆すときは古い決定を見送った案に書く | 消える。改めた決定へのリンクは、覆した古い決定を新しい決定の見送った案に書くことに置き換わる。revision_link_invalid は無くなる。見出しは `docs/decisions/core/decision-documents.md` のもの |
| TBL-core-023 | - | 事実が変わったら書き直し、決定を覆すときは古い決定を見送った案に書く | 消える。改めた決定へのリンクは、覆した古い決定を新しい決定の見送った案に書くことに置き換わる。revision_link_invalid は無くなる。見出しは `docs/decisions/core/decision-documents.md` のもの |
| EX-core-106 | - | 事実が変わったら書き直し、決定を覆すときは古い決定を見送った案に書く | 消える。superseded_by のリンクの場面。見出しは `docs/decisions/core/decision-documents.md` のもの |
| EX-core-107 | - | 事実が変わったら書き直し、決定を覆すときは古い決定を見送った案に書く | 消える。superseded_by のリンクの場面。見出しは `docs/decisions/core/decision-documents.md` のもの |
| EX-core-108 | - | 事実が変わったら書き直し、決定を覆すときは古い決定を見送った案に書く | 消える。superseded_by のリンクの場面。見出しは `docs/decisions/core/decision-documents.md` のもの |
| EX-core-109 | - | 事実が変わったら書き直し、決定を覆すときは古い決定を見送った案に書く | 消える。superseded_by のリンクの場面。見出しは `docs/decisions/core/decision-documents.md` のもの |
| EX-core-112 | - | 事実が変わったら書き直し、決定を覆すときは古い決定を見送った案に書く | 消える。superseded_by のリンクの場面。見出しは `docs/decisions/core/decision-documents.md` のもの |
| EX-core-116 | - | 事実が変わったら書き直し、決定を覆すときは古い決定を見送った案に書く | 消える。superseded_by のリンクの場面。見出しは `docs/decisions/core/decision-documents.md` のもの |
| EX-core-117 | - | 事実が変わったら書き直し、決定を覆すときは古い決定を見送った案に書く | 消える。superseded_by のリンクの場面。見出しは `docs/decisions/core/decision-documents.md` のもの |
| EX-core-119 | - | 事実が変わったら書き直し、決定を覆すときは古い決定を見送った案に書く | 消える。superseded_by のリンクの場面。見出しは `docs/decisions/core/decision-documents.md` のもの |

## テストごとの表

| テストのファイル | テストの関数名 | 今のマーカーの ID | 付け替えたあとの ID | 理由 |
|---|---|---|---|---|
| tests/step8_record_form.rs | req_132_superseded_by_without_link_is_invalid | REQ-core-132, TBL-core-023, EX-core-107 | - | 古い形のコードと一緒に消す |
| tests/step8_record_form.rs | req_132_href_outside_the_records_place_is_invalid | REQ-core-132, TBL-core-023, EX-core-108 | - | 古い形のコードと一緒に消す |
| tests/step8_record_form.rs | req_132_number_only_in_undecided_is_invalid | REQ-core-132, TBL-core-023, EX-core-109 | - | 古い形のコードと一緒に消す |
| tests/step8_record_form.rs | req_132_heading_anchor_is_invalid | REQ-core-132, TBL-core-023, EX-core-112 | - | 古い形のコードと一緒に消す |
| tests/step8_record_form.rs | req_132_empty_path_means_the_same_record | REQ-core-132, TBL-core-023, EX-core-117 | - | 古い形のコードと一緒に消す |
| tests/step8_record_form.rs | req_132_target_that_is_not_a_record_is_invalid | REQ-core-132, TBL-core-023, EX-core-119 | - | 古い形のコードと一緒に消す |
| tests/step8_record_form.rs | tbl_023_same_href_twice_on_one_line_yields_two_findings | REQ-core-132, TBL-core-023 | - | 古い形のコードと一緒に消す |
| tests/step8_record_form.rs | tbl_023_unclosed_link_is_skipped_and_falls_to_no_link | REQ-core-132, TBL-core-023 | - | 古い形のコードと一緒に消す |
| tests/step8_record_form.rs | tbl_023_absolute_href_is_outside_the_place | REQ-core-132, TBL-core-023 | - | 古い形のコードと一緒に消す |
| tests/step8_record_form.rs | tbl_023_link_scan_resumes_after_the_closing_paren | TBL-core-023 | - | 古い形のコードと一緒に消す |
| tests/step8_record_form.rs | req_132_record_without_context_resolves_two_links_in_superseded | REQ-core-132, TBL-core-023, EX-core-106 | - | 古い形のコードと一緒に消す |
| tests/step8_record_form.rs | req_132_parent_directory_href_to_superseded_number_passes | REQ-core-132, TBL-core-023, REQ-core-110, EX-core-116 | - | この話題の分は消える（マーカーから外す）。ほかの話題の ID も指す: REQ-core-110（base-directory。対応表あり）。テストを残すかどうかはその話題の対応表で決まる |
| tests/step8_record_form.rs | req_132_a_parent_step_in_the_records_place_is_folded | REQ-core-132, REQ-core-110, EX-core-289 | - | この話題の分は消える（マーカーから外す）。ほかの話題の ID も指す: REQ-core-110、EX-core-289（base-directory。対応表あり）。テストを残すかどうかはその話題の対応表で決まる |
| tests/step8_record_form.rs | req_133_empty_superseded_by_is_not_a_link_finding | REQ-core-133, REQ-core-132 | - | この話題の分は消える（マーカーから外す）。ほかの話題の ID も指す: REQ-core-133（record-form。同じ依頼で対応表を書いた）。テストを残すかどうかはその話題の対応表で決まる |
| tests/step8_record_form.rs | req_135_superseded_by_in_revisions_is_not_read | REQ-core-135, REQ-core-132 | - | この話題の分は消える（マーカーから外す）。ほかの話題の ID も指す: REQ-core-135（record-form。同じ依頼で対応表を書いた）。テストを残すかどうかはその話題の対応表で決まる |
| tests/step8_record_form.rs | req_129_broken_link_in_a_file_without_decision_sections_is_not_checked | REQ-core-129, REQ-core-132 | - | この話題の分は消える（マーカーから外す）。ほかの話題の ID も指す: REQ-core-129（record-form。同じ依頼で対応表を書いた）。テストを残すかどうかはその話題の対応表で決まる |

## ガイドのマーカー

次のガイドのマーカーが、この話題の ID を指している。付け替え先は元の ID の表の「決定の見出し」の列。見出しの列が「-」の ID や、消える指摘を説明する節は、ガイドの節ごと書き直すか消すことになる。

- `docs/guides/findings.ja.md` の 372 行: REQ-core-132
- `docs/guides/findings.md` の 372 行: REQ-core-132

## 消えない振る舞い（利用者に聞くこと）

無い。この話題の振る舞いで、決定の文書にも意味があり、`docs/decisions/core/` のどこにも決まっていないものは見つからなかった。
