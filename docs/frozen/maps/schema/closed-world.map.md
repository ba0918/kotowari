# closed-world の対応表

元: `docs/ir/schema/closed-world.md`（日本語は `closed-world.ja.md`）。書き直した文書: `closed-world.md`。

## 元の ID の表

| 元の ID | 新しい ID | 決定の見出し | 理由 |
|---|---|---|---|
| REQ-schema-001 | C-schema-closed-1 | 既定は閉じた世界にし、宣言していない見出しと行を指摘にする | テストが確かめている場面（題名を宣言しないスキーマの題名、題名より前の見出し、箇条書きの中の入れ子）を1行にまとめた |
| REQ-schema-002 | C-schema-closed-2 | 開いた世界は、宣言していない構造とその内側だけを許す | スキーマの "open: true" と `--open` の値違いを1行にまとめた。題名より前の見出しを許すこと（mds-spec の A9）と、題名を宣言しないスキーマの題名（review4-gaps の A1）も入れた |
| REQ-schema-003 | C-schema-closed-3 | 開いた世界は、宣言していない構造とその内側だけを許す | そのまま |
| REQ-schema-004 | C-schema-closed-4, C-schema-closed-5 | 開いた世界は、宣言していない構造とその内側だけを許す | 欠落を許さないことを C-4 にした。出現回数と形の違反を許さないことは、開いた世界で確かめるテストが無いので C-4 の文に入れず、決めたことの文と、性質の C-5 に任せた |
| REQ-schema-055 | C-schema-closed-6 | 宣言していない行の指摘に、その行をどう読んだかの種別を持たせる | そのまま。cli の指摘の形の決定からは、この見出しへリンクで指される |
| PROP-schema-001 | C-schema-closed-5 | 開いた世界は、宣言していない構造とその内側だけを許す | 性質の行にした。指すテストが無い（下の「テストの無い行」） |
| EX-schema-001 | C-schema-closed-2 | 開いた世界は、宣言していない構造とその内側だけを許す | そのまま |
| EX-schema-002 | C-schema-closed-3 | 開いた世界は、宣言していない構造とその内側だけを許す | そのまま |

元の文書に TBL は無い。

## テストごとの表

