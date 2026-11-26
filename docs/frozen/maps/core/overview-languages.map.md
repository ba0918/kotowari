# overview-languages の対応表

元: `docs/ir/core/overview-languages.md`（日本語は `overview-languages.ja.md`）。書き直した文書: `overview-languages.md`。短い名前は `ovlang`。

## 元の ID の表

| 元の ID | 新しい ID | 決定の見出し | 理由 |
|---|---|---|---|
| REQ-core-353 | C-core-ovlang-1, C-core-ovlang-2, C-core-ovlang-3 | 全体像は、言語の一覧の言語ごとに、その言語の側から書く | そのまま。2026-10-05-localization.md の A10、A20、A21、A22（利用者）。ページの名前（元の REQ-core-293）は全体像の書き出しの話題。古い節を何と照らすかは `languages-and-guides.md` の「全体像の元データは、決定の文書を指す」（C-core-lang-9、提案）にあり、この決定にはその言語の側のマーカーから求めることだけを書いた |
| REQ-core-354 | C-core-ovlang-11 | ほかの言語のページでは、参照の本文をそのページの言語の側の決定の文書から取る | 読み替え（提案）。A34（利用者）。「IR の項目、シナリオ、問題の記録の項目を指す参照の本文を、そのページの言語の側から取る」を、決定を指す参照（決定の見出しか確認することの ID）の本文を、決定の文書のそのページの言語の側から取る、に読み替えた。判断の記録、ADR、判断の記録でない Markdown を指す参照の扱い（A34、A38）は消える（判断の記録と ADR は凍結した木にあり、check も全体像も読まない。`docs/decisions/core/decision-documents.md` の「決定の文書は docs/decisions に置き、古い形の文書は凍結した木へ移す」と `form-switch.md` の「ADR はやめ、今あるものは凍結する」） |
| REQ-core-355 | C-core-ovlang-4, C-core-ovlang-5 | 描画のエンジンには、ページの言語、その言語の UI の文字、ほかの言語の名前とページの置き場を渡す | そのまま。A9、A20、A32（利用者）。ほかの言語を言語の一覧の順に渡すことは、描画の入力の欄の形（D1、利用者が細部を LLM に任せた）なので落とした |
| REQ-core-351 | C-core-ovlang-6, C-core-ovlang-7 | UI の文字は、英語を kotowari が持ち、ほかの言語は設定の labels に書く | そのまま。A5、A7、A9、A21、A40（利用者） |
| REQ-core-352 | C-core-ovlang-8, C-core-ovlang-9, C-core-ovlang-10, C-core-ovlang-12, C-core-ovlang-13 | UI の文字の設定が欠けているか、数の入れ方が違えば、設定の誤りで停止する | そのまま。A7、A8、A21、A40（利用者）。"labels" に言語の一覧に無い言語タグの鍵があるときと、"labels.en" に表に無い鍵があるときの停止（D3、LLM）は、安全に関わる細部として C-12、C-13 に戻した |
| TBL-core-046 | C-core-ovlang-6, C-core-ovlang-7 | UI の文字は、英語を kotowari が持ち、ほかの言語は設定の labels に書く | そのまま。鍵の名前は D1（利用者が細部を LLM に任せた）で決まったが、設定のキーとして利用者から見える名前なので残した。鍵が要ること、使う所、数を入れるかは A5、A8（利用者）が裏付ける。英語の文字の列は D3（LLM）なので落とした |
| EX-core-536 | C-core-ovlang-6 | UI の文字は、英語を kotowari が持ち、ほかの言語は設定の labels に書く | そのまま |
| EX-core-537 | C-core-ovlang-8 | UI の文字の設定が欠けているか、数の入れ方が違えば、設定の誤りで停止する | そのまま |
| EX-core-538 | C-core-ovlang-10 | UI の文字の設定が欠けているか、数の入れ方が違えば、設定の誤りで停止する | そのまま |
| EX-core-539 | C-core-ovlang-12 | UI の文字の設定が欠けているか、数の入れ方が違えば、設定の誤りで停止する | そのまま。"labels" に言語の一覧に無い言語タグがあるときの停止（D3、LLM）を、安全に関わる細部として戻した |
| EX-core-540 | C-core-ovlang-1, C-core-ovlang-5 | 全体像は、言語の一覧の言語ごとに、その言語の側から書く | そのまま |
| EX-core-541 | C-core-ovlang-3 | 全体像は、言語の一覧の言語ごとに、その言語の側から書く | そのまま |
| EX-core-542 | C-core-ovlang-11 | ほかの言語のページでは、参照の本文をそのページの言語の側の決定の文書から取る | 読み替え（提案）。準備の "REQ-001" を、決定の文書の対の決定に読み替える。"docs/decision/records/r.md#A1" を指す参照の部分は消える |

