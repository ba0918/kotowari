# 壁打ちの記録: 後回しの要求

## Context

kakoi の "kotowari check" は error 198 件で、そのうち約90件は上流の制約で今は作らないと決めた要求と例（動的公開の10文書と IPv6 リンクローカルの2文書、要求31件・例67件）の requirement_without_test と scenario_without_test である。
残りの約100件は作ったのにテストの印が無い本物の未完で、両者が同じ error に混ざって後者が埋もれ、status の complete も後者の有無を表さない。
kakoi からの要望（kakoi リポジトリの ".agents/notes/2026-09-25-kotowari-deferred-request.md"、git の管理外）は、要求を「後回し」と宣言でき、宣言した項目はテストの無さを誤りにせず、形と参照の検査と ID の予約は続けること。
宣言の書き方、例の扱い、テストの無さを何も出さないか注意にするか、表とプロパティも対象かを kotowari 側で決める。

Position: 承認待ち（2026-09-25）。4ラウンド（A1〜A22）で木は尽き、IR を書く段と敵対的レビューと照合レビューの細部を A23〜A26 で足した。IR は docs/ir/core/deferred.md（新設）と CONTEXT、coverage、finding-order、findings、form-contract、guides、ir-items、list、sources、status の各文書。実装は plan → cycle

## Agreements

- A1 後回しは要求ごとに、要求の見出しの下の "- deferred:" の行で宣言する。加えて、話題ごとの文書の範囲の行の間に "- deferred:" の行を書くと、その文書のすべての要求を後回しにする省略形になる
  - why: 食い違いの検査は要求ごとに判断するので、要求単位が基本になる。範囲の行の間の "- " の行は今は unknown_field で、既存の文書とぶつからない（実測）。kakoi の後回しは文書単位（12文書）
  - rejected: 要求単位の書き方だけにする（kakoi の31行で足りるが、要望の「少なくとも文書ごと」に届かない）
  - decided_by: 利用者（推奨を採用）

- A2 "- deferred:" の値は出典で、"- source:" と同じ形（コンマ区切りで複数可）と同じ規則で検査する。値が空なら detail を "deferred" にした missing_source、同じ場所に2回書けば duplicate_field にし、新しい指摘の種類は作らない
  - why: 理由の無い後回しを作れないようにする（要望）。既存の検査と種類を使えば覚えることが増えない
  - decided_by: 利用者（推奨を採用）

- A3 シナリオには後回しの宣言を書かない。scenario_without_test の判断で、検証が review の要求と同じく後回しの要求を数えない。"@about" の要求がすべて後回しか review なら出さず、後回しでない要求が1つでもあれば今までどおりテストを求める
  - why: シナリオのタグは3つと決まっていて、増やさずに済む。後回しでない要求も指すシナリオは、その要求を確かめる例でもある
  - decided_by: 利用者（推奨を採用）

- A4 後回しの要求と、A3 で対象から外れたシナリオには、テストが無くても指摘を何も出さない。後回しの件数は status に出す
  - why: 注意にすると kakoi で約100件並び、埋もれている本物の未完がまた埋もれる。注意は終了コードにも complete にも効かないので、違いは見え方だけ。件数を status に出せば黙って増え続けない
  - rejected: 注意として出す
  - decided_by: 利用者（推奨を採用）

- A5 用語集 "docs/ir/core/CONTEXT.md" に「後回し」を足す。行の名前と出力の鍵は英語の "deferred" にする
  - why: 出力のトークンを英語にする [2026-09-23-ir-english-tokens](./2026-09-23-ir-english-tokens.md) の方針に揃える
  - decided_by: 利用者（推奨を採用）

- A6 "- deferred:" の行はガイドの指紋に入れる。"- source:" のような除外を作らない
  - why: 後回しを外すことはその機能を出すことなので、そのとき guide_stale でガイドの見直しを促すのが正しい。宣言を付けたときの guide_stale は1回だけで、受け入れる
  - decided_by: 利用者（推奨を採用）

- A7 mutants は後回しで変えない
  - why: mutants の仕様（docs/ir/core/mutants.md、mutants-input.md）は要求の検証や種類を見ず、コード（mutants.rs、cargo_mutants.rs）にも verification を扱う箇所が無い（検索で確認。動かしての確認はしていない）
  - decided_by: 利用者（推奨を採用）

- A8 status の requirements の群と scenarios の群に "deferred" の鍵を足す。requirements は後回しの要求の数、scenarios は A3 でテストを求めなくなったシナリオの数で、どちらも "with_tests" と "without_tests" には数えない。"text" では各行の末尾に "deferred=N" を置く。"unit"、"property"、"proof"、"review" の数は後回しも数える
  - why: 後回しを without_tests に数えたままだと、その数が本物の未完を表さない。検証の値ごとの数は検証の分布で、後回しかどうかと別
  - decided_by: 利用者（推奨を採用）

