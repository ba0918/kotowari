# 壁打ちの記録: 読み取りのコマンド群

## Context

kotowari の初版は `check` だけを持つと絞った（[A19（records）](./records.md#A19)、[A8（mutation-tests）](./2026-09-17-mutation-tests.md#A8) で `mutants` を足して2つ）。これは初版の絞り込みで、読み取りのコマンド（list / trace / query / status）を不要と決めたものではない。
check が組み立てている同じデータ（IR の項目、印のあるテスト、変異の結果）を、人と LLM が読める形で出す口が無く、要求とテストの対応は `docs/trace.md` に手で保つしかなかった（[A2（check-reach）](./2026-09-17-check-reach.md#A2) で unit / property の行は捨て、review の確かめ方だけ残した）。
今決めるのは、読み取りのコマンド群を持つか、最初に何を作るか、出力の形、そして trace / query / status の位置づけ。次のリリースはこのコマンドの追加の後にする（利用者、2026-09-19）。

Position: 承認済み（2026-09-20）。3ラウンド（A1〜A16）で木は尽き、IR を書くときに分かった順序の制約を A17 で足し、照合レビューの指摘から細部を A18〜A20 で決めた。実装計画を書く段で見つかった仕様の穴を A21〜A24 と Revisions で埋めた。IR は docs/ir/list.md（新設）、cli.md、ir-items.md、ir-missing.md。実装は plan → cycle

## Agreements

- A1 check 以外に読み取りのコマンド群を足す。初版の「check だけ」の絞り込みを解く
  - why: IR と印とテストの対応は check が既に組み立てている。それを読める形で出すのは同じデータの別の切り口で、新しい状態も外部依存も増えない
  - decided_by: 利用者（推奨を採用）

- A2 最初に作るのは `list`。要求ごとに印のあるテストを含めて出し、`docs/trace.md` の手書きの表を置き換える。trace は別のコマンドにせず list に吸収する。`docs/trace.md` は削除する。人間向けの見やすさは `--format` で調整する範疇とし、何が見やすいかは別途検討する
  - why: 手で保つ表は腐る。list の出力に含めれば生成物になる（利用者）
  - rejected: `trace` を独立のコマンドにする（list の出力に含まれる情報と同じ）
  - decided_by: 利用者（推奨を採用）

- A3 JSON の形は項目単位の新しい形で、check の `findings` とは分ける。1 回の list で必要な情報が揃うようにし、不足があって別途取りに行く状態を避ける。`query <spec-id>` があるなら、さらに詳しい情報はそちらで取る
  - why: 指摘と項目は別物なので混ぜない。LLM が list を読んでから別のコマンドを呼び直すのは読み込みの往復が増える（利用者）
  - decided_by: 利用者（推奨を採用）

- A4 `query <id>` と `status` は list の後で、それぞれ小さい追記の壁打ちにする。query は list の 1 件版なので list の形が決まれば安い。status は「何が揃えば complete か」の基準が要るので別扱い
  - why: list の形が決まらないと query の形も決まらない。status の基準は仕様の判断
  - decided_by: 利用者（推奨を採用）

- A5 `docs/trace.md` に残っている `検証: review` の要求の確かめ方の表は、各要求の本文の下の `- 確かめ方:` の補足の行として IR に移す（項目が持てる行として ir-form に足す）。移した後に trace.md を削除する
  - why: list はテストから生成するので、テストの無い review の要求の確かめ方は生成できない。要求と確かめ方が同じ場所にあれば list の出力にもそのまま載り、手で保つ表が残らない
  - rejected: trace.md を残して list と無関係にする（手で保つ表が1つ残る）
  - decided_by: 利用者（推奨を採用）

- A6 list が出す項目は ID を持つ5種類（REQ、TBL、PROP、EX、FLAG）。用語は ID が無いので載せない。1件の鍵は `id`、`kind`、`name`、`path`、`line`、要求なら `type`（種類）、`verification`（検証）、`examples`（自分を about に持つ EX の ID）、`how_to_verify`（確かめ方。無ければ null）、全種類に `sources`（出典）と `tests`（印のあるテストの `path`、`line`、`name`）
  - why: A3 の「1回で揃える」。check が既に持つ情報だけで組み立てられる
  - decided_by: 利用者（推奨を採用）

- A7 `--format` は `json`（既定。check と揃える）と `text`。yaml は「何が見やすいか」の検討の結果を待ち、要るなら足す。初版の text は1項目1行（ID、検証、名前、path:line、テストの数）で、テストの行を字下げで続ける最低限の形
  - why: 形式は1つ増えるごとに契約が増える。見やすさの検討は別途（A2）
  - decided_by: 利用者（推奨を採用）

- A8 絞り込みのオプションは初版では持たない。全項目を出し、絞るのは `jq` か後の `query <id>`
  - why: A3 の「1回で揃える」に沿う。絞り込みは query の仕事
  - decided_by: 利用者（推奨を採用）

- A9 list の終了コードは読めれば 0。設定の誤りや読めないファイルなど check が停止する条件では同じく停止（2）。指摘は出さない。IR に誤りがあっても読めた範囲の項目を出す
  - why: 指摘は check の仕事。list は読み取りで、誤りの有無で出力を止める理由が無い
  - decided_by: 利用者（推奨を採用）

- A10 IR の置き場は新しい話題の文書 `docs/ir/list.md`（コマンドの振る舞い、項目の形、text の形）。`cli.md` の REQ-001「コマンドは2つ」を3つに改訂する。`output.md` の TBL-005 は check の形なので触らない
  - why: list は check と責務が別。cli.md はコマンドの数を列挙で決めているので改訂が要る
  - decided_by: 利用者（推奨を採用）

- A11 `- 確かめ方:` の行の有無を check では検査しない。置けるのは要求だけ、と形の契約（TBL-011）に書くに留める
  - why: 「review の要求に確かめ方が必須か」は status の「何が揃えば complete か」と同じ問いなので、status の壁打ちで一緒に決める
  - decided_by: 利用者（推奨を採用）

- A12 JSON の最上位は `items` だけ。順は check と同じく path の昇順、同じ path の中は line の昇順。`tests` の中も path → line
  - why: check の出力と同じ順なら読み手が迷わない
  - decided_by: 利用者（推奨を採用）

- A13 問い合わせの無い言語のテスト（生テキスト走査で印だけ拾う）も同じ `tests` に載せ、`name` は null。path と line は印の行
  - why: 印の有無は言語に依らず check が数えている。名前だけが取れない
  - decided_by: 利用者（推奨を採用）

- A14 FLAG は `relations`（関係の ID）を持つ。要求は `definition`（定義の TBL / PROP の ID）を持ち、TBL / PROP の側には逆向きの鍵を付けない
  - why: 片方向で十分。逆引きは query で行う
  - decided_by: 利用者（推奨を採用）

- A15 スキルには新しい場面を作らない。場面 `check` の reference `findings.md` に「list の読み方」の短い節を足す。発火語は増やさない
  - why: list は check と同じデータの別の切り口で、読む場面も同じ
  - decided_by: 利用者（推奨を採用）

- A16 `docs/trace.md` の削除と、review の要求 18 件への `- 確かめ方:` の移し替えは、この壁打ちの承認の差分に入れる
  - why: 18 行の手作業の移し替えで、IR の改訂そのもの
  - decided_by: 利用者（推奨を採用）
  - superseded_by: [A17](#A17)

- A17 A16 を改め、`- 確かめ方:` の行の移し替えと trace.md の削除は、実装の cycle の最後（check がその行を知っている行として読めるようになった後）に行う。承認の差分に入れるのは形の契約（TBL-011 に行を足す）と list.md と cli.md の改訂まで
  - why: 今の check は TBL-011 に無い行を unknown_field の誤りにする（[REQ-044](../../ir/ir-items.md#REQ-044)）。行を先に足すと承認の時点で IR の誤りが 18 件出て、承認の手順（IR の誤り 0）を満たせない。IR を書く段で分かった順序の制約
  - decided_by: 主セッション（事実の制約。利用者に報告済み）

- A18 `TBL-026` の鍵の細部: `kind` の値は英語（requirement、table、property、scenario、flag）。`type` は要求と問題の記録が持つ（どちらも "- 種類:" を持つ）。`verification` と `how_to_verify` は無ければ null、`definition` は無ければ空の並び。`examples` は要求・決定表・性質が持つ（"@about" の正引き。A14 の「逆向きの鍵を付けない」は `definition` の逆引きの話）。シナリオの `name` は "Scenario:" の後の文字、`line` はその行。`path` と tests の `path` は基準のディレクトリからの相対
  - why: A6 の鍵の一覧を IR に書ける精度にする。英語の鍵と値は check の JSON の鍵（findings、counts、tests）と揃える
  - decided_by: 利用者（推奨を採用）

- A19 text の形の細部: 1行目は "ID 検証 名前 パス:行 tests=数" を半角空白で区切る。要求以外は検証の欄を "-"。テストの行は半角空白2つで字下げした "パス:行 名前"、名前が null なら "-"。出力して見づらければ利用者が変更を求める（初版の最低限の形、A7）
  - why: A7 の「最低限の形」を IR に書ける精度にする。見やすさの検討は別途
  - decided_by: 利用者（推奨を採用。見づらければ変更要求を出す）

- A20 list は check と同じ4つのオプション（"--format"、"--config"、"--help"、"--version"）だけを受ける。停止は check と同じ理由と文言。コマンド無しの停止の文言は "expected command: check, list or mutants" に改め、TBL-020（cli-environment.md）も同じ文言にする。REQ-002 の「どちらのコマンドでも」は「どのコマンドでも」に直す
  - why: list は check と同じ読み取りなので、受けるものと止まり方も同じにする。文言はコマンドの列挙なので3つに合わせる
  - decided_by: 利用者（推奨を採用）

- A21 用語集の「項目」はシナリオを含まないままにし、list.md の側を「項目とシナリオ」と書き分ける。JSON の並びの1件を指すときは "items" の1件と書く
  - why: [sources.md の REQ-115](../../ir/sources.md#REQ-115) など、既存の要求が「項目なら出典の行、シナリオならタグの行」と2語を使い分けている。用語集を広げるとそれらの文の意味が変わる
  - decided_by: 利用者（「適した方に合わせて」。主セッションが影響の少ない側を選んだ）

- A22 シナリオの "name" は "Scenario:" の後の文字から前後の半角空白とタブを除いたもの
  - why: 実装が持つ "Scenario:" の行は字下げを含む生の行で、A18 の「後の文字」だけでは空白の扱いが決まらない。前後の半角空白とタブを除くのは [A52（records）](./records.md#A52) と同じ切り方
  - decided_by: 利用者（推奨を採用）

- A23 値が空の "- 確かめ方:" の行は、行が無いものとして扱う（REQ-098 の列挙に足す）。list の `how_to_verify` は null
  - why: "- 種類:"、"- 検証:"、"- 定義:"、"- 関係:" と同じ扱い（[A157（records）](./records.md#A157)）。空の文字を確かめ方として出す意味が無い
  - decided_by: 利用者（推奨を採用）

- A24 同じテストに同じ ID の印が複数あるとき、list の `tests` は印の出現ごとに1件
  - why: unresolved_reference の数え方（[A152（records）](./records.md#A152)）と揃える。テストごとに1件にすると、どの行を "line" にするかの決まりがもう1つ要る
  - decided_by: 利用者（推奨を採用）

- A25 text の "検証" の欄は、"- 検証:" の行の無い要求でも "-"
  - why: A19 は「要求以外は "-"」だけを決めていて、行の無い要求（IR に誤りがある場面）の欄が決まっていなかった。同じ "-" なら欄の幅と読み方が変わらない
  - decided_by: 利用者（推奨を採用。cycle のレビューの指摘から）

- A26 `examples` の並びは `ID` の昇順
  - why: A12 は items と tests の順だけを決めていた。ID の昇順は文書の位置に依らず安定で、実装が既にその順で出している
  - decided_by: 利用者（推奨を採用。cycle のレビューの指摘から）

## Rejected

- R1 `trace` を独立のコマンドにする
  - why: list の出力に含まれる情報と同じ（A2）
- R2 trace.md を残して list と無関係にする
  - why: 手で保つ表が1つ残る（A5）
- R3 初版に yaml の形式を足す
  - why: 形式は1つ増えるごとに契約が増える。見やすさの検討の後に要るなら足す（A7）

## Revisions

- A20 は cli.md の REQ-004 の列挙（"check" でも "mutants" でもない1つ目の位置引数、"check" に付けた "--tool"、"check" の後の位置引数）を直し忘れていた。REQ-004 に "list" を足した（実装計画を書く段で判明。2026-09-20）
- A5、A16、A17 の「18 件」は数え違いで、`docs/trace.md` の表の行は 16。計画は 16 で書く（2026-09-20）
- A5 の「移す」は文字をそのまま移すことで、移した後に古くなった確かめ方の文（REQ-105 のモジュールの列挙、REQ-109 の関数名）は cycle の終端で今のコードに合わせて直した（2026-09-20）
