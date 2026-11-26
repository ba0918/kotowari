# library-async の対応表

元: `docs/ir/core/library-async.md`（日本語は `library-async.ja.md`）。書き直した文書: `library-async.md`。

## 元の ID の表

| 元の ID | 新しい ID | 決定の見出し | 理由 |
|---|---|---|---|
| REQ-core-318 | C-core-libasync-1, C-core-libasync-2 | 高水準の操作を、既定で無効な tokio feature で Tokio から待てるようにする | そのまま。AsyncProject が同期と同じ結果を返すことを C-1、feature が既定で無効で spawn_blocking に渡し CLI は同期を使うことを C-2（読んで確認）にした。AsyncSchemaLoader の分は schema の「スキーマと文書を読み込むライブラリ」の C-schema-io-6、C-schema-io-7 にある。ガイド `docs/guides/public-crate-api*.md` の「Tokioから待つ」のマーカーの REQ-core-318 はこの見出しへ |
| REQ-core-319 | C-core-libasync-3, C-core-libasync-4 | 同時に動かす数は、非同期の入口のオブジェクトごとに限り、複製どうしは実行枠を共有する | そのまま。既定の上限と上限の数を C-3、オブジェクトごとの実行枠と複製の共有を C-4 にした |
| REQ-core-320 | C-core-libasync-5, C-core-libasync-6, C-core-libasync-7 | 待機をやめても、始まった処理は止めず、終わるまで実行枠を持つ | そのまま。枠待ちの取り消しを C-5、投入後の枠の保持を C-6、始まった処理が止まらないことを利用者に示すことを C-7（読んで確認）にした |
| REQ-core-321 | C-core-libasync-8, C-core-libasync-9 | ランタイムの外で待つか、ランタイムが止まったときは、panic せず種類を見分けられる失敗を返す | そのまま。出典は A45（設計担当、D1 の委任）だけだが、ランタイムが無いときとタスクの失敗を型付きの失敗にすることは安全に関わる細部で、テストが確かめているので、新しい決定にした。内部のランタイムを作らないことも同じ決定の文に入れた。呼び出し側のランタイムの spawn_blocking に渡すことは C-2 の文。Send であることは落とした細部 |
| EX-core-492 | C-core-libasync-1 | 高水準の操作を、既定で無効な tokio feature で Tokio から待てるようにする | そのまま。同じ値と指摘が C-1。「Future と結果を別スレッドへ渡せる」は落とした細部 |
| EX-core-493 | C-core-libasync-4, C-core-libasync-6 | 待機をやめても、始まった処理は止めず、終わるまで実行枠を持つ | そのまま。複製どうしの共有が C-4、待機をやめても枠を持ち続けることが C-6 |
| EX-core-494 | C-core-libasync-5 | 待機をやめても、始まった処理は止めず、終わるまで実行枠を持つ | そのまま |
| EX-core-495 | C-core-libasync-8 | ランタイムの外で待つか、ランタイムが止まったときは、panic せず種類を見分けられる失敗を返す | そのまま。ランタイムの外で待つと RuntimeUnavailable になる場面 |

元の文書に TBL と PROP は無い。

## テストごとの表