- A9 list と query の json では、すべての項目とシナリオに真偽値の "deferred" を付ける。要求は宣言（要求ごとか文書単位）があれば true、シナリオは A3 でテストを求めなくなったものが true、ほかは false。text では後回しの行の末尾に " deferred" を付ける。出典の鍵は足さない
  - why: 後回しかどうかが一覧で分かれば足りる。要求ごとの出典は query の "body" の "- deferred:" の行で見える。文書単位の宣言の出典は query に出ないが、文書を開けば見える
  - decided_by: 利用者（推奨を採用）

- A10 要求ごとの宣言と文書単位の宣言が重なっても指摘を出さず、どちらの出典も検査する。文書単位の宣言は話題ごとの文書だけに置け、CONTEXT.md と FLAGS.md の範囲の行の間では今までどおり unknown_field にする。要求の無い文書の文書単位の宣言には指摘を出さない
  - why: 重なりは害が無く、どちらを外しても意味が通る。用語集と問題の記録には後回しにする要求が無い
  - decided_by: 利用者（推奨を採用）

- A11 後回しの要求か A3 でテストを求めなくなったシナリオの ID を含む印があるとき、その項目の見出しの行（シナリオはタグの行）に detail をその ID にした deferred_with_test の注意を1件出す。問い合わせの無い言語で拾った印も数える
  - why: 作り終えて後回しを外し忘れたか、印の付け間違いで、どちらも人が気付くべき状態
  - decided_by: 利用者（推奨を採用）

- A12 後回しでない要求かプロパティが "- definition:" か文の中のバッククォートで囲んだ ID で後回しの要求を指すとき、およびシナリオの "@about" が後回しの要求と後回しでない要求の両方を指すとき、参照1件ごとに detail を「参照元の ID と参照先の ID を半角空白で区切ったもの」にした depends_on_deferred の注意を出す。後回しの項目から後回しでない項目への参照には出さない
  - why: 今の製品が作っていないものに依存している。誤りにすると complete が落ちて止まるので注意にする
  - decided_by: 利用者（推奨を採用）

- A13 スキルの文面を変える。kotowari スキル（ir-form.md、findings.md）に "- deferred:" の行と新しい2つの注意を載せ、plan は後回しの要求の ID を計画に含めず、cycle と review は「テスト側の指摘を0件にする」の対象に後回しが入らないと書き、brainstorm は後回しにするとき理由の決定を記録に書いてそれを出典にする。編集は plan と cycle で行う
  - why: 後回しの扱いが工程ごとにぶれないようにする（要望の6）
  - decided_by: 利用者（推奨を採用）

- A14 計画の完了条件に、kakoi の IR の使い捨ての写しで後回しの宣言の前後の requirement_without_test と scenario_without_test を数え、12文書の分が消え、形の誤りと duplicate_id が残ることを確かめる、を入れる。kakoi の文書そのものの書き換えは kakoi 側の作業とする
  - why: 要望の受け入れの目安を測れる形にする。他リポジトリの書き換えは kotowari の変更の範囲の外
  - decided_by: 利用者（推奨を採用）

- A15 シナリオの "@about" に後回しの要求が1つ以上あり、"@about" の要求がすべて後回しか検証が review のとき、そのシナリオを後回しのシナリオとする（A3 でテストを求めず、A8 と A9 で deferred に数える）。決定表だけを指すシナリオは後回しでない。後回しの要求と後回しでない要求の両方を指すシナリオは後回しでなく、テストを求め、A12 の depends_on_deferred を出す
  - why: A3、A8、A9、A12 が同じ定義を使うように1つに決める
  - decided_by: 利用者（推奨を採用）

- A16 status の requirements の "without_examples" には後回しの要求も数える
  - why: 後回しでも草案の質を落とさない（要望の3）。例の無さは抜けとして見えるほうがよい
  - decided_by: 利用者（推奨を採用）

- A17 後回しでないシナリオのステップの中でバッククォートで囲んだ後回しの要求の ID にも depends_on_deferred を出す
  - why: query の "referenced_by" はステップの中の ID も参照（via "text"）に数えており、それと揃える
  - decided_by: 利用者（推奨を採用）

- A18 "- deferred:" の行があれば、値が空でも出典として誤りでも、その要求を後回しにする。値の誤りは A2 の missing_source か source_invalid で知らせる
  - why: どちらでも誤りが1件残り complete は false のまま。値の直し方と後回しかどうかを分けておくほうが説明しやすい
  - decided_by: 利用者（推奨を採用）

- A19 文書単位の "- deferred:" の行は文書が扱う範囲の行に数えない。範囲の行が "- deferred:" だけの文書は missing_scope にする。文書単位の "- deferred:" の行が2つ以上あれば duplicate_field にする
  - why: 範囲の行は文書が何を扱うかの説明で、宣言はそれに当たらない。重複は見出しの下の行と同じ扱いに揃える
  - decided_by: 利用者（推奨を採用）