元の文書に PROP は無い。

## テストごとの表

| テストのファイル | テストの関数名 | 今のマーカーの ID | 付け替えたあとの ID | 理由 |
|---|---|---|---|---|
| crates/kotowari-core/tests/translations.rs | req_core_351_english_is_the_built_in_text_with_overrides_and_others_are_the_labels | REQ-core-351, TBL-core-046 | C-core-ovlang-6, C-core-ovlang-7 | 英語は上書きを重ねたもの、日本語は "labels.ja" の値そのものになることを確かめる。英語の文字の値は落とした細部で、テストの中に残る |
| crates/kotowari-core/tests/translations.rs | req_core_335_invalid_tags_stop_even_with_complete_labels | REQ-core-335, REQ-core-352 | - | この話題の分は無い。"labels" が揃っていても言語タグの形の誤りで止まることを確かめるテストで、対の話題の対応表が C-core-pairs-3 を付けるので、マーカーは無くならない |
| crates/kotowari-core/tests/translations.rs | req_core_352_unknown_keys_and_a_doubled_number_placeholder_stop | REQ-core-352 | C-core-ovlang-9, C-core-ovlang-10, C-core-ovlang-13 | "labels.ja" の表に無い鍵、"labels.en" の表に無い鍵、"{n}" が2つの値で止まることを確かめる |
| tests/step1_config.rs | ex_core_537_labels_of_a_language_other_than_english_missing_a_key_stops | REQ-core-352, EX-core-537 | C-core-ovlang-8 | |
| tests/step1_config.rs | ex_core_538_a_counted_text_without_the_number_placeholder_stops | EX-core-538 | C-core-ovlang-10 | |
| tests/step1_config.rs | ex_core_539_labels_of_a_language_not_in_the_list_stops | EX-core-539 | C-core-ovlang-12 | "languages" の鍵が無い設定に "labels.ja" があると止まる |
| tests/step21_translations.rs | three_languages_keep_their_own_html_language_and_contents | REQ-core-288, REQ-core-293, REQ-core-353, REQ-core-355 | C-core-ovlang-2, C-core-ovlang-4 | この話題の分だけ。言語ごとの一覧の題名と lang の属性を確かめる。REQ-core-288 と REQ-core-293 の分は全体像の元データの話題と全体像の書き出しの話題 |
| tests/step21_translations.rs | a_linked_language_directory_stops_before_any_page_is_written | REQ-core-296, REQ-core-324, REQ-core-353 | - | この話題の分は無い。言語のディレクトリがシンボリックリンクのときに何も書かずに止まることを確かめる、キャッシュの置き場の安全のテストで、全体像のコマンドの話題の対応表が C-core-ovcmd-5 を付けるので、マーカーは無くならない |
| tests/step21_translations.rs | an_inaccessible_language_cache_reports_a_cache_error | REQ-core-324, REQ-core-353 | - | 同上（C-core-ovcmd-7 が付く） |
| tests/step21_translations.rs | ex_core_536_english_is_the_built_in_text_with_the_labels_on_top | REQ-core-351, EX-core-536 | C-core-ovlang-6 | |
| tests/step21_translations.rs | ex_core_540_english_pages_are_written_under_en_and_link_each_other | REQ-core-353, REQ-core-355, EX-core-540 | C-core-ovlang-1, C-core-ovlang-5 | |
| tests/step21_translations.rs | ex_core_541_one_language_writes_no_language_directory | EX-core-541 | C-core-ovlang-3, C-core-ovlang-7 | "1 ページ" で、"labels.ja" の値で描かれることも確かめる |
| tests/step21_translations.rs | ex_core_542_english_pages_take_ir_bodies_from_the_english_side_and_no_record_bodies | REQ-core-354, EX-core-542 | C-core-ovlang-11 | 実装のときに直す。準備の IR の対を決定の文書の対に、参照を決定に直す。判断の記録の参照の部分は古い形のコードと一緒に消す |

