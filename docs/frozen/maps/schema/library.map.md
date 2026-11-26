# library の対応表

元: `docs/ir/schema/library.md`（日本語は `library.ja.md`）。書き直した文書: `library.md`。

## 元の ID の表

| 元の ID | 新しい ID | 決定の見出し | 理由 |
|---|---|---|---|
| REQ-schema-049 | C-schema-lib-1, C-schema-lib-4 | 依存するクレートは、決めた入口だけで参照の読み取りから抽出までを通せる | 入口だけで一通りを通せることを C-1 にした。指摘を読む口（C-4）は、指摘を作る関数を crate の中だけにした決定の裏側なので「内部の実装の型を、公開の署名に出さない」に置いた |
| REQ-schema-050 | C-schema-lib-5, C-schema-lib-6 | ライブラリはファイルを読まず、URL のスキーマも取得しない | URL を返して取得しないことを C-5、読み書きのコードと依存を持たないことを C-6（読んで確認）にした |
| REQ-schema-051 | C-schema-lib-2, C-schema-lib-3 | 内部の実装の型を、公開の署名に出さない | 元が verification: review なので、どちらも（読んで確認）。入口と入出力の型を互換性の対象にすることは「依存するクレートは、決めた入口だけで…」の決めたことに入れた。ガイド `docs/guides/package-validation*.md` のマーカーはこの見出しへ |
| TBL-schema-010 | C-schema-lib-1 | 依存するクレートは、決めた入口だけで参照の読み取りから抽出までを通せる | 表を「決めたこと」に移し、「返すもの」の列は「何をするか」に畳んだ（戻り値の細部は落とした細部） |
| EX-schema-015 | C-schema-lib-1 | 依存するクレートは、決めた入口だけで参照の読み取りから抽出までを通せる | そのまま |
| EX-schema-016 | C-schema-lib-5 | ライブラリはファイルを読まず、URL のスキーマも取得しない | そのまま |
| EX-schema-084 | C-schema-lib-2 | 内部の実装の型を、公開の署名に出さない | そのまま。候補の署名を判定する場面は、公開の署名を読んで確かめる行にした |

元の文書に PROP は無い。

## テストごとの表

| テストのファイル | テストの関数名 | 今のマーカーの ID | 付け替えたあとの ID | 理由 |
|---|---|---|---|---|
| crates/kotowari-markdown-schema/tests/library.rs | a_dependent_crate_can_run_the_whole_pipeline | REQ-schema-049, EX-schema-015 | C-schema-lib-1 | ast_json、frontmatter_schema、resolve_schema、Schema::parse、Document::parse、validate、抽出を順に呼び、参照・位置・指摘・値・構文木の JSON を確かめている |
| crates/kotowari-markdown-schema/tests/library.rs | findings_carry_the_kind_line_and_detail | REQ-schema-049 | C-schema-lib-4 | validate の指摘の kind()、line()、detail() を公開の口で読んでいる |
| crates/kotowari-markdown-schema/tests/library.rs | a_url_schema_resolves_to_a_url_for_the_caller_to_fetch | REQ-schema-049, REQ-schema-050, EX-schema-016 | C-schema-lib-5 | URL の参照で resolve_schema が URL を返すことを確かめている |
| crates/kotowari-markdown-schema/tests/library.rs | a_null_schema_reference_has_an_explanatory_public_error | REQ-schema-049, REQ-schema-014 | - | "$schema" が null のときに誤りになることは「スキーマの解決」の話題（REQ-schema-014）の決定で、この話題の分は、誤りの説明の文字が空でないという落とした細部だけ。REQ-schema-014 の分は、その話題の書き直しで付け替える |
| crates/kotowari-markdown-schema/src/frontmatter.rs | broken_frontmatter_is_an_error | REQ-schema-014, REQ-schema-049 | - | frontmatter が壊れていれば誤りになることは「スキーマの解決」の話題（REQ-schema-014）の決定。この話題の分は、誤りが std::error::Error であるという落とした細部だけ |

## 落とした細部

### 安全に関わるもの

なし。

### そのほか

| 要求のどの部分 | 誰が決めたと記録にあるか |
|---|---|
| TBL-schema-010: 「返すもの」の列の細部（frontmatter が壊れていれば誤り、Schema が不変であること、ast_json が実行の失敗を返すこと、extract_typed_partial が指摘も返すこと） | 設計担当（2026-10-03-public-crate-api.md の A56、A42。D1 の型設計の委任）。frontmatter の誤りは「スキーマの解決」の話題（REQ-schema-014）にもある。「参照、または無し」「位置」「構造」「指摘の並び」は利用者が追認した A58（2026-09-21-mds-spec.md）にあるので、「何をするか」の列に残した |
| REQ-schema-051: rustdoc で公開の入口と入出力の型を確かめるという確かめ方（how_to_verify） | 記録に無い（IR の確かめ方の行）。（読んで確認）の行にした |
| REQ-schema-051、EX-schema-084: 候補の公開署名を「適合と判定しない」という判定の手順 | 記録に無い。署名を読んで確かめる行にした |
| テスト: 誤りの説明の文字が空でないこと、誤りが std::error::Error を実装すること | 記録に無い（テストの中だけ）。テストの中に残る |

