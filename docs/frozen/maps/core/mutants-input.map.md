# mutants-input の対応表

元: `docs/ir/core/mutants-input.md`（日本語は `mutants-input.ja.md`）。書き直した文書: `mutants-input.md`。確認することの ID の短い名前は `mutin`。

## 元の ID の表

| 元の ID | 新しい ID | 決定の見出し | 理由 |
|---|---|---|---|
| REQ-core-138 | C-core-mutin-1, C-core-mutin-2, C-core-mutin-3, C-core-mutin-4 | 変異の結果は道具を知らない形で持ち、cargo-mutants の結果のファイルを表のとおりに写す | 定義の TBL-core-024 の行ごとに分けた。項目の写し方を C-1、結果の値を C-2、基準の実行が無いことと知らない鍵を C-3、パスの正規化を C-4 にした |
| REQ-core-144 | C-core-mutin-5, C-core-mutin-6, C-core-mutin-7 | 形の合わない結果のファイルでは、結果の誤りで停止する | 結果の誤りの条件を C-5 にまとめ、基準のディレクトリの外を指すパスを C-6、結果のファイルが無いときを C-7 にした。「基準の実行が1件も無い結果では停止しない」と「表に無い鍵は見ない」は C-3 |
| TBL-core-024 | C-core-mutin-1, C-core-mutin-2, C-core-mutin-4 | 変異の結果は道具を知らない形で持ち、cargo-mutants の結果のファイルを表のとおりに写す | 表は「決めたこと」に Markdown の表で書いた。REQ-core-110 の正規化は「設定のパスと同じ正規化」と言葉で書いた |
| EX-core-207 | C-core-mutin-5 | 形の合わない結果のファイルでは、結果の誤りで停止する | 値違いの場面（EX-core-207、208、222〜225）を C-5 の1行にまとめた |
| EX-core-208 | C-core-mutin-5 | 形の合わない結果のファイルでは、結果の誤りで停止する | 同上 |
| EX-core-222 | C-core-mutin-5 | 形の合わない結果のファイルでは、結果の誤りで停止する | 同上 |
| EX-core-223 | C-core-mutin-5 | 形の合わない結果のファイルでは、結果の誤りで停止する | 同上 |
| EX-core-224 | C-core-mutin-5 | 形の合わない結果のファイルでは、結果の誤りで停止する | 同上 |
| EX-core-225 | C-core-mutin-5 | 形の合わない結果のファイルでは、結果の誤りで停止する | 同上 |
| EX-core-226 | C-core-mutin-6 | 形の合わない結果のファイルでは、結果の誤りで停止する | そのまま |
| EX-core-227 | C-core-mutin-3 | 変異の結果は道具を知らない形で持ち、cargo-mutants の結果のファイルを表のとおりに写す | そのまま |

元の文書に PROP は無い。

## テストごとの表

| テストのファイル | テストの関数名 | 今のマーカーの ID | 付け替えたあとの ID | 理由 |
|---|---|---|---|---|
| tests/step9_mutants.rs | req_144_unknown_outcome_value_stops | REQ-core-144, EX-core-207 | C-core-mutin-5 | 知らない "summary" の値で終了コード2、標準エラーが "results error: outcomes.json: " で始まる |
| tests/step9_mutants.rs | req_144_failed_baseline_stops | REQ-core-144, EX-core-208 | C-core-mutin-5 | 基準の実行の "Failure" で結果の誤り |
| tests/step9_mutants.rs | req_144_broken_json_stops | REQ-core-144, EX-core-222 | C-core-mutin-5 | JSON として読めない |
| tests/step9_mutants.rs | req_144_wrong_key_type_stops | REQ-core-144, TBL-core-024, EX-core-223 | C-core-mutin-5 | "span.start.line" が文字列 |
| tests/step9_mutants.rs | req_144_name_without_the_location_prefix_stops | REQ-core-144, TBL-core-024, EX-core-224 | C-core-mutin-5 | "name" に前置きが無い |
| tests/step9_mutants.rs | req_144_line_zero_stops | REQ-core-144, EX-core-225 | C-core-mutin-5 | 行が0 |
| tests/step9_mutants.rs | req_144_path_outside_the_base_stops | REQ-core-144, EX-core-226 | C-core-mutin-6 | "../x/src/a.rs" で結果の誤り。絶対パスの場面は確かめていない |
| tests/step9_mutants.rs | req_144_results_without_baseline_and_with_unknown_keys_are_read | REQ-core-144, EX-core-227 | C-core-mutin-3 | 基準の実行が無く "extra" の鍵がある結果で、終了コード0、caught が1 |
| tests/step9_mutants.rs | req_139_survived_mutant_is_an_error | REQ-core-138, REQ-core-139, TBL-core-024, EX-core-204 | C-core-mutin-1, C-core-mutin-2 | "file"、行、前置きを除いた "name" が path、line、detail になり、"MissedMutant" が見逃しになる。REQ-core-139 の分は mutants の対応表に書いた |
| tests/step9_mutants.rs | req_144_line_one_is_read | REQ-core-144, REQ-core-139 | C-core-mutin-5 | 行が1の境界で停止しないこと（C-5 の行の最後の文）。REQ-core-139 の分は mutants の対応表に書いた |
| tests/step1_cli.rs | req_144_missing_result_file_is_an_unreadable_file | REQ-core-144 | C-core-mutin-7 | 結果のファイルが無いと終了コード2、"unreadable file: " で始まる |

## マーカーに足すテスト

