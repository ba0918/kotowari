# library-io の対応表

元: `docs/ir/schema/library-io.md`（日本語は `library-io.ja.md`）。書き直した文書: `library-io.md`。

## 元の ID の表

| 元の ID | 新しい ID | 決定の見出し | 理由 |
|---|---|---|---|
| REQ-schema-072 | C-schema-io-1, C-schema-io-3, C-schema-io-4 | スキーマと文書の読み込みは、別のクレートに置いて CLI と外のアプリで共用する | 入口と load・check・抽出を C-1 に、read_document を別の決定「文書だけを読む入口を公開し、mds の ast もそれを使う」に分けて C-3、C-4 にした。位置の契約（REQ-core-323）は core の「ライブラリ入力のパス」の話題に任せた |
| REQ-schema-073 | C-schema-io-2, C-schema-io-8 | スキーマと文書の読み込みは、別のクレートに置いて CLI と外のアプリで共用する | 既存の CLI の規則をそのまま移すことを、探索の結果が CLI と同じになる行（C-2）にした。カレントディレクトリを変えないこと（安全に関わる細部）を C-8 に戻した。端末に書かないことは落とした細部 |
| REQ-schema-074 | C-schema-io-5 | 読み込みの失敗は、文書の指摘と分けて返す | そのまま |
| REQ-schema-075 | C-schema-io-6, C-schema-io-7 | Tokio の feature を選んだ呼び出しにも、同期と同じ結果を返す | 同期と同じ結果を C-6、feature が既定で無効で純粋なクレートが Tokio に依存しないことを C-7（読んで確認）にした。実行枠・キャンセル・ランタイムの契約は core の「任意の Tokio 対応」の話題（REQ-core-318〜321）に任せた。ガイド `docs/guides/public-crate-api*.md` の REQ-schema-075 のマーカーはこの見出しへ |
| EX-schema-090 | C-schema-io-1, C-schema-io-8 | スキーマと文書の読み込みは、別のクレートに置いて CLI と外のアプリで共用する | 「カレントディレクトリを変更せず」の部分は C-8 に戻した。「結果を端末へ表示しない」の部分は落とした細部 |
| EX-schema-091 | C-schema-io-5 | 読み込みの失敗は、文書の指摘と分けて返す | そのまま |
| EX-schema-092 | C-schema-io-6 | Tokio の feature を選んだ呼び出しにも、同期と同じ結果を返す | そのまま |

元の文書に TBL と PROP は無い。

## テストごとの表

