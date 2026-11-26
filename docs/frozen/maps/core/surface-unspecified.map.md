# surface-unspecified の対応表

元: `docs/ir/core/surface-unspecified.md`（日本語は `surface-unspecified.ja.md`）。書き直した文書: `surface-unspecified.md`。

## 元の ID の表

| 元の ID | 新しい ID | 決定の見出し | 理由 |
|---|---|---|---|
| REQ-core-231 | C-core-unspec-5, C-core-unspec-8 | 一覧のファイルが無いか読めなければ、check と status は停止する | 停止することは A24（利用者）。停止の理由の種類（読めないファイル、UTF-8 でないファイル、設定の誤り）は A22（まとめて採用）だが、安全に関わる細部として決定に戻した（C-8）。詳細のパスと、空のファイルを0件にすることは落とした。鍵が無ければ0件で続けることは「まだ仕様にしていない面は…」の決めたことに書いた。"surface.rules" が空で鍵があるときの停止は面の検査の話題（REQ-core-225） |
| REQ-core-232 | C-core-unspec-1, C-core-unspec-2, C-core-unspec-6 | まだ仕様にしていない面は、理由を付けて未記載の面の一覧に載せれば誤りにしない | 一致した面を誤りにしないことを C-1、種類か名前が違えば一致しないことを C-2 にした。前後の空白を除かない完全な一致（A22）は、安全に関わる細部として決定に戻した（C-6）。同じ内容の1件を検査しないことは A22 なので落とした |
| REQ-core-233 | C-core-unspec-3, C-core-unspec-7 | 形の誤った1件は誤りにし、要らなくなった1件は注意にする | 鍵と値の組でない1件と、理由を空とみなすときに除く文字（前後の半角空白とタブ）は、安全に関わる細部として決定に戻した（C-7）。場所（一覧のファイル、line は null）と detail の形は落とした |
| REQ-core-234 | C-core-unspec-4 | 形の誤った1件は誤りにし、要らなくなった1件は注意にする | 「一致する面が IR にある」を「決定の文書に書かれた」に読み替えた（形に依存する要求）。場所と detail の形、形の誤った1件に出さないことは落とした |
| EX-core-419 | C-core-unspec-1 | まだ仕様にしていない面は、理由を付けて未記載の面の一覧に載せれば誤りにしない | "surface" の "unspecified" が 1 になることは面の検査の話題（REQ-core-228）の分 |
| EX-core-420 | C-core-unspec-2 | まだ仕様にしていない面は、理由を付けて未記載の面の一覧に載せれば誤りにしない | そのまま |
| EX-core-421 | C-core-unspec-3 | 形の誤った1件は誤りにし、要らなくなった1件は注意にする | 誤りが出て面が外れないことを C-3 にした。surface_unspecified_stale が出ないことは落とした細部で、テストの中に残る |
| EX-core-422 | C-core-unspec-4 | 形の誤った1件は誤りにし、要らなくなった1件は注意にする | そのまま |
| EX-core-423 | C-core-unspec-4 | 形の誤った1件は誤りにし、要らなくなった1件は注意にする | 「IR に書いた面」を「決定の文書に書かれた面」に読み替えた。"unspecified" が 0 になることは面の検査の話題の分 |
| EX-core-424 | C-core-unspec-5, C-core-unspec-8 | 一覧のファイルが無いか読めなければ、check と status は停止する | 停止することと、その理由が "unreadable file" であること（C-8） |
| EX-core-425 | C-core-unspec-5, C-core-unspec-8 | 一覧のファイルが無いか読めなければ、check と status は停止する | 停止することと、その理由が "config error" であること（C-8）。詳細のパスは落とした細部で、テストの中に残る |

元の文書に TBL と PROP は無い。

## テストごとの表

