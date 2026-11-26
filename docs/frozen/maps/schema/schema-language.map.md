# schema-language の対応表

元: `docs/ir/schema/schema-language.md`（日本語は `schema-language.ja.md`）。書き直した文書: `schema-language.md`。

## 元の ID の表

| 元の ID | 新しい ID | 決定の見出し | 理由 |
|---|---|---|---|
| REQ-schema-016 | C-schema-lang-1 | スキーマは YAML で書き、題名・前置部・節・文書の直下の項目を根に置く | YAML のマッピングとして読むことと根のノードを C-1 にした。"document.item" の宣言の形は決めたことに書いたが、この話題の ID を指すテストが無いので行を置かず、文書の直下の項目の読み方は「文書の骨格」の話題（元の REQ-schema-061）に任せた。YAML として読めないスキーマの停止は C-schema-lang-4 |
| REQ-schema-017 | C-schema-lang-2 | 規則種別は9つにし、それぞれ置ける場所を決める | 元の確認の方法が review で、その how_to_verify を読んで確認の行にした |
| REQ-schema-060 | C-schema-lang-5, C-schema-lang-6 | 読み方は段落を既定にし、行で読む読み方をスキーマで選べる | 既定が段落であることを C-5、受けない値の停止を C-6 にした。"line" の読み分けは「見出しの下の行の規則」の話題（元の REQ-schema-058、TBL-schema-011）に任せた |
| REQ-schema-018 | C-schema-lang-3, C-schema-lang-4 | スキーマに知らないキーや形に合わない値があれば、検査せずに停止する | 知らないキーを C-3 にした。この要求を指すテストのうち型の違いを確かめるものは、同じ禁止（P1）から C-4 にした |
| REQ-schema-019 | C-schema-lang-7, C-schema-lang-8, C-schema-lang-9, C-schema-lang-10 | 出現回数は5つの書き方で宣言する | 定義の TBL-schema-005 の行ごとに分けた。範囲の内の個数で指摘が出ないことを確かめるテストは無い（「テストの無い行」） |
| REQ-schema-020 | C-schema-lang-12 | 条件付き規則は、条件が真の間だけ添えた制約を効かせる | そのまま |
| REQ-schema-021 | C-schema-lang-13 | 条件付き規則は、条件が真の間だけ添えた制約を効かせる | 同じ記録の決定（A28）なので、条件付き規則の決定にまとめた |
| TBL-schema-004 | C-schema-lang-2 | 規則種別は9つにし、それぞれ置ける場所を決める | 決めたことの表にした。規則種別の名前は用語集の語のまま、バッククォートを外した |
| TBL-schema-005 | C-schema-lang-7, C-schema-lang-8, C-schema-lang-9, C-schema-lang-10 | 出現回数は5つの書き方で宣言する | 決めたことの表にした |
| PROP-schema-004 | C-schema-lang-11 | 出現回数の範囲を宣言したノードは、1件でも配列で抽出する | 独立した決定にした。箇条書きと表と区切り文字を宣言したフィールド行がいつも配列になることは、「抽出と素の構文木」の話題（元の REQ-schema-045、TBL-schema-008）に任せた |
| PROP-schema-008 | C-schema-lang-5 | 読み方は段落を既定にし、行で読む読み方をスキーマで選べる | 性質を、EX-schema-040 のテストが確かめる行にまとめた |
| EX-schema-007 | C-schema-lang-12 | 条件付き規則は、条件が真の間だけ添えた制約を効かせる | そのまま |
| EX-schema-008 | C-schema-lang-3 | スキーマに知らないキーや形に合わない値があれば、検査せずに停止する | 場面は終了コード 2 を見るが、指しているテストはスキーマの読み込みが誤りになることを見る。行はテストに合わせ、終了コードを書かなかった |
| EX-schema-040 | C-schema-lang-5 | 読み方は段落を既定にし、行で読む読み方をスキーマで選べる | そのまま |
| EX-schema-041 | C-schema-lang-6 | 読み方は段落を既定にし、行で読む読み方をスキーマで選べる | そのまま |

## テストごとの表