## 落とした細部

### 安全に関わるもの

無い。書き直しの決まり（安全に関わる落とした細部は決定に戻す）で、ここにあった2件を決定に戻した（下の「戻した細部」）。

### 戻した細部

| 細部 | 新しい ID | 確かめているもの |
|---|---|---|
| REQ-core-352: 設定の "labels" に、言語の一覧に無い言語タグの鍵があれば、設定の誤りで停止する | C-core-ovlang-12（決定「UI の文字の設定が欠けているか、…」） | テスト ex_core_539_labels_of_a_language_not_in_the_list_stops（"languages" の鍵が無く英語だけの設定に "labels.ja" があると止まる） |
| REQ-core-352: "labels.en" に表に無い鍵があれば、設定の誤りで停止する | C-core-ovlang-13（同じ決定） | テスト req_core_352_unknown_keys_and_a_doubled_number_placeholder_stop の2つ目の場合（"labels.en" の "unknown"） |

### そのほか

| 要求のどの部分 | 誰が決めたと記録にあるか |
|---|---|
| TBL-core-046: 英語の文字の列（"English"、"Overview"、"{n} pages"、"Not reviewed since the IR changed" など） | LLM（D3。D1 の範囲の細部） |
| REQ-core-355: ほかの言語ごとの名前とパスを、言語の一覧の順に渡す | LLM（D1 の「view の描画の入力に足す欄の形」） |

## 形に依存する要求

| 元の ID | どうなるか |
|---|---|
| REQ-core-354 | 新しい形で読み替えて残る（提案）。IR の項目、シナリオ、問題の記録の項目を指す参照を、決定の見出しか確認することの ID を指す参照に読み替える。判断の記録、ADR、判断の記録でない Markdown を指す参照の本文の扱いは消える（`decision-documents.md` の「決定の文書は docs/decisions に置き、古い形の文書は凍結した木へ移す」、`form-switch.md` の「ADR はやめ、今あるものは凍結する」） |
| EX-core-542 | 読み替えて残る（提案）。判断の記録の部分は消える |
| TBL-core-046 の superseded と deferred | 鍵は残る。view が描画の入力にそろった UI の文字の一式を求め（`docs/decisions/view/languages.md`）、view には今も置き換え済みと後回しの参照を描く決定があるため。切り替えのあと、kotowari は置き換え済みの参照を作らず、後回しは「提案」の決定を指す参照に付く（利用者の答え。overview-commands の C-core-ovcmd-29、提案） |

### テストの無い行

無い。

## 誰が決めたか見分けられなかったもの

- 無い。2026-10-05-localization.md の A 行は decided_by が利用者、D1 は利用者が細部を LLM に任せた承認、D2 と D3 は LLM が決めた細部。

## 理由や見送った案が記録に無かった決定