| テストのファイル | テストの関数名 | 今のマーカーの ID | 付け替えたあとの ID | 理由 |
|---|---|---|---|---|
| tests/step17_surface.rs | req_232_a_matching_entry_removes_the_error | REQ-core-232 | C-core-unspec-1 | 一覧に載った面に誤りが出ないことを確かめる。同じ1件を2つ書いても誤りにならないことも確かめるが、それは落とした細部 |
| tests/step17_surface.rs | ex_core_420_an_entry_of_another_kind_does_not_match | EX-core-420, REQ-core-232 | C-core-unspec-2 | 種類が違う1件は外さない |
| tests/step17_surface.rs | req_232_names_are_compared_without_trimming | REQ-core-232 | C-core-unspec-2, C-core-unspec-6 | 名前の後ろに空白がある1件は名前が違うので外さず、surface_without_spec が出る |
| tests/step17_surface.rs | ex_core_421_an_entry_whose_reason_is_blank_is_invalid_and_removes_nothing | EX-core-421, REQ-core-233, REQ-core-027, TBL-core-006, TBL-core-008 | C-core-unspec-3, C-core-unspec-7 | この話題の分だけ。理由が " \t " の1件が形の誤りになり、面を外さない。REQ-core-027、TBL-core-006、TBL-core-008 はほかの話題（指摘の形と終了コード）の ID |
| tests/step17_surface.rs | req_233_each_malformed_entry_is_invalid_with_the_written_kind_and_name | REQ-core-233 | C-core-unspec-3, C-core-unspec-7 | 形の誤りの各場合（文字列だけの1件を含む）に誤りが出ることを確かめる。detail の形は落とした細部で、テストの中に残る |
| tests/step17_surface.rs | ex_core_422_an_entry_for_a_surface_gone_from_the_code_is_a_notice | EX-core-422, REQ-core-234, REQ-core-027, TBL-core-009 | C-core-unspec-4 | この話題の分だけ。REQ-core-027、TBL-core-009 はほかの話題の ID |
| tests/step17_surface.rs | ex_core_423_an_entry_for_a_surface_now_in_the_ir_is_a_notice | EX-core-423, REQ-core-234 | C-core-unspec-4 | 仕様に書かれた面の1件が注意になる。テストの準備は IR の文書なので、付け替えのときに決定の文書の準備に直す必要がある |
| tests/step17_surface.rs | ex_core_424_a_missing_list_file_stops_as_unreadable | EX-core-424, REQ-core-231, TBL-core-001 | C-core-unspec-5, C-core-unspec-8 | この話題の分だけ。終了コード2で、標準エラーが "unreadable file: " で始まる。TBL-core-001 は終了コードの表（CLI の話題） |
| tests/step17_surface.rs | ex_core_425_a_list_that_is_not_a_sequence_stops_naming_the_list | EX-core-425, REQ-core-231, TBL-core-020 | C-core-unspec-5, C-core-unspec-8 | この話題の分だけ。最上位が並びでない一覧で "config error: " の停止。TBL-core-020 は停止の文言の表（CLI の話題） |
| tests/step17_surface.rs | req_231_a_list_that_is_not_yaml_or_not_utf8_stops_check_and_status | REQ-core-231, TBL-core-001, TBL-core-020 | C-core-unspec-5, C-core-unspec-8 | check と status の両方で、YAML でない一覧は "config error"、UTF-8 でない一覧は "non-UTF-8 file" の停止になることを確かめる。この話題の分だけ |
| tests/step17_surface.rs | req_231_a_blank_list_has_no_entries | REQ-core-231 | C-core-unspec-5 | 落とした細部のテスト。いちばん近い行を指す。空白と注釈だけのファイルを0件として読むこと（A22 の細部）を確かめ、停止しないことが C-5 の裏側に当たる |
| tests/step17_surface.rs | ex_core_419_a_listed_surface_is_not_an_error_and_is_counted | EX-core-419, REQ-core-232, REQ-core-228 | C-core-unspec-1 | この話題の分だけ。外した件数が 1 になることは REQ-core-228（面の検査の話題）の分 |
| tests/step17_surface.rs | ex_core_423_a_listed_surface_now_in_the_ir_is_not_counted | EX-core-423, REQ-core-228 | C-core-unspec-4 | 仕様に書かれた面の1件に注意が出ることを確かめる。この話題の分だけ。件数に数えないことは REQ-core-228 の分 |

