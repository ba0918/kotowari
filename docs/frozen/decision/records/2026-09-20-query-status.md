# 壁打ちの記録: query と status

## Context

読み取りのコマンド群の壁打ち（[A4（read-commands）](./2026-09-19-read-commands.md#A4)）で、`query <id>` と `status` は `list` の後にそれぞれ小さい追記の壁打ちにすると決めた。`list` は 2026-09-20 に main に入り、項目とシナリオを印のあるテストと一緒に出せる。
`query <id>` は list の 1 件版で、逆引き（[A14（read-commands）](./2026-09-19-read-commands.md#A14)）と「さらに詳しい情報」（[A3（read-commands）](./2026-09-19-read-commands.md#A3)）の置き場と決まっているが、中身は未定。`status` は「何が揃えば complete か」の基準が要り、review の要求に `- 確かめ方:` が必須かの問い（[A11（read-commands）](./2026-09-19-read-commands.md#A11)）もここで決めると先送りにした。
障害が1つある: [REQ-core-008](../../ir/cli.md#REQ-core-008)（[P1（records）](./records.md#P1)）は "query" を「plan と cycle への受け渡し」として作らないと決めていて、テストが `kotowari query` の停止を固定している。
2つの話題を1つの記録で扱い、query を先に、status を後に決める（A4 の「それぞれ」は順の話で、記録を分ける理由は無い）。次のリリースはこの2つの後。

Position: 承認待ち（2026-09-20）。3ラウンド（A1〜A19）で木は尽き、IR を書く段の細部を A20 で足した（照合レビューの指摘から）。IR は docs/ir/query.md と docs/ir/status.md（新設）、cli.md、cli-environment.md、ir-missing.md。実装は plan → cycle

## Agreements

- A1 [REQ-core-008](../../ir/cli.md#REQ-core-008) の "query" の禁止を解き、REQ-core-008 は "render" だけを作らないに改める。`query <id>` は「1件を読む」コマンドで、P1 が禁じた「plan と cycle への受け渡しの仕組み」ではない
  - why: trace は list に吸収済み（[A2（read-commands）](./2026-09-19-read-commands.md#A2)）。P1 の意図は受け渡しを製品に組み込まないことで、読み取りのコマンドはそれに触れない
  - rejected: 名前を `show` にして REQ-core-008 を触らない（既存の記録が query の名前で語っている。[A3（read-commands）](./2026-09-19-read-commands.md#A3)、[A14（read-commands）](./2026-09-19-read-commands.md#A14)）
  - decided_by: 利用者（推奨を採用）

- A2 query の1件は list の1件に `body` を足す。`body` は見出しの次の行から次の項目の前までの生の行の並び（要求なら "- 種類:" などの行と文、決定表なら表、シナリオならタグの行と "Scenario:" の行とステップ）
  - why: LLM が1件を読むときに IR の文書を開き直さなくて済む。それが list に無い query の値打ち（[A3（read-commands）](./2026-09-19-read-commands.md#A3)）
  - decided_by: 利用者（推奨を採用）

- A3 query の1件は `referenced_by` を持つ。その ID を指している項目の並びで、1件は `id`、`kind`、`path`、`line`、`via`。`via` は "definition"（"- 定義:"）、"relations"（"- 関係:"）、"about"（"@about"）、"text"（要求の文・性質の文・シナリオのステップの中の ID）のいずれか
  - why: [A14（read-commands）](./2026-09-19-read-commands.md#A14) の「逆引きは query で行う」。拾う場所は check が unresolved_reference のために ID を読んでいる場所と同じで、新しい解析は要らない
  - decided_by: 利用者（推奨を採用）

- A4 query の JSON の最上位は list と同じ `items` だけで、その中の1件に `body` と `referenced_by` が増えた形
  - why: query は「list を1件に絞って詳しくしたもの」と読め、鍵の説明が二重にならない（[A8（read-commands）](./2026-09-19-read-commands.md#A8) の「絞り込みは query の仕事」）
  - decided_by: 利用者（推奨を採用）

- A5 query の位置引数はちょうど1つの ID。0個、2個以上、ID の形でない文字は引数の誤りで停止。オプションは list と同じ4つ
  - why: mutants の位置引数（[REQ-core-149](../../ir/cli.md#REQ-core-149)）と同じ止まり方。ID の形は [REQ-core-124](../../ir/ir-references.md#REQ-core-124) で決まっている
  - decided_by: 利用者（推奨を採用）

- A6 指定した ID が IR に無いときは引数の誤りで停止し、詳細は "unknown id: " と ID。同じ ID の項目が複数あるとき（duplicate_id）は一致する項目を全部 `items` に出して終了コード 0
  - why: LLM が読むとき、空の items と終了コード 0 より、非 0 と理由が出るほうが取り違えない。重複は check の指摘で、query が隠す理由は無い
  - decided_by: 利用者（推奨を採用）

- A7 query の text は、list の text と同じ1行目とテストの行に続けて、`body` の各行を半角空白2つで字下げして出し、最後に `referenced_by` の1件ごとに "  <- ID via パス:行" の行を出す
  - why: list の text の上に足す形なら読み方が1つ増えない。body は IR の生の行なので字下げだけで区別できる
  - decided_by: 利用者（推奨を採用）

- A8 status は「この IR はいま揃っているか」を、統計（数）と統合して分かる状態（complete の真偽）で答えるコマンド。check は1件ずつの指摘、list は1件ずつの項目、status はその上に立つ数と真偽の3段。初版の数は次のとおり: 文書の数と行数、種類ごとの項目の数（FLAG を含む）、要求の検証の種類ごとの数、テストのある要求とない要求の数（review を分母から除く）、review の要求のうち確かめ方があるものとないもの、具体例の無い要求の数、テストのある具体例とない具体例、印の数と読んだテストのファイルの拡張子ごとの数、check の誤りと注意の数
  - why: 利用者の認識「統計情報や統合して分かる状態を集計して分かりやすい形で出す」と一致した（2026-09-20 に実データの見本で確認）。数の追加と削除は使いながら判断する（利用者）
  - decided_by: 利用者（見本を見て合意）

- A9 complete は「check の誤りが 0、かつ FLAG が 0」。status 独自の判定規則は持たず、check が誤りにしないこと（具体例の無い要求など）は complete を妨げない
  - why: 判定の規則が2か所にあると食い違う。check に無い基準は check に足す
  - decided_by: 利用者（推奨を採用）

- A10 検証が review の要求には "- 確かめ方:" の行が必須で、無ければ check が誤りを出す。誰が確かめるか（人か LLM か）は仕様で決めない。[A11（read-commands）](./2026-09-19-read-commands.md#A11) の先送りをここで解く
  - why: review の要求にはテストが無く、手順が無いと人でも LLM でも確かめようが無い。実データで、review の要求 17 件のうち [REQ-core-150](../../ir/mutants.md#REQ-core-150) だけ行が無く、check が必須にしていれば承認の時点で見つかっていた
  - decided_by: 利用者（推奨を採用。「誰が確認するかは重視しない。本当に人間でないと確認できないもの以外は LLM に委譲する」）

- A11 status の終了コードは complete なら 0、そうでなければ 1、停止は 2
  - why: CI で `kotowari status` 1つで「揃っているか」を判定できる
  - rejected: 常に 0（読み取りだけ）。list と違い status は判定を持つので、判定を終了コードに映す
  - decided_by: 利用者（推奨を採用）

- A12 テストのある要求の分母から review の要求を除く。具体例の無い要求の数は統計に出す（欠陥ではない）
  - why: review はテストを求めない（[REQ-core-085](../../ir/coverage.md#REQ-core-085)）。具体例の無い要求は仕様の薄い所を見つける手がかり
  - decided_by: 利用者（推奨を採用）

- A13 CLI の出力のうち kotowari が決める固定の文言（status の項目名、query の "<-" と via の値など）は英語。IR やテストから写す文字はそのまま
  - why: 停止の文言（[REQ-core-005](../../ir/cli.md#REQ-core-005)）と JSON の鍵が英語で、揃える（利用者）
  - decided_by: 利用者

- A14 status の text は1つの群を1行にして "群名 鍵=値 鍵=値 …" の形。鍵は JSON と同じ語。桁揃えの空白は入れない
  - why: text と JSON の対応を覚えなくてよい。桁揃えは値の桁で変わり、契約にしにくい
  - decided_by: 利用者（推奨を採用）

- A15 query の `referenced_by` の並びは path の昇順、同じ path の中は line の昇順
  - why: list の順（[A12（read-commands）](./2026-09-19-read-commands.md#A12)）と同じ
  - decided_by: 利用者（推奨を採用）

- A16 `body` の境界は、見出しの次の行から、次の "### " か "## " の見出しの前の行まで。先頭と末尾の空行は落とす。シナリオは "@id" のタグの行から最後のステップの行まで
  - why: 項目の見出しの下に置ける行はすべて次の見出しまでにある（[TBL-core-011](../../ir/ir-items.md#TBL-core-011)）。先頭と末尾の空行は区切りで内容ではない（先頭は実装計画のレビューで足した。IR の見出しの直後には空行があるのが普通で、落とさないと本文が空行で始まる）
  - decided_by: 利用者（推奨を採用）

- A17 review の要求に "- 確かめ方:" の行が無いときの指摘は、新しい種類を作らず既存の missing_field を detail "確かめ方" で出す（[REQ-core-098](../../ir/ir-missing.md#REQ-core-098) に場面を足す）
  - why: 「必須の行が無い」は missing_field の意味そのもの。種類が増えなければ skill の表も変わらない
  - decided_by: 利用者（推奨を採用）

- A18 IR の置き場は新しい話題の文書 `docs/ir/query.md` と `docs/ir/status.md`。cli.md の REQ-core-001 はコマンド5つ、REQ-core-002 と REQ-core-004 に query と status を足し、TBL-core-020 と具体例の文言は "expected command: check, list, mutants, query or status"、REQ-core-008 は render だけ。REQ-core-098 に確かめ方の場面を足す
  - why: [A10（read-commands）](./2026-09-19-read-commands.md#A10) と同じ切り方（コマンドごとに話題の文書、cli.md は列挙の改訂）
  - decided_by: 利用者（推奨を採用）

- A19 query と status は check と同じ設定と置き場から同じものを読み、check と同じ条件・理由・文言で停止する。query は IR に誤りがあっても読めた項目を出して終了コード 0。status は check と同じ検査を走らせ、指摘は出さず数だけ出す
  - why: list と同じ作り（[A9（read-commands）](./2026-09-19-read-commands.md#A9)、[A20（read-commands）](./2026-09-19-read-commands.md#A20)）。status の数は check の指摘から作るので、検査を走らせないと出せない
  - decided_by: 利用者（推奨を採用）

- A20 IR を書く段の細部: (1) query の "items" の順は list と同じ（[REQ-core-154](../../ir/list.md#REQ-core-154)）。(2) status の JSON の最上位は TBL-core-028 の群の鍵だけで、群の順は表の順、"complete" が最後。text の "complete" の行は "complete true" か "complete false"。(3) requirements の unit / property / proof / review は "- 検証:" の行の無い要求をどれにも数えない。(4) tests の marks は印の出現を ID ごとに1つと数える（list の tests の1件と同じ数え方。[A24（read-commands）](./2026-09-19-read-commands.md#A24)）。(5) documents の files と lines、tests の files は check の同じ鍵と同じ値。(6) with_tests の「テストがある」は [REQ-core-085](../../ir/coverage.md#REQ-core-085) と同じ判定（"@about" のシナリオの印を含む）
  - why: A8 の数の一覧を IR に書ける精度にする（[A18（read-commands）](./2026-09-19-read-commands.md#A18) と同じ扱い）。数え方を check と揃えないと、status の数と check の指摘が食い違う
  - decided_by: 主セッション（承認時に提示）

## Rejected

- R1 query の名前を `show` にして REQ-core-008 を触らない
  - why: 既存の記録が query の名前で語っている（A1）
- R2 status の終了コードを常に 0 にする
  - why: status は判定を持つので、終了コードに映す（A11）
- R3 確かめ方の無い要求に新しい指摘の種類を作る
  - why: missing_field の意味そのもの（A17）

## Undecided

- U1 "render" を作らない決定（REQ-core-008 に残る唯一の禁止）を今後も残すか。利用者は「render は作る意義をだんだん失っている可能性がある」と述べた（2026-09-20）
  - decides: 利用者。次に REQ-core-008 に触る壁打ちで
  - related: A1

## Revisions

- A16 の「末尾の空行」を「先頭と末尾の空行」に改めた。A3 の "text" の参照がバッククォートの中の ID だけ（REQ-core-054 と同じ判定）であることを TBL-core-027 に明記し、EX-core-257 の Given もバッククォートで書いた（実装計画のレビューで判明。2026-09-20）
- A20 (2) の JSON の群の順を REQ-core-166 の文に書き、A10 の必須化を TBL-core-011 の要求の行にも映し、REQ-core-105 の確かめ方のモジュールの列挙に query と status を足した（cycle のレビューの指摘。2026-09-20）