- 「描画のエンジンには、…を渡す」: 見送った案が記録に無い。
- 「UI の文字の設定が欠けているか、…設定の誤りで停止する」: 見送った案が記録に無い。戻した2つの停止（D3）の理由も記録に無く（D3 の why は「決めていなかった境目を埋める」）、「記録に残っていない」と書いた。
- 「ほかの言語のページでは、参照の本文を…」: 見送った案が記録に無い。A34 の rejected（ほかの言語のページでも判断の記録の決定の文を原文のまま開く）は判断の記録の参照についてのもので、消える部分なので書かなかった。理由は、A34 の why（訳さない文書の本文をほかの言語のページで開くと読めない文になる）の中身から書いた。

## 分けた所、まとめた所

- REQ-core-351 と TBL-core-046 を1つの決定にした（鍵の表そのものが決定）。
- REQ-core-352 は理由（A7、A8）が UI の文字の決め方と重なるが、停止の条件という別の決まりなので、独立の決定にした。
- REQ-core-354 は、残る部分（ページの言語の側から本文を取る）だけを提案の決定にした。

## あとでリンクにする所

- 無い。view の languages、languages-and-guides の2つの決定、overview-data の「部品の中の参照は、…」、overview-commands の「参照の表には、…」と「overview build は、毎回すべてを描き、…」は、見出しか文書へのリンクにした。

## 迷った判断と、どう決めたか

- 鍵の名前は LLM が決めたが（D1）、設定のキーとして利用者から見えるので、表に残した。表に書いた振る舞いは、view が書く文字のすべてを UI の文字にすること（A5）と、数を入れる文字の形（A8）という利用者の決定が裏付けるものだけにした。
- superseded と deferred の鍵は残した。参照の状態の問いへの利用者の答え（書き直しの依頼で受けたもの）で、kotowari は「置き換え済み」の参照を作らず、「後回し」は「提案」の決定への参照に付ける（overview-commands の C-core-ovcmd-29）。deferred の鍵は使い道が残り、superseded の鍵は kotowari からは使われなくなる。view の側の決定が今もその2つのマーカーを持ち、kotowari はそろった一式を渡す決まり（A9）なので、鍵を外すと view の入力が欠ける。外すかどうかは、view の決定と一緒に決めることで、この文書では決めない。
- 参照の本文をページの言語の側から取る決定は、決定の見出しを指す参照が側ごとにどう対応するかで迷った。ガイドのマーカーは、どの側でも先頭の言語の側の決定の文書のパスと見出しの行き先の名前を書く（骨組みとして側どうしで同じ）ので、全体像の参照も同じ書き方になる。本文はそのページの言語の側の「同じ決定」から取る、とだけ書き、どう対応させるかは実装する側の細部にした。

## 実装する側が決める細部

- 先頭の言語の側の見出しの行き先の名前で書いた参照を、ほかの言語の側の決定にどう対応させるか（対の骨組みが同じことを使い、決定の並びの位置で対応させる、など）。
- stale_mark の英語の文字（今は "Not reviewed since the IR changed"）は IR に触れているので、切り替えのときに直すかどうか。文字は D3（LLM）の細部で、決定には書いていない。

## 利用者に聞くこと

- 無い。

## 「提案」で書いた決定の一覧

| 決定 | 今のコードのどこを変えることになりそうか |
|---|---|
| ほかの言語のページでは、参照の本文をそのページの言語の側の決定の文書から取る | `crates/kotowari-overview/src/references.rs` の `Resolver::for_language` が IR の対の側から本文を集めているのを、決定の文書の対の側（`crates/kotowari-core/src/decisions.rs` が読む決定）から集めるようにし、`crates/kotowari-overview/src/lib.rs` の言語ごとの組み立てで渡す |

## 数

- 元: 要求 5、場面 7、決定表 1、性質 0
- 読んだ記録のファイル: 1（2026-10-05-localization.md）
- 出来上がり: 決定 5（実装済み 4、提案 1）、確認することの行 13（うち「（読んで確認）」0）
- 戻した安全の細部: 2件（C-core-ovlang-12、C-core-ovlang-13）
- テスト: 13（付け替え先のあるもの 10、「-」3。「-」の3本はほかの話題の対応表が ID を付ける）
