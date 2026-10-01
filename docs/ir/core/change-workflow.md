# 変更の照合を回すスキルとフック・CI

実装中の記録、独立した review、途中と最終の完了条件、人への確認、フックと CI の分担を扱う。これは配布するスキルと導入手順の契約であり、CLI が意味を判定する要求ではない。

## Requirements

### REQ-core-256: 実装中に生まれた判断を残す

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-change-conformance.md#A1, docs/decision/records/2026-10-01-change-conformance.md#A9
- verification: review
- how_to_verify: 計画・実装・cycle のスキルを読み、記録する条件と情報、IR に反映する条件が A1 と A9 に一致することを確認する。

kotowari のスキルは常に、実装や計画で新しい判断をした役に、選択と根拠と判断した役を`判断の記録`へ残し、振る舞い・制約の仕様を変える場合は`IR`にも反映する手順を持つ。

### REQ-core-257: 実装側とは別の review

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-change-conformance.md#A6, docs/decision/records/2026-10-01-change-conformance.md#A8
- verification: review
- how_to_verify: cycle と review のスキルを読み、実装側の申告だけで最終の照合済みにしないことと、確認する3点が A6 と A8 に一致することを確認する。

kotowari のスキルは常に、最終の照合で、実装側とは別の review が根拠の妥当性、仕様と実装の意味の一致、委譲範囲を確認し、照合対象と結果を記録する手順を持つ。

### REQ-core-258: 途中のコミットの完了条件

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-change-conformance.md#A8, docs/decision/records/2026-10-01-change-conformance.md#A9
- verification: review
- how_to_verify: 導入手順、実装と cycle のスキルを読み、pre-commit の対象・必要な段階・指摘から再検査までの流れが A8 と A9 に一致することを確認する。

kotowari の導入手順とスキルは常に、pre-commit に、HEAD とステージ済みの内容に対する実装側の判断・根拠・対応記録の機械検査を置き、指摘を LLM が読んで記録や必要な`IR`を更新し、再検査する手順を持つ。途中のコミットごとに最終 review の完了を要求しない。

### REQ-core-259: 取り込み前の完了条件

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-change-conformance.md#A2, docs/decision/records/2026-10-01-change-conformance.md#A8, docs/decision/records/2026-10-01-change-conformance.md#A12
- verification: review
- how_to_verify: 導入手順と cycle のスキルを読み、対象がブランチ全体であり、両コマンドを必須にし、check だけで完了としないことが A2・A8・A12 に一致することを確認する。

kotowari の導入手順とスキルは常に、cycle の最終検査と CI に、ブランチ全体について review の照合までを要求する機械検査を置き、取り込み前に check と changes の両方を必須にする。check の成功だけを今回の変更が照合済みである証拠にしない。

### REQ-core-260: 人への確認の境界

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-change-conformance.md#A9
- verification: review
- how_to_verify: 実装・cycle・review のスキルを読み、自律更新の条件と人へ戻す条件が A9 に一致し、承認済みの制約変更を記録作業の権限で許していないことを確認する。

kotowari のスキルは常に、記録の欠落だけで人へ戻さず、委譲範囲内で根拠のある判断と記録の更新を自律的に行う手順を持つ。委譲範囲を超える変更、根拠から選択を決められない場合、不可逆・危険・外部公開の操作は人へ戻す。

### REQ-core-261: CI が比較対象を決める

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-change-conformance.md#A12
- verification: review
- how_to_verify: CI の導入例を読み、イベントに対応する対象を明示し、記録内の比較元だけに依存せず、changes を同じエンジンで実行することが A12 に一致することを確認する。

kotowari の導入手順は常に、CI のイベントから比較元と対象を呼び出し側が決め、照合記録の自己申告だけから比較対象を採らず、ローカルと同じ機械検査エンジンを実行する手順を持つ。

### REQ-core-262: 仕様の穴を記録だけで終えない

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-change-conformance.md#A10, docs/decision/records/2026-10-01-change-conformance.md#A11
- verification: review
- how_to_verify: review と cycle のスキルを読み、分類と3種類の処理先が A10 と A11 に一致し、記録だけの扱いで処理先の記録が不要にならないことを確認する。

kotowari の review と cycle のスキルは常に、仕様の穴を重要度・修正アクションとは別に分類し、共通の形式で処理先を残す手順を持つ。"info" や "record_only" だけで仕様の穴を処理済みにしない。

### REQ-core-275: 自律的な仕様の追記の境界

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-change-details.md#A13
- verification: review
- how_to_verify: 配布スキルと導入例を読み、出典 A13 の入力・責任境界・再実行条件を満たすことを確認する。

kotowari のスキルと導入手順は常に、次の契約を満たす。implement と fixer は、承認済み要求の制約を変えない委譲範囲内の具体化に限り、根拠と判断者を記録して IR に追加できる。承認済み要求を変更・削除する判断、既存の選択と矛盾する追加、根拠から決められない判断は人へ戻す。cycle 自身は実装や意味の判断を行わず、実装側へ記録を委譲し別の review で確認する。実装側の IR 追加後は check と、その追加を含む仕様・実装の照合を必ず再実行する。review の findings JSON は継続して内部用とし、実装側と review 側は共通の YAML 照合記録を別々に作成する。

### REQ-core-276: 導入と再照合

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-change-details.md#A14
- verification: review
- how_to_verify: 配布スキルと導入例を読み、出典 A14 の入力・責任境界・再実行条件を満たすことを確認する。

kotowari のスキルと導入手順は常に、次の契約を満たす。導入例では changes.files に製品コード・テスト・配布スキル・ビルドとフックと CI の設定を列挙し、changes.records は "docs/changes/**/*.yaml" とする。生成物は明示した exclude だけで外す。pre-commit は整形後の index に対して "changes --base HEAD --staged --phase implementation" を行い、check も別に行う。CI は pull_request の比較元を base SHA と head SHA の merge-base、対象を head SHA とし、両履歴を取得して "changes --base <比較元> --head <対象> --phase review" と check を行う。merge 用の SHA は対象に使わない。push の導入例はイベントの before と after の比較とし、before が全0の新規ブランチは停止して比較元を明示する。途中のコミット用には HEAD を base とした実装側の件を作り、最終検査用にはブランチの比較元を base とした実装側の件と review 側の件を作り直す。再照合は変わったファイルまたは IR を含む件ごとに行い、同じ件の全ファイルと関連 IR を再確認する。


### REQ-core-277: 優先して再照合するテストの変更

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-change-conformance.md#A7
- verification: review
- how_to_verify: review のスキルと照合結果を読み、要求 ID のあるテストの追加・期待値変更・削除を優先して確認し、製品コードだけの変更も対象に残すことを確認する。

kotowari の review のスキルは常に、要求 ID のあるテストの追加・期待値変更・削除を優先して再照合し、その期待値が変わらないことだけを理由に製品コードや補助関数の変更を対象から外さない手順を持つ。
