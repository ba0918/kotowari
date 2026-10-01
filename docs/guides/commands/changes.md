# kotowari changes — 変更と照合記録の検査

変更を Git から独立して列挙し、呼び出し側が書いた YAML 記録との対応と鮮度を確かめます。根拠の妥当性、意味の一致、review の独立性は別の review で確認します。

## 比較対象と段階

<!-- @kotowari[REQ-core-263:ec4ff4c1, REQ-core-265:1c472ad0] -->

```sh
kotowari changes --base HEAD --staged --phase implementation
kotowari changes --base <base-sha> --head <head-sha> --phase review --format json
```

base、head または staged、phase はすべて必須で既定を持ちません。staged は base HEAD と implementation の組み合わせだけです。commit の REV は完全な object ID に解決されます。index と commit は設定・記録・IR も同じ snapshot から読み、未ステージ・未追跡の内容で補いません。

## 記録

<!-- @kotowari[REQ-core-268:6d4f5042, REQ-core-270:d878aaf0] -->

設定は [config.md](../config.md) の `changes` に書き、呼び出し側が `changes.records` の glob に当たる YAML を作成します。kotowari は記録を書きません。具体的な保存形式は [change-record-format.md](../../ir/core/change-record-format.md)、運用手順は [変更照合](../../../agent/skills/kotowari/references/changes.md) を見てください。

1ファイルに version: 1 と entries を置き、各件は id、base、role、files、ir、conclusion、reason、requirements、decisions、handoff、gaps を持ちます。role は implementer/reviewer、conclusion は existing/new/deferred です。件の id と両役のファイルのまとめ方は一致しなくても構いません。

files の path、before、after は比較元と対象の識別値を記します。追加は before:null、削除は after:null。識別値は Git の6文字 mode、NUL、blob の全バイトの SHA-256 で、`sha256:` と64桁の小文字16進を使います。ir の path と sha256 は IR 全バイトの SHA-256 です。ファイルと IR の一覧内でパスを重複させません。

existing は対応する要求とその定義 IR を、new は判断への参照を、deferred は判断への参照と handoff を持ちます。判断はリポジトリ相対の `path#A1` のような参照です。gaps は missing_spec/spec_conflict/premise_conflict と recorded/fixed/deferred と refs を持ち、処理先を記録します。

## 合否と出力

<!-- @kotowari[REQ-core-272:32f1f5d8, REQ-core-273:870d0970, REQ-core-274:5071dd96] -->

implementation は各ファイルの実装側の記録を要求します。review は両役の記録を要求し、reviewer は対応する implementer の関連 IR をすべて含めます。件の対象内ファイルまたは IR が古いと、その件の全ファイルの coverage が無効になります。review の deferred と結論の衝突は誤りです。設定に当たる記録はすべて形式と現在の参照を検査し、別の base は鮮度と coverage に使いません。

JSON は base、target、phase、files、covered、findings の6鍵です。target は commit ID または index、files と covered はファイル数です。findings と text の1件の表記は check と共通で、誤りなしは終了0、誤りありは1、入力や実行の停止は2です。Git 読み取りの停止理由は `git error` です。

途中用の記録は HEAD を base にし、最終用はブランチ全体の比較元を base にして実装側と独立 reviewer が別々に作ります。現在の比較だけを固定ファイルに置き、完了時も同じファイルを維持します。履歴は Git に残し、過去の比較はその commit の設定と記録から再検証します。変更・IR・判断の意味の修正、rebase、cherry-pick、並行変更の取り込みで無効になった両役の最終記録を消し、件全体を独立再照合して書き直し、check と changes を再実行します。check/status の成功だけでは今回の変更が照合済みとは言えません。

開発中の version: 1 から `state` を除去しました。旧 `state` 付き記録は未知キーとして拒否し、自動変換しません。旧形式の過去 commit は当時のツール版で再検証してください。新版形式を持つ過去 commit は新版でもその snapshot から読めます。