| テストのファイル | テストの関数名 | 今のマーカーの ID | 付け替えたあとの ID | 理由 |
|---|---|---|---|---|
| crates/kotowari-markdown-schema/src/schema/tests.rs | loads_a_schema_with_all_rule_kinds | REQ-schema-016, REQ-schema-017 | C-schema-lang-1 | 題名、前置部、節、項目と、その下の規則種別を宣言したスキーマが読み込まれることを確かめている。REQ-schema-017 の分は読んで確認の行（C-2）なのでテストを付けない |
| crates/kotowari-markdown-schema/src/schema/tests.rs | unknown_key_is_an_error | REQ-schema-018, EX-schema-008 | C-schema-lang-3 | そのまま |
| crates/kotowari-markdown-schema/src/schema/tests.rs | type_violation_is_an_error | REQ-schema-018 | C-schema-lang-4 | 最上位の "open" の値の型の違い |
| crates/kotowari-markdown-schema/src/schema/tests.rs | min_greater_than_max_is_an_error | REQ-schema-019 | C-schema-lang-4 | 下限が上限より大きい出現回数を形の違反として受け付けないこと。この組み合わせを誤りにすることは記録に無い細部（落とした細部）で、行は「形に合わない」として指す |
| crates/kotowari-markdown-schema/src/schema/tests.rs | negative_repeat_is_an_error | REQ-schema-019 | C-schema-lang-4 | 出現回数の値の型の違い |
| crates/kotowari-markdown-schema/src/schema/tests.rs | non_integer_repeat_is_an_error | REQ-schema-019 | C-schema-lang-4 | 同上 |
| crates/kotowari-markdown-schema/src/schema/tests.rs | when_with_both_operators_is_an_error | REQ-schema-020 | C-schema-lang-4 | 比べ方を2つとも書いた条件付き規則を受け付けないこと |
| crates/kotowari-markdown-schema/src/schema/tests.rs | when_with_no_operator_is_an_error | REQ-schema-020 | C-schema-lang-4 | 比べ方を書かない条件付き規則を受け付けないこと |
| crates/kotowari-markdown-schema/src/schema/tests.rs | invalid_regex_pattern_is_an_error | REQ-schema-016 | C-schema-lang-4 | 正規表現として読めない値 |
| crates/kotowari-markdown-schema/src/schema/tests.rs | invalid_item_id_regex_is_an_error | REQ-schema-016, REQ-schema-025 | C-schema-lang-4 | 項目の ID の正規表現が読めない値。REQ-schema-025（「文書の骨格」の話題）の分は、その話題の書き直しで付け替える |
| crates/kotowari-markdown-schema/src/schema/tests.rs | not_yaml_is_an_error | REQ-schema-016 | C-schema-lang-4 | YAML として読めないスキーマ |
| crates/kotowari-markdown-schema/src/schema/tests.rs | unknown_key_under_bullets_is_an_error | REQ-schema-018 | C-schema-lang-3 | 箇条書きの下の知らないキー |
| crates/kotowari-markdown-schema/src/schema/tests.rs | type_violation_under_children_is_an_error | REQ-schema-018 | C-schema-lang-4 | 箇条書きの子の規則の値の型の違い |
| crates/kotowari-markdown-schema/src/schema/tests.rs | repeat_with_equal_min_and_max_is_accepted | REQ-schema-019 | C-schema-lang-10 | そのまま |
| crates/kotowari-markdown-schema/src/schema/tests.rs | item_table_invalid_repeat_is_rejected | REQ-schema-019 | C-schema-lang-4 | 項目の表の下限が上限より大きい出現回数。min_greater_than_max_is_an_error と同じ |
| crates/kotowari-markdown-schema/src/schema/tests.rs | item_codeblock_invalid_repeat_is_rejected | REQ-schema-019 | C-schema-lang-4 | 項目のコードブロックの、同じ形の誤り |
| crates/kotowari-markdown-schema/src/validate/tests.rs | two_top_level_bullets_satisfy_min_two | REQ-schema-019 | C-schema-lang-9 | 下限2の箇条書きが、いちばん外側の2行で満たされる（下限に満たなければ指摘が出ることの裏側）。子の行を数えないことは「見出しの下の行の規則」の話題の細部 |
| crates/kotowari-markdown-schema/src/validate/tests.rs | child_bullet_when_references_a_sibling_field | REQ-schema-021 | C-schema-lang-13 | 箇条書きの子の条件が兄弟のフィールド行を参照する |
| crates/kotowari-markdown-schema/src/validate/tests.rs | children_are_validated_when_parent_bullets_when_is_true | REQ-schema-020, REQ-schema-031 | - | 条件が真のときに子の行を子の規則で照らすこと。確かめているのは「見出しの下の行の規則」の話題（REQ-schema-031）の振る舞いで、条件付き規則については何も見分けていない。その話題の書き直しで付け替える |
| crates/kotowari-markdown-schema/src/validate/tests.rs | children_are_validated_when_parent_bullets_when_is_false | REQ-schema-020, REQ-schema-031 | C-schema-lang-12 | 条件が偽でも子の照合は止まらない、つまり "when" が効かせるのは添えた制約だけであることを確かめている。REQ-schema-031 の分は「見出しの下の行の規則」の話題で付け替える |
| crates/kotowari-markdown-schema/src/validate/tests.rs | child_bullets_are_counted_by_children_rule_not_parent_repeat | REQ-schema-019, REQ-schema-031 | C-schema-lang-9 | 子の箇条書きの下限に満たなければ repeat_min_not_met が出る。親の出現回数で子を数えないことは REQ-schema-031 の分で、その話題で付け替える |
| crates/kotowari-markdown-schema/src/validate/tests.rs | missing_bullets_is_found | REQ-schema-004, REQ-schema-019 | C-schema-lang-7 | 出現回数を書かない箇条書きが無ければ欠落の指摘が出る。REQ-schema-004（「閉じた世界と開いた世界」の話題）の分は、その話題で付け替える |
| crates/kotowari-markdown-schema/src/validate/tests.rs | repeat_min_not_met_for_section_is_found | REQ-schema-019 | C-schema-lang-9 | そのまま |
| crates/kotowari-markdown-schema/src/validate/tests.rs | repeat_min_not_met_for_field_is_found | REQ-schema-019 | C-schema-lang-9 | そのまま |
| crates/kotowari-markdown-schema/src/validate/tests.rs | repeat_max_exceeded_for_optional_section_is_found | REQ-schema-019 | C-schema-lang-8 | 必須の宣言を偽にした節が2個あると repeat_max_exceeded |
| crates/kotowari-markdown-schema/src/validate/tests.rs | repeat_max_exceeded_for_field_is_found | REQ-schema-019 | C-schema-lang-9 | そのまま |
| crates/kotowari-markdown-schema/src/validate/tests.rs | repeat_with_max_only_omits_min_as_zero | REQ-schema-019 | C-schema-lang-9 | 上限だけのときの下限が0 |
| crates/kotowari-markdown-schema/src/validate/tests.rs | item_with_no_repeat_requires_exactly_one | REQ-schema-019 | C-schema-lang-7 | 出現回数を書かない項目が無ければ指摘が出る |
| crates/kotowari-markdown-schema/src/validate/tests.rs | item_repeat_max_exceeded_is_found | REQ-schema-019 | C-schema-lang-9 | そのまま |
| crates/kotowari-markdown-schema/src/validate/tests.rs | item_with_required_false_is_optional | REQ-schema-019 | C-schema-lang-8 | 必須の宣言を偽にした項目は、無くても指摘が出ない |
| crates/kotowari-markdown-schema/src/validate/tests.rs | when_eq_true_requires_the_field | REQ-schema-020, EX-schema-007 | C-schema-lang-12 | そのまま |
| crates/kotowari-markdown-schema/src/validate/tests.rs | when_eq_false_skips_the_requirement | REQ-schema-020 | C-schema-lang-12 | 条件が偽のときは効かない |
| crates/kotowari-markdown-schema/src/validate/tests.rs | when_with_missing_reference_makes_eq_false | REQ-schema-021 | C-schema-lang-13 | そのまま |
| crates/kotowari-markdown-schema/src/validate/tests.rs | when_with_missing_reference_makes_ne_true | REQ-schema-021 | C-schema-lang-13 | そのまま |
| crates/kotowari-markdown-schema/src/validate/tests.rs | when_eq_is_false_when_reference_name_is_undeclared | REQ-schema-021 | C-schema-lang-13 | 宣言していない名前の行はフィールド行にならないので、見つからないときに当たる |
| crates/kotowari-markdown-schema/src/validate/tests.rs | when_ne_is_true_when_reference_name_is_undeclared | REQ-schema-021 | C-schema-lang-13 | 同上 |
| crates/kotowari-mds/tests/cli.rs | check_child_bullet_when_uses_sibling_field_exits_one | REQ-schema-021 | C-schema-lang-13 | CLI の側の同じ入力 |
| crates/kotowari-mds/tests/cli.rs | ex_schema_040_schema_without_reading_reads_by_paragraph | EX-schema-040 | C-schema-lang-5 | そのまま |
| crates/kotowari-mds/tests/cli.rs | ex_schema_041_unknown_reading_value_stops | EX-schema-041 | C-schema-lang-6 | そのまま |
| crates/kotowari-markdown-schema/src/extract/tests.rs | repeated_node_is_an_array_when_present_and_omits_key_when_absent | REQ-schema-019, REQ-schema-038 | C-schema-lang-11 | 範囲を宣言したフィールド行が1件でも配列になる。0件でキーを出さないことは REQ-schema-038（「抽出と素の構文木」の話題）の分で、その話題で付け替える |
| crates/kotowari-markdown-schema/src/extract/tests.rs | non_repeated_node_is_single_and_omits_key_when_missing | REQ-schema-019, REQ-schema-038 | C-schema-lang-11 | 範囲を宣言しないフィールド行が単一の値になる。REQ-schema-038 の分は同上 |
| crates/kotowari-markdown-schema/src/extract/tests.rs | repeated_field_with_separator_extracts_array_of_arrays | REQ-schema-019, REQ-schema-035, REQ-schema-045 | C-schema-lang-11 | 範囲を宣言したので外側が配列になる分だけ。区切り文字で内側が配列になることは REQ-schema-045 と REQ-schema-035（「抽出と素の構文木」の話題）の分 |
| crates/kotowari-markdown-schema/src/extract/tests.rs | repeated_section_extracts_array_of_bodies | REQ-schema-019, REQ-schema-035 | C-schema-lang-11 | 範囲を宣言した節が配列になる。REQ-schema-035 の分は「抽出と素の構文木」の話題で付け替える |

