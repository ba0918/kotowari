# library-extraction の対応表

元: `docs/ir/schema/library-extraction.md`（日本語は `library-extraction.ja.md`）。書き直した文書: `library-extraction.md`。

## 元の ID の表

| 元の ID | 新しい ID | 決定の見出し | 理由 |
|---|---|---|---|
| REQ-schema-068 | C-schema-libext-1, C-schema-libext-2, C-schema-libext-7 | スキーマは意味の検査を通したものだけを返し、解析済みのものを繰り返し使える | 不正なスキーマを Schema にしないことを C-1、解析済みの値を使い回すことを C-2 にした。外から組み立て・変更・直接の Deserialize で検証を迂回できないこと（安全に関わる細部）を C-7（読んで確認）に戻した。ガイド `docs/guides/public-crate-api*.md` のマーカーはこの見出しへ |
| REQ-schema-069 | C-schema-libext-3 | 値の抽出の通常の入口は、違反の無い文書からだけ値を返す | そのまま。ガイドのマーカーはこの見出しへ |
| REQ-schema-070 | C-schema-libext-4 | ライブラリの検証は、スキーマの open を打ち消さない | 論理和を決定表にした。ガイドのマーカーはこの見出しへ |
| REQ-schema-071 | C-schema-libext-5, C-schema-cli-27 | CLI の values と ast --schema は、違反のある文書でも今までどおり値を出す | extract_typed_partial を C-5 にした。values の CLI の振る舞いは cli の「values と ast --schema は、検査の合否と関係なく値を組み立てる」（C-schema-cli-27）に一本化し、この決定からはリンクで指す（以前の C-libext-6 は欠番） |
| EX-schema-085 | C-schema-libext-2 | スキーマは意味の検査を通したものだけを返し、解析済みのものを繰り返し使える | そのまま |
| EX-schema-086 | C-schema-libext-1 | スキーマは意味の検査を通したものだけを返し、解析済みのものを繰り返し使える | そのまま |
| EX-schema-087 | C-schema-libext-3 | 値の抽出の通常の入口は、違反の無い文書からだけ値を返す | そのまま |
| EX-schema-088 | C-schema-libext-4 | ライブラリの検証は、スキーマの open を打ち消さない | 既定の値の場面を、open と緩める指定の4通りの1行にまとめた |
| EX-schema-089 | C-schema-libext-5, C-schema-cli-27 | CLI の values と ast --schema は、違反のある文書でも今までどおり値を出す | 場面を指していたテストは extract_typed_partial を確かめるので C-5。values のコマンドの側は cli の C-27 に一本化した |

元の文書に TBL と PROP は無い。

## テストごとの表

| テストのファイル | テストの関数名 | 今のマーカーの ID | 付け替えたあとの ID | 理由 |
|---|---|---|---|---|
| crates/kotowari-markdown-schema/tests/library.rs | parsed_values_can_be_reused_for_validated_extraction | REQ-schema-068, REQ-schema-069, EX-schema-085 | C-schema-libext-2 | 1つの Schema と Document で extract_validated を2回呼び、どちらも値が返ることを確かめている |
| crates/kotowari-markdown-schema/tests/library.rs | semantic_schema_errors_are_rejected_before_document_operations | REQ-schema-068, EX-schema-086 | C-schema-libext-1 | min が max より大きいスキーマで Schema::parse が失敗することを確かめている。誤りが std::error::Error であることはテストの中に残る細部 |
| crates/kotowari-markdown-schema/tests/library.rs | violating_documents_have_partial_values_but_no_validated_values | REQ-schema-069, EX-schema-087 | C-schema-libext-3 | 違反のある文書で extract_validated が指摘を返し、extract_partial が同じ種類の指摘と値を返すことを確かめている |
| crates/kotowari-markdown-schema/tests/library.rs | validation_respects_schema_open_and_explicit_relaxation | REQ-schema-069, REQ-schema-070, EX-schema-087, EX-schema-088 | C-schema-libext-4 | open と relax の4通りで validate・extract_partial・extract_validated の結果を確かめている。C-3 の違反の区別そのものは確かめていない |
| crates/kotowari-markdown-schema/tests/library.rs | typed_partial_json_keeps_obtainable_values_and_schema_name | REQ-schema-071, EX-schema-089 | C-schema-libext-5 | extract_typed_partial の型付きの JSON に type と取れた値が入り、指摘も返ることを確かめている。values のコマンドと終了コード（cli の C-27）は確かめていない |

## 落とした細部

### 安全に関わるもの

なし。前の版でここに置いた1件（Schema の検証の迂回の禁止）は、doctest が確かめているので決定に戻した（下の「戻した細部」）。

### そのほか

| 要求のどの部分 | 誰が決めたと記録にあるか |
|---|---|
| REQ-schema-071: ast_json と extract_typed_partial の JSON の形（既存の素の AST JSON、型付きの抽出 JSON） | 設計担当（A43）。形は「抽出と素の構文木」「CLI と結果の出し方」の話題に任せ、言葉で指した |
| テスト: 型付きの JSON の type にスキーマの名前が入ること | 元の文書の外（テストの中と、CLI の話題の ast --schema の決定）。C-5 の文には「スキーマの名前」として入れた |

## 戻した細部

