# ライブラリのクレート境界

[English](library-crates.md) | 日本語

公開する Rust のライブラリと CLI のパッケージの分け方、それぞれの責務と依存の向き、利用者がどの入口を選ぶかを扱う。Tokio から待つ入口は[任意の Tokio 対応](library-async.ja.md)で、Markdown スキーマのライブラリがファイルを読まないことは「ライブラリの入口」の話題の[ライブラリはファイルを読まず、URL のスキーマも取得しない](../schema/library.ja.md#ライブラリはファイルを読まずurl-のスキーマも取得しない)で決める。プロジェクトを使う操作の入口と結果は[プロジェクトを使う Rust API](library-api.ja.md)で、メモリ上の入力の持ち方は[メモリの入力と読み込んだ結果](library-inputs.ja.md)で決める。

## 用途ごとにパッケージを分け、利用者は用途でパッケージを選ぶ

状態: 実装済み

### 問題

kotowari を、別の Rust のアプリから検査や問い合わせを呼ぶ用途と、kotowari とは独立に Markdown スキーマで検証する用途に使えるようにしたい。これまでは責務が少数のクレートの内部に集まっていて、利用者が用途に合うクレートと API を選べなかった。ライブラリの利用者が `cargo add kotowari` から始められるようにしたい。

メモリ上の検査だけを使う利用者には、ファイルの探索や Git、HTTP のような環境の操作と、CLI のための依存を求めない。依存の向きをコンパイラで縛るためでもある。そのためにクレートの数とリリースの手間が増えることは受け入れた。

### 決めたこと

パッケージは次の9つにする。

| パッケージ | 使う目的 |
|---|---|
| kotowari | プロジェクトを読み、型付きの操作を呼ぶ |
| kotowari-core | メモリ上の仕様の解析と検査、結果の組み立て |
| kotowari-source-analysis | メモリ上のソースの文字列から、テスト、マーカー、面を見つける |
| kotowari-overview | メモリ上の全体像の元データを検査し、参照の表、古い節、描画の入力を作る |
| kotowari-cli | kotowari コマンドの引数、表示、終了コードと、全体像を配る serve |
| kotowari-markdown-schema | メモリ上の Markdown スキーマの検証と抽出 |
| kotowari-markdown-schema-io | 文書とスキーマの読み込みと、検査、抽出 |
| kotowari-markdown-view | メモリ上の描画の入力から、全体像のページを作る |
| kotowari-mds | kotowari-mds コマンドの引数、表示、終了コード |

ライブラリのパッケージの名前は kotowari、CLI のパッケージの名前は kotowari-cli にする。実行するコマンドの名前は kotowari のままにする。

計算は kotowari-core と kotowari-source-analysis に分ける。ソースを解析する依存（ast-grep）は kotowari-source-analysis に閉じ込め、仕様の文書だけを扱う利用者に求めない。kotowari-core の中は、責務ごとのモジュールに分ける。単独のクレートにする効果がまだ明らかでない計算は、クレートでなくモジュールで分ける。

依存の向きは、CLI からライブラリへ、ライブラリから計算へ、の一方向にする。

- CLI と外のアプリは、同じ高水準の操作を使う。kotowari-cli は kotowari を使い、kotowari-mds は kotowari-markdown-schema-io を使う。
- kotowari は kotowari-core、kotowari-source-analysis、kotowari-overview に依存する。
- kotowari-overview は kotowari-core、kotowari-markdown-schema、kotowari-markdown-view に依存する。kotowari-core は全体像を知らない。
- kotowari-markdown-view は、workspace の中のどのパッケージにも依存しない。kotowari を知らない汎用の描画のエンジンだからである。
- kotowari-markdown-schema-io は kotowari-markdown-schema の上に、ファイルと URL からの読み込みを足す。

kotowari-markdown-view と kotowari-overview も、ほかのパッケージと同じく公開できる形にする。公開できる kotowari の依存先は、公開できなければならないからである。

### 見送った案

- kotowari-markdown-view と kotowari-overview を、公開しない内部のクレートにする。kotowari を公開できなくなる。

### 確認すること

- C-core-libcrates-1 （読んで確認）workspace のパッケージは表の9つで、workspace の中の通常依存が、上に書いた依存の向きに従い、CLI のパッケージに依存するライブラリが無い
- C-core-libcrates-2 （読んで確認）表のライブラリごとに、その使う目的の入口を外から呼ぶ利用例が `examples/` にある

## 計算のクレートは環境を操作せず、Tokio と CLI の依存を持たない

状態: 実装済み

### 問題

メモリ上の計算と検査だけを使う利用者に、ファイルの探索、Git の操作、HTTP の取得のための依存と、CLI のための依存を求めたくない。計算を環境の操作から切り離しておけば、依存の向きをコンパイラで縛れる。

### 決めたこと

kotowari-core、kotowari-source-analysis、kotowari-overview は、入力をファイル、環境変数、Git、HTTP から取らない。ファイル、ネットワーク、環境変数を操作しない。これらのクレートは、Tokio と CLI のための依存を持たない。

読み書きは kotowari のライブラリが受け持つ。全体像の元データを読むことと、全体像を書き出すことも kotowari のライブラリが行い、全体像を HTTP で配る serve は kotowari-cli が行う。serve を CLI に置くことで、ライブラリの利用者に HTTP のサーバの依存が入らない。

### 見送った案

- kotowari-overview に、全体像の元データの読み書きと HTTP のサーバまで置く。計算のクレートに環境の操作を置かないことと合わず、ライブラリの利用者に HTTP のサーバの依存が入る。

### 確認すること

- C-core-libcrates-3 （読んで確認）kotowari-core、kotowari-source-analysis、kotowari-overview のコードは、ファイル、環境変数、Git、HTTP、ネットワークを操作しない
- C-core-libcrates-4 （読んで確認）kotowari-core、kotowari-source-analysis、kotowari-overview の通常依存に、Tokio、CLI のため、HTTP のためのクレートが無く、kotowari-core だけに依存する外のアプリの依存の木に ast-grep が入らない

## テストとの対応の判定は見つけたテストとマーカーを受け取り、ソースを解析しない

状態: 実装済み

### 問題

ソースの解析と、仕様に対する判定の依存の向きを分けたい。テストとの対応の判定がソースの解析を呼ぶと、仕様の文書だけを扱う利用者にもソースを解析する依存が要る。

### 決めたこと

kotowari-core のテストとの対応の判定は、見つけたテストとマーカーを受け取って判定し、ソースの解析を呼ばない。

ソースの文字列からテスト、マーカー、面を見つけるのは kotowari-source-analysis の `Analyzer` で、見つけたものと解析の指摘を返す。`Analyzer` はファイルを取りに行かず、対応の判定もしない。

### 確認すること

- C-core-libcrates-5 kotowari-core の検査は、渡された見つけたテストとマーカーだけで、ソースのファイルもソースの解析も求めずに、テストとの対応を判定する
- C-core-libcrates-6 `Analyzer` は、ファイルの無いパスを付けたソースの文字列からも、テスト、マーカー、解析の指摘を返す

## 公開の署名に内部の実装の型を出さない

状態: 実装済み

### 問題

外の利用者が、内部の実装の型に頼るのを避けたい。Markdown の解析の道具が持つ構文木の型や、内部の検索のエンジンの型を公開の署名に出すと、その道具を替えるだけで公開の API が壊れる。

### 決めたこと

公開の署名に、Markdown の解析の道具の構文木の型と、ソースを検索するエンジンの型を含めない。Markdown スキーマのライブラリで具体的にどの型を出さないかは、「ライブラリの入口」の話題の[内部の実装の型を、公開の署名に出さない](../schema/library.ja.md#内部の実装の型を公開の署名に出さない)で決める。

依存先として公開する下位のクレートの API も、公開の API として互換性を管理する。実装の細部は公開しない。

### 確認すること

- C-core-libcrates-7 （読んで確認）公開するクレートの公開の署名に、Markdown の解析の道具の構文木の型と、ソースを検索するエンジン（ast-grep）の型が出てこない

## 公開にするかは外の利用者が頼るかで決め、kotowari は入口から届く下位の型だけを同じ型のまま再公開する

状態: 実装済み

### 問題

下位のクレートも互換性を管理するので、外の利用者が頼らない項目を公開のままにすると、その分だけ変えにくくなる。一方で、ワークスペースの中で使われているかで決めると、仕様に入口として書いた口まで消すことになる。

kotowari の利用者には、kotowari に依存するだけで、入口で使う型が手に入るようにしたい。

### 決めたこと

公開するクレートの API の要素を公開にするかは、ワークスペースで使われているかではなく、外の利用者が約束として頼るかで決める。次のものは公開しない。

- 読み取り専用のつもりの結果型を、外から作る関数
- 受け取って読まない引数と欄
- 外の呼び手が要らない途中の部品

仕様に入口として書いたもの、自然な読み取りの口、kotowari の結果から届く型の再公開は残す。

kotowari は、入口で使う下位の入力型と結果型を複製せず、同じ型のまま再公開する。入口の入力と結果から届かない下位の型は再公開しない。メモリ上で検査する利用者は、そのための入力型を kotowari-core から取る。

### 見送った案

- クレートの外から使われていない公開の項目を、全部狭める。仕様に入口として書いた口まで変えることになり、外の利用者が頼る口を消す。
- 入口から届かない型の再公開を残す。入口で使う型を再公開する、という決まりより広いまま残る。
- kotowari の側で、core の結果を包む別の結果型を作る。集計が2か所になる。

### 確認すること

- C-core-libcrates-8 （読んで確認）読み取り専用のつもりの結果型を外から作る関数、受け取って読まない引数と欄、外の呼び手が要らない途中の部品が、公開されていない
- C-core-libcrates-9 （読んで確認）kotowari が再公開する下位の型は、どれも kotowari の入口の入力か結果から届く
- C-core-libcrates-10 kotowari が再公開した型は、下位のクレートの同じ型として受け渡せる
