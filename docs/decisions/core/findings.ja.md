# 指摘の種類

[English](findings.md) | 日本語

kotowari のコマンドが出す指摘の種類ごとに、重さ（誤りか注意か）と、detail に何を入れるかを扱う。それぞれの種類がどんなときに出るかは、その種類を決めた話題で決める。指摘の並べ方と line は[指摘の並べ方と行](finding-order.ja.md)、指摘の JSON の鍵と文字の出力の形は[出力の形](output.ja.md)で決める。

## 指摘は誤りと注意に分け、注意にするのは表の種類だけにする

状態: 実装済み

### 問題

指摘の種類ごとに、終了コードを変える誤りにするか、変えない注意にするかを決める必要があった。

ガイドの節が古くなったことは、気付くべきことではあるが、ガイドが遅れているだけで CI が落ちるのは重い。注意なら status の complete を妨げず、check と status の数には残り続けるので、放置されたガイドはいつでも見える。未記載の面の一覧の、要らなくなった1件は、変異テストの等価の一覧の要らなくなった1件と同じ扱いにする。

### 決めたこと

指摘は、誤りか注意のどちらかの重さを持つ。注意は、終了コードを変えない指摘である。注意だけが出たとき、check は誤りが無いときと同じ終了コードで終わる（終了コードの一覧は[終了コードは、誤りが無ければ0、誤りがあれば1、停止したら2にする](cli.ja.md#終了コードは誤りが無ければ0誤りがあれば1停止したら2にする)で決める）。

注意にする種類は、次の表のものだけにする。表に無い種類の指摘は、すべて誤りにする。

| 種類 | 何に出すか | 出る条件を決める所 |
|---|---|---|
| mutant_timeout | 時間切れの変異 | [見逃しは誤り、時間切れは注意にし、捕まえた変異とビルド不能の変異には何も出さない](mutants.ja.md#見逃しは誤り時間切れは注意にし捕まえた変異とビルド不能の変異には何も出さない) |
| equivalent_stale | 文面がソースから無くなった、等価の一覧の1件 | [文面がソースから無くなった1件には、equivalent_stale の注意を出す](equivalents.ja.md#文面がソースから無くなった1件にはequivalent_stale-の注意を出す) |
| guide_stale | 指していた決定が変わったか無くなった、ガイドのマーカーの1件 | [ガイドは、決めたことと確認することが変わったら古いとする](languages-and-guides.ja.md#ガイドは決めたことと確認することが変わったら古いとする) |
| surface_unspecified_stale | 要らなくなった、未記載の面の一覧の1件 | [形の誤った1件は誤りにし、要らなくなった1件は注意にする](surface-unspecified.ja.md#形の誤った1件は誤りにし要らなくなった1件は注意にする) |

### 見送った案

- guide_stale を誤りにする。ガイドが遅れているだけで CI が落ちる。

### 確認すること

- C-core-findings-1 kotowari が出す指摘の種類のうち、注意の表にある種類だけが severity "notice" を持ち、ほかの種類はすべて severity "error" を持つ
- C-core-findings-2 注意だけが出た check は、注意を出力に残したまま、終了コード0で終わる

## 指摘の detail は、その指摘を特定する短い文字列にし、種類ごとに表で決める

状態: 実装済み

### 問題

種類ごとに、detail に何を入れるかを決める必要があった。

どの文書への指摘かは path で区別できるので、detail には path と重ねて書かない。名前の取れないテストへの指摘の detail が空の文字列では、どのテストかが分からない。ガイドの古さの注意は、その1行だけで、どの行き先を何に書き換えればよいかまで分かるようにする。

### 決めたこと

指摘の detail は、その指摘を特定する短い文字列にする。この文書で detail を決める種類は、次の表のとおりにする。「行の文字」は、読んだ行の文字そのまま（字下げと末尾の空白を含み、組み立て直さない）である。

| 種類 | 重さ | detail |
|---|---|---|
| duplicate_id | 誤り | ID |
| invalid_id | 誤り | 書かれた値 |
| unresolved_reference | 誤り | マーカーに書かれた ID |
| test_without_id | 誤り | テストの名前。名前の取れないテストでは、テストの節の最初の行の全体の文字から前後の空白を除いたもの |
| invalid_marker | 誤り | マーカーのある行の文字 |
| unparsable_file | 誤り | ファイルのパス |
| surface_without_spec | 誤り | 面の種類と名前を、1つの半角空白で区切った文字列 |
| overview_form_invalid | 誤り | frontmatter の形の違反では "frontmatter" |
| overview_toc_invalid | 誤り | 目次の中の、合わなかった場所の JSON Pointer |
| overview_toc_page_missing | 誤り | 目次に無いページの名前 |
| overview_toc_page_unknown | 誤り | その項目の JSON Pointer |
| overview_toc_page_duplicate | 誤り | その項目の JSON Pointer |
| overview_toc_group_empty | 誤り | その目次の群の JSON Pointer |
| multiple_titles | 誤り | 2つ目からの題名の文字から、"#" と前後の空白を除いたもの |
| guide_stale | 注意 | マーカーの1件の行き先、その1件に書かれた指紋、今の指紋を、1つの半角空白で区切った文字列。行き先の決定が無ければ、今の指紋は "-" |

次の種類の detail と出る条件は、それぞれの話題で決める。どれも誤りである。

| 種類 | 何の誤りか | 決める所 |
|---|---|---|
| mutant_survived | テストが見逃した変異 | [見逃しは誤り、時間切れは注意にし、捕まえた変異とビルド不能の変異には何も出さない](mutants.ja.md#見逃しは誤り時間切れは注意にし捕まえた変異とビルド不能の変異には何も出さない) |
| equivalent_invalid | 等価の一覧の、形の誤った1件 | [形の誤った1件は equivalent_invalid の誤りにし、どの見逃しも外さない](equivalents.ja.md#形の誤った1件は-equivalent_invalid-の誤りにしどの見逃しも外さない) |
| invalid_plan | 形から外れた計画書 | [形の指摘は invalid_plan の1種類にまとめ、スキーマの側の種類と詳細を detail に入れる](plan.ja.md#形の指摘は-invalid_plan-の1種類にまとめスキーマの側の種類と詳細を-detail-に入れる) |
| surface_unspecified_invalid | 未記載の面の一覧の、形の誤った1件 | [形の誤った1件は誤りにし、要らなくなった1件は注意にする](surface-unspecified.ja.md#形の誤った1件は誤りにし要らなくなった1件は注意にする) |
| confirmation_without_test | 実装済みの決定の、テストが指していない確認することの行 | [check は、確認することの行ごとに、それを指すテストがあるかを数える](confirmations.ja.md#check-は確認することの行ごとにそれを指すテストがあるかを数える) |
| decision_state_invalid | 状態の行が無いか、「提案」でも「実装済み」でもない決定 | [1つの決定は、問題・決めたこと・見送った案・確認することの文章で書く](decision-documents.ja.md#1つの決定は問題決めたこと見送った案確認することの文章で書く) |
| decision_section_missing | 「問題」か「決めたこと」の節が無い決定 | 同上 |
| topic_lead_missing | 題名のあとに、話題が何を扱うかの段落が無い文書 | 同上 |
| missing_title | 題名の無い決定の文書 | 同上。detail は[題名の無い決定の文書の missing_title は、detail を文書名にする](#題名の無い決定の文書の-missing_title-はdetail-を文書名にする) |
| unclosed_code_block | 閉じられずに終わるコードブロック | [閉じないコードブロックは unclosed_code_block の誤りにし、開始から文書の終わりまでを読まない](ir-input.ja.md#閉じないコードブロックは-unclosed_code_block-の誤りにし開始から文書の終わりまでを読まない) |
| id_domain_mismatch | まとまりの名前が、置いてあるまとまりのフォルダと合わない確認することの ID | [確認することの ID のまとまりの名前が、置いてあるまとまりのフォルダと合わなければ、id_domain_mismatch の誤りにする](#確認することの-id-のまとまりの名前が置いてあるまとまりのフォルダと合わなければid_domain_mismatch-の誤りにする) |
| link_target_missing | 無い文書か無い見出しを指す、決定の文書の中のリンク | [ほかの決定を指すときは、見出しへリンクを張る](decision-documents.ja.md#ほかの決定を指すときは見出しへリンクを張る) |
| link_to_record | 決定の文書かその対の側の中の、凍結した木（"docs/frozen/"）の下を指すリンク（今の名前。読み替えるか改名するかは実装する側が決める） | [決定の文書とその対の側から、凍結した木へのリンクは誤りにする](translation-structure.ja.md#決定の文書とその対の側から凍結した木へのリンクは誤りにする) |
| vague_word | 曖昧語の一覧にある語を含む文 | [用語集は人が読む文書として残し、用語の検査はやめる](decision-documents.ja.md#用語集は人が読む文書として残し用語の検査はやめる) |
| overview_part_unknown | 全体像の元データの、知らない種類の部品 | [部品の中身は、kotowari の側が view の部品のスキーマで検査する](overview-data.ja.md#部品の中身はkotowari-の側が-view-の部品のスキーマで検査する) |
| overview_part_invalid | 全体像の元データの、読めないか形の合わない部品 | 同上 |
| overview_lead_missing | 冒頭の部品の無い全体像の元データ | [題名の直後の最初の部品は、lead にする](overview-data.ja.md#題名の直後の最初の部品はlead-にする) |
| overview_ir_missing | 全体像の元データの一覧の、無い決定の文書を指す1件（今の名前。改名する） | [全体像の元データは、決定の文書を指す](languages-and-guides.ja.md#全体像の元データは決定の文書を指す) |
| overview_ir_shared | 2つ以上の全体像の元データの一覧にある決定の文書（今の名前。改名する） | [1つの決定の文書は、多くても1つの全体像の元データの一覧に入れる](overview-data.ja.md#1つの決定の文書は多くても1つの全体像の元データの一覧に入れる) |
| overview_ref_unresolved | 全体像の部品の中の、解決できない参照 | [部品の中の参照は、refs と ref の欄に書く](overview-data.ja.md#部品の中の参照はrefs-と-ref-の欄に書く) |
| overview_name_conflict | 名前が重なる全体像の元データ | [overview build は、毎回すべてを描き、中身の違うファイルだけを書き、作られなくなったファイルを消す](overview-commands.ja.md#overview-build-は毎回すべてを描き中身の違うファイルだけを書き作られなくなったファイルを消す) |
| translation_missing | 対のどれかの言語の側が無い文書 | [ガイド、全体像の元データ、目次は、言語の一覧のすべての言語の側を持つ](translation-pairs.ja.md#ガイド全体像の元データ目次は言語の一覧のすべての言語の側を持つ) |
| translation_record_invalid | 形の誤った一致の記録 | [一致の記録は、最後に同じ中身だと確かめたときの各側の blob hash を持つ](translation-pairs.ja.md#一致の記録は最後に同じ中身だと確かめたときの各側の-blob-hash-を持つ) |
| translation_stale | どれかの側の今の中身が、一致の記録と違う対 | [側を直して一致の記録を直していなければ、その側を古いとする](translation-pairs.ja.md#側を直して一致の記録を直していなければその側を古いとする) |
| translation_structure_mismatch | 骨組みが側どうしで違う対 | [対の側どうしは、自然言語の文のほかの骨組みが一致しなければならない](translation-structure.ja.md#対の側どうしは自然言語の文のほかの骨組みが一致しなければならない) |
| translation_switcher_invalid | 言語の切り替えの行が無いか、形か行き先が違う側 | [ガイドの対の各側は、題名の直後に切り替えの行を持つ](translation-structure.ja.md#ガイドの対の各側は題名の直後に切り替えの行を持つ) |
| link_language_mismatch | 対の文書へのリンクが、ほかの言語の側を指す | [ガイドと全体像の元データの中のリンクは、同じ言語の側を指す](translation-structure.ja.md#ガイドと全体像の元データの中のリンクは同じ言語の側を指す) |

### 見送った案

- frontmatter の形の違反の detail に、形の検査の側が持つ、ほかの意味の種類の名前を借りる。形の検査の側は frontmatter を検査しないので当てはまる名前が無く、別の意味の名前を借りると読み違える。

### 確認すること

- C-core-findings-3 形の誤ったテストのマーカーとガイドのマーカーには、マーカーのある行の文字そのまま（字下げを含む）を detail にした invalid_marker が出る
- C-core-findings-4 形の誤った確認することの ID には書かれた値を detail にした invalid_id が、どの文書にも無い ID を指すマーカーにはその ID を detail にした unresolved_reference が出る
- C-core-findings-5 名前の取れないテストへの test_without_id の detail は、テストの節の最初の行の全体の文字から前後の空白を除いたものになる
- C-core-findings-6 決定の文書に出てこない面への surface_without_spec の detail は、面の種類と名前を1つの半角空白で区切った文字列になる（"flag --verbose"）
- C-core-findings-7 ガイドのマーカーの1件が変わった決定を指すと、detail が行き先・書かれた指紋・今の指紋の guide_stale が出て、無い決定を指すと今の指紋が "-" になる
- C-core-findings-13 決定の文書の2つ目からの題名（"#  二つ目 "）への multiple_titles の detail は、"#" と前後の空白を除いた題名の文字（"二つ目"）になる

## 同じ ID が何か所にもあれば、2つ目以降の場所ごとに duplicate_id を出す

状態: 実装済み

### 問題

同じ ID の確認することの行が2つあれば誤りにすることは、[check は、確認することの行ごとに、それを指すテストがあるかを数える](confirmations.ja.md#check-は確認することの行ごとにそれを指すテストがあるかを数える)で決めている。ここでは、何か所にもあるときに、どこに何件出すかを決める。理由は残っていない。

### 決めたこと

同じ ID が2か所以上にあるとき、2つ目以降の場所ごとに1件ずつ duplicate_id を出し、line はその場所の行にする。1つ目の場所には出さない。

1つ目は、パスのバイト順で先の文書の場所、同じ文書の中では行の小さい場所とする。

### 確認すること

- C-core-findings-8 同じ ID が3か所にあると duplicate_id が2件出て、1つ目の場所には出ない
- C-core-findings-9 2つの文書に同じ ID があると、パスのバイト順で後の文書の、その ID の行に duplicate_id が出る

## 題名の無い決定の文書の missing_title は、detail を文書名にする

状態: 提案

### 問題

題名の無い文書への missing_title の detail は、ディレクトリを除いたファイル名にしている。どのディレクトリの文書かは path が示す。決定の文書を読む check は、題名の無い文書の missing_title の detail を空の文字列にしていて、この決まりと合っていない。文書名にした理由は残っていない。

### 決めたこと

題名の無い決定の文書には、line を null にし、detail をその文書のディレクトリを除いたファイル名にした missing_title を出す。

### 確認すること

- C-core-findings-10 置き場の下のディレクトリにある、題名の無い決定の文書には、line が null で、detail がディレクトリを除いたファイル名の missing_title が出る

## 確認することの ID のまとまりの名前が、置いてあるまとまりのフォルダと合わなければ、id_domain_mismatch の誤りにする

状態: 提案

### 問題

確認することの ID には、まとまりの名前を含める（`C-<まとまり>-<話題の短い名前>-<連番>`）。名前を書く人に任せたので、名前の正しさを見る者が要る。まとまりの名前と、文書を置いたまとまりのフォルダが合っているかを見れば、まとまりの取り違えと、文書の置き違いの両方が機械で捕まり、名前の決まりが崩れない。

決定の文書を読む今のコードは、確認することの ID の形だけを見て、まとまりのフォルダと合っているかは見ない。

### 決めたこと

決定の文書の置き場の直下のディレクトリ（まとまりのフォルダ）の下にある決定の文書で、確認することの ID のまとまりの名前が、そのフォルダの名前と違うとき、その確認することの行に id_domain_mismatch の誤りを出す。置き場の直下にある文書の ID は、この検査で見ない。

### 見送った案

- 一致を検査せず、名前の決まりに任せる。暗黙の約束が増える。

### 確認すること

- C-core-findings-11 置き場の "core" のフォルダにある決定の文書の確認することの行の ID が "C-view-" で始まると、その行に id_domain_mismatch の誤りが出て、"C-core-" で始まる行には出ない
- C-core-findings-12 置き場の直下にある決定の文書の確認することの行には、ID のまとまりの名前が何でも id_domain_mismatch が出ない
