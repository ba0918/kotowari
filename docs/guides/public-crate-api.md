# Rustライブラリから使う

実行ファイルを起動せずに仕様の検査やMarkdownの値の抽出を使う方法を説明します。

## クレートを選ぶ

<!-- @kotowari[REQ-core-306:12ba818e, REQ-core-308:3cb3e06d, TBL-core-040:d398d029] -->

リポジトリの操作には`kotowari`、メモリだけの計算には`kotowari-core`を使います。
`kotowari-source-analysis`は文字列からテストや面を発見し、元の文字列と診断を含むcoreの型を返します。ファイル取得やIRとの対応づけは行いません。
`kotowari-overview`はメモリ上の全体像の元データを検査し、参照の表・古い節・描画の入力を作ります。`kotowari-markdown-view`は描画の入力から全体像のページをメモリ上に作り、workspaceのほかのパッケージに依存しません。
スキーマの計算には`kotowari-markdown-schema`、取得には`kotowari-markdown-schema-io`を使います。
CLIのパッケージは`kotowari-cli`と`kotowari-mds`です。全体像を配る`serve`は`kotowari-cli`にあり、ライブラリにはありません。

## 読込みと結果の寿命

<!-- @kotowari[REQ-core-310:ba432526, REQ-core-316:a7ac5b52, TBL-core-041:0dbc6ef0] -->

`Project::new(ProjectOptions::new(absolute_start))`で開始点を明示します。
`check`、`list`、`query`、`status`、`plan`、`mutants`、`changes`は型付きの結果を返します。`check`と`status`は全体像の元データの指摘と`overview`の群を含みます。
`overview_prepare`は全体像の元データを検査して描画し、ファイルを書きません。返った結果の`write()`で`.kotowari/cache/overview/`の下へ書きます。`overview_build`はこの2つを続けて行い、書いたファイル・消したファイルの一覧と書かなかったファイルの数を返します。`AsyncProject`にも同じ2つがあります。
完了した指摘は結果の`findings()`で読み、読めないファイルや設定の誤りは`Error`と`ErrorKind`で区別します。
`read()`と`inspect()`の結果は保持して再利用でき、後からファイルが変わっても自動更新されません。読み直す場合はProjectを再度呼びます。
`plan`や`mutants`はIRを読みません。

## メモリ入力とパス

<!-- @kotowari[REQ-core-314:8f4d9ebd, REQ-core-315:5358c14d, REQ-core-322:2a2cab1a, REQ-core-323:d15de12c] -->

`SourceText`は相対の論理パスと元の文字列を保持します。区切りやドット成分を正規化し、絶対パスと空のパスを拒否します。入力群の構築では同一群の重複も拒否します。親相対パスは使えます。
別の有効な入力群で同じパスを使う場合、文字列も一致させます。
入力の`None`は未提供、`Some(vec![])`は提供したが空、という意味です。設定で有効な群は必須で、無効な群は無視します。
`ir::parse`は不正・欠落したIDを持つ項目も、得られた位置や指摘とともに返します。CLIの一覧対象とは異なります。
ProjectとSchemaLoaderの開始点、Loaderのキャッシュ基準は絶対パスです。構築後に呼出側のカレントディレクトリが変わっても解決基準は変わりません。

## スキーマと値の抽出

<!-- @kotowari[REQ-schema-068:627f96eb, REQ-schema-069:e54e85ec, REQ-schema-070:e745e567] -->

`Schema::parse`で構文と意味を検証し、`Document::parse`の結果と再利用します。
`extract_validated`は成功した値だけを返します。指摘があっても値を読む場合は`extract_partial`で値と指摘を両方確認します。
型付きの部分抽出は`extract_typed_partial`を使います。素のAST JSONとは別の形式です。
`ValidationOptions`の既定値はスキーマのopen設定を尊重します。明示した緩和はopenでないスキーマにも適用できます。
SchemaLoaderの`load`はスキーマと文書の組を返し、再度取得せずに検証・抽出できます。

## Tokioから待つ

<!-- @kotowari[REQ-core-318:462e2e15, REQ-core-319:3282b943, REQ-core-320:4e20b2da, REQ-core-321:4b500917, REQ-schema-075:df620a37] -->

高水準の2ライブラリで、既定で無効な`tokio` featureを有効にすると`AsyncProject`と`AsyncSchemaLoader`を使えます。
呼出側のTokioランタイムで待ち、内部ランタイムは作りません。ランタイム不在やタスクの失敗も型で返します。
`AsyncOptions`は既定の上限1を持ち、`NonZeroUsize`で変更できます。複製したオブジェクトは枠を共有します。
枠待ちのFutureを破棄すると、その処理は投入されません。投入済みの処理は待機をやめても完了まで枠を保持します。Git、HTTP、計算やキャッシュ書込みの中断は保証しません。

各クレートの`examples/`に呼出例があります。配布物からの実行は[パッケージ検証](package-validation.md)を参照してください。
