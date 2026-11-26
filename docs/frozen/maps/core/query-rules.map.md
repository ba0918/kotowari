# query-rules の対応表

元: `docs/ir/core/query-rules.md`（日本語は `query-rules.ja.md`）。書き直した文書: `query-rules.md`。

## 元の ID の表

| 元の ID | 新しい ID | 決定の見出し | 理由 |
|---|---|---|---|
| REQ-core-121 | C-core-qrules-1, C-core-qrules-2 | 設定の tests.rules で問い合わせを足し、同梱の問い合わせは外せない | 足したルールが効くことを 1、同梱が残ることを 2 にした |
| REQ-core-186 | C-core-qrules-3, C-core-qrules-4, C-core-qrules-5, C-core-qrules-6 | ルールのファイルは、ast-grep のルールの YAML をそのまま読む | "language" の突き合わせ（元は REQ-core-189 の2文目）と、TypeScript と Tsx を分けること（記録の A21）も、ルールの形の読み方なのでここに入れた |
| REQ-core-187 | C-core-qrules-7 | ルールの files と ignores は、ast-grep と同じ読み方で守る | そのまま |
| REQ-core-188 | C-core-qrules-8 | ルールの fix、message、severity、note、metadata は使わない | そのまま |
| REQ-core-189 | C-core-qrules-9, C-core-qrules-10, C-core-qrules-11, C-core-qrules-5 | ルールのファイルに誤りがあれば、設定の誤りとして停止する | 停止の場面を 9、詳細のパスを 10、知らない言語の説明を 11 にした。"language" の突き合わせの文は C-5（別の決定）に移した |
| EX-core-311 | C-core-qrules-1 | 設定の tests.rules で問い合わせを足し、同梱の問い合わせは外せない | そのまま |
| EX-core-312 | C-core-qrules-2 | 設定の tests.rules で問い合わせを足し、同梱の問い合わせは外せない | 同じ節を1つと数えることは test-discovery の C-core-discover-12 |
| EX-core-313 | C-core-qrules-7 | ルールの files と ignores は、ast-grep と同じ読み方で守る | そのまま |
| EX-core-314 | C-core-qrules-8 | ルールの fix、message、severity、note、metadata は使わない | そのまま |
| EX-core-315 | C-core-qrules-9 | ルールのファイルに誤りがあれば、設定の誤りとして停止する | 値違いの場面として C-9 にまとめた |
| EX-core-316 | C-core-qrules-9 | ルールのファイルに誤りがあれば、設定の誤りとして停止する | 同上 |
| EX-core-321 | C-core-qrules-9 | ルールのファイルに誤りがあれば、設定の誤りとして停止する | 同上 |
| EX-core-322 | C-core-qrules-5 | ルールのファイルは、ast-grep のルールの YAML をそのまま読む | そのまま |
| EX-core-378 | C-core-qrules-10 | ルールのファイルに誤りがあれば、設定の誤りとして停止する | "config error: " で始まることは停止の話題（TBL-core-020）の文言 |
| EX-core-379 | C-core-qrules-11 | ルールのファイルに誤りがあれば、設定の誤りとして停止する | そのまま |

元の文書に TBL と PROP は無い。

## テストごとの表

| テストのファイル | テストの関数名 | 今のマーカーの ID | 付け替えたあとの ID | 理由 |
|---|---|---|---|---|
| tests/step13_test_languages.rs | ex_core_311_added_rule_counts_tests | REQ-core-186, EX-core-311 | C-core-qrules-1 | そのまま |
| tests/step13_test_languages.rs | req_186_rules_separated_by_dashes_all_apply_and_give_a_language_a_query | REQ-core-186 | C-core-qrules-4 | そのまま |
| tests/step13_test_languages.rs | ex_core_312_rule_hitting_the_bundled_node_counts_once | REQ-core-121, REQ-core-181, EX-core-312 | C-core-qrules-2 | この話題の分（同梱と設定の両方があっても動く）だけ。REQ-core-181 の分は test-discovery の対応表に書いた |
| tests/step13_test_languages.rs | ex_core_313_rule_is_not_applied_outside_its_files | REQ-core-187, EX-core-313 | C-core-qrules-7 | そのまま |
| tests/step13_test_languages.rs | req_187_rule_is_not_applied_to_its_ignores | REQ-core-187 | C-core-qrules-7 | そのまま |
| tests/step13_test_languages.rs | ex_core_314_rule_with_severity_off_is_applied | REQ-core-188, EX-core-314 | C-core-qrules-8 | そのまま |
| tests/step13_test_languages.rs | req_188_fix_message_note_and_metadata_do_not_change_the_tests | REQ-core-188 | C-core-qrules-8 | そのまま |
| tests/step13_test_languages.rs | ex_core_315_missing_rule_file_stops | REQ-core-189, EX-core-315 | C-core-qrules-9 | そのまま |
| tests/step13_test_languages.rs | ex_core_316_rule_of_an_unknown_language_stops | REQ-core-189, EX-core-316 | C-core-qrules-9 | そのまま |
| tests/step13_test_languages.rs | ex_core_378_a_missing_rule_file_stop_points_at_the_rule_file | REQ-core-189, TBL-core-020, EX-core-378 | C-core-qrules-10 | この話題の分だけ。TBL-core-020 は停止の話題 |
| tests/step13_test_languages.rs | ex_core_379_an_unknown_language_stop_names_the_language_in_one_line | REQ-core-189, EX-core-379 | C-core-qrules-11 | そのまま |
| tests/step13_test_languages.rs | ex_core_321_same_rule_file_twice_stops | REQ-core-189, EX-core-321 | C-core-qrules-9 | そのまま |
| tests/step13_test_languages.rs | req_189_unreadable_or_malformed_rule_files_stop | REQ-core-189 | C-core-qrules-9 | ディレクトリ、UTF-8 でない、YAML でない、ルールでない、の4つ |
| tests/step13_test_languages.rs | ex_core_322_language_alias_is_accepted | REQ-core-189, EX-core-322 | C-core-qrules-5 | そのまま |
| tests/step13_test_languages.rs | req_189_language_is_matched_without_case | REQ-core-189 | C-core-qrules-5 | そのまま |
| tests/step13_test_languages.rs | req_181_rule_with_files_keeps_its_place_in_the_order | REQ-core-181, REQ-core-187 | C-core-qrules-7 | この話題の分（"files" に当たるファイルにはルールを当てる）だけ。REQ-core-181 の分（名前の選び方）は落とした細部 |
| tests/step1_config.rs | req_121_bundled_query_stays_when_rules_are_added | REQ-core-121 | C-core-qrules-2 | そのまま |
| tests/step1_config.rs | tbl_004_tests_rules_is_read_and_defaults_to_empty | TBL-core-004, REQ-core-186 | C-core-qrules-3 | この話題の分だけ。TBL-core-004 は設定の話題 |