| テストのファイル | テストの関数名 | 今のマーカーの ID | 付け替えたあとの ID | 理由 |
|---|---|---|---|---|
| crates/kotowari-markdown-schema-io/tests/loader.rs | directory_checks_dot_prefixed_documents_but_skips_hidden_directories | REQ-schema-044, REQ-schema-073 | C-schema-io-2 | cli.rs の同じ名前のテストと同じ入力で、SchemaLoader の check が同じ文書だけを選ぶことを確かめている。REQ-schema-044（CLI の話題）の分は、その話題の書き直しで付け替える |
| crates/kotowari-mds/tests/cli.rs | directory_check_reports_dot_prefixed_markdown_and_skips_hidden_directories | REQ-schema-044, REQ-schema-073, REQ-core-313 | C-schema-io-2 | mds の check の側の同じ入力の結果。REQ-schema-044 と REQ-core-313 の分は、それぞれの話題で付け替える |
| crates/kotowari-markdown-schema-io/tests/loader.rs | loaded_pairs_are_reusable_and_findings_do_not_prevent_partial_extraction | REQ-schema-072, REQ-schema-073, EX-schema-090 | C-schema-io-1 | load した組の validate・extract_validated・extract_partial と、loader の extract_validated・extract_partial・check を確かめている。端末に書かないことは確かめていない |
| crates/kotowari-markdown-schema-io/tests/loader.rs | execution_failures_are_typed_and_distinct_from_document_findings | REQ-schema-074, REQ-schema-073, EX-schema-091 | C-schema-io-5 | 読めない文書、見つからないスキーマ、壊れたスキーマ、UTF-8 でない文書で、型付きの失敗の種類を確かめている。REQ-schema-073 の分は確かめていない |
| crates/kotowari-markdown-schema-io/tests/loader.rs | later_current_directory_changes_do_not_change_the_load_base | REQ-core-323, EX-core-501, REQ-schema-073 | C-schema-io-8 | この話題の分は、load のあとのカレントディレクトリが、テストが移した先のままであること（load が変えない）。安全に関わる細部として戻した行。あとでカレントディレクトリを変えても読み込みの基準が変わらないことは core の「ライブラリ入力のパス」の話題の分で、その話題で付け替える |
| crates/kotowari-markdown-schema-io/tests/loader.rs | read_document_strips_the_bom_and_rejects_non_utf8_with_the_given_path | REQ-schema-072 | C-schema-io-3 | BOM を除くことと、UTF-8 でない文書が unreadable_file になることを確かめている。誤りの詳細が渡したパスで始まることは、テストの中に残る細部 |
| crates/kotowari-markdown-schema-io/tests/async.rs | loader_futures_and_results_are_send_and_match_sync_operations | REQ-schema-075, REQ-core-318, REQ-core-321, EX-schema-092, EX-core-492, EX-core-495 | C-schema-io-6 | AsyncSchemaLoader の load・check・extract_partial・extract_validated と失敗の種類が同期と同じことを確かめている。Send であることとランタイムが無いときの失敗は core の「任意の Tokio 対応」の話題の分 |
| crates/kotowari-markdown-schema-io/src/async_tests.rs | clones_hold_started_permits_and_cancelled_waiters_are_never_submitted | REQ-core-319, REQ-core-320, REQ-schema-075, EX-core-493, EX-core-494, EX-schema-092 | - | 確かめているのは複製どうしの実行枠と、待機を取り消した処理を投入しないこと。core の「任意の Tokio 対応」の話題の確認することに当たる |
| crates/kotowari-markdown-schema-io/src/async_tests.rs | configured_limits_allow_two_workers_and_independent_objects_do_not_share_slots | REQ-core-319, REQ-schema-075, EX-core-493, EX-schema-092 | - | 確かめているのは同時実行数の上限とオブジェクトごとの実行枠。core の話題の分 |
| crates/kotowari-markdown-schema-io/src/async_tests.rs | runtime_shutdown_of_queued_blocking_work_is_a_typed_task_failure | REQ-core-321, REQ-schema-075, EX-core-492, EX-schema-092 | - | 確かめているのはランタイムの停止が型付きの失敗になること。core の話題の分 |
| crates/kotowari/src/async_tests.rs | clones_hold_started_permits_and_cancelled_waiters_are_never_submitted | REQ-core-319, REQ-core-320, REQ-schema-075, EX-core-493, EX-core-494, EX-schema-092 | - | kotowari のクレートの側の同じテスト。AsyncSchemaLoader は通らず、core の話題の分だけ |
| crates/kotowari/src/async_tests.rs | configured_limits_allow_two_workers_and_independent_objects_do_not_share_slots | REQ-core-319, REQ-schema-075, EX-core-493, EX-schema-092 | - | 同上 |
| crates/kotowari/src/async_tests.rs | runtime_shutdown_of_queued_blocking_work_is_a_typed_task_failure | REQ-core-321, REQ-schema-075, EX-core-492, EX-schema-092 | - | 同上 |

## 落とした細部

### 安全に関わるもの

なし。前の版でここに置いた1件（カレントディレクトリを変えない）は、テストが確かめているので決定に戻した（下の「戻した細部」）。

### そのほか

| 要求のどの部分 | 誰が決めたと記録にあるか |
|---|---|
| REQ-schema-073、EX-schema-090: 結果を標準出力と標準エラーへ書かず、表示と終了コードの決定を CLI に残す | 設計担当（A44）。ライブラリは結果を型で返し表示は呼び出し側が決めるという利用者の A7 は、この要求の出典に引かれていない。決めたことには書かなかった |
| REQ-schema-072: 位置の契約（作業開始位置とキャッシュ基準は絶対パス、相対なら InvalidInput） | 設計担当（A52）。core の「ライブラリ入力のパス」の話題に任せ、決めたことでは言葉で指した |
| REQ-schema-075: 非同期の操作の範囲、実行枠、待機の取り消し、ランタイムの細部 | core の「任意の Tokio 対応」の話題に任せた。そのうち待機を取り消しても始まった処理は止まらないこと（A24）、実行枠の上限（A23、A30〜A32）は利用者の決定で、落としていない |
| テスト: 誤りの詳細が渡したパスで始まること、失敗の種類の名前（UnreadableFile、SchemaNotFound、SchemaInvalid、RuntimeUnavailable） | 元の文書の外（テストの中だけ） |

## 戻した細部

| 細部 | 新しい ID | 確かめているもの |
|---|---|---|
| REQ-schema-073、EX-schema-090: I/O のクレートはカレントディレクトリを変更しない（設計担当の A44。why は「A17 を、複数のプロジェクトを同一プロセスで扱える I/O 入口に具体化する」） | C-schema-io-8（「スキーマと文書の読み込みは、別のクレートに置いて CLI と外のアプリで共用する」） | テスト later_current_directory_changes_do_not_change_the_load_base（子のプロセスで、loader を作ってからカレントディレクトリを移し、load のあとも移した先のままであることを見る）。テストが見るのは load だけなので、行も load に絞った。check、extract_validated、extract_partial は確かめていない |

## 誰が決めたか見分けられなかったもの

- 入口の名前と、load・check・extract_validated・extract_partial が返すもの（REQ-schema-072）は設計担当の A44（D1）。I/O のクレートを置くこと（A17）は利用者の決定。名前は残し、load の組を繰り返し使えることは A18（利用者。読み込み済みデータを複数操作で使い回す）と同じ向きなので書いた。
- 「同期と同じ値・指摘・失敗の種類」（A45、設計担当）は、利用者の A22「検査ロジックを共用する」から導いたものとして書いた。

