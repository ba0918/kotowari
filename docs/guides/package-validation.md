# 配布パッケージを検証する

クレートを公開せずに、実際のアーカイブと独立した利用側を検証する手順です。

## ローカルの検証

<!-- @kotowari[REQ-core-309:06cb6202, REQ-schema-051:eb9c1f03] -->

Cargo 1.98以降と安全なtar展開を提供するPython 3.11以降を使い、変更をコミットした清潔な作業ツリーで実行します。

```sh
python3 scripts/check-packages.py --output "$ABSOLUTE_NEW_SCRATCH_DIRECTORY"
```

出力先はリポジトリ外の新しい絶対パスです。既存ディレクトリは上書きしません。
ヘルパーはロック済みの外部依存をvendorした後、ソース置換を使わないオフラインのCargo workspace stagingで9個のアーカイブを作ります。
その後、元のアーカイブを安全に展開し、チェックサム付きの独立したdirectory sourceで各パッケージをビルド・テストします。
Tokioの有効・無効、別クレートからの呼出、再公開した型の同一性、両CLIの互換テストも確かめます。
アーカイブや出荷したmanifestを書き換えず、作業ツリーへのpath依存やpatchは使いません。
出力にはコマンドと終了コード、manifest・ロック・依存グラフ、アーカイブのハッシュ、利用側の実行結果を残します。`result.json`のPASSはローカル検証の成功であり、公開済みという意味ではありません。

## 将来の公開順

<!-- @kotowari[TBL-core-040:d398d029] -->

公開を行う人は、検証と独立レビューが済んだ同じ変更について、各製品の版を上げてから実行します。
依存される側を先に公開します。

1. `kotowari-markdown-schema`と`kotowari-markdown-view`
2. `kotowari-core`と`kotowari-markdown-schema-io`
3. `kotowari-source-analysis`、`kotowari-mds`、`kotowari-overview`
4. `kotowari`
5. `kotowari-cli`

同じ段の独立したクレートの順番は任意です。これは将来の人による公開手順であり、検証ヘルパーはアップロードしません。
kotowari系の版の基準は根の`Cargo.toml`、スキーマ系は`crates/kotowari-markdown-schema/Cargo.toml`です。`kotowari-overview`はkotowari系、`kotowari-markdown-view`はスキーマ系の版に従います。
スキーマ系の版変更時はcoreとoverviewから入る依存宣言も更新します。既存の2製品のタグ、バイナリ名、配布アーカイブ名を変更する手順ではありません。