## 落とした細部

### 安全に関わるもの

無い。停止の条件（REQ-core-189）は、どれも利用者が決めたと記録にあるので、決定に残した。

### そのほか

無い。

## 形に依存する要求

- REQ-core-186（記録の multi-language-tests A10 の「IR には ast-grep のルールの形を契約として書く」）: 新しい形で読み替えて残る。IR の要求ではなく、決定の文書の決定「ルールのファイルは、ast-grep のルールの YAML をそのまま読む」に契約として書く。

## テストの無い行

- C-core-qrules-6（"language: typescript" のルールは ".tsx" のファイルに当たらない）: 要るテストは、"tests.files" に "tests/**/*.tsx" を入れ、"language: typescript" で "bench($NAME, $$$)" に当たるルールを足し、"tests/a.test.tsx" の "bench('fast', () => {})" に test_without_id が出ないことを確かめるもの。

C-core-qrules-9 は「読めない」（権限が無いなど）の場合を行に書いていない。決めたことには書いたが、それを確かめるテストが無いため。足すなら、ルールのファイルの読み取りの権限を外して終了コード 2 を確かめるテストが要る。

## 誰が決めたか見分けられなかったもの

無い。この話題の出典はどれも decided_by が「利用者（推奨を採用）」で、細部を任せた承認の記録ではない。

## 理由や見送った案が記録に無かった決定

- 「ルールの files と ignores は、ast-grep と同じ読み方で守る」: 見送った案が無い（"tests.files" の読み方に合わせない、は A45 の決定の中身で、検討した案としての理由は「ast-grep と同じ結果になる」だけ）。
- 「ルールの fix、message、severity、note、metadata は使わない」: 見送った案が無い。
- 「ルールのファイルに誤りがあれば、設定の誤りとして停止する」: 見送った案が無い。

## 分けた所、まとめた所

- REQ-core-189 の "language" の突き合わせの文を、ルールの形の読み方の決定に移した。停止の決定とは別の振る舞い（停止しないで受ける場合）なので。
- 記録の A21（TypeScript と Tsx は別の言語）を、ルールの形の読み方の決定に入れた。同梱の問い合わせを両方に付けることは test-queries の決定に書いた。
- 停止の値違いの場面（EX-core-315、EX-core-316、EX-core-321 と、まとめたテスト）は C-9 の1行にまとめた。

## あとでリンクにする所

確定の段で、次を見出しへのリンクにした。下書きどうしのリンクも `<file>.ja.md#<anchor>` に直した。

- 停止の理由の文言: cli-environment の「停止の理由は、英語の固定の文言で示す」（「ルールのファイルに誤りがあれば…」）
- 相対パスの基準: base-directory の「設定ファイルの値のパスと、出力の "path" は、基準のディレクトリからの相対にする」（「ルールのファイルは…」）
- "mutants.equivalents": equivalents の「等価の判断は、設定の mutants.equivalents が指す YAML の一覧に置く」（「ルールのファイルは…」）

残り: 設定の話題の "tests.rules" の鍵。この文書の文では設定のキーの一覧を指していないので、リンクにする所は無い

## 迷った判断と、どう決めたか

- 見送った案の「以前の決定: 設定で問い合わせを足せないことを、禁止として決めておく」は、records.md の A128（第9ラウンド、利用者の答え「進めて」）を multi-language-tests の A2 が改めたもので、Revisions に書いてある。覆した古い決定なので「以前の決定」として書いた。
- ast-grep、"ast-grep scan -r" の名前は、使う人が書くルールの形そのもの（A10）なので残した。
- 「など」「のような」を直した: 決めたことの「"ts"、"py" のような別名」を「ast-grep の言語の別名（例: "ts"、"py"）」に、C-4 の「（"go" など）」を「（例: "go"）」に、C-5 の「"ts" のような別名」を「言語の別名（例: "ts"）」にした。
- 停止の詳細のパスが ".kotowari/config.yaml" を含まないことは、guide-gaps の A2（利用者）の「詳細はルールのファイルのパス」の帰結として、確認することの行に入れた。

## 数

- 元: 要求 5、場面 11、決定表 0、性質 0
- 読んだ記録のファイル: 4（2026-09-24-multi-language-tests.md、2026-09-24-guide-gaps.md、records.md（A128、R3）、ir-form.md（出力の節））
- 出来上がり: 決定 5、確認することの行 11（うち「（読んで確認）」0、テストの無い行 1）
- テスト: 18