この話題の ID を今は指していないが、付け替えたあとの確認することの行を確かめているテスト。今のマーカー（別の話題の ID）は残したまま、この話題の ID を足す。同じテストがほかの話題の対応表にも載っているときは、それぞれの表の ID を合わせて足す。

| テストのファイル | テストの関数名 | 今のマーカーの ID | 足す ID | 理由 |
|---|---|---|---|---|
| tests/step9_mutants.rs | req_139_caught_and_unviable_mutants_yield_nothing | REQ-core-139, REQ-core-145, EX-core-205 | C-core-mutin-2 | 今のマーカーはこの話題の ID を指していないが、"CaughtMutant" と "Unviable" が caught と unviable に数えられることを確かめている。C-2 を確かめる "Timeout" の分と合わせて、付け替えるときに足す |
| tests/step9_mutants.rs | req_140_timeout_is_a_notice_even_when_listed | REQ-core-140, EX-core-206 | C-core-mutin-2 | 同上。"Timeout" が時間切れになることを確かめている |

## 落とした細部

### 安全に関わるもの

無い。基準のディレクトリの外を指すパスで停止すること（A51）と、知らない値で止まること（A32）は利用者の決定なので、決めたことに残した。

### そのほか

| 要求のどの部分 | 誰が決めたと記録にあるか |
|---|---|
| REQ-core-144: 「結果の誤り」の詳細の誤りの説明が英語であること | 利用者（2026-09-17-mutation-tests.md の A39）。停止の文言の決まりは「コマンドライン」の話題の停止の詳細（TBL-core-020）に属するので、ここでは「誤りの説明」とだけ書いた |
| A33: Rust のプロジェクトが基準のディレクトリより下にある場合には初版では対応しない | 利用者（A33）。IR の要求の文に無い（出典に引かれているが文にしていない）ので、書かなかった |

## 形に依存する要求

無い。

## テストの無い行

- C-core-mutin-4（結果の "file" の正規化が指摘の path に出る）: 今のテストは一覧の側の "./src/a.rs"（equivalents の C-core-equiv-8）だけを確かめていて、結果の側の正規化を確かめるテストが無い。結果の "file" を "./src/a.rs" にした見逃しの1件から、path が "src/a.rs" の mutant_survived が出ることを確かめるテストが要る。

## 誰が決めたか見分けられなかったもの

無い。読んだ決定はすべて decided_by が利用者（推奨を採用）。

## 理由や見送った案が記録に無かった決定

- 「形の合わない結果のファイルでは、結果の誤りで停止する」: 見送った案が記録に無い。節を置かなかった。

## 分けた所、まとめた所

- REQ-core-138 と TBL-core-024 に、道具を知らない形を持つこと（A12、A13）、実物の形式を読むこと（A6）、"--tool" で必ず指定すること（A14）、関数名を外したこと（A56）をまとめて、1つの決定にした。
- "--tool" が無いか知らない値のときの停止（元の REQ-core-149）は「コマンドライン」の話題にあり、この話題には "--tool" で指定するという決定と名前だけを書いた。
- 結果のファイルが無い、読めない、UTF-8 でないときの停止（元の REQ-core-144 の最後の文）は、結果の誤りと同じ決定に入れた。

## あとでリンクにする所

確定の段で、次を見出しへのリンクにした。下書きどうしのリンクも `<file>.ja.md#<anchor>` に直した。

- cli の「kotowari mutants は道具の名前を --tool で必ず受け、結果のファイルをちょうど1つ受ける」（冒頭の段落の "--tool"。元の REQ-core-149）
- base-directory の「設定ファイルの値のパスは、比べる前と出力の前に正規化する」（冒頭の段落と、写し方の表の「設定のパスと同じ正規化」。元の REQ-core-110）

残り: 「形の合わない結果のファイルでは…」の停止の理由と文言（元の TBL-core-001、TBL-core-018、TBL-core-020）と、停止と指摘の振り分け（元の REQ-core-109）。この文書は "results error" と詳細の形を自分の決定の中で書いていて、別の決定を指す文が無いので、リンクにする所は無い


## 迷った判断と、どう決めたか

- 停止の文言 "results error" は、「コマンドライン」の話題の停止の文言の一覧（TBL-core-018）にもある。この話題のテストが文言まで確かめているので、振る舞いが見えるこちらに書き、向こうからは言葉で指してもらう。
- "--tool" の決定（A14）は、名前と「必ず指定する」までをここに書き、見送った案もここに置いた。引数の誤りで停止する振る舞いの確認することは「コマンドライン」の話題に任せた。向こうの書き直しで見送った案を二重に書かないよう、あとで調整が要る。
- C-5 は値違いの場面6つを1行にまとめた。「要る鍵が無い」場面はテストが無いので、行の列挙から外し、決めたことの箇条には残した。
- 「1件の結果が写し方の表のどの行にも当たらない」に、変異の1件の "Success" と "Failure" を含めた。A42 の文で、IR では TBL-core-024 の表に行が無いことから導かれていた。
- 「のように」を直した: C-4 の「"./src/a.rs" のように書き方だけ違っても」を「"./src/a.rs" と書いた結果でも」にした（テストが確かめる書き方は "./src/a.rs" だけなので、例の範囲を言い切った）。

## 数

- 元: 要求 2、場面 8、決定表 1、性質 0
- 読んだ記録のファイル: 1（2026-09-17-mutation-tests.md）
- 出来上がり: 決定 2、確認することの行 7（うち「（読んで確認）」0）
- テスト: 13（この話題の ID を指すもの 11、マーカーを足すもの 2）