## 理由や見送った案が記録に無かった決定

- 「スキーマと文書の読み込みは、別のクレートに置いて…」: 見送った案が記録に無い。
- 「文書だけを読む入口を公開し、mds の ast もそれを使う」: 見送った案が記録に無い。
- 「読み込みの失敗は、文書の指摘と分けて返す」: 見送った案が記録に無い。
- 「Tokio の feature を選んだ呼び出しにも、同期と同じ結果を返す」: 見送った案（R2、R3）の採らなかった理由は、記録では「A 案を採用した」だけ。利用者がもう一方の案を選んだ、と書いた。

## 分けた所、まとめた所

- REQ-schema-072 を2つの決定に分けた: 入口と読み込み（A17、A44）と、read_document（2026-10-06-todo-zero.md の A9。別の記録で理由を持つため）。
- REQ-schema-073 は、REQ-schema-072 の入口の決定にまとめた。どちらも「既存の CLI の読み込みを別のクレートに移して共用する」（A17）から来ている。
- REQ-schema-075 の非同期の決定は、core の「任意の Tokio 対応」の話題（REQ-core-318）と同じ決定で、AsyncProject と AsyncSchemaLoader の両方を扱う。この話題には AsyncSchemaLoader の振る舞い（同期と同じ結果、feature が既定で無効）だけを書き、実行枠などは言葉で指した。

## あとでリンクにする所

- core の「ライブラリ入力のパス」の話題（`docs/ir/core/library-paths.md`、元の REQ-core-323）: 「スキーマと文書の読み込みは、別のクレートに置いて…」の作業開始位置とキャッシュ基準
- core の「任意の Tokio 対応」の話題（`docs/ir/core/library-async.md`、元の REQ-core-318〜321）: 「Tokio の feature を選んだ呼び出しにも…」の実行枠、取り消し、ランタイム

## 迷った判断と、どう決めたか

- 設計担当が D1 の委任で決めた名前（SchemaLoader::new、LoaderOptions、load、check、extract_validated、extract_partial、AsyncSchemaLoader と、失敗の型が Result の Err で std::error::Error と Display を実装すること）は、利用者から見える名前を残す決まりに従って残した。read_document は、利用者の決定（todo-zero の A9 の「文書を読む入口」）の名前なので、この問題に当たらない。振る舞いは利用者の決定（A17、A9、A8、A22、A27、todo-zero の A9）で裏付けられる分だけを書いた。形の決まりで足りなかった所。
- 「既存の CLI のファイルの探索、URL の解決、キャッシュ、取得の上限とタイムアウトの規則をそのまま移す」の並びは A44（設計担当）の文だが、A17（利用者）の「既存 CLI のファイル探索・HTTP 取得・キャッシュ処理を移す」の言い換えなので残した。取得の上限とタイムアウトは停止の条件に関わるので、落とさない側に倒した。
- カレントディレクトリを変えないことは A44（設計担当）にしか無いが、安全に関わる細部をテストが確かめているなら決定に戻す、という利用者の決定で C-8 に戻した。問題の節には A44 の why（複数のプロジェクトを同じプロセスで扱える入口）と、落とすと起きること（同じプロセスのほかの処理が読む位置まで変わる）を書いた。
- 「既存の CLI の規則をそのまま移す」の規則の置き場として、schema の cli と schema-resolution の見出しへのリンクを足した。core の「ライブラリ入力のパス」「任意の Tokio 対応」の話題は同時に書き直されているので、言葉のまま残した（「あとでリンクにする所」）。
- 非同期の決定をどちらに書くか。core の REQ-core-318 も同じ決定で、振る舞いはどちらの側にも見える。この話題の REQ-schema-075 にガイドのマーカーとテストがあるので、AsyncSchemaLoader の分はここに置いた。見送った案（R2、R3）は AsyncProject にも当てはまるので、core の話題を書き直すときに、どちらに置くかを決め直す必要がある。
- async_tests.rs の6つのテストは REQ-schema-075 も指しているが、確かめているのは core の契約（実行枠、取り消し、ランタイム）なので、この話題では「-」にした。
- C-schema-io-2 は、ディレクトリの探索の規則そのもの（点で始まる文書は読み、隠しディレクトリは読み飛ばす）を書かず、CLI と同じ結果になることだけを書いた。規則は CLI の話題（REQ-schema-044）の決定である。

## 数

- 元: 要求 4、場面 3、決定表 0、性質 0
- 読んだ記録のファイル: 2（2026-10-03-public-crate-api.md、2026-10-06-todo-zero.md）
- 出来上がり: 決定 4、確認することの行 8（うち「（読んで確認）」2）
- テスト: 13（行の数え直し。前の版の 12 は数え違い）