| テストのファイル | テストの関数名 | 今のマーカーの ID | 付け替えたあとの ID | 理由 |
|---|---|---|---|---|
| crates/kotowari-markdown-schema/src/validate/tests.rs | undeclared_heading_is_found | REQ-schema-001, REQ-schema-024 | C-schema-closed-1 | 宣言していない節が undeclared_heading。REQ-schema-024 の分は「文書の骨格」の話題で付け替える |
| crates/kotowari-markdown-schema/src/validate/tests.rs | undeclared_line_is_found | REQ-schema-001 | C-schema-closed-1 | そのまま |
| crates/kotowari-markdown-schema/src/validate/tests.rs | code_block_child_of_list_item_is_undeclared_in_closed_world | REQ-schema-001, REQ-schema-031 | C-schema-closed-1 | 箇条書きの中のコードブロック。REQ-schema-031 の分は「見出しの下の行の規則」の話題で付け替える |
| crates/kotowari-markdown-schema/src/validate/tests.rs | code_block_as_first_child_of_list_item_is_undeclared_in_closed_world | REQ-schema-001, REQ-schema-031 | C-schema-closed-1 | 同上 |
| crates/kotowari-markdown-schema/src/validate/tests.rs | table_as_first_child_of_list_item_is_undeclared_in_closed_world | REQ-schema-001, REQ-schema-031 | C-schema-closed-1 | 箇条書きの中の表。REQ-schema-031 の分は同上 |
| crates/kotowari-markdown-schema/src/validate/tests.rs | table_child_of_list_item_is_undeclared_in_closed_world | REQ-schema-001, REQ-schema-031 | C-schema-closed-1 | 同上 |
| crates/kotowari-markdown-schema/src/validate/tests.rs | paragraph_after_code_block_lead_is_undeclared_line_in_closed_world | REQ-schema-001, REQ-schema-032 | C-schema-closed-1 | 箇条書きの中の宣言していない段落。REQ-schema-032 の分は「見出しの下の行の規則」の話題で付け替える |
| crates/kotowari-markdown-schema/src/validate/tests.rs | paragraph_after_table_lead_is_undeclared_line_in_closed_world | REQ-schema-001, REQ-schema-032 | C-schema-closed-1 | 同上 |
| crates/kotowari-markdown-schema/src/validate/tests.rs | undeclared_child_is_undeclared_even_when_open | REQ-schema-003, REQ-schema-031 | C-schema-closed-3 | 宣言済みの節の箇条書きの下の宣言していない子が、開いた世界でも undeclared_line。REQ-schema-031 の分は同上 |
| crates/kotowari-markdown-schema/src/validate/tests.rs | missing_required_child_field_is_found | REQ-schema-004, REQ-schema-031 | - | 閉じた世界での欠落の検出で、開いた世界で緩めないことは確かめていない。欠落そのものは REQ-schema-031（「見出しの下の行の規則」の話題）の分 |
| crates/kotowari-markdown-schema/src/validate/tests.rs | nested_list_items_are_each_reported_in_closed_world | REQ-schema-001, REQ-schema-031 | C-schema-closed-1 | 入れ子の箇条書きの行がそれぞれ undeclared_line。REQ-schema-031 の分は同上 |
| crates/kotowari-markdown-schema/src/validate/tests.rs | missing_required_section_is_found | REQ-schema-004, REQ-schema-024 | - | 閉じた世界での節の欠落の検出で、開いた世界は確かめていない。REQ-schema-024 の分は「文書の骨格」の話題 |
| crates/kotowari-markdown-schema/src/validate/tests.rs | undeclared_field_name_line_is_undeclared_when_no_bullets_declared | REQ-schema-001, REQ-schema-028 | C-schema-closed-1 | 宣言していない一覧の行が undeclared_line。REQ-schema-028 の分は「見出しの下の行の規則」の話題 |
| crates/kotowari-markdown-schema/src/validate/tests.rs | open_allows_undeclared_heading_and_line_but_not_missing_required | REQ-schema-002, REQ-schema-004 | C-schema-closed-2, C-schema-closed-4 | 開いた世界で宣言していない節と行を許し、必須の節の欠落は指摘する |
| crates/kotowari-markdown-schema/src/validate/tests.rs | title_without_rule_is_undeclared_heading_in_closed_world | REQ-schema-001, REQ-schema-022 | C-schema-closed-1 | 題名を宣言しないスキーマの題名が undeclared_heading。REQ-schema-022 の分は「文書の骨格」の話題 |
| crates/kotowari-markdown-schema/src/validate/tests.rs | title_without_rule_is_allowed_when_open | REQ-schema-002 | C-schema-closed-2 | そのまま |
| crates/kotowari-markdown-schema/src/validate/tests.rs | undeclared_section_lines_are_flagged_in_closed_world | REQ-schema-001 | C-schema-closed-1 | 宣言していない節と項目の見出しと、その内側の行 |
| crates/kotowari-markdown-schema/src/validate/tests.rs | undeclared_preamble_lines_are_flagged_in_closed_world | REQ-schema-001, REQ-schema-023 | C-schema-closed-1 | 宣言していない前置部の行。REQ-schema-023 の分は「文書の骨格」の話題 |
| crates/kotowari-markdown-schema/src/validate/tests.rs | undeclared_preamble_lines_are_allowed_when_open | REQ-schema-002, REQ-schema-023 | C-schema-closed-2 | 同上の開いた世界 |
| crates/kotowari-markdown-schema/src/validate/tests.rs | undeclared_section_lines_are_allowed_when_open | REQ-schema-002 | C-schema-closed-2 | そのまま |
| crates/kotowari-markdown-schema/src/validate/tests.rs | undeclared_item_inside_declared_section_is_flagged_even_when_open | REQ-schema-003, REQ-schema-027 | C-schema-closed-3 | 宣言済みの節の中の宣言していない項目。REQ-schema-027 の分は「文書の骨格」の話題 |
| crates/kotowari-markdown-schema/src/validate/tests.rs | stray_heading_in_declared_preamble_is_undeclared_even_when_open | REQ-schema-003, REQ-schema-023 | C-schema-closed-3 | 宣言済みの前置部の中の見出し。REQ-schema-023 の分は同上 |
| crates/kotowari-markdown-schema/src/validate/tests.rs | stray_heading_lines_in_declared_preamble_are_undeclared_even_when_open | REQ-schema-003, REQ-schema-023 | C-schema-closed-3 | その見出しの内側の行。REQ-schema-023 の分は同上 |
| crates/kotowari-markdown-schema/src/validate/tests.rs | stray_heading_before_title_is_allowed_when_open | REQ-schema-002 | C-schema-closed-2 | 題名より前の見出しとその内側の行 |
| crates/kotowari-markdown-schema/src/validate/tests.rs | stray_heading_before_title_is_undeclared_in_closed_world | REQ-schema-001 | C-schema-closed-1 | 同上の閉じた世界 |
| crates/kotowari-markdown-schema/src/validate/tests.rs | undeclared_line_in_declared_section_is_flagged_even_when_open | REQ-schema-003, EX-schema-002 | C-schema-closed-3 | そのまま |
| crates/kotowari-markdown-schema/src/validate/tests.rs | missing_required_field_is_found | REQ-schema-004 | C-schema-lang-7 | 閉じた世界での必須のフィールド行の欠落の検出で、開いた世界は確かめていない。この話題の行には当たらない。出現回数を書かないフィールド行が無ければ欠落の指摘（missing_required_field）が出ることなので、schema-language の C-schema-lang-7 を付け替え先にした |
| crates/kotowari-markdown-schema/src/validate/tests.rs | missing_statement_is_found | REQ-schema-004, REQ-schema-032 | - | 閉じた世界での欠落の検出で、この話題の行には当たらない。付け替え先（schema-language の C-schema-lang-7）は block-rules の対応表に書いた |
| crates/kotowari-markdown-schema/src/validate/tests.rs | missing_bullets_is_found | REQ-schema-004, REQ-schema-019 | - | 閉じた世界での欠落の検出。付け替え先は schema-language の対応表の C-schema-lang-7 |
| crates/kotowari-markdown-schema/src/validate/tests.rs | undeclared_field_line_carries_the_field_rule_kind | REQ-schema-055 | C-schema-closed-6 | フィールド行 |
| crates/kotowari-markdown-schema/src/validate/tests.rs | undeclared_bullet_carries_the_bullets_rule_kind | REQ-schema-055 | C-schema-closed-6 | 箇条書き |
| crates/kotowari-markdown-schema/src/validate/tests.rs | an_empty_bullet_item_is_an_undeclared_line | REQ-schema-001 | C-schema-closed-1, C-schema-closed-6 | 空の箇条書きの行が undeclared_line で、種別が箇条書き |
| crates/kotowari-markdown-schema/src/validate/tests.rs | an_empty_ordered_list_item_is_an_undeclared_line | REQ-schema-001 | C-schema-closed-1, C-schema-closed-6 | 空の順序付きリストの行が undeclared_line で、種別が順序付きリスト |
| crates/kotowari-markdown-schema/src/validate/tests.rs | undeclared_table_carries_the_table_rule_kind | REQ-schema-055 | C-schema-closed-6 | 表 |
| crates/kotowari-mds/tests/cli.rs | check_open_relaxes_undeclared_headings | REQ-schema-002, EX-schema-001 | C-schema-closed-2 | `check --open` |
| crates/kotowari-mds/tests/cli.rs | check_undeclared_child_under_declared_field_exits_one | REQ-schema-003, REQ-schema-031 | C-schema-closed-1 | `--open` を付けずに検査しているので、確かめているのは閉じた世界の undeclared_line。開いた世界は確かめていない。REQ-schema-031 の分は「見出しの下の行の規則」の話題 |
| crates/kotowari-mds/tests/cli.rs | check_directory_relaxes_a_schema_that_declares_open | REQ-schema-002, REQ-schema-010 | C-schema-closed-2 | ディレクトリの検査でも "open: true" が効く。REQ-schema-010 の分は cli の対応表で「-」にした |
| crates/kotowari-mds/tests/cli.rs | check_json_undeclared_line_has_rule_kind | REQ-schema-055 | C-schema-closed-6 | `check --format json` の出力 |