## 落とした細部

### 安全に関わるもの

無し。停止の条件（知らないキー、形に合わない値、受けない "reading" の値）は、どれも利用者の決定（P1、ir-engine-gaps の A30）が裏付けるので、決定に残した。

### そのほか

| 要求のどの部分 | 誰が決めたと記録にあるか |
|---|---|
| 下限が上限より大きい出現回数をスキーマの誤りにすること、出現回数に負の数や整数でない値を書けないこと（テストの中だけ） | 記録に無い（元の文書の外）。行は「形に合わない値」（P1）として C-schema-lang-4 で指し、具体の組み合わせはテストに任せた |
| 条件付き規則に "eq" と "ne" のちょうど1つを書くこと（テストの中だけ） | 記録に無い。A28 は比べ方が2つあることだけを決めている。C-schema-lang-4 の例に「比べ方が1つだけ書かれていない」と書いたのは、テストが指す先を示すためで、値の細部は決めたことに書かなかった |
| 誤りの文言（"unknown field"、"repeat min is greater than max" など） | 記録に無い（テストの中だけ） |

## 形に依存する要求

元の ID のうち、古い仕様の形そのものについて振る舞いを決めたものは無い。出典の記録のうち、次の部分は IR の形のためのもので、決めたことにも理由にも書かなかった。