| テストのファイル | テストの関数名 | 今のマーカーの ID | 付け替えたあとの ID | 理由 |
|---|---|---|---|---|
| crates/kotowari/tests/async.rs | every_operation_is_send_and_reuses_sync_results_and_failure_kinds | REQ-core-318, REQ-core-321, EX-core-492, EX-core-495 | C-core-libasync-1, C-core-libasync-8 | AsyncProject の check、status、list、query、read、inspect、plan、mutants の結果と失敗の種類が同期と同じこと（C-1）と、ランタイムの外で待つと RuntimeUnavailable になること（C-8）を確かめている。Send であることは落とした細部で、テストの中に残る |
| crates/kotowari/tests/async.rs | overview_prepare_and_build_return_what_the_sync_operations_return | REQ-core-310, TBL-core-041, REQ-core-318 | C-core-libasync-1 | 全体像の2つの操作も同期と同じ結果を返すことを確かめている。REQ-core-310 と TBL-core-041 の分は「プロジェクトを使う Rust API」の話題で付け替える |
| crates/kotowari-markdown-schema-io/tests/async.rs | loader_futures_and_results_are_send_and_match_sync_operations | REQ-schema-075, REQ-core-318, REQ-core-321, EX-schema-092, EX-core-492, EX-core-495 | C-core-libasync-8 | この話題の分は、AsyncSchemaLoader をランタイムの外で待つと RuntimeUnavailable になること（C-8）。同期と同じ結果を返すことは schema の C-schema-io-6 で、その表に書く。Send は落とした細部 |
| crates/kotowari/src/async_tests.rs | clones_hold_started_permits_and_cancelled_waiters_are_never_submitted | REQ-core-319, REQ-core-320, REQ-schema-075, EX-core-493, EX-core-494, EX-schema-092 | C-core-libasync-3, C-core-libasync-4, C-core-libasync-5, C-core-libasync-6 | 既定の上限が1であること（C-3）、複製が待たされること（C-4）、枠待ちで取り消した処理が投入されないこと（C-5）、始まった処理への待機をやめても複製からの処理が投入されないこと（C-6）を確かめている。REQ-schema-075 の分は schema の話題では「-」にしてある |
| crates/kotowari/src/async_tests.rs | configured_limits_allow_two_workers_and_independent_objects_do_not_share_slots | REQ-core-319, REQ-schema-075, EX-core-493, EX-schema-092 | C-core-libasync-3, C-core-libasync-4 | 上限2で2つが同時に動き3つ目が待つこと（C-3）と、別に作ったオブジェクトが枠を共有しないこと（C-4）を確かめている |
| crates/kotowari/src/async_tests.rs | runtime_shutdown_of_queued_blocking_work_is_a_typed_task_failure | REQ-core-321, REQ-schema-075, EX-core-492, EX-schema-092 | C-core-libasync-9 | ランタイムが止まったときに、実行枠を待っていた処理が TaskFailure になり、始まっていた処理は終わることを確かめている（AsyncProject の実行枠） |
| crates/kotowari-markdown-schema-io/src/async_tests.rs | clones_hold_started_permits_and_cancelled_waiters_are_never_submitted | REQ-core-319, REQ-core-320, REQ-schema-075, EX-core-493, EX-core-494, EX-schema-092 | C-core-libasync-3, C-core-libasync-4, C-core-libasync-5, C-core-libasync-6 | kotowari の側と中身が同じテスト（AsyncSchemaLoader の実行枠） |
| crates/kotowari-markdown-schema-io/src/async_tests.rs | configured_limits_allow_two_workers_and_independent_objects_do_not_share_slots | REQ-core-319, REQ-schema-075, EX-core-493, EX-schema-092 | C-core-libasync-3, C-core-libasync-4 | 同上 |
| crates/kotowari-markdown-schema-io/src/async_tests.rs | runtime_shutdown_of_queued_blocking_work_is_a_typed_task_failure | REQ-core-321, REQ-schema-075, EX-core-492, EX-schema-092 | C-core-libasync-9 | 同上（AsyncSchemaLoader の実行枠） |

## 戻した細部

| 細部 | 新しい ID | 確かめているもの |
|---|---|---|
| REQ-core-321、EX-core-495: Tokio のランタイムの外で待ったときと、タスクの実行に失敗したとき（待っている間にランタイムが止まったとき）に、panic せず型付きの失敗（RuntimeUnavailable、TaskFailure）を返す | C-core-libasync-8, C-core-libasync-9 | テスト every_operation_is_send_and_reuses_sync_results_and_failure_kinds（kotowari）、loader_futures_and_results_are_send_and_match_sync_operations（kotowari-markdown-schema-io）が RuntimeUnavailable を、runtime_shutdown_of_queued_blocking_work_is_a_typed_task_failure（両方のクレートの src/async_tests.rs）が TaskFailure を確かめている。新しい決定「ランタイムの外で待つか、ランタイムが止まったときは、…」にした |
| REQ-core-321: 内部のランタイムを作らない | C-core-libasync-8 | 同じ決定の文に入れた。ランタイムの外で待つと RuntimeUnavailable になること（同じテスト）が、内部のランタイムで動かさないことを確かめている |

## 落とした細部

### 安全に関わるもの

無い。

### そのほか

| 要求のどの部分 | 誰が決めたと記録にあるか |
|---|---|
| REQ-core-321、EX-core-492: 渡す入力、返す結果、公開の Future がスレッドの間を移れる（Send）こと | 設計担当（A45。検証の方法の委任）。テストの中に残る |
| REQ-core-318: 非同期の操作の範囲が「それぞれの同期の高水準の操作と読み込みの操作」であること | 設計担当（A45）。利用者が決めたのは「高水準の操作を await できる」（A22）なので、その言い方で書いた |
| 非同期の制限と取り消しを、実時間を待たずに処理の開始と解放を制御して検証すること | 設計担当（A47）。テストの書き方なので決定に書かなかった |

## 形に依存する要求

無い。

## テストの無い行

無い（読んで確認の行は C-core-libasync-2、C-core-libasync-7）。

## 誰が決めたか見分けられなかったもの

- 「同じ入力には、同期と同じ値、指摘、失敗の種類を返す」（REQ-core-318）は A45（設計担当）の文。利用者の A22「同期処理を spawn_blocking で実行し、検査ロジックを共用する」から導いたものとして書いた。schema の話題も同じ扱いにしている。
- `NonZeroUsize` の名前は A45（設計担当）。上限を正の整数で変えられることは A30（利用者）なので、名前を残し、振る舞いは A30 のとおりに書いた。

## 理由や見送った案が記録に無かった決定

