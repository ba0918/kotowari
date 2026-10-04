# 全体像を公開クレートの構成に載せる

## Context

全体像（overview）の仕様は [全体像の記録](./2026-10-02-whole-picture.md) で、kotowari を公開クレートに分ける仕様は [公開クレートの記録](./2026-10-03-public-crate-api.md) で承認した。後者の [A33](./2026-10-03-public-crate-api.md#A33) は、全体像を新しい構成に載せることと、検査結果を check に混ぜる先を kotowari ライブラリへ移すことだけを決めた。
全体像の実装の計画を書く前に突き合わせると、全体像の I/O の置き場、core が全体像を知らないまま指摘を混ぜる方法、ライブラリの公開 API、新しい2つのクレートの配り方が、どちらの記録でも決まっていなかった。
計画が推測で埋めないよう、ここで決める。

Position: 利用者が4問すべてに推奨で答えた（2026-10-04）。

## Agreements

- A1 kotowari-overview は全体像の元データの文字列から、指摘、参照の表、古い節、描画の入力を作る計算だけを持ち、ファイル、ネットワーク、環境変数を操作しない。元データの読み込みと ".kotowari/cache/overview/" への書き込み（build）は kotowari ライブラリが行い、HTTP で配る serve は kotowari-cli が行う。
  - why: 公開クレートの構成は、計算のクレートに環境の操作を置かず、読み書きを kotowari ライブラリに置く（[REQ-core-307](../../ir/core/library-crates.md#REQ-core-307)）。serve を CLI に置けば、ライブラリの利用者に HTTP サーバの依存が入らない
  - rejected: [A49（全体像）](./2026-10-02-whole-picture.md#A49) のまま kotowari-overview に読み書きと HTTP サーバまで置く
  - decided_by: 利用者（推奨を採用）

- A2 core の検査の入力は、名前を付けた追加の指摘の群（読んだファイルの数、印の数、指摘）を受け取れる。core はその群の指摘をほかの指摘と合わせて並べ、counts、終了コード、complete に数え、群の数を結果に持たせる。core は群の意味を知らない。kotowari ライブラリは全体像の指摘と "overview" の群をこの形で渡す。
  - why: core は全体像を知らない（[REQ-core-287](../../ir/core/overview-data.md#REQ-core-287)）。検査の結果は core だけが組み立て、外から組み立てたり書き換えたりさせない公開 API の決まりを保つ。並べ方と集計を2か所に持たない
  - rejected: kotowari の側で core の結果を包む別の結果型を作る（core の型をそのまま再公開する [REQ-core-309](../../ir/core/library-crates.md#REQ-core-309) とぶつかり、集計が2か所になる）
  - decided_by: 利用者（推奨を採用）

- A3 Project の check、status、inspect は全体像の指摘と "overview" の群を含める。Project に全体像を build する操作を足し、書いたファイル、消したファイル、書かなかったファイルの数を返す。AsyncProject にも同じ操作を足す。serve はライブラリに置かない。
  - why: CLI は Project の上の薄い層なので、check の統合は Project で行う。非同期の入口は同期の高水準操作に対応する（[A45（公開クレート）](./2026-10-03-public-crate-api.md#A45)）
  - rejected: build を同期だけにする（非同期との対応が崩れる）、全体像の操作を公開しない
  - decided_by: 利用者（推奨を採用）

- A4 kotowari-markdown-view と kotowari-overview は、ほかのパッケージと同じく公開できる形にし、今回は公開しない。kotowari-markdown-view は Markdown スキーマ系、kotowari-overview は kotowari 系の版に載せ、配布物の検証の対象に加える。kotowari は kotowari-overview に通常依存し、kotowari-overview は kotowari-core、kotowari-markdown-schema、kotowari-markdown-view に通常依存する。kotowari-markdown-view は workspace の中のパッケージに依存しない。
  - why: 公開できる kotowari の依存先は公開できなければならない。view は mds と同じく kotowari を知らない汎用のエンジン（[A41（全体像）](./2026-10-02-whole-picture.md#A41)）
  - rejected: 2つを publish = false の内部のクレートにする（kotowari を公開できなくなる）
  - decided_by: 利用者（推奨を採用）

- A5 全体像の元データの形のスキーマは、".kotowari/schemas/overview.yaml" でなく kotowari-overview のパッケージの中の "crates/kotowari-overview/schemas/overview.yaml" に置き、コンパイル時に取り込む。
  - why: 公開できるパッケージは自分の配布物だけでビルドできなければならず、core の埋め込みのスキーマも同じ理由でパッケージの中へ移した（[A46（公開クレート）](./2026-10-03-public-crate-api.md#A46)）。置き場が変わるだけで、形の決まりは [A77（全体像）](./2026-10-02-whole-picture.md#A77) のまま
  - decided_by: LLM（A4 から導いた細部。承認の時に利用者に示す）

- A6 serve が割り込み（Ctrl-C）で終了コード0で終わるために、シグナルを受けるクレート ctrlc を kotowari-cli に足す。
  - why: [REQ-core-297](../../ir/core/overview-commands.md#REQ-core-297) は割り込みで終了コード0を求めるが、既定の SIGINT はプロセスをシグナルで終わらせ0にならない。[A37（全体像）](./2026-10-02-whole-picture.md#A37) が足すとしたのは HTTP サーバのクレート1つで、シグナルの扱いは決めていなかった。シグナルの扱いは OS ごとに安全性の落とし穴があるので自作しない
  - rejected: libc などでシグナルの扱いを自作する
  - decided_by: LLM（計画の段で見つけた穴。承認の時に利用者に示す）

- A7 Project の全体像の build は、検査と描画をメモリの中で済ませて何も書かない準備（overview_prepare）と、準備の結果を ".kotowari/cache/overview/" へ書く段に分ける。overview_build はこの2つを続けて行う。AsyncProject も両方を持つ。serve は準備、ポートの確保、書き込みの順に呼ぶ。
  - why: [REQ-core-297](../../ir/core/overview-commands.md#REQ-core-297) は serve に検査、ポートの確保、描画と書き込みの順を求める。build を1つの呼び出しにすると、CLI は検査より先にポートを取るか、ポートより先に書くしかなく、使用中のポートと元データの誤りが重なったときの停止の理由や、ポートが使えないのに書いてしまうかが仕様と食い違う
  - rejected: serve だけ check を丸ごと走らせてから build する（全体像と関係の無い誤りで serve が止まる）
  - decided_by: LLM（計画の敵対的レビューで見つけた穴。承認の時に利用者に示す）

- A8 停止の文言 "overview error" は kotowari ライブラリの失敗の種類として持ち、CLI がその文言に写す。"port error" は serve を持つ kotowari-cli の停止の理由として持つ。core の停止の理由には足さない。
  - why: core は全体像を知らず（A2）、serve は CLI にある（A1）。文言の置き場を決めないと、core に全体像と serve の文言が入る
  - decided_by: LLM（計画の敵対的レビューで見つけた穴。承認の時に利用者に示す）

- A9 build と serve は、".kotowari"、".kotowari/cache"、".kotowari/cache/overview" のどれかがシンボリックリンクか、ディレクトリでない別の種類のファイルなら、何も書かず消さずに止める。置き場の作成、ファイルの書き込み、削除に失敗したときも止める。止める理由は新しい「置き場の誤り」（"cache error"）とし、詳細は問題のパスと、あれば OS の誤りの文にする。
  - why: 置き場かその上がリンクだと、書き込みと削除がリンク先、つまり基準のディレクトリの外に届く（レビューで実際に外のファイルが消えた）。リンクは複製したリポジトリにコミットできる。書けない置き場を「読めないファイル」で止めると、読みの失敗と取り違える
  - rejected: 既存の「読めないファイル」で止める
  - decided_by: 利用者（推奨を採用）

- A10 `全体像の元データ`の題名の後、最初の "## " の見出しより前の、lead に続く`部品`は、ページで lead の後、最初の節の前に並びの順に描く。view の`文書`はそのための冒頭の`部品`の並びを持つ。
  - why: 形の決まり（[TBL-core-038](../../ir/core/overview-data.md#TBL-core-038)）はそこに部品を置けるとしているのに、描く場所が無く、検査を通った部品が黙って消えていた
  - rejected: 最初の "## " より前を lead だけにして、ほかの部品を形の誤りにする（承認した形を狭める）
  - decided_by: 利用者（推奨を採用）

- A11 frontmatter の形の違反（無い、読めない、知らない鍵、"ir" が無いか空か文字列の一覧でない）の overview_form_invalid の detail は "frontmatter" の1語にする。
  - why: kotowari-markdown-schema は frontmatter を検査せず、当てはまる種類の名前が無い。別の意味の種類の名前を借りると読み違える
  - rejected: 実装が借りた missing_required_field などの名前をそのまま決める
  - decided_by: 利用者（推奨を採用）

- A12 [EX-core-219](../../ir/core/cli.md#EX-core-219) と [EX-core-241](../../ir/core/cli.md#EX-core-241) の「REQ-core-001 の7つのコマンド名」を8つに直す。
  - why: overview を足して REQ-core-001 のコマンドは8つになったが、2つのシナリオの数が直っていなかった。印の付いたテストは8つを確かめている
  - decided_by: LLM（数の直しだけ。承認の時に利用者に示す）

- A13 status の`部品`の札は、"決定"、"予定"、"未決"、"取り下げ" の4つの状態を互いに見分けられる見た目で描く。
  - why: 実装は状態ごとに色を変えていたが、仕様に無いので確かめるテストを根拠つきで書けず、変異テストの見逃しになっていた。状態は一目で見分けられてこそ役に立つ
  - decided_by: 利用者（推奨を採用）

- A14 "kotowari overview serve" が配っている間に接続の受け付けに失敗したら、ポートの誤りを理由に止める（終了コード2）。詳細は "127.0.0.1:<ポート>" と OS の誤りの文にする。
  - why: 使っている HTTP のクレートは受け付けに一度失敗すると受け付けを再開しないので、生きたまま何も配らなくなり、[REQ-core-297](../../ir/core/overview-commands.md#REQ-core-297) の「配り続ける」を守れない。黙って固まるより止めて知らせる
  - rejected: 受け付けに失敗したらサーバを作り直して続ける
  - decided_by: 利用者（推奨を採用）

## Revisions

- [A77（全体像）](./2026-10-02-whole-picture.md#A77) のスキーマの置き場を A5 で改めた。
- [A49（全体像）](./2026-10-02-whole-picture.md#A49) のうち、build と serve を kotowari-overview に置く部分を A1 で置き換えた。読み取りと検査のうち、ファイルの読み込みも A1 で kotowari ライブラリへ移した。
