# 照合の段階と出力

変更照合の具体的な入力、記録または合否を定める。具体的な契約と判断の根拠は出典の記録で追う。

## Requirements

### REQ-core-272: 段階ごとの合否

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-change-details.md#A10
- verification: unit

変更照合の検査は常に、次の契約を満たす。implementation 段階では各ファイルの同じ比較元と before・after に対応する active な implementer の件を、review 段階では active な implementer と reviewer の両方の件を要求する。coverage の単位は各ファイルで、両役のファイルのまとめ方と件の id は一致しなくてもよい。reviewer の関連 IR の集合は対応する implementer の件の関連 IR をすべて含め、追加の IR も含められる。必要な関連 IR を含まない reviewer の件はそのファイルの review の coverage を満たさず、change_uncovered を出す。結論 deferred は implementation 段階で形式・参照が揃えば通せるが、review 段階では change_deferred の誤りとして通さない。同じ変更が複数件に現れること自体は許すが、同じ base とファイルの before・after の組に対応する active な件に異なる結論があれば change_conclusion_conflict の誤りとする。照合漏れと covered の集計はファイル単位、鮮度の検査は件単位とし、対象変更を含む active な件の対象内のファイルか関連 IR が1つでも不一致ならその件の全ファイルを covered に数えない。保留を正式に採用する場合は判断と IR にその許容を明示して既存仕様または新判断として再照合する。記録の role は独立性の証明ではなく、別の review の実行はスキルが保証する。

### REQ-core-273: 古い履歴の扱い

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-change-details.md#A11
- verification: unit

変更照合の検査は常に、次の契約を満たす。changes は形式の検査を全照合記録に行い、参照の存在・整合の検査は active な件だけに行う。coverage と鮮度は active で指定された base と一致する件についてだけ検査する。check と status も形式は全件、参照の存在・整合は active な件だけを検査する。archived な件は現在の参照切れを誤りにせず、coverage の根拠にも使わない。完了した比較の記録は呼び出し側が archived に更新してリポジトリに保持する。base が異なる履歴の件を古さの誤りとしては出さない。対象変更に対応する件が無いときは change_uncovered、before・after の不一致は change_stale、IR の不一致は change_ir_stale とする。比較元に一致する記録が対象外になったファイルを持つ場合、そのファイルの coverage と鮮度は検査しない。

### REQ-core-274: 出力と停止

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-change-details.md#A12
- verification: unit

変更照合の検査は常に、次の契約を満たす。changes の JSON は base、target、phase、files、covered、findings の6鍵を持つ。base は解決した完全な object ID、target は commit の object ID または "index"、phase は指定値、files は列挙した対象変更数、covered はその段階の記録と鮮度が揃い結論が適合する変更数。findings の形と text の1件の表記は check と同じとする。形式・結論・必須情報・参照の静的な不整合は change_record_invalid とする。今回の新しい指摘はすべて severity error、path は照合記録の相対パス、line は null、detail は関連する件の id と対象パスを示す。change_record_invalid では取得できない id やパスを要求せず、ファイル全体の不正は "file: " と説明、件の不正は "entry " と entries 内の0始まりの位置と ": " と説明を出す。id を取得できるときだけ説明に含める。ただし change_uncovered は対象ファイルのパス、line null、detail は必要な role とする。指摘は path、kind、detail のバイト順で並べる。誤り0件で終了0、誤りありで終了1、実行や入力の停止で終了2。Git の読み取りの停止理由は "git error" とする。check と status は記録の静的検査の誤りを既存の findings に算入し、新しい最上位集計鍵を加えない。status の complete は差分の最終照合を保証しない。

## Examples

```gherkin
@id=EX-core-450 @about=REQ-core-272 @source=docs/decision/records/2026-10-01-change-details.md#A10,docs/decision/records/2026-10-01-change-details.md#A12,docs/decision/records/2026-10-01-change-details.md#A11
Scenario: 最終検査の保留
  Given 両役の記録は揃うが結論は deferred である
  When review 段階の changes を実行する
  Then change_deferred の誤りが出て終了1となる

@id=EX-core-451 @about=REQ-core-273 @source=docs/decision/records/2026-10-01-change-details.md#A11,docs/decision/records/2026-10-01-change-details.md#A12
Scenario: 過去の完了記録を保存する
  Given 形式と参照は正しいが別の base の過去の記録がある
  When 現在の base の changes を実行する
  Then 過去の記録の内容の古さは誤りにしない

@id=EX-core-452 @about=REQ-core-274 @source=docs/decision/records/2026-10-01-change-details.md#A12,docs/decision/records/2026-10-01-change-details.md#A11
Scenario: 対象と段階を出力する
  Given 対象変更1件に両役の適合する記録がある
  When review 段階の changes を JSON で実行する
  Then 指定された base と target と phase、files 1、covered 1、空の findings が出て終了0となる

@id=EX-core-454 @about=REQ-core-272 @source=docs/decision/records/2026-10-01-change-details.md#A10,docs/decision/records/2026-10-01-change-details.md#A12
Scenario: 両役でファイルのまとめ方が違う
  Given implementer の1件が2ファイルを持ち、reviewer は各ファイルを別の件にしている
  And 同じ base と内容に対応し、結論が同じで鮮度が揃い、reviewer は implementer の関連 IR をすべて確認している
  When review 段階の changes を実行する
  Then files 2、covered 2 で成功する

@id=EX-core-455 @about=REQ-core-273 @source=docs/decision/records/2026-10-01-change-details.md#A11
Scenario: 完了した照合の参照先が廃止される
  Given archived な件の形式は正しく、参照する要求は現在の IR から廃止されている
  When check を実行する
  Then archived な件の参照切れは誤りにしない
```