- ir-engine-gaps の A13 と A20 の「IR の3つのスキーマ（ir、ir-context、ir-flags）は "reading: line" を書く」: 消える。IR のスキーマが無くなる。読み方を選べること自体は形に依存しないので残した
- ir-engine-gaps の A13 の why の「IR の文の定義は行単位」と、A15 の why の「flags.yaml を実現する」: 消える。理由は、形に依存しない部分（行単位で文を数える文書の形を検査したい、項目を節の下にしか宣言できない）だけを書いた
- ir-engine-gaps の A15 の「flags.yaml は節の下と文書の直下の両方を宣言する」: 消える。問題の記録の文書が無くなる

## テストの無い行

確認することの行はどれもテストが指す。ただし次の部分には、確かめるテストが無い。

- 「下限と上限を書く」の行の意味（範囲の内の個数なら指摘が出ない）: 今は C-schema-lang-10 を、下限と上限を同じ数で書いた出現回数がスキーマの誤りにならないことに絞った。範囲を確かめるなら、下限2・上限3の節を2個と3個置いて指摘が出ず、1個と4個で repeat_min_not_met と repeat_max_exceeded が出るテストが要る
- 「既定はちょうど1個」の2個の側: C-schema-lang-7 は無いときの欠落だけを書いた。2個あるときの指摘を確かめるテストは無い