- 「同時に動かす数は、非同期の入口のオブジェクトごとに限り…」: 見送った案が記録に無い。節を置かなかった。
- 「ランタイムの外で待つか、ランタイムが止まったときは、…」: 理由も見送った案も記録に無い（A45 の why は呼び出しの型の具体化だけ）。問題の節に、落とすと何が起きるかと「理由は記録に残っていない」を書いた。
- 「高水準の操作を、既定で無効な tokio feature で…」と「待機をやめても…」の見送った案（R2、R3）は、採らなかった理由が記録では「A 案を採用した」だけ。利用者がもう一方の案を選んだ、と書いた。

## 分けた所、まとめた所

- 非同期の決定は schema の話題（REQ-schema-075）と同じ決定から来ている。AsyncSchemaLoader が同期と同じ結果を返すことは schema の話題に書いてあるので、ここでは AsyncProject の分と、両方に共通の実行枠、取り消し、ランタイムを書いた。
- 見送った案 R2 は AsyncProject の決定に、R3（始まった処理を止める仕組みを足す）は取り消しの決定に置いた。schema の話題の「Tokio の feature を選んだ呼び出しにも、同期と同じ結果を返す」にも R2 と R3 が書いてあり、両方に同じ案がある。
- REQ-core-321 のうち、呼び出し側のランタイムの spawn_blocking に渡すことは最初の決定（C-2）に、内部のランタイムを作らないことと型付きの失敗は新しい決定「ランタイムの外で待つか、ランタイムが止まったときは、…」に分けた。後者は AsyncProject と AsyncSchemaLoader に共通で、AsyncProject だけを扱う最初の決定には入らないため。
- 値違いの場面（既定の上限1と、上限2）は C-3 の1行にまとめた。

## あとでリンクにする所

- 無い。「ライブラリのクレート境界」と schema の「スキーマと文書を読み込むライブラリ」へのリンクは、`library-crates.ja.md#…` と `../schema/library-io.ja.md#…` の見出しへのリンクにした。
- schema の側（`../schema/library-io.ja.md` の「Tokio の feature を選んだ呼び出しにも、同期と同じ結果を返す」）にも見送った案 R2、R3 が書いてある。この文書に残すので、schema の側は R2、R3 を消して、この文書の「高水準の操作を、既定で無効な tokio feature で Tokio から待てるようにする」（`library-async.ja.md#高水準の操作を既定で無効な-tokio-feature-で-tokio-から待てるようにする`）と「待機をやめても、始まった処理は止めず、終わるまで実行枠を持つ」（`library-async.ja.md#待機をやめても始まった処理は止めず終わるまで実行枠を持つ`）へリンクするのがよい。schema の側の文書は schema の書き直し役が持つので、ここでは直していない。schema の側の「どのランタイムの上で動くかは…『任意の Tokio 対応』の話題で決める」も、この文書の「ランタイムの外で待つか、…」（`library-async.ja.md#ランタイムの外で待つかランタイムが止まったときはpanic-せず種類を見分けられる失敗を返す`）へのリンクにできる。

## 迷った判断と、どう決めたか

- REQ-core-321（ランタイムとスレッドの境界）は、出典が A45（設計担当、D1 の委任）だけである。前の下書きでは落としたが、型付きの失敗は安全に関わり、テストが確かめているので、利用者の決定（安全に関わるものはテストがあれば戻す）に従って決定に戻した。A45 の why は呼び出しの型の具体化で、panic させない理由ではないので、問題の節には落とすと何が起きるかを書き、「理由は記録に残っていない」と書いた。Send であることは、安全に関わらないので落としたままにした。
- C-core-libasync-9 のテストは、実行枠を待っていた処理だけが TaskFailure になり、始まっていた処理は終わることを見ている。行は「実行枠を待っている処理があるうちにランタイムが止まる」場合に絞り、「タスクの実行に失敗したとき」一般には広げなかった。
- 見送った案 R2、R3 はこの文書に残す（依頼の指示）。schema の側はこの文書へのリンクにする（上の「あとでリンクにする所」）。
- C-7 は A24（利用者）の「公開 API の契約に明示する」を確かめる行。rustdoc には書かれておらず、ガイド `docs/guides/public-crate-api*.md` に書いてあるので、ガイドの文を読む行にした。
- REQ-core-320 の出典の changes-rethink の A10（利用者。changes を製品から外す）は、止まらない処理の並びから Git の実行が消えた理由である。決めたことには今の並び（HTTP の取得、計算、キャッシュの書き込み）だけを書いた。

## 数

- 元: 要求 4、場面 4、決定表 0、性質 0
- 読んだ記録のファイル: 3（2026-10-03-public-crate-api.md、2026-10-06-changes-rethink.md、2026-10-04-overview-on-public-api.md の A3）
- 出来上がり: 決定 4、確認することの行 9（うち「（読んで確認）」2）
- 戻した細部: 1（安全の一覧の22。内部のランタイムを作らないことも同じ決定に入れた）
- テスト: 9