## 戻した細部

| 細部 | 新しい ID | 確かめているもの |
|---|---|---|
| REQ-core-232: 一致は "kind" と "name" の完全な一致で、前後の空白を除かない（外す面を広げない） | C-core-unspec-6（「まだ仕様にしていない面は…」の決めたことに文を足した） | `tests/step17_surface.rs` の req_232_names_are_compared_without_trimming（"--verbose " の1件が "--verbose" の面を外さない） |
| REQ-core-233: 1件が鍵と値の組でないことを形の誤りにすること、"why" を空とみなすときに前後の半角空白とタブを除くこと | C-core-unspec-7（「形の誤った1件は誤りにし…」の決めたことに文を足した） | `tests/step17_surface.rs` の req_233_each_malformed_entry_is_invalid_with_the_written_kind_and_name（文字列だけの1件）と ex_core_421_an_entry_whose_reason_is_blank_is_invalid_and_removes_nothing（理由が " \t " の1件） |
| REQ-core-231: 停止の理由の種類（無いか読めない→読めないファイル、UTF-8 でない→UTF-8 でないファイル、YAML でないか最上位が並びでない→設定の誤り） | C-core-unspec-8（「一覧のファイルが無いか読めなければ…」の決めたことに文を足した） | `tests/step17_surface.rs` の ex_core_424_a_missing_list_file_stops_as_unreadable、ex_core_425_a_list_that_is_not_a_sequence_stops_naming_the_list、req_231_a_list_that_is_not_yaml_or_not_utf8_stops_check_and_status |

## 落とした細部

### 安全に関わるもの

無い（3件とも下の「戻した細部」で決定に戻した）。

### そのほか

| 要求のどの部分 | 誰が決めたと記録にあるか |
|---|---|
| REQ-core-231: 指す先が空（0バイトか注釈だけ）なら0件 | 記録に無い（A22 は等価の一覧に揃えるとだけ書く） |
| REQ-core-231: 設定の誤りの詳細に一覧のファイルの相対パスを出す | 利用者（A22、まとめて採用） |
| REQ-core-232: 同じ内容の1件が2つ以上あることを検査しない | 利用者（A22、まとめて採用） |
| REQ-core-233、REQ-core-234: "path" を一覧のファイル、"line" を null にする | 記録に無い |
| REQ-core-233、REQ-core-234: detail を書かれたままの "kind" と "name" を半角空白1つで区切った文字列にし、無いか文字列でない値は空の文字列にする | 利用者（A22、まとめて採用） |
| REQ-core-234: 形の誤った1件には surface_unspecified_stale を出さない | 記録に無い |

## 形に依存する要求

| 元の ID | どうなるか |
|---|---|
| REQ-core-232 | 新しい形で読み替えて残る。「一致した面が IR になくても」を「仕様（決定の文書）に書かれていなくても」と読む。今のコードは、IR の文書と決定の文書の両方から引用を集めている |
| REQ-core-234 | 新しい形で読み替えて残る。「一致する面が IR にある」を「その面が決定の文書に書かれている」と読む |
| EX-core-423 | 新しい形で読み替えて残る。準備の「要求の文に書き足した」を「決定の文書に書き足した」と読む。テストの準備も IR の文書から決定の文書に直す |

テストの無い行は無い。

## 誰が決めたか見分けられなかったもの

- 2026-09-27-surface-check.md の A15〜A22 は decided_by が「利用者（IR を書く段で決めた細部をまとめて採用）」である。利用者が1件ずつ決めたのか、まとめて示された細部を承認しただけなのかを、記録から見分けられない。描画の話題の対応表で「承認のときにまとめて示した」細部を落とした扱いに揃え、agent の細部として落とした。ただし指摘の種類の名前（A15）は、利用者から見える名前として残した。