## 誰が決めたか見分けられなかったもの

- 規則種別ごとの「置ける場所」の列の細部（フィールド行と箇条書きを箇条書きの子に置けること）は、A34（利用者）にある。フィールド行・文・箇条書き・表・コードブロックを前置部に置けることは A38（利用者）。ほかのセルを決めた行は記録に見つからなかったが、A39（利用者）が9つの規則種別を決め、表の形は散文の仕様から利用者が批准したものなので、表ごと残した。

## 理由や見送った案が記録に無かった決定

- 「スキーマは YAML で書き、題名・前置部・節・文書の直下の項目を根に置く」: 見送った案は R1 だけ。文書の直下の項目の見送った案は記録に無い。
- 「規則種別は9つにし、それぞれ置ける場所を決める」: A39、A34、A38 の理由が記録に無い（A12 の理由だけがある）。見送った案が記録に無い。
- 「スキーマに知らないキーや形に合わない値があれば、検査せずに停止する」: 見送った案が記録に無い。
- 「出現回数は5つの書き方で宣言する」: 理由が記録に無い（A40 は not recorded）。見送った案が記録に無い。
- 「出現回数の範囲を宣言したノードは、1件でも配列で抽出する」: 理由が記録に無い（A49 は not recorded）。見送った案が記録に無い。
- 「条件付き規則は、条件が真の間だけ添えた制約を効かせる」: 理由が記録に無い（A28 は not recorded）。見送った案が記録に無い。

## 分けた所、まとめた所

- REQ-schema-016 と TBL-schema-004 の出典は重なるが、根のノードを決めた A3・A5・A15・A23 と、規則種別を決めた A39・A12・A34・A38 で分けた。
- REQ-schema-020 と REQ-schema-021 は、同じ A28 から来ているので1つの決定にまとめた。
- REQ-schema-060 と PROP-schema-008 は、同じ読み方の宣言の決定にまとめた。
- PROP-schema-004 は、出現回数の書き方（A40）と出典が違う（A49）ので、別の決定にした。

## あとでリンクにする所

無し。前の版で挙げたものは、どれも見出しへのリンクにした。

- 「文書の骨格」の話題: 「スキーマは YAML で書き、…根に置く」の文書の直下の項目の読み方（2つの決定へ）と、「規則種別は9つにし、…」の前置部が抽出を宣言しないこと
- 「見出しの下の行の規則」の話題: 「読み方は段落を既定にし、…」の読み方ごとの読み分け
- 「抽出と素の構文木」の話題: 「出現回数の範囲を宣言したノードは、…」の箇条書き・表・区切り文字を宣言したフィールド行の形（2つの決定へ）
- 「CLI と結果の出し方」の話題: 「スキーマに知らないキーや形に合わない値があれば、…」の停止の知らせ方（停止の理由は5つにする）

## 迷った判断と、どう決めたか

