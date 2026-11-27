# 指摘の種類

[English](findings.md) | 日本語

<!-- @kotowari[docs/decisions/core/findings.md#the-detail-of-a-finding-is-a-short-string-that-identifies-that-finding-decided-per-kind-in-a-table:9134c812, docs/decisions/core/findings.md#findings-are-divided-into-errors-and-notices-and-only-the-kinds-in-the-table-are-notices:d2f0a1cd] -->

`kotowari check`、`kotowari mutants`、`kotowari plan` が出す指摘の種類を、すべて並べたページです。
出力に知らない種類が出たら、まず下の一覧で引き、原因と直し方は種類をまとめた節で読んでください。

指摘には、誤り（`error`）と注意（`notice`）の2つの重さがあります。

| 重さ | 終了コード | 意味 |
|---|---|---|
| 誤り（`error`） | 1件でもあれば1 | 直す必要がある |
| 注意（`notice`） | 変えない | 見直す価値のある手がかり。放置しても `status` の `complete` は妨げない |

注意は `mutant_timeout`、`equivalent_stale`、`guide_stale`、`surface_unspecified_stale` の4種類だけで、ほかはすべて誤りです。
指摘の鍵（`kind`、`severity`、`path`、`line`、`detail`）と text の形は [cli.md](cli.ja.md#指摘の出し方) にあります。

## 種類の一覧

<!-- @kotowari[docs/decisions/core/findings.md#the-detail-of-a-finding-is-a-short-string-that-identifies-that-finding-decided-per-kind-in-a-table:9134c812, docs/decisions/core/findings.md#findings-are-divided-into-errors-and-notices-and-only-the-kinds-in-the-table-are-notices:d2f0a1cd, docs/decisions/core/finding-order.md#the-line-of-findings-that-have-a-line-is-decided-per-kind-in-one-table:848f58fc, docs/decisions/core/finding-order.md#line-is-counted-from-1-and-a-finding-that-covers-the-whole-document-is-null:b98176aa] -->

「行」の列は、指摘の `line` が指す行です。
「なし」は文書全体への指摘で、`line` は null、text では `-` になります。
「行の文字」は、読んだ行をそのまま（字下げと末尾の空白を含めて）入れたものです。

| 種類 | 重さ | detail | 行 | 節 |
|---|---|---|---|---|
| `missing_title` | 誤り | 文書のファイル名 | なし | [文書の形](#文書の形) |
| `multiple_titles` | 誤り | 2つ目以降の題名（`# ` を除く） | なし | [文書の形](#文書の形) |
| `topic_lead_missing` | 誤り | 題名 | 題名の行 | [文書の形](#文書の形) |
| `unclosed_code_block` | 誤り | 開きの行の文字 | 開きの行 | [文書の形](#文書の形) |
| `decision_state_invalid` | 誤り | 状態の行の値。無いときは空 | 状態の行。無いときは決定の見出し | [決定と節](#決定と節) |
| `decision_section_missing` | 誤り | 無い節の名前 | 決定の見出し | [決定と節](#決定と節) |
| `unknown_heading` | 誤り | 書かれたままの見出しの行 | その行 | [決定と節](#決定と節) |
| `unknown_line` | 誤り | 行の文字 | ブロックの最初の行 | [決定と節](#決定と節) |
| `invalid_id` | 誤り | 行の最初の語 | その行 | [確認することの行と ID](#確認することの行と-id) |
| `duplicate_id` | 誤り | ID | 2つ目以降の行 | [確認することの行と ID](#確認することの行と-id) |
| `id_domain_mismatch` | 誤り | ID | その行 | [確認することの行と ID](#確認することの行と-id) |
| `vague_word` | 誤り | その語 | その行 | [曖昧語](#曖昧語) |
| `link_target_missing` | 誤り | 書かれたままのリンク先 | リンクの行 | [リンク](#リンク) |
| `link_to_record` | 誤り | 書かれたままのリンク先 | リンクの行 | [リンク](#リンク) |
| `confirmation_without_test` | 誤り | 行の ID | その行 | [テストとの対応](#テストとの対応) |
| `test_without_id` | 誤り | テストの名前 | テストの最初の行 | [テストとの対応](#テストとの対応) |
| `unresolved_reference` | 誤り | 印に書いた ID | 印のある行 | [テストとの対応](#テストとの対応) |
| `invalid_marker` | 誤り | 行の文字 | 印のある行 | [テストとの対応](#テストとの対応)、[ガイドの印](#ガイドの印) |
| `unparsable_file` | 誤り | ファイルのパス | なし | [テストとの対応](#テストとの対応)、[面の検査](#面の検査) |
| `mutant_survived` | 誤り | 変更の説明 | 変異の結果の行 | [mutants と plan](#mutants-と-plan) |
| `equivalent_invalid` | 誤り | 一覧の1件の `file` と `change` を `: ` でつないだもの | なし | [mutants と plan](#mutants-と-plan) |
| `invalid_plan` | 誤り | スキーマの側の種類と詳細を `: ` でつないだもの | スキーマの側が返した行 | [mutants と plan](#mutants-と-plan) |
| `surface_without_spec` | 誤り | `面の種類 面の名前` | 面のノードの最初の行 | [面の検査](#面の検査) |
| `surface_unspecified_invalid` | 誤り | 一覧に書かれたままの `kind` と `name` を半角空白1つで区切ったもの | なし | [面の検査](#面の検査) |
| `overview_form_invalid` | 誤り | kotowari-markdown-schema の指摘の種類の名前。frontmatter の違反なら `frontmatter` | その行。文書全体への指摘はなし | [全体像の元データ](../decisions/core/overview-data.ja.md) |
| `overview_part_unknown` | 誤り | 部品の種類の名前 | 部品の囲みの開きの行 | [全体像の元データ](../decisions/core/overview-data.ja.md) |
| `overview_part_invalid` | 誤り | 部品の種類の名前、半角空白1つ、合わなかった場所（値全体は `(root)`、YAML として読めないときは `(yaml)`） | 部品の囲みの開きの行。YAML として読めないときは、YAML の読み手が返した誤りの行（返さなければ囲みの開きの行） | [全体像の元データ](../decisions/core/overview-data.ja.md) |
| `overview_lead_missing` | 誤り | 文書の名前（ディレクトリを除いたファイル名） | なし | [全体像の元データ](../decisions/core/overview-data.ja.md) |
| `overview_document_missing` | 誤り | `documents` の1件の文字 | なし | [全体像の元データ](../decisions/core/overview-data.ja.md) |
| `overview_document_shared` | 誤り | 2つ以上の全体像の元データの `documents` に載った決定の文書のパス | なし | [全体像の元データ](../decisions/core/overview-data.ja.md) |
| `overview_ref_unresolved` | 誤り | 参照の文字 | 部品の囲みの開きの行 | [全体像の元データ](../decisions/core/overview-data.ja.md) |
| `overview_name_conflict` | 誤り | `.md` を除いたファイル名 | なし | [全体像の元データ](../decisions/core/overview-data.ja.md) |
| `overview_toc_invalid` | 誤り | 目次の合わなかった場所（`overview_part_invalid` と同じ書き方。値全体は `(root)`、YAML として読めないときは `(yaml)`） | なし | [目次の検査](../decisions/core/overview-toc.ja.md) |
| `overview_toc_page_missing` | 誤り | 目次に無い全体像の元データの名前（`.md` を除いたファイル名） | なし | [目次の検査](../decisions/core/overview-toc.ja.md) |
| `overview_toc_page_unknown` | 誤り | 全体像の元データの無い名前を持つ項目の JSON Pointer（例: `/items/1`） | なし | [目次の検査](../decisions/core/overview-toc.ja.md) |
| `overview_toc_page_duplicate` | 誤り | 同じ名前の2つ目以降の項目の JSON Pointer | なし | [目次の検査](../decisions/core/overview-toc.ja.md) |
| `overview_toc_group_empty` | 誤り | `items` が空の群の JSON Pointer。いちばん外側は `(root)` | なし | [目次の検査](../decisions/core/overview-toc.ja.md) |
| `translation_missing` | 誤り | 無い側のパス | なし | [対](#対) |
| `translation_record_invalid` | 誤り | `missing`、`yaml`、`keys`、`value` のどれか | なし | [対](#対) |
| `translation_stale` | 誤り | `記録の hash 今の hash` | なし | [対](#対) |
| `translation_structure_mismatch` | 誤り | 最初に違う部分の名前（例えば `heading`、`confirmation`、`mark`、`code`、`table`、`link`） | 最初に違う要素の行。数が違うときと、その側に要素が無いときはなし | [対](#対) |
| `translation_switcher_invalid` | 誤り | あるべき切り替えの行 | 題名の後の最初の空でない行。無ければ題名の行、題名も無ければなし | [対](#対) |
| `link_language_mismatch` | 誤り | 書かれたままのリンク先 | リンクの行 | [対](#対) |
| `mutant_timeout` | 注意 | 変更の説明 | 変異の結果の行 | [mutants と plan](#mutants-と-plan) |
| `equivalent_stale` | 注意 | 一覧の1件の `file` と `change` を `: ` でつないだもの | なし | [mutants と plan](#mutants-と-plan) |
| `guide_stale` | 注意 | `行き先 書いた指紋 今の指紋` | ガイドの印が始まる行 | [ガイドの印](#ガイドの印) |
| `surface_unspecified_stale` | 注意 | 一覧に書かれたままの `kind` と `name` を半角空白1つで区切ったもの | なし | [面の検査](#面の検査) |

一覧は[仕様の指摘の種類](../decisions/core/findings.ja.md)に、行は[指摘の並べ方と行](../decisions/core/finding-order.ja.md)に決めてあります。

## 誤り: 決定の文書

<!-- @kotowari[docs/decisions/core/decision-documents.md#write-one-decision-as-text-in-problem-decision-rejected-options-and-confirmations:3990395b] -->

この部の種類は、決定の文書（`decisions.documents` が指す置き場のファイル。既定は `docs/decisions/`）に出ます。
各節の text の行は、わざと壊した決定の文書で実際に出したものです。出力の全体は下の[例](#例)にあります。
決定の文書の書き方は [決定の文書を書く](decision-documents.ja.md) にあります。

### 文書の形

<!-- @kotowari[docs/decisions/core/ir-document.md#a-decision-document-has-one-title-and-each-title-from-the-second-on-is-a-multiple_titles-error:e414db7f, docs/decisions/core/findings.md#missing_title-on-a-decision-document-without-a-title-has-the-document-name-as-its-detail:3a7a6a61, docs/decisions/core/ir-document.md#for-the-paragraph-after-the-title-only-its-presence-is-checked-not-its-content-line-count-or-separation-of-responsibilities:cfff5c08, docs/decisions/core/ir-input.md#an-unclosed-code-block-is-an-unclosed_code_block-error-and-from-its-start-to-the-end-of-the-document-is-not-read:c1db707e] -->

```text
docs/decisions/shop/notitle.md:- [error] missing_title notitle.md
docs/decisions/shop/price.md:- [error] multiple_titles Another title
docs/decisions/shop/price.md:13 [error] topic_lead_missing Price
docs/decisions/shop/price.md:17 [error] unclosed_code_block ```text
```

| 種類 | よくある原因 | 直し方 |
|---|---|---|
| `missing_title` | `# ` の題名が無い | 文書の先頭に `# 題名` を1つ書く |
| `multiple_titles` | `# ` の見出しが2つ以上ある | 題名を1つに絞り、ほかは `## ` 以下にするか、文書を分ける |
| `topic_lead_missing` | 題名と最初の決定のあいだに、話題が何を扱うかを書いた段落が無い。対の切り替えの行は数えない | 題名の直後（対では切り替えの行の後）に、話題が何を扱うかを段落で書く。見るのは段落があるかだけで、中身と長さは見ない |
| `unclosed_code_block` | コードブロックを閉じ忘れた | 開きと同じ文字で、同じ数以上の閉じの行を書く。開きの行から文書の終わりまでは、決定としても行としても読まない |

### 決定と節

<!-- @kotowari[docs/decisions/core/decision-documents.md#write-one-decision-as-text-in-problem-decision-rejected-options-and-confirmations:3990395b] -->

```text
docs/decisions/shop/cart.md:25 [error] decision_section_missing Decision
docs/decisions/shop/cart.md:27 [error] decision_state_invalid done
docs/decisions/shop/cart.md:33 [error] unknown_heading ### Notes
docs/decisions/shop/cart.md:35 [error] unknown_line The rate is in percent.
docs/decisions/shop/cart.md:37 [error] decision_section_missing Problem
docs/decisions/shop/cart.md:41 [error] unknown_line An explanation written before the sections.
```

| 種類 | よくある原因 | 直し方 |
|---|---|---|
| `decision_state_invalid` | 決定の見出しの下の最初の段落が状態の行でない。値が `状態: 提案`、`状態: 実装済み`、`Status: proposed`、`Status: implemented` のどれでもない | その側の言語で状態を書く。提案（決めたがまだ作っていない）か、実装済み（作った。今の振る舞い） |
| `decision_section_missing` | 決定に `### 問題` か `### 決めたこと` の節が無い。detail は無い節の名前で、決定の状態の行の言語で綴る | 無い節を書く |
| `unknown_heading` | 決定の中の `###` の見出しが `問題`、`決めたこと`、`見送った案`、`確認すること`（とその英語の綴り）のどれでもない。最初の決定の前、状態の行と最初の節のあいだ、`## 未決` の下に見出しを置いた | 4つの節のどれかの名前にするか、中身を節に移す |
| `unknown_line` | 決定の見出しと最初の節のあいだに、状態の行のほかのブロックがある。`### 確認すること` の下のブロックが箇条書きでない。知らない見出しの下にブロックがある。`## 未決` の下のブロックが箇条書きでない | 文を決定の節に移す。未決の問いは1つずつ箇条書きの項目にする |

`## 未決` のほかの `##` の見出しは、どれも1つの決定を始めます。節の中の `####` 以下の見出しは、その節に属します。

### 確認することの行と ID

<!-- @kotowari[docs/decisions/core/confirmations.md#check-counts-per-confirmation-row-whether-a-test-points-at-it:f6a31de4, docs/decisions/core/findings.md#if-the-same-id-is-in-several-places-duplicate_id-is-reported-for-each-place-from-the-second-on:3c1e544c, docs/decisions/core/findings.md#if-the-group-name-of-a-confirmation-id-does-not-match-the-group-folder-it-is-placed-in-it-is-an-id_domain_mismatch-error:cf755b08] -->

```text
docs/decisions/shop/cart.md:21 [error] duplicate_id C-shop-cart-2
docs/decisions/shop/cart.md:22 [error] id_domain_mismatch C-cart-3
docs/decisions/shop/cart.md:23 [error] invalid_id cart-4
```

| 種類 | よくある原因 | 直し方 |
|---|---|---|
| `invalid_id` | `### 確認すること` の下の行が `C-<名前>-<番号>` の形の ID で始まらない（名前は小文字の英字、数字、`-`。番号は1からで、先頭に0を付けない） | 行を `C-shop-cart-4` のような ID で始める |
| `duplicate_id` | 2つの行が同じ ID を持つ。パスのバイト順、行の順で2つ目以降の行に出る | 後の行に、その話題で使った最大の番号の次を付ける。消した行の番号は使い回さない |
| `id_domain_mismatch` | 群のフォルダ（`docs/decisions/shop/`）の下の文書で、行の ID の群の名前がそのフォルダの名前と違う | `C-shop-cart-3` のように、名前をフォルダに合わせる。置き場の直下に置いた文書は見ない |

### 曖昧語

<!-- @kotowari[docs/decisions/core/decision-documents.md#keep-the-glossary-as-a-document-for-people-and-drop-the-term-check:270bb62b, docs/decisions/core/finding-order.md#the-line-of-the-vague-word-finding-vague_word-is-the-line-that-has-the-word:b01cfba2, docs/decisions/core/translation-pairs.md#the-vague-word-check-is-made-on-decision-and-confirmations-of-every-side-of-decision-documents:c6a9614d] -->

```text
docs/decisions/shop/cart.md:15 [error] vague_word 適切に
```

| 種類 | よくある原因 | 直し方 |
|---|---|---|
| `vague_word` | 決定の `### 決めたこと` か `### 確認すること` の節に曖昧語がある（部分一致）。既定は「適切に」「必要に応じて」「通常は」「など」 | 何が起きるかを具体的に書く。語の一覧は設定の `vague_words` で変えられる |

`### 問題` と `### 見送った案` は見ません。コードブロックと二重引用符の中も見ません。
対では、どの側も見ます。
用語の検査はありません。群の用語集 `CONTEXT.md` は人が読む文書で、check は中身を読みません。

### リンク

<!-- @kotowari[docs/decisions/core/decision-documents.md#to-refer-to-another-decision-link-to-its-heading:41ba3d75, docs/decisions/core/translation-structure.md#links-from-decision-documents-and-their-paired-sides-to-the-frozen-tree-are-errors:f09f73fc] -->

```text
docs/decisions/shop/cart.md:3 [error] link_target_missing price.md#prices-exclude-tax
docs/decisions/shop/cart.md:45 [error] link_to_record ../../frozen/stock.md
```

| 種類 | よくある原因 | 直し方 |
|---|---|---|
| `link_target_missing` | 決定の文書のリンクが、決定の文書の置き場の中で、決定の文書でない `.md` のファイルか、その文書に無い見出しを指す。見出しの文を変えて、そこへのリンクを直さなかった | 有る文書と見出しを指す。行き先の名前は GitHub の見出しの規則に従う。見出しの文を変えたら、そこを指すリンクとガイドの印を同じ変更で直す |
| `link_to_record` | 決定の文書のどれかの側のリンクが `docs/frozen/` の下を指す | リンクを外す。凍結した木は古い形の文書を置く所で、今の規則の根拠にしない |

決定の文書の置き場の外へのリンクと、`.md` でないファイルへのリンクは、`link_target_missing` では見ません。

## 誤り: テストとガイド

### テストとの対応

<!-- @kotowari[docs/decisions/core/confirmations.md#check-counts-per-confirmation-row-whether-a-test-points-at-it:f6a31de4, docs/decisions/core/confirmations.md#write-by-reading-at-the-head-of-a-line-not-checked-by-tests:bfbb46c7, docs/decisions/core/coverage.md#a-test-without-a-marker-gets-a-test_without_id-error:2f609678, docs/decisions/core/coverage.md#a-test-that-has-no-row-can-be-declared-with-kotowaridetail-and-a-declared-test-gets-no-test_without_id:5f7fbfcf, docs/decisions/core/test-markers.md#a-malformed-marker-gets-an-invalid_marker-error-pointing-at-the-markers-line:915d8722, docs/decisions/core/test-discovery.md#a-test-file-with-a-syntax-error-gets-unparsable_file-and-is-skipped:4d52e291] -->

```text
docs/decisions/shop/cart.md:20 [error] confirmation_without_test C-shop-cart-2
tests/broken.rs:- [error] unparsable_file tests/broken.rs
tests/cart.rs:5 [error] unresolved_reference C-shop-cart-77
tests/cart.rs:9 [error] invalid_marker // @kotowari[
tests/cart.rs:11 [error] test_without_id broken_mark
tests/cart.rs:14 [error] test_without_id no_mark
```

| 種類 | よくある原因 | 直し方 |
|---|---|---|
| `confirmation_without_test` | 実装済みの決定の行の ID を挙げた印が、どのテストにも無い。その行は `（読んで確認）` で始まらない | その行を確かめるテストの直前のコメントに `@kotowari[C-…]` を書く。状態が提案の決定には出ない |
| `test_without_id` | テストに印が無い。印を関数の本体の中や、空行で切り離したコメントに書いた | テストの直前のコメントの塊に印を書く。どの行も確かめないテストには、宣言 `@kotowari[detail]` を書く（[marks.ja.md](marks.ja.md#行を持たないテスト)） |
| `unresolved_reference` | テストの印に、どの決定の文書にも無い ID を書いた。多くは書き間違い | ID を直す。`C-shop-cart` のような形の崩れた要素も、この誤りになる |
| `invalid_marker` | テストの印の中が空か区切りだけ。`]` が同じ行に無い | `@kotowari[ID, ID]` を1行で閉じる |
| `unparsable_file` | テストのファイルに構文の誤りがあり、tree-sitter で読めない | ファイルの構文を直す。このファイルは飛ばされ、印も読まれない |

どのテストの直前のコメントの塊にも無い印は、指摘も出さずに無視されます。
そのため印の置き場を間違えると、`invalid_marker` ではなく `test_without_id` と `confirmation_without_test` が出ます。
印の書き方は [marks.md](marks.ja.md) にあります。

### ガイドの印

<!-- @kotowari[docs/decisions/core/guides.md#one-entry-of-a-guide-marker-joins-the-target-of-a-decision-and-a-fingerprint-with-a-colon:7ba923d7, docs/decisions/core/guides.md#a-stale-marker-is-a-guide_stale-notice-and-does-not-change-the-exit-code-or-complete:7b8dc3ce, docs/decisions/core/languages-and-guides.md#a-guide-is-stale-when-the-decision-or-confirmations-change:de5a25a2] -->

```text
guides/cart.md:3 [notice] guide_stale docs/decisions/shop/cart.md#the-total-is-rounded 12345678 -
guides/cart.md:3 [notice] guide_stale docs/decisions/shop/cart.md#the-total-is-the-sum-of-the-line-prices 00000000 ff4409e4
guides/cart.md:7 [error] invalid_marker <!-- @kotowari[C-shop-cart-1] -->
```

| 種類 | 重さ | よくある原因 | 直し方 |
|---|---|---|---|
| `guide_stale` | 注意 | ガイドを書いた後で決定の決めたことか確認することが変わり、印に書いた指紋と今の指紋が違う。今の指紋が `-` なら、行き先がどの決定も指していない（文書か見出しが無くなった、見出しの文が変わった） | 節を読み直し、`kotowari query '<行き先>'` の今の決定に合わせて直してから、detail の3つ目（今の指紋）を印に写す。`-` なら、今その中身を持つ決定へ1件を付け替えるか、1件を外す |
| `invalid_marker` | 誤り | ガイドの印の1件に `:指紋` が無い。指紋が小文字の16進8文字でない。`:` の前が行き先の形 `<パス>#<行き先の名前>` でない。`]` が同じ行に無い | `<行き先>:<指紋>` の形にする。行き先は `kotowari list` の `id` から、指紋は `fingerprint` から写す |

形の誤ったガイドの印は、その中の正しい1件も照合されません。
節を直さずに指紋だけ写すと、古い節が隠れるだけです。
手順は [writing-guides.md](writing-guides.ja.md) にあります。

### 面の検査

<!-- @kotowari[docs/decisions/core/surface.md#a-surface-not-in-the-specification-gets-one-surface_without_spec-error-at-the-place-in-the-code-where-it-is-first-found:210038a0, docs/decisions/core/surface.md#if-the-name-of-a-surface-is-quoted-in-decision-or-confirmations-of-a-decision-that-surface-is-in-the-specification:6c702ea9, docs/decisions/core/surface-unspecified.md#a-malformed-entry-is-an-error-and-an-entry-no-longer-needed-is-a-notice:090bd4da, docs/decisions/core/surface.md#only-surface-files-in-the-languages-of-the-rules-are-read-and-parsed-into-syntax-trees-and-other-surface-files-are-not-read:82d0c416] -->

設定の `surface.rules` を書いたプロジェクトでだけ出ます（[surface.md](surface.ja.md)）。

```text
docs/surface-unspecified.yaml:- [error] surface_unspecified_invalid flag --legacy
docs/surface-unspecified.yaml:- [notice] surface_unspecified_stale flag --old
src/cli.rs:12 [error] surface_without_spec flag --verbose
```

| 種類 | 重さ | よくある原因 | 直し方 |
|---|---|---|---|
| `surface_without_spec` | 誤り | コードから取り出した面の名前が、どの決定の決めたことと確認することにも、二重引用符かバッククォートで囲んだ中身として完全に一致して出てこない。同じ種類と名前の面は、パスのバイト順、行の順で最初の1か所に1件だけ出る | その面を決める決定に名前を引用して書く（仕様の変更なので壁打ちで）。今は仕様にしないなら、壁打ちか導入で理由を付けて未記載の面の一覧に載せる。実装の途中で一覧に足して通さない |
| `surface_unspecified_invalid` | 誤り | 未記載の面の一覧の1件が `kind`、`name`、`why` のちょうど3つの鍵を持たない、値が文字列でない、`why` が空白だけ | 1件の形を直す。直すまでその1件はどの面も外さない |
| `surface_unspecified_stale` | 注意 | 未記載の面の一覧の1件に一致する面がコードに無い、または一致する面が決定に書かれた | その1件を消す。面の名前を変えたなら `name` を直す |

面の規則の言語のファイルに構文の誤りがあると、`unparsable_file` が出ます。ほかの面のファイルは読まないので、何も出ません。
同じファイルにテストのファイルとして `unparsable_file` を出したときは、重ねて出しません。

### 対

<!-- @kotowari[docs/decisions/core/translation-pairs.md#guides-overview-source-data-and-the-table-of-contents-have-a-side-for-every-language-of-the-language-list:a581e9d7, docs/decisions/core/translation-pairs.md#the-agreement-record-holds-the-blob-hash-of-each-side-as-of-the-last-time-they-were-confirmed-to-have-the-same-content:f29e5af9, docs/decisions/core/translation-pairs.md#if-a-side-is-changed-without-updating-the-agreement-record-that-side-is-stale:af90c9ef, docs/decisions/core/translation-structure.md#the-sides-of-a-pair-must-match-in-their-skeleton-everything-other-than-natural-language-sentences:ee743f40, docs/decisions/core/translation-structure.md#each-side-of-a-guide-pair-has-a-switcher-line-right-after-the-title:b65862b3, docs/decisions/core/translation-structure.md#links-inside-guides-and-overview-source-data-point-at-the-side-of-the-same-language:5e21d8c6, docs/decisions/core/languages-and-guides.md#decision-documents-can-also-be-kept-as-multilingual-pairs:535e227a, docs/decisions/core/translation-pairs.md#for-pairs-of-decision-documents-counting-and-output-are-done-on-the-first-languages-side-only:f5bf6692, docs/decisions/core/translation-pairs.md#the-staleness-of-guide-markers-is-reported-on-every-side-of-a-guide-pair:de48ce93] -->

設定の `languages` に2つ以上の言語を書いたプロジェクトでだけ出ます（[言語と対](config.ja.md#言語と対--languages-と-labels)）。
決定の文書とその用語集、ガイド、全体像の元データ、目次を対で持ちます。
translation_missing と translation_stale は側ごとに出ます。

```text
docs/decisions/shop/cart.ja.md:- [error] translation_stale de4818cf633031472beea547627e00905cf3dede 2246e3d2ff17d246033d32bcb65c8bb8717bd840
docs/decisions/shop/cart.ja.md:22 [error] translation_structure_mismatch confirmation
guides/a.i18n.yaml:- [error] translation_record_invalid missing
guides/a.md:- [error] translation_missing guides/a.ja.md
guides/b.md:3 [error] translation_switcher_invalid English | [日本語](b.ja.md)
guides/b.md:5 [error] link_language_mismatch ../docs/decisions/shop/cart.ja.md#合計は行の値段の和にする
```

| 種類 | よくある原因 | 直し方 |
|---|---|---|
| `translation_missing` | 対のどれかの言語の側が無い。先頭の言語の側が無く、ほかの言語の側だけがあるときは、その側に出て、その側はほかの検査で読まない | 無い側を、ある側の訳として書く。不要な側なら消す |
| `translation_record_invalid` | 一致の記録 `<幹>.i18n.yaml` が無い、YAML として読めない、鍵が各言語のファイル名とちょうど同じでない、値が40文字の16進の小文字でない。これが出ている間は `translation_stale` を出さない | `kotowari list` の `translations` の hash で記録を書く |
| `translation_stale` | ある側の今の git の blob hash が記録と違う。片方だけを直した | ほかの側を同じ内容に直してから、記録の hash を `kotowari list` の値に書き直す |
| `translation_structure_mismatch` | 文以外の骨組みが先頭の言語の側と違う。決定の文書では、見出しの深さと順と各 `###` がどの節か、各決定の状態、各行の ID と `（読んで確認）` で始まるか。ガイドでは、見出し、ガイドの印、コードブロック、表の形、リンク先。全体像の元データと目次では、部品の文でない欄と、入れ子と名前 | 先頭の言語の側と同じ骨組みにし、文だけを訳す。意味が同じかはレビューで確かめる |
| `translation_switcher_invalid` | 決定の文書かガイドの側の、題名の後の最初の空でない行が切り替えの行でない。切り替えの行は、`languages` の順に各言語の `language_name` を ` \| ` で区切り、自分の言語は文字だけ、ほかの言語は `[名前](その側のファイル名)` にした行 | detail の行をそのまま題名の次に書く |
| `link_language_mismatch` | 対の側の中のリンクが、ほかの言語の側を指す。スキームで始まる URL と `#` だけのリンク、切り替えの行のリンクは検査しない | 同じ言語の側を指す |

決定、行の ID、テストの印との照合、指紋、`list` と `query` の出力は、先頭の言語の側だけで決まります。
決定の文書のほかの側では、見出し、リンク、曖昧語だけを検査します。
ガイドの印の `guide_stale` は、ガイドのどの側にも出ます。

### mutants と plan

<!-- @kotowari[docs/decisions/core/mutants.md#a-miss-is-an-error-and-a-timeout-is-a-notice-and-nothing-is-output-for-caught-mutations-and-unviable-mutations:e1818807, docs/decisions/core/equivalents.md#an-entry-whose-text-is-gone-from-the-source-gets-an-equivalent_stale-notice:c9878315, docs/decisions/core/equivalents.md#a-malformed-entry-is-an-equivalent_invalid-error-and-removes-no-miss:78271fb2, docs/decisions/core/plan.md#findings-on-the-form-are-gathered-into-the-single-kind-invalid_plan-with-the-schema-sides-kind-and-details-in-detail:1608f12c] -->

この5種類は `kotowari check` では出ません。
`kotowari mutants` と `kotowari plan` だけが出します。
下の行は、小さな結果のファイルと等価の一覧で `kotowari mutants --tool cargo-mutants outcomes.json --format text` を実行したものです。

```text
.kotowari/equivalents.yaml:- [notice] equivalent_stale src/price.rs: replace - with + in discount
src/price.rs:3 [error] mutant_survived replace >= with > in total
src/price.rs:18 [notice] mutant_timeout replace += with *= in count_up
```

| 種類 | 重さ | 出る条件 | 直し方 |
|---|---|---|---|
| `mutant_survived` | 誤り | 変異を入れてもテストが全部通った（見逃し）。等価の一覧のどの1件にも一致しない | 変異を捕まえるテストを足す。出力が変わらない変異なら、等価の一覧に理由と一緒に足す |
| `mutant_timeout` | 注意 | 変異の結果が時間切れ | 変異がテストを止まらなくしていないかを確かめる |
| `equivalent_stale` | 注意 | 等価の一覧の1件の `text` と同じ文面の行が、`file` のファイルに無い | コードが変わっている。その1件を消すか、今の文面に直す |
| `equivalent_invalid` | 誤り | 等価の一覧の1件の鍵が足りない、余分がある、値の形が違う | 1件の形を直す |
| `invalid_plan` | 誤り | 計画書が決まった節とステップの形から外れている | detail のスキーマの側の説明に従って直す |

細部は [commands/mutants.md](commands/mutants.ja.md) と [commands/plan.md](commands/plan.ja.md) にあります。

## 例

<!-- @kotowari[docs/decisions/core/finding-order.md#findings-are-arranged-by-path-line-kind-and-detail-in-that-order:430be622] -->

壊した決定の文書、テスト、ガイドを置いた小さなリポジトリで `check` を実行した結果です。
上の各節の text の行は、ここから取りました。
指摘はパス、行、種類の順に並ぶので、ここの22行目のように、1つの行に指摘がいくつも付くことがあります。

```console
$ kotowari check --format text
docs/decisions/shop/cart.md:3 [error] link_target_missing price.md#prices-exclude-tax
docs/decisions/shop/cart.md:15 [error] vague_word 適切に
docs/decisions/shop/cart.md:20 [error] confirmation_without_test C-shop-cart-2
docs/decisions/shop/cart.md:21 [error] duplicate_id C-shop-cart-2
docs/decisions/shop/cart.md:22 [error] confirmation_without_test C-cart-3
docs/decisions/shop/cart.md:22 [error] id_domain_mismatch C-cart-3
docs/decisions/shop/cart.md:23 [error] invalid_id cart-4
docs/decisions/shop/cart.md:25 [error] decision_section_missing Decision
docs/decisions/shop/cart.md:27 [error] decision_state_invalid done
docs/decisions/shop/cart.md:33 [error] unknown_heading ### Notes
docs/decisions/shop/cart.md:35 [error] unknown_line The rate is in percent.
docs/decisions/shop/cart.md:37 [error] decision_section_missing Problem
docs/decisions/shop/cart.md:41 [error] unknown_line An explanation written before the sections.
docs/decisions/shop/cart.md:45 [error] link_to_record ../../frozen/stock.md
docs/decisions/shop/notitle.md:- [error] missing_title notitle.md
docs/decisions/shop/price.md:- [error] multiple_titles Another title
docs/decisions/shop/price.md:13 [error] topic_lead_missing Price
docs/decisions/shop/price.md:17 [error] unclosed_code_block ```text
guides/cart.md:3 [notice] guide_stale docs/decisions/shop/cart.md#the-total-is-rounded 12345678 -
guides/cart.md:3 [notice] guide_stale docs/decisions/shop/cart.md#the-total-is-the-sum-of-the-line-prices 00000000 ff4409e4
guides/cart.md:7 [error] invalid_marker <!-- @kotowari[C-shop-cart-1] -->
tests/broken.rs:- [error] unparsable_file tests/broken.rs
tests/cart.rs:5 [error] unresolved_reference C-shop-cart-77
tests/cart.rs:9 [error] invalid_marker // @kotowari[
tests/cart.rs:11 [error] test_without_id broken_mark
tests/cart.rs:14 [error] test_without_id no_mark
$ echo $?
1
```

## よくあるつまずき

### 印の ID を書き間違えたのに `test_without_id` にならない

<!-- @kotowari[docs/decisions/core/test-markers.md#even-a-marker-that-points-only-at-ids-in-no-document-makes-its-test-count-as-one-with-a-marker:5e328dc1, docs/decisions/core/confirmations.md#check-counts-per-confirmation-row-whether-a-test-points-at-it:f6a31de4] -->

存在しない ID だけを指す印でも、そのテストは「印あり」と数えます。
代わりに、印の行に `unresolved_reference` が、そのテストで確かめるつもりだった行に `confirmation_without_test` が出ます。

```console
$ kotowari check --format text      # the mark says C-shop-cart-11
docs/decisions/shop/cart.md:19 [error] confirmation_without_test C-shop-cart-1
tests/cart.rs:1 [error] unresolved_reference C-shop-cart-11
```

### 注意が残っているのに終了コードが0

<!-- @kotowari[docs/decisions/core/findings.md#findings-are-divided-into-errors-and-notices-and-only-the-kinds-in-the-table-are-notices:d2f0a1cd] -->

仕様どおりです。
注意の4種類は、終了コードも `status` の `complete` も変えません。

## 関連

- 仕様: [指摘の種類](../decisions/core/findings.ja.md)、[指摘の並べ方と行](../decisions/core/finding-order.ja.md)
- 指摘の形と並び: [cli.md](cli.ja.md#指摘の出し方)
- 指摘を出すコマンド: [check](commands/check.ja.md)、[mutants](commands/mutants.ja.md)、[plan](commands/plan.ja.md)
- 決定の文書の書き方: [decision-documents.md](decision-documents.ja.md)
- テストの印: [marks.md](marks.ja.md)
- ガイドの印: [writing-guides.md](writing-guides.ja.md)
