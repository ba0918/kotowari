# 指摘の種類

<!-- @kotowari[REQ-core-029:f774ea58, REQ-core-030:4fe666c1, REQ-core-031:7277c20b] -->

`kotowari check`、`kotowari mutants`、`kotowari plan` が出す指摘の種類を、すべて並べたページです。
出力に知らない種類が出たら、まず下の一覧で引き、原因と直し方は種類をまとめた節で読んでください。

指摘には、誤り（`error`）と注意（`notice`）の2つの重さがあります。

| 重さ | 終了コード | 意味 |
|---|---|---|
| 誤り（`error`） | 1件でもあれば1 | 直す必要がある |
| 注意（`notice`） | 変えない | 見直す価値のある手がかり。放置しても `status` の `complete` は妨げない |

注意は `too_many_lines`、`too_many_requirements`、`mutant_timeout`、`equivalent_stale`、`guide_stale`、`deferred_with_test`、`depends_on_deferred`、`surface_unspecified_stale` の8種類だけで、ほかはすべて誤りです。
指摘の鍵（`kind`、`severity`、`path`、`line`、`detail`）と text の形は [cli.md](cli.md#指摘の出し方) にあります。

## 種類の一覧

<!-- @kotowari[TBL-core-008:11d73b51, TBL-core-009:e42a4a68, TBL-core-019:67506434, REQ-core-027:7f4780ac] -->

「行」の列は、指摘の `line` が指す行です。
「なし」は文書全体への指摘で、`line` は null、text では `-` になります。
「行の文字」は、読んだ行をそのまま（字下げと末尾の空白を含めて）入れたものです。

| 種類 | 重さ | detail | 行 | 節 |
|---|---|---|---|---|
| `missing_title` | 誤り | 文書のファイル名 | なし | [文書の形](#文書の形) |
| `multiple_titles` | 誤り | 2つ目以降の題名（`# ` を除く） | なし | [文書の形](#文書の形) |
| `missing_scope` | 誤り | 文書のファイル名 | なし | [文書の形](#文書の形) |
| `unknown_line` | 誤り | 行の文字 | その行 | [文書の形](#文書の形) |
| `unknown_code_block` | 誤り | 開始の行の文字 | 開始の行 | [文書の形](#文書の形) |
| `unclosed_code_block` | 誤り | 開始の行の文字 | 開始の行 | [文書の形](#文書の形) |
| `unknown_heading` | 誤り | 見出しの文字 | その行 | [項目の見出しと行](#項目の見出しと行) |
| `unknown_field` | 誤り | 行の文字 | その行 | [項目の見出しと行](#項目の見出しと行) |
| `duplicate_field` | 誤り | 行の名前 | 2つ目の行 | [項目の見出しと行](#項目の見出しと行) |
| `missing_field` | 誤り | 無い行の名前 | 項目の見出し | [項目の見出しと行](#項目の見出しと行) |
| `missing_statement` | 誤り | 項目の ID | 項目の見出し | [項目の値と文](#項目の値と文) |
| `missing_table` | 誤り | 決定表の ID | 項目の見出し | [項目の値と文](#項目の値と文) |
| `verification_missing` | 誤り | 要求の ID | 項目の見出し | [項目の値と文](#項目の値と文) |
| `verification_invalid` | 誤り | 値 | 項目の見出し | [項目の値と文](#項目の値と文) |
| `unknown_kind` | 誤り | 値 | 項目の見出し | [項目の値と文](#項目の値と文) |
| `algorithm_without_definition` | 誤り | 要求の ID | 項目の見出し | [項目の値と文](#項目の値と文) |
| `missing_source` | 誤り | 項目の ID か用語。値の空の `- deferred:` の行では `deferred` | 項目の見出し。シナリオはタグの行、用語は表の行、値の空の `- deferred:` の行はその行 | [出典](#出典) |
| `source_invalid` | 誤り | 出典の文字列 | 出典を書いた行 | [出典](#出典) |
| `unknown_term` | 誤り | バッククォートで囲んだ文字列 | その行 | [用語と曖昧語](#用語と曖昧語) |
| `unclosed_backtick` | 誤り | 行の文字 | その行 | [用語と曖昧語](#用語と曖昧語) |
| `vague_word` | 誤り | 語 | その行 | [用語と曖昧語](#用語と曖昧語) |
| `missing_document` | 誤り | 文書名の参照の文字列 | その行 | [用語と曖昧語](#用語と曖昧語) |
| `glossary_invalid` | 誤り | 文書のファイル名 | なし | [用語集](#用語集) |
| `glossary_title_invalid` | 誤り | 題名の行の文字 | 題名の行 | [用語集](#用語集) |
| `invalid_glossary_row` | 誤り | 行の文字 | その行 | [用語集](#用語集) |
| `duplicate_term` | 誤り | 用語 | 重複した側の行 | [用語集](#用語集) |
| `duplicate_id` | 誤り | ID | 2つ目以降の見出し | [ID と参照](#id-と参照) |
| `unresolved_reference` | 誤り | ID | 参照を書いた行 | [ID と参照](#id-と参照) |
| `invalid_id` | 誤り | 値 | タグの行 | [ID と参照](#id-と参照) |
| `id_domain_mismatch` | 誤り | ID | 見出しかタグの行 | [ID と参照](#id-と参照) |
| `missing_tag` | 誤り | 無いタグの名前 | タグの行 | [シナリオ](#シナリオ) |
| `unknown_tag` | 誤り | タグの名前 | タグの行 | [シナリオ](#シナリオ) |
| `invalid_gherkin_line` | 誤り | 行の文字 | その行 | [シナリオ](#シナリオ) |
| `requirement_without_test` | 誤り | 要求の ID | 項目の見出し | [テストとの対応](#テストとの対応) |
| `scenario_without_test` | 誤り | シナリオの ID | タグの行 | [テストとの対応](#テストとの対応) |
| `test_without_id` | 誤り | テストの名前 | テストの最初の行 | [テストとの対応](#テストとの対応) |
| `invalid_marker` | 誤り | 行の文字 | 印のある行 | [テストとの対応](#テストとの対応)、[ガイドの印](#ガイドの印) |
| `unparsable_file` | 誤り | ファイルのパス | なし | [テストとの対応](#テストとの対応)、[面の検査](#面の検査) |
| `record_field_missing` | 誤り | 無い補足の行の名前 | 番号の行 | [判断の記録](#判断の記録) |
| `record_field_unknown` | 誤り | 補足の行の名前 | その行 | [判断の記録](#判断の記録) |
| `revision_link_invalid` | 誤り | リンクの href | その行 | [判断の記録](#判断の記録) |
| `mutant_survived` | 誤り | 変更の説明 | 変異の結果の行 | [mutants と plan](#mutants-と-plan) |
| `equivalent_invalid` | 誤り | 一覧の `file` と `change` を `: ` でつないだもの | なし | [mutants と plan](#mutants-と-plan) |
| `invalid_plan` | 誤り | スキーマの側の種類と詳細を `: ` でつないだもの | スキーマの側が出した行 | [mutants と plan](#mutants-と-plan) |
| `surface_without_spec` | 誤り | `面の種類 面の名前` | 面の節の最初の行 | [面の検査](#面の検査) |
| `surface_unspecified_invalid` | 誤り | 一覧に書かれたままの `kind` と `name` を半角空白1つで区切ったもの | なし | [面の検査](#面の検査) |
| `overview_form_invalid` | 誤り | kotowari-markdown-schema の指摘の種類の名前。frontmatter の違反では `frontmatter` | その行。文書全体にかかるものはなし | [全体像の元データ](../ir/core/overview-data.md) |
| `overview_part_unknown` | 誤り | 部品の種類の名前 | 部品のフェンスの開始の行 | [全体像の元データ](../ir/core/overview-data.md) |
| `overview_part_invalid` | 誤り | 部品の種類の名前、半角空白1つ、合わなかった場所（値の全体は `(root)`、YAML として読めないときは `(yaml)`） | 部品のフェンスの開始の行。YAML として読めないときは、YAML の読み取りが返した誤りの行（返らなければフェンスの開始の行） | [全体像の元データ](../ir/core/overview-data.md) |
| `overview_lead_missing` | 誤り | 文書名（ディレクトリを除いたファイル名） | なし | [全体像の元データ](../ir/core/overview-data.md) |
| `overview_ir_missing` | 誤り | `ir` の1件の文字 | なし | [全体像の元データ](../ir/core/overview-data.md) |
| `overview_ir_shared` | 誤り | 2つ以上の全体像の元データの `ir` にある IR の文書のパス | なし | [全体像の元データ](../ir/core/overview-data.md) |
| `overview_ref_unresolved` | 誤り | 参照の文字 | 部品のフェンスの開始の行 | [全体像の元データ](../ir/core/overview-data.md) |
| `overview_name_conflict` | 誤り | `.md` を除いたファイル名 | なし | [全体像の元データ](../ir/core/overview-data.md) |
| `overview_toc_invalid` | 誤り | 目次の合わなかった場所（`overview_part_invalid` と同じ書き方。値の全体は `(root)`、YAML として読めないときは `(yaml)`） | なし | [目次の検査](../ir/core/overview-toc.md) |
| `overview_toc_page_missing` | 誤り | 目次に無い全体像の元データの名前（`.md` を除いたファイル名） | なし | [目次の検査](../ir/core/overview-toc.md) |
| `overview_toc_page_unknown` | 誤り | 全体像の元データの無い名前の項目の JSON Pointer（例 `/items/1`） | なし | [目次の検査](../ir/core/overview-toc.md) |
| `overview_toc_page_duplicate` | 誤り | 同じ名前の2つ目以降の項目の JSON Pointer | なし | [目次の検査](../ir/core/overview-toc.md) |
| `overview_toc_group_empty` | 誤り | `items` が空の群の JSON Pointer。いちばん外側なら `(root)` | なし | [目次の検査](../ir/core/overview-toc.md) |
| `translation_missing` | 誤り | 無い側のパス | なし | [対](#対) |
| `translation_record_invalid` | 誤り | `missing`、`yaml`、`keys`、`value` のどれか | なし | [対](#対) |
| `translation_stale` | 誤り | `記録の hash 今の hash` | なし | [対](#対) |
| `translation_structure_mismatch` | 誤り | 最初に食い違った部分の名前（`heading`、`field`、`table`、`gherkin`、`code`、`glossary`、`flag`、`mark`、`link`、`frontmatter`、`part`、`toc`） | 最初に食い違った要素の行。数の違いか、要素がその側に無いときはなし | [対](#対) |
| `translation_switcher_invalid` | 誤り | あるべき切り替えの行 | 題名の後の最初の空でない行。無ければ題名の行、題名も無ければなし | [対](#対) |
| `link_language_mismatch` | 誤り | 書かれたリンク先 | リンクの行 | [対](#対) |
| `link_to_record` | 誤り | 書かれたリンク先 | リンクの行 | [対](#対) |
| `too_many_lines` | 注意 | 行数 | なし | [文書の大きさ](#文書の大きさ) |
| `too_many_requirements` | 注意 | 要求の数 | なし | [文書の大きさ](#文書の大きさ) |
| `mutant_timeout` | 注意 | 変更の説明 | 変異の結果の行 | [mutants と plan](#mutants-と-plan) |
| `equivalent_stale` | 注意 | 一覧の `file` と `change` を `: ` でつないだもの | なし | [mutants と plan](#mutants-と-plan) |
| `guide_stale` | 注意 | `ID 書かれた指紋 今の指紋` | ガイドの印の始まりの行 | [ガイドの印](#ガイドの印) |
| `deferred_with_test` | 注意 | 後回しの要求か後回しのシナリオの ID | 要求の見出し。シナリオはタグの行 | [後回し](#後回し) |
| `depends_on_deferred` | 注意 | `参照元の ID 参照先の ID` | 参照を書いた行 | [後回し](#後回し) |
| `surface_unspecified_stale` | 注意 | 一覧に書かれたままの `kind` と `name` を半角空白1つで区切ったもの | なし | [面の検査](#面の検査) |

一覧の定義は [findings.md の TBL-core-008 と TBL-core-009](../ir/core/findings.md)、行の定義は [finding-order.md の TBL-core-019](../ir/core/finding-order.md) にあります。

## 誤り: IR の文書と項目

この節の種類は、IR の置き場（既定は `docs/ir/`）の文書に出ます。
各節の text の行は、形を崩した IR で実際に出したもので、全体は下の[例](#例)にあります。

### 文書の形

<!-- @kotowari[REQ-core-034:c8bed0c9, REQ-core-035:43c6f733, REQ-core-036:613b3ecf, REQ-core-174:43144ec6, REQ-core-112:204f8368] -->

```text
docs/ir/misc/notitle.md:- [error] missing_title notitle.md
docs/ir/misc/table.md:- [error] missing_scope table.md
docs/ir/misc/table.md:- [error] multiple_titles もう一つの題名
docs/ir/misc/table.md:6 [error] unknown_line 説明の行をここに書いた。
docs/ir/misc/table.md:14 [error] unknown_code_block ```text
docs/ir/misc/table.md:18 [error] unclosed_code_block ```gherkin
```

| 種類 | よくある原因 | 直し方 |
|---|---|---|
| `missing_title` | `# ` の題名が無い | 文書の先頭に `# 題名` を1つ書く |
| `multiple_titles` | `# ` の見出しが2つ以上ある | 題名を1つにし、ほかは `## ` 以下にするか文書を分ける |
| `missing_scope` | 題名と最初の `## ` の間に、文書が扱う範囲の行が無い | 題名の直後に、その文書が扱うことを1〜3行で書く（用語集と問題の記録は対象外） |
| `unknown_line` | `## ` の直下（最初の `### ` より前）に地の文を書いた | 説明は範囲の行か項目の文に移す |
| `unknown_code_block` | `## Examples` の下に gherkin でないコードブロックを置いた | 具体例は ` ```gherkin ` のブロックにする。gherkin でないブロックの中の行は gherkin として読まれないので、`invalid_gherkin_line` は出ない |
| `unclosed_code_block` | コードブロックを閉じ忘れた | 開始と同じ記号で、同じ数以上の閉じの行を書く。閉じるまで、その後ろは検査されない |

### 項目の見出しと行

<!-- @kotowari[REQ-core-043:435e99a0, REQ-core-044:de58fae5, REQ-core-045:718cfc29, REQ-core-098:0ccd2409] -->

```text
docs/ir/shop/cart.md:12 [error] duplicate_field verification
docs/ir/shop/cart.md:26 [error] unknown_field - owner: 山田
docs/ir/shop/cart.md:30 [error] missing_field how_to_verify
docs/ir/shop/cart.md:47 [error] unknown_heading ### 備考
```

| 種類 | よくある原因 | 直し方 |
|---|---|---|
| `unknown_heading` | `### ` の見出しが `### REQ-…: 名前` の形でない。`### EX-…` を見出しにした。`#### ` 以下の見出しを使った | `### ID: 名前` の形にする。具体例は見出しでなく gherkin のシナリオに書く |
| `unknown_field` | 項目の種類が持たない `- xxx:` の行か、`xxx:` の形でない一覧の行を書いた | 項目の種類ごとに持てる行だけにする（[ir-items.md の TBL-core-011](../ir/core/ir-items.md)）。説明は文に書く |
| `duplicate_field` | 同じ `- xxx:` の行を2つ書いた | 1つにまとめる。読まれるのは1つ目の値だけ |
| `missing_field` | 要求に `- kind:` が無い。`verification: review` の要求に `- how_to_verify:` が無い。問題の記録に `- kind:` か `- related:` が無い | 無い行を足す。review の要求には、人か LLM が確かめる手順を `- how_to_verify:` に書く |

`- verification:` が無いときは `verification_missing` だけ、`- source:` が無いときは `missing_source` だけが出て、`missing_field` は出ません。
形に合わない見出しの下も項目として読まれるので、`unknown_heading` と一緒にほかの指摘が並ぶことがあります（[よくあるつまずき](#1か所の誤りから同じ行に指摘がいくつも出る)）。

### 項目の値と文

<!-- @kotowari[REQ-core-047:20ccf180, REQ-core-099:772548c7, REQ-core-048:16bdfee2, REQ-core-049:4ff32073, REQ-core-050:5e8a4e67, REQ-core-051:ed035cbf] -->

```text
docs/ir/shop/cart.md:16 [error] algorithm_without_definition REQ-shop-002
docs/ir/shop/cart.md:22 [error] unknown_kind sometimes
docs/ir/shop/cart.md:22 [error] verification_invalid manual
docs/ir/misc/table.md:8 [error] missing_table TBL-misc-001
```

| 種類 | よくある原因 | 直し方 |
|---|---|---|
| `missing_statement` | algorithm 以外の要求、性質、問題の記録に文が無い | 見出しの下に、一覧でも表でもない文を書く |
| `missing_table` | 決定表に Markdown の表が無い | 表を書く |
| `verification_missing` | 要求に `- verification:` の行が無い | `unit`、`property`、`proof`、`review` のどれかを書く |
| `verification_invalid` | `- verification:` の値が4つのどれでもない | 4つのどれかに直す |
| `unknown_kind` | `- kind:` の値が決まった値でない | 要求は `event_driven`、`state_driven`、`ubiquitous`、`prohibition`、`invariant`、`algorithm`。問題の記録は `contradiction`、`gap`、`ambiguity` |
| `algorithm_without_definition` | `kind: algorithm` の要求に、決定表か性質を指す `- definition:` が無い | 規則を決定表か性質に書き、`- definition: TBL-…` で指す |

### 出典

<!-- @kotowari[REQ-core-059:877343dd, REQ-core-058:0012abf7, TBL-core-012:b17febcd, REQ-core-210:3ab477dc] -->

```text
docs/ir/shop/cart.md:22 [error] missing_source REQ-shop-003
docs/ir/shop/cart.md:52 [error] missing_source EX-shop-001
docs/ir/greet/greet.md:18 [error] source_invalid docs/decision/records/2026-09-24-greet.md#A3
```

| 種類 | よくある原因 | 直し方 |
|---|---|---|
| `missing_source` | 項目に `- source:` が無いか空。シナリオに `@source` が無い。用語集の出典の列が空。`- deferred:` の値が空（detail は `deferred`、行はその行） | 元になった決定を `パス#決定の番号` で書く。`@id` の無いシナリオでは、detail が `Scenario:` の行の文字になる |
| `source_invalid` | 出典の先に、その決定の番号の行が無い。パスが `decisions.records` と `decisions.adr` のどちらの中でもない。`パス#印` の形でない。`- deferred:` の値も同じ規則で検査し、行はその `- deferred:` の行 | 判断の記録に実在する決定の番号（`A12` など）か、ADR の `## ` の見出しの文字を指す。パスは基準のディレクトリからの全体を書く（`docs/decision/records/records.md#A26`） |

kotowari が見るのは、出典の先が実在するかだけです。
その決定が要求の内容を本当に述べているかは見ません。

### 用語と曖昧語

<!-- @kotowari[REQ-core-064:8b495194, REQ-core-065:6917c07d, REQ-core-116:97fdb62a, REQ-core-066:5abc2374, REQ-core-070:be69b7b3] -->

```text
docs/ir/greet/greet.md:21 [error] unknown_term 停止
docs/ir/shop/cart.md:28 [error] unclosed_backtick 在庫が無いとき、システムは`在庫切れ を出し、適切に知らせる。
docs/ir/shop/cart.md:28 [error] vague_word 適切に
docs/ir/shop/cart.md:36 [error] missing_document price.md
```

`unknown_term`、`unclosed_backtick`、`vague_word` が見るのは、要求と性質の文と、gherkin の Given、When、Then、And、But の行です。
`- ` の行、タグの行、`Scenario:` の行、用語集の意味の列、問題の記録の本文は見ません。

| 種類 | よくある原因 | 直し方 |
|---|---|---|
| `unknown_term` | バッククォートで囲んだ語が、用語集の用語でも ID でもない。パスやコード片も例外にならない | 用語なら、その文書のディレクトリか上のディレクトリの `CONTEXT.md` に行を足す。具体的な値なら二重引用符（`"…"`）で囲む。文書の上に用語集が1つも無いと、ID 以外の囲みはすべてこの誤りになる |
| `unclosed_backtick` | 行の中のバッククォートの数が奇数（閉じ忘れ） | 閉じる。この誤りの行では、用語と ID の検査が行われない |
| `vague_word` | 曖昧語が文に含まれる（部分一致）。既定は「適切に」「必要に応じて」「通常は」「など」 | 何をするかを具体的に書く。語の一覧は設定の `vague_words` で変えられる |
| `missing_document` | 文の中の `xxx.md` の参照の先の文書が無い | 文書名を直す。`/` の無い名前は同じディレクトリの文書、`/` のある名前は IR の置き場からの相対パスとして探す。上のディレクトリへは辿らない |

### 用語集

<!-- @kotowari[REQ-core-117:e5395be3, REQ-core-122:1b0fa7a3, REQ-core-123:e560f6b5, REQ-core-174:43144ec6] -->

```text
docs/ir/misc/CONTEXT.md:- [error] glossary_invalid CONTEXT.md
docs/ir/misc/CONTEXT.md:1 [error] glossary_title_invalid # 用語
docs/ir/shop/CONTEXT.md:6 [error] duplicate_term かご
docs/ir/shop/CONTEXT.md:7 [error] invalid_glossary_row | 在庫 | |
```

| 種類 | よくある原因 | 直し方 |
|---|---|---|
| `glossary_invalid` | `CONTEXT.md` に `| Term | Meaning | Source |` のヘッダと区切りの行を持つ表が無い（列名を日本語にした、など） | 題名の後、最初の `## ` より前に、このヘッダの表を置く。この誤りの間、その用語集の用語は0語として扱われる |
| `glossary_title_invalid` | 用語集の題名が `# Glossary` でない（`# 用語集` にした、など） | 題名を `# Glossary` にする |
| `invalid_glossary_row` | 表の行のセルが3つ未満か、用語のセルが空 | 用語、意味、出典の3つのセルを埋める |
| `duplicate_term` | 同じ用語集の前の行か、上のディレクトリの用語集に同じ用語がある | 重複した行を消す。見えるのは、根に近い側の1つ目の定義 |

### ID と参照

<!-- @kotowari[REQ-core-032:cc6f33bd, REQ-core-054:fa605207, REQ-core-114:2dd7868e, REQ-core-167:4886b6e7] -->

```text
docs/ir/shop/cart.md:30 [error] duplicate_id REQ-shop-001
docs/ir/shop/cart.md:38 [error] id_domain_mismatch REQ-cart-004
docs/ir/shop/cart.md:62 [error] invalid_id EX-1
tests/cart.rs:1 [error] unresolved_reference REQ-shop-077
```

| 種類 | よくある原因 | 直し方 |
|---|---|---|
| `duplicate_id` | 同じ ID の見出しかシナリオが2か所以上ある | 2つ目以降の ID を変える。1つ目は、パスのバイト順で先の文書、同じ文書なら上のもの |
| `unresolved_reference` | `- definition:`、`@about`、`- related:`、文の中のバッククォートの ID、テストの印が、存在しない ID を指す。多くは書き間違い | ID を直す。印の中の `REQ001` のような形の崩れた要素もこの誤りになる |
| `invalid_id` | `@id` の値が `EX-…` の ID の形でない | `EX-名前-001` の形にする。数字は3桁以上 |
| `id_domain_mismatch` | ID の名前が、IR の置き場からの第1階層のディレクトリ名と違う | `docs/ir/shop/` の文書なら `REQ-shop-001` のように、名前をディレクトリに揃える |

ID の形は [ir-references.md の REQ-core-124](../ir/core/ir-references.md) にあります。

### シナリオ

<!-- @kotowari[REQ-core-053:c2921456, REQ-core-052:861436ec, REQ-core-113:1ea82137] -->

```text
docs/ir/shop/cart.md:52 [error] unknown_tag @tags
docs/ir/shop/cart.md:58 [error] missing_tag @id
docs/ir/shop/cart.md:64 [error] invalid_gherkin_line   Examples:
```

| 種類 | よくある原因 | 直し方 |
|---|---|---|
| `missing_tag` | シナリオの直前の行に `@id` か `@about` が無いか、値が空 | `@id=EX-… @about=REQ-… @source=…` を `Scenario:` の直前の1行に書く |
| `unknown_tag` | `@id`、`@about`、`@source` 以外のタグを書いた。`@` で始まらない語をタグの行に書いた | その3つだけにする |
| `invalid_gherkin_line` | gherkin のブロックに、タグ、`Scenario:`、Given/When/Then/And/But のステップ、`#` の注釈、空行のどれでもない行を書いた（`Feature:`、`Background:`、`Scenario Outline:`、`Examples:`、データ表を含む）。`Scenario:` より前にステップを書いた | シナリオは `Scenario:` とステップの行だけで書く。場面が複数あるならシナリオを分ける |

### 文書の大きさ

<!-- @kotowari[REQ-core-038:4dafbd6a, REQ-core-039:bb042097] -->

この2つは注意で、終了コードを変えません。

```text
docs/ir/shop/cart.md:- [notice] too_many_lines 65
docs/ir/shop/cart.md:- [notice] too_many_requirements 5
```

| 種類 | 出る条件 | 見直し方 |
|---|---|---|
| `too_many_lines` | 文書の行数が `limits.lines`（既定200）を超えた | 1つの文書に複数の責務が混ざっていないかを見る。混ざっていなければそのままでよい |
| `too_many_requirements` | 話題ごとの文書の要求の数が `limits.requirements`（既定10）を超えた | 同上 |

行数や要求の数だけを理由に文書を割る必要はありません。
上の例は `limits.lines` を60、`limits.requirements` を3に下げた設定で出したものです。

## 誤り: テストとガイドと判断の記録

### テストとの対応

<!-- @kotowari[REQ-core-085:9c02a2ea, REQ-core-137:192fc62f, REQ-core-086:190ec5a3, REQ-core-072:51247600, REQ-core-083:9db1b29c] -->

```text
docs/ir/greet/greet.md:15 [error] requirement_without_test REQ-greet-002
docs/ir/greet/greet.md:26 [error] scenario_without_test EX-greet-001
tests/greet.rs:6 [error] test_without_id rejects_empty_name
tests/cart.rs:5 [error] invalid_marker // @kotowari[
tests/cart.rs:9 [error] invalid_marker // @kotowari[]
tests/broken.rs:- [error] unparsable_file tests/broken.rs
```

| 種類 | よくある原因 | 直し方 |
|---|---|---|
| `requirement_without_test` | review 以外の後回しでない要求に、その ID を挙げた印も、その要求を `@about` に持つシナリオの ID を挙げた印も無い | テストの直前のコメントに `@kotowari[REQ-…]` か `@kotowari[EX-…]` を書く。シナリオの印は、そのシナリオの `@about` の要求の分も満たす。今は作らないと決めた要求なら、テストの代わりに後回しにする（[deferred.md](deferred.md)） |
| `scenario_without_test` | シナリオの ID を挙げた印が無い（`@about` の要求がすべて review か後回しのシナリオには出ない） | そのシナリオを確かめるテストに `@kotowari[EX-…]` を書く。要求の印はシナリオの分を満たさない |
| `test_without_id` | テストに印が無い。印を関数の本体の中や、空行で切り離したコメントに書いた | テストの直前のコメントの塊に印を書く |
| `invalid_marker` | テストの印の中が空か区切りだけ。`]` が同じ行に無い | `@kotowari[ID, ID]` を1行で閉じる |
| `unparsable_file` | テストのファイルに構文の誤りがあり、tree-sitter で読めない | ファイルの構文を直す。このファイルは飛ばされ、印も読まれない |

どのテストの直前のコメントの塊にも無い印は、指摘も出さずに無視されます。
そのため印の置き場を間違えると、`invalid_marker` ではなく `test_without_id` と `requirement_without_test` が出ます。
印の書き方は [marks.md](marks.md) にあります。

### 後回し

<!-- @kotowari[REQ-core-211:d7879896, REQ-core-212:e533fb7e] -->

```text
docs/ir/greet/greet.md:13 [notice] depends_on_deferred REQ-greet-001 REQ-greet-003
docs/ir/greet/greet.md:15 [notice] deferred_with_test REQ-greet-003
```

| 種類 | 重さ | よくある原因 | 直し方 |
|---|---|---|---|
| `deferred_with_test` | 注意 | 後回しの要求か後回しのシナリオの ID を含む印がある。作り終えたのに `- deferred:` を消し忘れたか、印の付け間違い | 作り終えたなら、決定を判断の記録に書いてから `- deferred:` の行を消す。まだ作らないなら印を消す |
| `depends_on_deferred` | 注意 | 後回しでない要求か性質か、後回しのシナリオでないシナリオが、後回しの要求を参照している（`- definition:`、`@about`、文とステップの中のバッククォートで囲んだ ID）。参照1件ごとに1件 | 参照元も後回しにするか、要求を後回しから戻すか、参照を外す。どれも仕様の判断 |

後回しの書き方と効き目は [deferred.md](deferred.md) にあります。

### ガイドの印

<!-- @kotowari[REQ-core-202:6536f7a7, REQ-core-204:a925c073] -->

```text
guides/cart.md:3 [notice] guide_stale REQ-shop-001 00000000 b27eedf4
guides/cart.md:3 [notice] guide_stale REQ-shop-404 12345678 -
guides/cart.md:7 [error] invalid_marker <!-- @kotowari[REQ-shop-002] -->
```

| 種類 | 重さ | よくある原因 | 直し方 |
|---|---|---|---|
| `guide_stale` | 注意 | ガイドを書いた後で IR の項目の本文が変わり、印に書いた指紋と今の指紋が違う。今の指紋が `-` なら、その ID は IR から消えた | 節を読み直し、`kotowari query ID` の今の本文に合わせて直してから、detail の3つ目（今の指紋）を印に写す |
| `invalid_marker` | 誤り | ガイドの印の1件に `:指紋` が無い。指紋が小文字の16進8文字でない。ID の形でない。`]` が同じ行に無い | `ID:指紋` の形にする。指紋は `kotowari query ID` の `fingerprint` から写す |

形の誤ったガイドの印は、その中の正しい1件も照合されません。
節を直さずに指紋だけ写すと、古い節が隠れるだけです。
手順は [writing-guides.md](writing-guides.md) にあります。

### 面の検査

<!-- @kotowari[REQ-core-227:136a7242, REQ-core-233:2469b632, REQ-core-234:a4232959, REQ-core-236:a75a21eb] -->

設定の `surface.rules` を書いたプロジェクトでだけ出ます（[surface.md](surface.md)）。

```text
docs/surface-unspecified.yaml:- [error] surface_unspecified_invalid flag --legacy
docs/surface-unspecified.yaml:- [notice] surface_unspecified_stale flag --old
src/cli.rs:12 [error] surface_without_spec flag --verbose
```

| 種類 | 重さ | よくある原因 | 直し方 |
|---|---|---|---|
| `surface_without_spec` | 誤り | コードから取り出した面の名前が、どの要求の文、決定表のセル、シナリオのステップにも、二重引用符かバッククォートで囲んだ中身として完全に一致して出てこない。同じ種類と名前の面は、パスのバイト順、行の順で最初の1か所に1件だけ出る | その面を決める要求に名前を引用して書く（仕様の変更なので壁打ちで）。今は仕様にしないなら、壁打ちか導入で理由を付けて未記載の面の一覧に載せる。実装の途中で一覧に足して通さない |
| `surface_unspecified_invalid` | 誤り | 未記載の面の一覧の1件が `kind`、`name`、`why` のちょうど3つの鍵を持たない、値が文字列でない、`why` が空白だけ | 1件の形を直す。直すまでその1件はどの面も外さない |
| `surface_unspecified_stale` | 注意 | 未記載の面の一覧の1件に一致する面がコードに無い、または一致する面が IR に書かれた | その1件を消す。面の名前を変えたなら `name` を直す |

面の規則の言語のファイルに構文の誤りがあると、`unparsable_file` が出ます。ほかの面のファイルは読まないので、何も出ません。
同じファイルにテストのファイルとして `unparsable_file` を出したときは、重ねて出しません。

### 判断の記録

<!-- @kotowari[REQ-core-130:f72db551, REQ-core-131:97405a57, REQ-core-132:cb010297] -->

```text
docs/decision/records/2026-09-24-shop.md:10 [error] record_field_missing why
docs/decision/records/2026-09-24-shop.md:13 [error] record_field_unknown reason
docs/decision/records/2026-09-24-shop.md:17 [error] revision_link_invalid #A9
```

`record_field_missing` と `record_field_unknown` は、`## Context` の見出しを持つ判断の記録だけが受けます。
`revision_link_invalid` は、すべての判断の記録が受けます。

| 種類 | よくある原因 | 直し方 |
|---|---|---|
| `record_field_missing` | 決定の行の下に、節ごとに必須の補足の行が無い（Agreements、Prohibitions、Delegated、Rejected は `why`、Undecided は `decides`、Superseded は `superseded_by`） | `  - why: 理由` のように補足の行を足す |
| `record_field_unknown` | 補足の行の名前が `why`、`rejected`、`decided_by`、`superseded_by`、`decides`、`related` のどれでもない | 6つのどれかに直す |
| `revision_link_invalid` | `superseded_by` の値にリンクが無い。リンクの先の記録か決定の番号が無い | `[A9](#A9)` や `[A3](./other.md#A3)` の形で、実在する決定を指す |

### 対

<!-- @kotowari[REQ-core-338:a190a3f8, REQ-core-339:726dd6bf, REQ-core-341:779f601b, REQ-core-345:031507d3, REQ-core-346:cd9ad349, REQ-core-347:7e67aa6b, REQ-core-348:038b6606, REQ-core-349:c5034509, REQ-core-342:56f127fe, REQ-core-343:f6bcc00a] -->

設定の `languages` に2つ以上の言語を書いたプロジェクトでだけ出ます（[言語と対](config.md#言語と対--languages-と-labels)）。
IR、ガイド、全体像の元データ、目次の対ごとに、1つの対には1回だけ出ます。

```text
docs/ir/a.md:- [error] translation_missing docs/ir/a.en.md
docs/ir/a.i18n.yaml:- [error] translation_record_invalid keys
guides/a.md:- [error] translation_stale 78981922613b2afb6025042ff6bd878ac1994e85 e69de29bb2d1d6434b8b29ae775ad8c2e48c5391
docs/ir/b.en.md:9 [error] translation_structure_mismatch field
guides/b.md:3 [error] translation_switcher_invalid 日本語 | [English](b.en.md)
guides/g.en.md:5 [error] link_language_mismatch ../docs/ir/a.md#REQ-001
guides/g.md:3 [error] link_to_record ../docs/decision/records/r.md#A1
```

| 種類 | よくある原因 | 直し方 |
|---|---|---|
| `translation_missing` | 対のどれかの言語の側が無い。先頭の言語の側が無く、ほかの言語の側だけがあるときは、その側に出て、その側はほかの検査で読まない | 無い側を、ある側の訳として書く。不要な側なら消す |
| `translation_record_invalid` | 一致の記録 `<幹>.i18n.yaml` が無い、YAML として読めない、鍵が各言語のファイル名とちょうど同じでない、値が40文字の16進の小文字でない。これが出ている間は `translation_stale` を出さない | `kotowari list` の `translations` の hash で記録を書く |
| `translation_stale` | ある側の今の git の blob hash が記録と違う。片方だけを直した | ほかの側を同じ内容に直してから、記録の hash を `kotowari list` の値に書き直す |
| `translation_structure_mismatch` | 文以外の骨組み（見出し、項目の欄と出典、表の形と ID、シナリオのタグとステップの語、コードブロック、用語集の出典、ガイドの印、リンク先、全体像の部品の文でない欄、目次の入れ子と名前）が先頭の言語の側と違う | 先頭の言語の側と同じ骨組みにし、文だけを訳す。意味が同じかはレビューで確かめる |
| `translation_switcher_invalid` | IR とガイドの側の、題名の後の最初の空でない行が切り替えの行でない。切り替えの行は、`languages` の順に各言語の `language_name` を ` \| ` で区切り、自分の言語は文字だけ、ほかの言語は `[名前](その側のファイル名)` にした行 | detail の行をそのまま題名の次に書く |
| `link_language_mismatch` | 対の側の中のリンクが、ほかの言語の側を指す。スキームで始まる URL と `#` だけのリンク、切り替えの行のリンクは検査しない | 同じ言語の側を指す |
| `link_to_record` | 対の側の中のリンクが、判断の記録か ADR の置き場のファイルを指す | リンクを外す。経緯は IR の出典から辿れる |

ほかの言語の IR の側には、用語（その言語の用語集 `CONTEXT.<言語タグ>.md` の連鎖から引く）、曖昧語、文書名の参照、閉じないバッククォート、用語集の形の検査だけを行います。
項目、ID、出典、テストの印との照合、指紋、`list` と `query` の出力は先頭の言語の側だけで決まります。
ガイドの印の `guide_stale` は、ガイドのどの側にも出ます。

### mutants と plan

<!-- @kotowari[REQ-core-139:0cb2c43f, REQ-core-140:062bb90d, REQ-core-142:8341bf8d, REQ-core-143:71734415, REQ-core-193:ae035033] -->

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

細部は [commands/mutants.md](commands/mutants.md) と [commands/plan.md](commands/plan.md) にあります。

## 例

形を崩した IR、テスト、ガイド、判断の記録を置いた小さなリポジトリで `check` を実行した結果です。
上の各節の text の行は、ここと、[commands/check.md](commands/check.md#例) の例から抜き出しました。
同じ行に複数の種類が並ぶことと、形に合わない見出しの下も項目として読まれて指摘が重なることが分かります。

```console
$ kotowari check --format text
docs/decision/records/2026-09-24-shop.md:10 [error] record_field_missing why
docs/decision/records/2026-09-24-shop.md:13 [error] record_field_unknown reason
docs/decision/records/2026-09-24-shop.md:17 [error] revision_link_invalid #A9
docs/ir/misc/CONTEXT.md:- [error] glossary_invalid CONTEXT.md
docs/ir/misc/CONTEXT.md:1 [error] glossary_title_invalid # 用語
docs/ir/misc/notitle.md:- [error] missing_title notitle.md
docs/ir/misc/table.md:- [error] missing_scope table.md
docs/ir/misc/table.md:- [error] multiple_titles もう一つの題名
docs/ir/misc/table.md:6 [error] unknown_line 説明の行をここに書いた。
docs/ir/misc/table.md:8 [error] missing_table TBL-misc-001
docs/ir/misc/table.md:14 [error] unknown_code_block ```text
docs/ir/misc/table.md:18 [error] unclosed_code_block ```gherkin
docs/ir/shop/CONTEXT.md:6 [error] duplicate_term かご
docs/ir/shop/CONTEXT.md:7 [error] invalid_glossary_row | 在庫 | |
docs/ir/shop/cart.md:- [notice] too_many_lines 65
docs/ir/shop/cart.md:- [notice] too_many_requirements 5
docs/ir/shop/cart.md:12 [error] duplicate_field verification
docs/ir/shop/cart.md:16 [error] algorithm_without_definition REQ-shop-002
docs/ir/shop/cart.md:16 [error] requirement_without_test REQ-shop-002
docs/ir/shop/cart.md:22 [error] missing_source REQ-shop-003
docs/ir/shop/cart.md:22 [error] requirement_without_test REQ-shop-003
docs/ir/shop/cart.md:22 [error] unknown_kind sometimes
docs/ir/shop/cart.md:22 [error] verification_invalid manual
docs/ir/shop/cart.md:26 [error] unknown_field - owner: 山田
docs/ir/shop/cart.md:28 [error] unclosed_backtick 在庫が無いとき、システムは`在庫切れ を出し、適切に知らせる。
docs/ir/shop/cart.md:28 [error] vague_word 適切に
docs/ir/shop/cart.md:30 [error] duplicate_id REQ-shop-001
docs/ir/shop/cart.md:30 [error] missing_field how_to_verify
docs/ir/shop/cart.md:36 [error] missing_document price.md
docs/ir/shop/cart.md:38 [error] id_domain_mismatch REQ-cart-004
docs/ir/shop/cart.md:47 [error] missing_field kind
docs/ir/shop/cart.md:47 [error] missing_source 備考
docs/ir/shop/cart.md:47 [error] missing_statement 備考
docs/ir/shop/cart.md:47 [error] unknown_heading ### 備考
docs/ir/shop/cart.md:47 [error] verification_missing 備考
docs/ir/shop/cart.md:52 [error] missing_source EX-shop-001
docs/ir/shop/cart.md:52 [error] scenario_without_test EX-shop-001
docs/ir/shop/cart.md:52 [error] unknown_tag @tags
docs/ir/shop/cart.md:58 [error] missing_tag @id
docs/ir/shop/cart.md:62 [error] invalid_id EX-1
docs/ir/shop/cart.md:64 [error] invalid_gherkin_line   Examples:
guides/cart.md:3 [notice] guide_stale REQ-shop-001 00000000 b27eedf4
guides/cart.md:3 [notice] guide_stale REQ-shop-404 12345678 -
guides/cart.md:7 [error] invalid_marker <!-- @kotowari[REQ-shop-002] -->
tests/broken.rs:- [error] unparsable_file tests/broken.rs
tests/cart.rs:1 [error] unresolved_reference REQ-shop-077
tests/cart.rs:5 [error] invalid_marker // @kotowari[
tests/cart.rs:7 [error] test_without_id broken_mark
tests/cart.rs:9 [error] invalid_marker // @kotowari[]
tests/cart.rs:11 [error] test_without_id empty_mark
$ echo $?
1
```

## よくあるつまずき

### 1か所の誤りから、同じ行に指摘がいくつも出る

<!-- @kotowari[REQ-core-043:435e99a0] -->

形に合わない見出し（上の例の `### 備考`）の下も、項目の規則で読まれます。
そのため `unknown_heading` と一緒に、`missing_source`、`missing_statement`、`verification_missing` などが並びます。
まず見出しを直してから、もう一度実行してください。

### 印の ID を書き間違えたのに `test_without_id` にならない

<!-- @kotowari[REQ-core-077:4f3f71d8, REQ-core-054:fa605207] -->

存在しない ID だけを指す印でも、そのテストは「印あり」と数えます。
代わりに、印の行に `unresolved_reference` が、要求の側に `requirement_without_test` が出ます。

```console
$ kotowari check --format text      # 印に REQ-greet-02 と書いた
docs/ir/greet/greet.md:15 [error] requirement_without_test REQ-greet-002
tests/greet.rs:5 [error] unresolved_reference REQ-greet-02
```

### 注意が残っているのに終了コードが0

<!-- @kotowari[REQ-core-031:7277c20b] -->

仕様どおりです。
注意の8種類は、終了コードも `status` の `complete` も変えません。
kotowari 自身のリポジトリでも、`too_many_lines` と `too_many_requirements` が8件出たまま、終了コードは0です。

## なぜこういう作りか

- **重さは誤りと注意の2段だけ。注意は終了コードを変えない。**
  行数や要求の数の超過を「warning」と呼んでいたころ、書き手が「対応必須」と読み、責務が同じ内容を行数で切ってしまうことが起きました。
  名前を notice に改め、助言の意味しか持たせていません。
  （[2026-09-16-notice.md A1](../decision/records/2026-09-16-notice.md#A1)、[records.md A17](../decision/records/records.md#A17)、[A29](../decision/records/records.md#A29)）
- **古いガイドは誤りでなく注意。**
  ガイドが遅れているだけで CI を落とさないためです。
  注意は `check` と `status` の数に残り続けるので、放置されたガイドはいつでも見えます。
  （[2026-09-24-doc-marks.md A8](../decision/records/2026-09-24-doc-marks.md#A8)）
- **`guide_stale` の detail に今の指紋まで入れる。**
  1行だけで、どの ID を何に書き替えるかが分かるようにしました。
  （[2026-09-24-doc-marks.md A14](../decision/records/2026-09-24-doc-marks.md#A14)）
- **バッククォートの中は例外なく用語か ID。**
  用語はバッククォート、具体的な値は二重引用符、と役割を分けるためです。
  （[records.md A31](../decision/records/records.md#A31)）
- **出典は実在だけを見る。**
  出典がその要求を本当に述べているかは CLI では判定できないので、LLM の照合レビューに残しています。
  （[records.md A4](../decision/records/records.md#A4)）
- **mutants の指摘も check と同じ形。**
  受け取る側が1つの読み方で済むようにしました。
  （[2026-09-17-mutation-tests.md A40](../decision/records/2026-09-17-mutation-tests.md#A40)）

## 関連

- 仕様: [検査の種類](../ir/core/findings.md)、[指摘の並べ方と行](../ir/core/finding-order.md)
- 指摘の形と並び: [cli.md](cli.md#指摘の出し方)
- 指摘を出すコマンド: [check](commands/check.md)、[mutants](commands/mutants.md)、[plan](commands/plan.md)
- テストの印: [marks.md](marks.md)
- ガイドの印: [writing-guides.md](writing-guides.md)