## 理由や見送った案が記録に無かった決定

- 「まだ仕様にしていない面は、理由を付けて未記載の面の一覧に載せれば誤りにしない」: 見送った案が記録に無い（A4、A6 に rejected が無い）。
- 「形の誤った1件は誤りにし、要らなくなった1件は注意にする」: 見送った案が記録に無い。
- 「一覧のファイルが無いか読めなければ、check と status は停止する」: 停止すること自体の理由が記録に無い。A24 の why は、check と status だけに限る理由（面を読むのはこの2つだけ）である。問題の節に「理由は残っていない」と書いた。見送った案も記録に無い。

## 分けた所、まとめた所

- REQ-core-232 を C-1（一致した面を外す）と C-2（種類か名前が違えば外さない）に分けた。
- REQ-core-233 と REQ-core-234 は、等価の一覧に倣うという同じ理由（A6）なので、1つの決定にまとめた。
- REQ-core-231 の停止の場合（無い、UTF-8 でない、YAML でない、並びでない）と、EX-core-424、EX-core-425 を C-5 の1行にまとめた。
- REQ-core-231 の「鍵が無ければ0件で続ける」は、停止の決定ではなく、一覧の置き場の決定の文に入れた。確認することの行は置かなかった。

## あとでリンクにする所

- 済み: 面の検査の話題（surface）を指していた所（冒頭の段落、外した件数の出力、鍵だけある設定の停止、どのコマンドが面を読むか）は、見出しへのリンクにした。
- 「等価の一覧」の話題（`docs/ir/core/equivalents.md`）: 「まだ仕様にしていない面は…」と「形の誤った1件は…」の問題の節で、変異テストの等価の一覧に倣ったと書いた所

## 迷った判断と、どう決めたか

- 安全に関わる細部3件（完全な一致、形の誤りの条件、停止の理由の種類）は、A22（まとめて採用）の細部だが、今のテストが確かめているので、書き直しの依頼への利用者の答えどおり決定に戻した。問題の節には、落とすと何が起きるかと、理由が等価の一覧と同じ作りにすることのほかは記録に無いことを書いた。停止の詳細に一覧のファイルのパスを出すことは安全に関わらないので、落としたままにした。
- `req_231_a_blank_list_has_no_entries` は、この話題の ID だけを持つので、いちばん近い C-5 を指した。

- A15〜A22 の「まとめて採用」を、利用者の決定とするか agent の細部とするかで迷った。描画の話題で、承認のときにまとめて示した細部を落とした前例に揃えて落とし、A4、A6、A24 が裏付ける振る舞いだけを書いた。そのため、detail の形と停止の詳細のパスは決定から消え、テストの中にだけ残る（安全に関わる3件は上の「戻した細部」のとおり決定に戻した）。
- 「理由は必ず書く」と「理由が空の1件は形の誤り」は、A4（1件ごとに理由を必須）と A6（why を必須、形の壊れた件は誤り）から書いた。空白だけを空とみなすことは、安全に関わる細部として決めたことに戻した（C-7）。
- 「種類と名前の両方が同じ面だけを外す」は A6（1件は kind と name と why）と A4 から読める範囲として書いた。完全一致で空白を除かないことは、安全に関わる細部として決めたことに戻した（C-6）。
- 「鍵が無ければ一覧は無いものとして続ける」は、A4 の「置けるようにする」（置かなくてもよい）から書いた。
- 「面が仕様に書かれているか」の判定は、今のコードが IR の文書と決定の文書の両方を見ている。切り替えたあとは決定の文書だけになるので、決定の文書と書いた。

## 数

- 元: 要求 4、場面 7、決定表 0、性質 0
- 読んだ記録のファイル: 1（2026-09-27-surface-check.md）
- 出来上がり: 決定 3、確認することの行 8（うち「（読んで確認）」0）
- テスト: 13（付け替え先のあるもの 13、「-」0）
- 戻した安全の細部: 3（C-core-unspec-6、C-core-unspec-7、C-core-unspec-8）