## 誰が決めたか見分けられなかったもの

- 入口の名前（Schema::parse、extract_validated、extract_partial、ast_json、extract_typed_partial）は設計担当の A42・A43（D1 の命名の委任）で、利用者が追認した A58 の名前（parse_schema、extract_values）から変わっている。入口を列挙して契約にすること（mds-spec の A24、A58）と、検証済みと部分の抽出を分けること（A20）は利用者の決定。名前は、利用者から見える名前を残す決まりに従って残し、入口ごとの細かい戻り値は落とした。

## 理由や見送った案が記録に無かった決定

- 「内部の実装の型を、公開の署名に出さない」: 見送った案が記録に無い。節を置かなかった。
- 「ライブラリはファイルを読まず、URL のスキーマも取得しない」: 見送った案が記録に無い。A57 そのものの why は not recorded で、理由は A18 と A11 の why から書いた。
- 「依存するクレートは、決めた入口だけで…」: A58（入口の一覧）の why は not recorded。理由は A24 と A16 の why から書いた。

## 分けた所、まとめた所

- REQ-schema-049 と TBL-schema-010 と、REQ-schema-051 のうち「入口とその入出力の型を互換性の対象にする」部分を、1つの決定「依存するクレートは、決めた入口だけで…」にまとめた。
- REQ-schema-051 の残り（内部の型を出さない、指摘を作る関数を出さない）を、独立の決定「内部の実装の型を、公開の署名に出さない」にした。指摘を作る関数の部分は、REQ-schema-051 を指していた 2026-10-23-next-release-bundle.md の A9 から入れた。
- 入口の振る舞い（Schema::parse、extract_validated、extract_partial、validate の open、ast_json、extract_typed_partial）は「検証と値の抽出」の話題に書き、この話題の表からはその見出しへリンクした。

## あとでリンクにする所

- なし。ほかの話題への参照は、一緒に依頼された「検証と値の抽出」「読み込みのライブラリ」の見出しへのリンクにした。

## 迷った判断と、どう決めたか

- この話題の具体的な入口と戻り値の出典（A42、A43、A56）は、どれも設計担当が D1（具体的な型名・関数名・配置の委任）で決めたもの。委任の承認だけなので agent の細部として落とす決まりと、利用者から見える名前は必ず残す決まりがぶつかる。名前は残し、振る舞いは利用者の決定（A24、A58、A16、A13、A57、A18、A11、next-release-bundle の A9）で裏付けられる分だけを書いた。形の決まりで足りなかった所。
- 「以前の決定」に A24（列挙に無い公開項目は実装の都合）を書いた。Revisions の V3 が A24 を A16・A40・A42・A43 に更新している。改めた理由は V3 に書かれていないので、A16 の why（下位のクレートに直接依存する利用者にも方針をはっきりさせる）で書いた。
- markdown::mdast::Node と regex::Captures の名前は A43（設計担当）だが、内部の型を公開しないこと自体は A13（利用者）なので、名前を挙げて決めたことに書いた。
- 指摘を作る関数の名前（new、at、maybe_at、of_node、of_rule）は、いまは crate の中だけで利用者から見えないが、利用者の決定（next-release-bundle の A9）に名前があるので書いた。
- C-schema-lib-4（指摘を読む口）は、next-release-bundle の A9 の「外からは読む口だけが要る」から起こした。元の要求には無い行だが、REQ-schema-049 だけを指すテスト findings_carry_the_kind_line_and_detail が確かめているものに当たる。

- schema-resolution の対応表が、frontmatter_schema が「参照が無い」を返すことだけを確かめるテスト（document_without_frontmatter_has_no_schema、frontmatter_without_schema_key_has_no_schema。REQ-schema-014 のマーカー）の付け替え先を、この話題の C-schema-lib-1 にした。入口の表の frontmatter_schema の行（参照か、参照が無いことを返す）に当たる。
- 「決めたこと」と「確認すること」に「など」は無かった。話題どうしのリンクは、書き直した下書きが取り込みで `.ja.md` になるので `<file>.ja.md#<anchor>` に直した。

## 数

- 元: 要求 3、場面 3、決定表 1、性質 0
- 読んだ記録のファイル: 3（2026-10-03-public-crate-api.md、2026-09-21-mds-spec.md、2026-10-23-next-release-bundle.md）
- 出来上がり: 決定 3、確認することの行 6（うち「（読んで確認）」3）
- テスト: 5