- PROP-schema-004（範囲と配列の対応）を、この話題に置くか「抽出と素の構文木」の話題に置くか。性質はこの話題にあり、この話題の REQ-schema-019 を指す抽出のテストが確かめているので、ここに置いた（依頼でも、ここに置き、extraction では言い直さないと確かめた）。箇条書き・表・区切り文字の側は extraction の2つの決定へリンクした。extraction の文書は、この決定へリンクするだけで同じ対応を言い直していないことを確かめた。
- 知らないキーの決定を、「CLI と結果の出し方」の話題の停止の理由（TBL-schema-009 の「スキーマが形に合わない」）とどう分けるか。何を停止にするかはここに、停止をどう知らせるかは cli に置く、という分け方を依頼で確かめた。cli の停止の理由の表の "reading" の行は、この話題の「読み方は段落を既定にし、…」へのリンクにしてあり、条件を言い直していない。
- ほかの話題の対応表が、欠落の指摘だけを確かめるテスト（closed-world の missing_required_field_is_found、block-rules の missing_statement_is_found、missing_table_is_found、missing_codeblock_is_found）の付け替え先を、この話題の C-schema-lang-7 にした。どれも出現回数を書かないノードが無いときの欠落の指摘を確かめている。
- 「など」を2つ直した（C-4 と C-6 を「例えば」で始まる例の書き方にした）。
- EX-schema-008 は終了コード 2 を見るが、指しているテストはスキーマの読み込みが誤りになることしか見ない。確認することの行はテストが確かめることに合わせて、「スキーマの誤りとして読み込まれない」と書いた。
- 形に合わない値（型の違い、壊れた正規表現、YAML として読めない）は元の文書では REQ-schema-016 と REQ-schema-018 のテストに分かれていたが、どれも P1 の「形に合わない入力を黙って飛ばさない」から来るので、1行（C-schema-lang-4）にまとめた。
- 見送った案「kotowari が生の行を読んで文を補う」の理由「Markdown を読まない」は、IR の形の要求（REQ-core-169）を指していた。kotowari が Markdown を自分で読まないという決まりは形に依存しないと判断し、中身だけを書いた。
- 出現回数の指摘の種類の名前（repeat_min_not_met、repeat_max_exceeded）は元の文書には無いが、出典の A40（利用者）の文にあり、テストも確かめているので、決めたことと確認することに書いた。

## mds のスキーマの言葉を広げる決定（提案）

出どころ: mds の形の追加の設計案（リポジトリには置いていない） の A1〜A5 と、利用者の答え（1 から 6）。この節から下は、その追加で足したもの。元の ID は無い（設計が名指しする元の要求は「置き換える決定」の表の理由に書いた）。

### 置き換える決定

実装する側は、新しい決定を作ったときに、左の列の文と行を消すか直し、新しい決定の状態を「実装済み」にする。左の列の決定は、それまで実装済みのまま残す。

| 古い決定の見出しと、置き換わる部分 | 置き換える決定の見出しと行 | 理由 |
|---|---|---|
| 「スキーマは YAML で書き、題名・前置部・節・文書の直下の項目を根に置く」の決めたことの2段目（"document.item" と "item" の文） | 「項目の規則は "items" に一覧で書き、"item" のキーは受けない」（C-schema-lang-18、C-schema-lang-19） | 答え3（"items" だけ）。設計の REQ-schema-016 の変更。2段目は "document.items" と "items" に書き直す。C-schema-lang-1 は残る |
| 「規則種別は9つにし、それぞれ置ける場所を決める」の見出しの「9つ」、決めたことの表、C-schema-lang-2 | 「深さ4以上の見出しの規則 "subheading" を、前置部・節・項目に宣言できる」（C-schema-lang-25 が C-schema-lang-2 に代わる。C-schema-lang-26、C-schema-lang-27） | A4。表に小見出しの行を足し、見出しを10にする。TBL-schema-004 の変更 |
| 「closed-world.md」の「開いた世界は、宣言していない構造とその内側だけを許す」の決めたことの2段目の1つ目（宣言済みのノードの中に足した構造と行を許さない）と C-schema-closed-3 | 「前置部・節・項目・箇条書きの子は、"open: true" を宣言すると、規則の無いブロックを受ける」（C-schema-lang-20〜C-schema-lang-24） | A3。設計は REQ-schema-002 を「ノードに広げる」とし、REQ-schema-003 は変えないとする。文と行に「"open: true" を宣言しないノードの中」と限りを足す直しで済む。closed-world の文書はこの依頼で触っていない |
| 「document-structure.md」の「節は、見出しの文字で見分ける」の決めたことの1文目と C-schema-doc-10 | 「節と項目の規則は、見出しを名前の一覧か正規表現で選ぶ」（C-schema-lang-14〜C-schema-lang-17） | A1。REQ-schema-024 の変更。見出しの文字の読み方（打ち消し線の文字を残す）の文と C-schema-doc-11 は残る |

