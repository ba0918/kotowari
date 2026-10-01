# 変更照合をフックと CI に導入する

## 対象と記録

<!-- @kotowari[REQ-core-264:1d5552d6, REQ-core-276:1d35ced8] -->

`changes.files` に製品コード、テスト、配布スキル、ビルド・フック・CI の設定を列挙し、生成物は `changes.exclude` に明示します。記録は `.kotowari/changes/*.yaml` に置き、途中用 `commit.yaml`、ブランチ全体用 `implementation.yaml`、独立 review 用 `review.yaml` の固定3ファイルを使います。これは導入先の約束で、コアのファイル名制約ではありません。保存形式は [changes](commands/changes.md) を見てください。記録自身・IR・判断の記録・使用する設定は差分の対象から外れます。

## 途中のコミット

<!-- @kotowari[REQ-core-258:67108915, REQ-core-276:1d35ced8] -->

実装側が HEAD の完全な ID を base とする `commit.yaml` を上書きして用意し、ファイル・IR・判断・記録をステージします。整形で変わるファイルも実際の index 内容で照合します。フックは rustfmt の直後に対象パスを明示して再ステージし、秘密情報と check の検査を保って、`changes --base HEAD --staged --phase implementation` を実行します。Lefthook の stage_fixed だけでは後続コマンドまでに再ステージが済まないため、明示した更新が必要です。

未ステージの記録は使われません。指摘があれば LLM が件全体を読み直し、委譲範囲で根拠のある判断・記録と必要な具体的 IR を更新して再検査します。途中のコミットに最終独立 review の完了は要求しません。

## 最終の確認

<!-- @kotowari[REQ-core-257:afa929fa, REQ-core-259:74f6c430, REQ-core-275:8370ba63] -->

ブランチ全体の比較元を固定し、その base で実装側の記録を作り直します。別の review が根拠の妥当性、仕様と実装の意味、委譲範囲を確認して自分の reviewer 記録を作ります。要求 ID のあるテストの追加・期待値変更・削除を優先して再照合し、製品コードと補助関数の変更も対象に含めます。

両役の記録と修正をコミットして対象の完全な SHA を固定し、テストと check、`changes --base <base> --head <head> --phase review` を行います。取り込みには check と changes の終了0が両方必要です。status complete だけでは最終照合を保証しません。変更や IR の修正があれば、その件の全ファイルと IR を再確認し、両役の記録を更新、再コミット、再照合します。途中用 `commit.yaml` は最終完了時に消し、両役の固定最終記録を維持します。日付・hash 付きの過去件や一覧は記録ディレクトリに置かず、履歴は Git に残します。変更・IR・判断の意味の修正や rebase、cherry-pick、並行統合で無効になった両役の最終記録を削除してから、独立再照合で書き直します。

仕様の穴は重要度とアクションとは別に分類し、選択と根拠の判断・関連 IR、既存仕様への修正と対応要求、または明示的保留と引き継ぎ先を記録します。info/record_only だけを処理済みにしません。承認済み要求を保つ委譲範囲の具体化のみを自律追記し、承認済み要求の変更・削除、矛盾する選択、根拠から決められない意味は人へ戻します。IR の追加後は check と、その追加を含む仕様・実装の独立照合を再実行します。

## CI の対象

<!-- @kotowari[REQ-core-261:ba974563, REQ-core-276:1d35ced8] -->

このリポジトリの PR workflow はイベントの base SHA と head SHA の履歴を取得し、merge-base を比較元、head SHA を対象にします。合成された merge SHA は対象にしません。同じエンジンで check と review 段階の changes を実行します。workflow を外部へ公開する前に承認が必要です。

push に導入する例では `scripts/change-base.sh push "$GITHUB_EVENT_PATH"` で before/after を採ります。before が全0の新規ブランチは比較元が未指定なので停止し、呼び出し側が比較元を明示します。記録内の自己申告だけでイベントの比較元を選びません。

## 普段の探索

`.ignore` に `.kotowari/changes/` を書くと、通常の rg 探索から機械用の長い記録を外せます。記録を確認するときは明示したファイルを読むか `rg --no-ignore` を使います。Git ignore ではないので記録は追跡し、check/status と changes は設定された記録を読みます。完了比較は当時の commit の設定と記録で検証します。旧 `state` 付き形式の過去 commit には当時のツール版が必要です。