## 落とした細部

### 安全に関わるもの

無し。

### そのほか

| 要求のどの部分 | 誰が決めたと記録にあるか |
|---|---|
| REQ-schema-031、REQ-schema-032 を指すテストの中身: 箇条書きの中に入れ子にした行、表、コードブロックが、閉じた世界で宣言していない行になること | 子の規則は利用者の A34（mds-spec）で、「見出しの下の行の規則」の話題の決定。この話題では「そのほかの宣言していない行」としてだけ書き、入れ子の中身の扱いは書かなかった |
| テスト: 種別の値の綴り（field、bullets、ordered_list、table など）と、宣言していない行の詳細の文言（"undeclared code block" など） | 元の文書の外（テストの中だけ） |
| REQ-schema-055: 呼ぶ側（kotowari）が、その種別で自分の2つの指摘の種類へ写し分けること | 利用者（2026-09-22-ir-engine.md の A74、推奨を採用）。古い仕様の形での kotowari の事情なので、決めたことには書かず、問題の節に「呼ぶ側が見分けられる」という中身だけを書いた |

## 形に依存する要求

無し。REQ-schema-055 の記録の理由は、kotowari が IR の指摘を写し分けることに依っているが、振る舞い（宣言していない行の指摘が種別を持つ）は mds のエンジンのもので、形が変わっても残る。

## テストの無い行

- C-schema-closed-5（開いた世界の指摘は閉じた世界の指摘に含まれる）: 元の PROP-schema-001 を指すテストが無い。要るのは、生成した文書とスキーマについて、`validate` を開いた世界と閉じた世界で呼び、前者の指摘が後者の指摘にすべて含まれることを確かめる性質のテスト。出現回数と形の違反を開いた世界でも許さないこと（決めたことの文にある）も、このテストで確かめられる。