- A20 文書単位の宣言がある文書では、その文書の各要求の指紋に文書の "- deferred:" の行も入れる
  - why: A6 の「後回しを外したとき guide_stale でガイドを見直させる」を、文書単位の宣言でも成り立たせる
  - decided_by: 利用者（推奨を採用）

- A21 depends_on_deferred は参照が書かれた行（"- definition:" の行、文の行、"@about" のタグの行、ステップの行）に出し、1行に後回しの要求への参照が複数あれば参照ごとに1件出す
  - why: 直す場所が行で分かる
  - decided_by: 利用者（推奨を採用）

- A22 後回しでない項目が後回しのシナリオの ID を参照しても、depends_on_deferred を出さない。A12 の参照先は後回しの要求だけとする
  - why: シナリオの ID を文から参照する書き方は今の IR でほとんど使われず、作る者のいない入力の検査は足さない
  - decided_by: 利用者（推奨を採用）

- A23 IR を書く段で決めた細部。値の空の "- deferred:" の missing_source はその行に出す。deferred_with_test は ID ごとに1件で、印の数は見ない。depends_on_deferred は同じ行に同じ参照先が2回あれば2件（用語の検査と同じく出現ごと）。文書単位の "- deferred:" の行は要求の指紋の行の並びの先頭に加える。スキルの文面の要求は検証を review にし、確かめ方を書く
  - why: A2、A11、A20、A21、A13 を IR に書くとき、行、件数、並びの位置を決めないと実装が決められない。スキルの文面は機械で確かめられない
  - decided_by: Claude（IR を書く段の細部。承認のときに利用者に示す）

- A24 敵対的レビューの指摘から決めた細部。同じ ID の要求やシナリオが2か所以上にあるときは、後回しかどうかを REQ-core-032 の1つ目で決め、deferred_with_test もその場所に出す。後回しのシナリオの判定で数える "@about" の要求は、REQ-core-137 と同じく検証の値が4つのいずれかのものに限る。要求の無い文書の文書単位の宣言も、出典の検査と duplicate_field は受ける。source_invalid の行は "- deferred:" の行にし、スキーマにその行番号を取る宣言を置く。文書が扱う範囲の定義を、実際の判定（一覧の行と表の行は範囲に数えない。2026-09-25 に実測）に合わせて書き直す。A19 で "- deferred:" の行は範囲の行でなくなったので、文書単位の宣言を置く位置を A1 の「範囲の行の間」から「題名の後で最初の "## " か "### " より前」に言い換える
  - why: 同じ ID の扱いを REQ-core-085 と REQ-core-137 に揃える。テストの検査から外すかどうかと、後回しとして数えるかどうかを一致させる。範囲の定義が "- deferred:" だけを名指しで除くと、ほかの一覧の行を範囲に数えると読める
  - decided_by: Claude（レビューの指摘を推奨で決めた。承認のときに利用者に示す）

- A25 後回しの要求も、形の検査（見出し、必須の行、出典、用語、曖昧語、gherkin の形）と参照の検査を後回しでない要求と同じに受け、ID は予約されたままで、同じ ID が別の場所にあれば duplicate_id になる
  - why: 草案の質を落とさず、後回しの ID を別の項目に使えないようにする（要望の3。仕様の置き場の外へ移す案を取らなかったのはこの保護のため）
  - decided_by: 利用者（kakoi からの要望の3をそのまま受けた。承認のときに示す）

- A26 照合レビューの指摘から決めた細部。文書単位の "- deferred:" の行の重なりは見出しの下の行の重なり（REQ-core-045）と同じく、2つ目の行に detail を行の名前 "deferred" にした duplicate_field を1件出し、1つ目の行の値だけを読み、指紋にも1つ目の行だけを加える。status の requirements の deferred は "unit" などと同じく項目の出現ごとに数える（2026-09-25 に実測）。用語「後回し」は「保留」「草案」「延期」と呼ばない
  - why: 重なりの扱いを既存の行と揃える。status の数え方を既存の鍵と揃える。「草案」は kakoi の文書の範囲の行で別の意味（書きかけ）に使われ、「保留」「延期」は決めていない状態と読める
  - decided_by: Claude（承認のときに利用者に示す）

## Prohibitions

## Undecided

- U1 版の上げ方。今ある文書の検査の結果は変わらないが、status の text の行と list、query の json の形が変わる
  - decides: 利用者（リリースのとき）

## Delegated

## Rejected

- R1 決定表とプロパティも後回しにできるようにする: どちらも元々テストの印を求められず、後回しにしても変わるのは表示だけ。必要な場面が出たら足す
  - why: 作る者のいない需要（kakoi の後回しの12文書に表もプロパティも無い）
  - decided_by: 利用者（推奨を採用）

## Revisions

- A24 は A1 の文書単位の宣言の位置「範囲の行の間」を「題名の後で最初の "## " か "### " より前」に言い換える（A19 で宣言の行が範囲の行でなくなったため。意味は変えない）