| 細部 | 新しい ID | 確かめているもの |
|---|---|---|
| REQ-schema-068: Schema は不変の型で、外からの欄の変更や、検証していない直接の Deserialize で、意味の検査を迂回できない（設計担当の A42。why は「再利用可能な検証・抽出を、検証の迂回や open 設定の取り違えが起きない手順に具体化する」） | C-schema-libext-7（「スキーマは意味の検査を通したものだけを返し、解析済みのものを繰り返し使える」）、（読んで確認） | `crates/kotowari-markdown-schema/src/schema.rs` の `Schema` に付いた3つの compile_fail の doctest（欄を指定した組み立て、`schema.open = true` の書き換え、`serde_saphyr::from_str` での Deserialize）。安全の一覧（と前の版の対応表）は「直接確かめるテストは無い」としていたが、doctest があった。doctest にはマーカーを付けられない（一時の置き場で `/// @kotowari[...]` を doctest の前に置いて check し、confirmation_without_test が出た）ので、テストで確かめる行ではなく、doctest を名指しした読んで確認の行にした |

## テストの無い行

無し。前の版の C-schema-libext-6（values のコマンドの振る舞い）はテストが無かったが、cli の C-schema-cli-27 に一本化し、そちらは ex_schema_080 のテストを付け替え先にした。

## 誰が決めたか見分けられなかったもの

- REQ-schema-070（ValidationOptions の既定は open に従い、緩める指定と論理和）: 文は設計担当の A42。緩める指定は緩める方向にしか働かないこと（2026-09-21-mds-spec.md の A2）は利用者の決定で、ライブラリの指定はその3つ目の緩める指定に当たる。A2 の当てはめとして決定に残した。
- REQ-schema-068 のうち Schema::parse が失敗を返すこと: 文は A42（設計担当）だが、スキーマの解析の入口が「スキーマ、または形の違反」を返すことは利用者が追認した A58（mds-spec、名前は parse_schema）にある。A58 は REQ-schema-068 の出典に引かれていないが、A58 を根拠に残した。

## 理由や見送った案が記録に無かった決定

- 「スキーマは意味の検査を通したものだけを返し、…」: 見送った案が記録に無い。Schema::parse が失敗を返すこと（A58）の理由は not recorded で、問題の節に「その理由は残っていない」と書いた。使い回すこと（A21）の理由はある。
- 「値の抽出の通常の入口は、…」: 見送った案が記録に無い。
- 「ライブラリの検証は、スキーマの open を打ち消さない」: 見送った案が記録に無い。理由は A2 の why と、A42 の why（open の設定の取り違えが起きないようにする）から書いた。
- 「CLI の values と ast --schema は、…」: 見送った案が記録に無い。

## 分けた所、まとめた所

- EX-schema-088（既定のオプションで open を打ち消さない）と、テストが確かめる緩める指定の場合を、C-4 の1行と決定表にまとめた。
- REQ-schema-071 を、extract_typed_partial の行（C-5）と values のコマンドの行に分けた。values のコマンドの行（前の版の C-6）は cli の C-schema-cli-27 に一本化した。
- 入口の一覧は「ライブラリの入口」の話題に置き、この話題では振る舞いだけを書いた。I/O のクレートの extract_validated と extract_partial は、「読み込みのライブラリ」の話題からこの話題の見出しへリンクしている。

## あとでリンクにする所

無し。前の版で挙げた2つ（「ライブラリの検証は、スキーマの open を打ち消さない」の、緩める指定は緩める方向にしか働かない決まり。「CLI の values と ast --schema は、…」の JSON の形と、values・ast --schema の出力）は、closed-world、extraction、cli の見出しへのリンクにした

## 迷った判断と、どう決めたか

- values のコマンドの振る舞い（前の版の C-schema-libext-6）は、cli の PROP-schema-002（C-schema-cli-27）と同じ振る舞いなので、振る舞いが見える CLI の側に一本化した（依頼で指定された置き場）。cli.rs の ex_schema_080 が、違反（2つ目の題名）のある文書に `values` が終了コード 0 で値を出すことを確かめていたので、そのテストを cli の C-27 の付け替え先にした。
- REQ-schema-070 の論理和は、設計担当の文だが利用者の A2 の当てはめなので残した（上の「誰が決めたか見分けられなかったもの」）。落とす側に倒すなら、ガイドのマーカー（REQ-schema-070）の付け替え先が「値の抽出の通常の入口は、…」になる。
- Schema の検証の迂回の禁止は、利用者の決定は見つからないが、安全に関わる細部を確かめているものがあれば決定に戻す、という利用者の決定で C-7 に戻した。問題の節には A42 の why（検証の迂回が起きない手順にする）と、落とすと起きること（意味の検査を通らないスキーマで検証と抽出が行われる）を書いた。
- 設計担当が名前を決めた型（ValidatedValues、PartialExtraction、ValidationOptions、serde_json::Value）は、利用者から見える名前を残す決まりに従って残した。

## 数

- 元: 要求 4、場面 5、決定表 0、性質 0
- 読んだ記録のファイル: 2（2026-10-03-public-crate-api.md、2026-09-21-mds-spec.md）
- 出来上がり: 決定 4、確認することの行 6（C-1〜C-5、C-7。C-6 は欠番。うち「（読んで確認）」1）
- テスト: 5