## 誰が決めたか見分けられなかったもの

- 箇条書きの中に入れ子にした表とコードブロック、表やコードブロックで始まる箇条書きの後の段落が、閉じた世界で宣言していない行になること（REQ-schema-031、REQ-schema-032 を指すテストが確かめている）: それを決めた記録の行は見つからなかった。閉じた世界の既定（A2、利用者）と、子の規則（A34、利用者）からの帰結と読めるので、決めたことには一般の文（宣言していない行は指摘にする）だけを書き、テストは C-schema-closed-1 に付け替えた。

## 理由や見送った案が記録に無かった決定

- 「既定は閉じた世界にし、…」: 見送った案が記録に無い。
- 「開いた世界は、宣言していない構造とその内側だけを許す」: 見送った案が記録に無い。緩める手段を持つことにした理由が記録に無い（A2 の why は閉じた世界の側の理由だけ）。問題の節に「理由は残っていない」と書いた。
- 「宣言していない行の指摘に、…」: 見送った案は記録の rejected には無く、why の中に「生の行の先頭を見て分ける案は判定を2か所に持つことになる」とある。検討して採らなかった案として、見送った案に書いた。

## 分けた所、まとめた所

- REQ-schema-002、REQ-schema-003、REQ-schema-004 と PROP-schema-001 を1つの決定「開いた世界は、宣言していない構造とその内側だけを許す」にまとめた。どれも mds-spec の A2 の「緩めるのは "open" と "--open" の2つで、緩める方向にしか働かない」から来ている。REQ-schema-001 は既定を決める別の決定にした。
- 題名より前の見出しを開いた世界で許すこと（mds-spec の A9、利用者）は、元の文書の出典に引かれていないが、REQ-schema-002 を指すテストが確かめているので、同じ決定の文と C-2 に入れた。

## あとでリンクにする所

無し。前の版で挙げたものは、どれも見出しへのリンクにした。

- 「文書の骨格」の話題: 前置部を宣言しない開いた世界で、最初の節より前の深さ3の見出しを許すこと（元の REQ-schema-061）。「開いた世界は、…」の決めたことの末尾から、その決定の見出しへリンクした。document-structure の側からも「開いた世界は、…」へリンクしている。
- 「抽出と素の構文木」の話題: 抽出が閉じた世界の設定を読まないこと（元の PROP-schema-007）。同じ段落からリンクした。
- 「見出しの下の行の規則」の話題: 「既定は閉じた世界にし、…」の箇条書きの子の行の宣言を、「箇条書きの子の一覧は、…」へのリンクにした。

## 迷った判断と、どう決めたか

- C-schema-closed-6 の「など」を消すとき、決めたことの6つ（フィールド行、箇条書き、順序付きリスト、文、表、コードブロック）を全部挙げると、テストのある4つ（フィールド行、箇条書き、順序付きリスト、表）より行が広がる。行はテストのある4つだけを、どの行がどの種別になるかの形で書いた。

- C-schema-closed-4 を欠落だけにした。決めたこと（A2、利用者）は出現回数と形の違反も緩めないと書いているが、開いた世界でそれを確かめるテストが無い。行に入れるとテストのある行にならないので、性質の C-5 で確かめる形にし、テストの無い行に挙げた。
- REQ-schema-004 の5本のテストは、どれも閉じた世界で欠落を確かめている。開いた世界で緩めないことを確かめていないので「-」にした。この話題の行には付けず、ほかの ID を持たない missing_required_field_is_found は、確かめていることがそのまま当たる schema-language の C-schema-lang-7（出現回数を書かないノードが無ければ欠落の指摘）を付け替え先にした。
- check_undeclared_child_under_declared_field_exits_one は REQ-schema-003 を指しているが、`--open` を付けていない。中身に合わせて C-1 に付け替えた。
- 理由（A2 の why、ir-engine の A74 と A27 の why）は、kotowari の名前と IR の事情を出さず、「呼ぶ側」が詳細の文言や生の行を読み直さずに済む、という中身で書いた。

## 数

- 元: 要求 5、場面 2、決定表 0、性質 1
- 読んだ記録のファイル: 4（2026-09-21-mds-spec.md、2026-09-22-ir-engine.md、2026-09-23-ir-engine-gaps.md、2026-09-24-review4-gaps.md。題名を宣言しないスキーマの題名を閉じた世界で宣言していない見出しにすることは、review4-gaps の A1（利用者）による）
- 出来上がり: 決定 3、確認することの行 6（うち「（読んで確認）」0）
- テスト: 38