### 「提案」で書いた決定の一覧

| 決定 | 変えることになりそうなコード（設計の §1 から） |
|---|---|
| 節と項目の規則は、見出しを名前の一覧か正規表現で選ぶ | `crates/kotowari-markdown-schema/src/schema.rs` の `Section` と `Item`、`validate.rs` の `validate_items` の見出しの選び方、`schema/check.rs` の `reject_duplicate_names` |
| 項目の規則は "items" に一覧で書き、"item" のキーは受けない | `schema.rs` の `Document.item` と節の `item`。`crates/kotowari-core/schemas/plan.yaml`、`kotowari-mds` の見本のスキーマ、エンジンのテスト、README の書き換え |
| 前置部・節・項目・箇条書きの子は、"open: true" を宣言すると、規則の無いブロックを受ける | `validate.rs` の `validate_container` の `open` の引数（節と項目で `false` に固定している所）と `validate_children` |
| 深さ4以上の見出しの規則 "subheading" を、前置部・節・項目に宣言できる | `schema.rs` に規則の構造体を足す。`validate.rs` の深さ4以上の見出しの扱い |

### 実装する側が決める細部

- スキーマの停止の条件（設計の A1 の「New schema stops」）: 節の規則に "name" と "pattern" の両方か、どちらも無い。項目の規則に "id" と "name" か "pattern" を一緒に書く。ID の規則の後に規則を書く。1つの一覧に同じ名前が2度出る。どれも `schema_invalid` で止めることは cli の「停止の理由は5つにする」に入る。行にするなら、この話題の続きの番号で足す。
- 正規表現の規則の指摘の詳細に正規表現を入れること（設計 A1 の Findings）。
- 名前の規則と正規表現の規則が選んだ項目の抽出（`name` は見出しの全体、`id` は無い）は、document-structure の C-schema-doc-24 に書いた。
- "subheading" の正規表現に合わないときの指摘の種類と、"subheading" の出現回数の指摘が持つ規則種別の値（cli の「出現回数の指摘に、数えたノードの規則種別を持たせる」）。
- "open: true" のノードで受けたブロックが、抽出の値（節と項目の本文）に入るかどうか。設計は書いていない。

### 利用者に聞くこと

無い。

### 迷った判断

- "subheading" を置ける場所を前置部・節・項目にした。設計の A4 は「深さ4以上の見出しは、それを含むノードのブロックになる」とだけ書き、コードの所（`document.rs` の見出しを持ち主に入れる所）の持ち主が前置部・節・項目なので、そう読んだ。箇条書きの子（一覧の項目の中の見出し）は表に入れなかった。違うなら C-schema-lang-25 の行と表の1行を直す。
- 選び方の決定（A1）、"items"（答え3）、ノードの "open"（A3）、"subheading"（A4）を、依頼のとおりこの話題に置いた。深さ4以上の見出しの読み方は document-structure に、ID の規則の見出しの読み方と "document.items" も document-structure に置き、ここからリンクした。
- 設計の「見送った案」の割り当て: 正規表現だけの綴りの候補と "label" は選び方の決定に、順序付きリストの規則と完全な入れ物、深さ4以上の見出しも受ける "open" はノードの "open" の決定に置いた。
- ノードの "open" は、開いた世界と別の語にするため、文の中で「"open: true" を宣言したノード」と書き、「開いた」を使わなかった。
- 確認することの行は、スキーマと mds の指摘の種類だけで書き、kotowari は「問題」の動機にだけ出した。

## 数

- 元: 要求 7、場面 4、決定表 2、性質 2（ID は計15）
- 読んだ記録のファイル: 3（2026-09-21-mds-spec.md、2026-09-22-ir-engine.md、2026-09-23-ir-engine-gaps.md）
- 出来上がり: 決定 7、確認することの行 13（うち「（読んで確認）」1）
- 足した提案: 決定 4、確認することの行 14（C-schema-lang-14〜C-schema-lang-27。うち「（読んで確認）」1）
- テスト: 43
