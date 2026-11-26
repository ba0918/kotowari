# 次のリリースの束：設定のディレクトリ、誤りの文字列の往復、型付きの抽出の文、公開 API の整理

## Context

全クレートのリファクタリング（PR #39）で、振る舞いが変わる案と公開 API を狭める案を保留にしていた。
残っていたのは、ライブラリで設定ファイルの指定がディレクトリのときの分類（[issue #40](https://github.com/ba0918/kotowari/issues/40)）、誤りの文字列に前置きを付けて剥がす往復（[issue #43](https://github.com/ba0918/kotowari/issues/43)）、スキーマの検査の文が CLI のサブコマンドの名前で理由を書いていること（[issue #42](https://github.com/ba0918/kotowari/issues/42)）、外の利用者が頼らない公開の項目である。
どれも Rust の API か利用者に見える文を変え、破壊的な変更を含むので、kotowari 0.6.0 と kotowari-mds 0.2.0 のリリースに束ねて決める。
リリースの手順そのもの（版、タグ、配布）は IR の外で、PROJECT.md の手順に従う。

Position: 壁打ちは 2026-10-24 に全ての問いに答えが出て収束した。次は計画（kotowari-plan）。

## Agreements

- A1 今回のリリースで kotowari 0.6.0 と kotowari-mds 0.2.0 の両方を出す。mds の CHANGELOG は、移す前のリポジトリで出した 0.1.0 と、取り込んだあとの変更に分けて整える
  - why: mds 側の変更（A5、A9）も破壊的で、溜まった変更と一緒に出すほうが利用者の移行が一度で済む。mds の CHANGELOG は次に mds を出すときに作ると決めていた（[A14 (release-flow)](./2026-09-26-release-flow.md#A14)）
  - decided_by: 利用者（推奨を採用）
- A2 kotowari のライブラリは、"ProjectOptions" で渡された設定ファイルがディレクトリのとき、無いときと同じく入力の誤り（"InvalidInput"）で止まり、文は "config file is a directory: <path>" にする。CLI は自前の判定を消してライブラリの結果を写し、標準エラーは "argument error: config file is a directory: <path>" になる。終了コードは変わらない
  - why: 今はディレクトリを CLI だけが引数を読む段階で見ていて、ライブラリでは読もうとして失敗し "ReadFailure" になる。無いときは "InvalidInput" なので、同じ指定の誤りが種類で分かれていた。判定をライブラリの1か所にまとめれば CLI とライブラリで揃う
  - decided_by: 利用者（推奨を採用）
- A3 kotowari-source-analysis の "AnalysisError::InvalidRules" は String のまま、停止の理由の前置き（"config error: "）の無い詳細を持つ。"detail()" は剥がさずにそのまま返し、"Display" は "invalid rules: <詳細>" になる
  - why: 今は前置き付きの文を入れて "detail()" で剥がしていて、前置きの文言を変えると二重の前置きになりうるのに型では捕まらない。運ぶ停止の理由は設定の誤りしか無いので、型を StopReason にするほどの理由は無い
  - decided_by: 利用者（推奨を採用）
- A4 "kotowari::Error" に、種類が "UnknownQuery" のときだけ知らない ID を返すメソッドを足す（非破壊）。CLI はそれを使い、出力 "argument error: unknown id: <ID>" は変わらない
  - why: 今の CLI はライブラリの文から "unknown query: " を剥がして ID を取り出していて、A3 と同じく文言の変更に弱い。足すだけの API で CLI の出力を保てる
  - decided_by: 利用者（推奨を採用）
- A5 スキーマの検査の文を `document has the key "<path>", which conflicts with "type", where typed extraction puts "name"` にし、[TBL-schema-009](../../ir/schema/cli.md#TBL-schema-009) の該当の条件（英語と日本語）も型付きの抽出の言い方に直す
  - why: 文を出すのは kotowari-markdown-schema（エンジン）で、ライブラリだけを使う人には使っていないサブコマンドの名前で理由が書かれていた。型付きの抽出はライブラリの入口（[TBL-schema-010](../../ir/schema/library.md#TBL-schema-010) の extract_typed_partial）にもある言い方である
  - decided_by: 利用者（推奨を採用）
- A6 公開 API は、ワークスペースで使われているかではなく、外の利用者が頼る約束に入るかで残すか狭めるかを決める。狭めるのは、読み取り専用のつもりの型を外から作れる口、受け取って読まない引数や欄、外の呼び手が要らない途中の部品、kotowari の入口から届かない下位の型の再公開。残すのは、IR に入口として書いてあるもの（resolve_schema、SchemaLoader::extract_*）、自然な読み取りの口（Schema::name）、kotowari の結果から届く型の再公開（Config、SurfaceTally、IrDocument など）
  - why: 使われているかで決めると、IR に入口として書いた口（[TBL-schema-010](../../ir/schema/library.md#TBL-schema-010)、library-io.md）まで消すことになる。下位のクレートも互換性管理の対象（[A16 (public-crate-api)](./2026-10-03-public-crate-api.md#A16)）なので、約束に入らない項目を公開のままにすると、その分だけ変えにくくなる
  - decided_by: 利用者（推奨を採用）
- A7 A4 のメソッドは `pub fn unknown_id(&self) -> Option<&str>` とする
  - why: CLI の文（unknown id）と名前が揃い、何の ID かが名前で読める
  - decided_by: 利用者（推奨を採用）
- A8 A6 を当てるのは、保留にしていた候補と、同じ場所で同じ理由が当たるものまでにする。core の terms の公開の関数4つ（check_unknown_terms、check_vague_words、check_document_references、check_terms_and_vague_words_with_duplicates）と、ir の fence の公開の項目3つ（CodeFence、parse_opening_fence、is_closing_fence）は crate の中だけにし、check_unknown_terms の使われない引数と CodeFence の読まれない欄 lang は中で消す。kotowari の ReadModel から届くのに再公開していない GuideReader を kotowari から再公開する。9つのクレートの全体の棚卸しはしない
  - why: 候補を確かめる途中で、隣に同じ理由のものが見つかった。全体を機械で棚卸しすると判断の回と実装、変異テストの量が増える。残りがあれば次の破壊的なリリースで扱える
  - decided_by: 利用者（推奨を採用）
- A9 kotowari-markdown-schema の Finding を作る関数（new、at、maybe_at、of_node、of_rule）を crate の中だけにする
  - why: Finding の欄はすでに crate の中だけで、外からは読む口だけが要る。作る関数は crate の中でしか使われず、外から作れると検証を通らない作り方が公開に残る（[REQ-schema-051](../../ir/schema/library.md#REQ-schema-051)）
  - decided_by: 利用者（仮決めの推奨をそのまま送った）
- A10 kotowari-core の TestAnalysis.language と SurfaceAnalysis.language を消し、kotowari-source-analysis も入れなくなる
  - why: source-analysis が言語の Debug 表記を入れているが、core も kotowari も CLI も読まない。問い合わせの有無は別の欄が運んでいる
  - decided_by: 利用者（仮決めの推奨をそのまま送った）
- A11 kotowari-core の check_documents_with_duplicates と GlossaryDuplicates を crate の中だけにする。check_documents は公開のまま
  - why: どちらも core の中でしか使わない検査の途中の部品で、外の呼び手は check_documents とメモリ上の入力からの検査を使う
  - decided_by: 利用者（仮決めの推奨をそのまま送った）
- A12 kotowari から CheckInputs、ReadInputs、IrOptions、SourceText、TestAnalysis、SurfaceAnalysis の再公開をやめる。メモリ上で検査する利用者は kotowari-core から取る
  - why: どれも core のメモリ上の入力で、kotowari の Project は受け取りも返しもしない。再公開は入口で使う型に限る（[A36 (public-crate-api)](./2026-10-03-public-crate-api.md#A36)）。kotowari-source-analysis の Analyzer はこれらを返すので、そちらの再公開は残る
  - decided_by: 利用者（仮決めの推奨をそのまま送った）
- A13 Tool を kotowari に定義し、kotowari-core からは消す。kotowari::Tool のパスは変わらない
  - why: Tool は core の根に定義されているが core の中では一度も使われず、使うのは kotowari と CLI だけである
  - decided_by: 利用者（仮決めの推奨をそのまま送った）
- A14 "--config" の指す先の判定（無い、ディレクトリ）は、どちらも設定を読むときにまとめて行う。ほかの引数の誤りと重なったときは、ほかの引数の誤りを先に報告する。終了コードはどちらも 2 で、違うのは標準エラーの1行目だけ
  - why: A2 で CLI の判定を消すと、ディレクトリの判定は引数を読む段階から設定を読む段階に移る。無いときの判定はもとから設定を読むときにあり、同じ順で報告するほうが揃う。[REQ-core-004](../../ir/core/cli.md#REQ-core-004) は重なったときの順を書いていないので IR は変えない
  - decided_by: 利用者（推奨を採用）
- A15 "--config" も "ProjectOptions" の設定ファイルも無く、既定の場所 ".kotowari/config.yaml" がディレクトリのときは今のまま、読めないファイルを理由に停止する（ライブラリでは "ReadFailure"）
  - why: 利用者は引数を渡していないので、引数の誤りにはしない。置かれたファイルが読めない扱いは停止の理由の表（[TBL-core-001](../../ir/core/cli.md#TBL-core-001)）の読めないファイルのとおり
  - decided_by: 利用者（推奨を採用）
- A16 IR に書くのは契約に当たるものだけにする。A5 は [TBL-schema-009](../../ir/schema/cli.md#TBL-schema-009)、A2 と A15 は [REQ-core-312](../../ir/core/library-api.md#REQ-core-312) とそのシナリオ、A4 と A7 は [REQ-core-312](../../ir/core/library-api.md#REQ-core-312)、A6 と A12 は [REQ-core-309](../../ir/core/library-crates.md#REQ-core-309) に書き、A9 は [REQ-schema-051](../../ir/schema/library.md#REQ-schema-051) の出典に足す。A3、A10、A11、A13 は IR に書かず、変更履歴に書く
  - why: ライブラリの IR は契約の粒度で書かれていて、Rust の型やメソッドの名前は入口の表のほかに出てこない。A3、A10、A11、A13 は下位のクレートの文や欄の細部で、CLI の出力も変えない。再公開の基準を IR に書いておけば、次に誰かが届かない型を再公開しようとしたときにレビューで止まる
  - decided_by: 利用者（仮決めの推奨をそのまま送った）
- A17 "config file is a directory: <path>" の <path> は、"--config"（ライブラリでは "ProjectOptions" の設定ファイル）に渡された文字のまま書く
  - why: "config file not found: <path>" と今の CLI の文も渡された文字のままで、停止の詳細の表（[TBL-core-020](../../ir/core/cli-environment.md#TBL-core-020)）の引数の誤りの行（問題の引数の文字）のとおりである
  - decided_by: 利用者（仮決めの推奨をそのまま送った）
- A18 束を1本のブランチで実装して main に入れ、そのあと release/kotowari-v0.6.0 と release/kotowari-mds-v0.2.0 を順に作る。変更履歴には、狭めた API を BREAKING (Rust API)、unknown_id を Added、CLI の文の変化（"--config is a directory" から "config file is a directory"）と AnalysisError の Display の変化を Changed として書く
  - why: kotowari 側と mds 側の変更は互いのコードに依存しないが、計画とレビュー、変異テストが1回で済む。CLI のディレクトリのテストは終了コードだけを見ていて文の変化に気づかないので、変更履歴に書くことを先に決めておく
  - decided_by: 利用者（仮決めの推奨をそのまま送った）
- A48 [TBL-schema-009](../../ir/schema/cli.md#TBL-schema-009) のうち、出典の決定に停止の理由の分類が書かれていなかった4つの条件は、今の IR と実装の分類のまま決定にする。対象に渡した`文書`に "$schema" が無いときはスキーマが見つからない停止、`スキーマ`の YAML が読めないときと`規則種別`の形に反するときはスキーマが形に合わない停止、"$schema" の値が空か空白だけのときは frontmatter が壊れている停止である
  - why: A5 で表を直したので照合の範囲に入り、この4つの条件の分類がどの出典にも書かれていないと分かった。IR はもとからこう書いていて、kotowari-mds もこの停止の理由を出す（2026-10-24 に実物で確かめた：schema_not_found、schema_invalid、frontmatter_invalid）。分類を変える理由は無い
  - decided_by: LLM（照合レビューの指摘から、既存の IR と実装の分類を写した。承認の時に利用者に示す）

## Prohibitions

- A19 版の宣言、タグ、配布を IR に書かない
  - why: 利用者が観察する振る舞いを変えない手順で、IR の外にある（PROJECT.md と release-flow の記録が持つ）
  - decided_by: 利用者（推奨を採用）

## Rejected

- A20 kotowari だけを出し、mds 側の変更は Unreleased に積む
  - why: mds 側の変更も破壊的で、溜まった変更と一緒に出すほうが利用者の移行が一度で済む
- A21 kotowari だけを出し、mds 側の項目は束から外す
  - why: #42 と schema の整理を別の機会に回す理由が無い
- A22 ライブラリにも判定を足し、CLI の判定と文はそのまま残す
  - why: 同じ条件を2か所で見ることになる
- A23 ライブラリではディレクトリを "ReadFailure" と仕様に書く
  - why: 無いときとディレクトリのときで分類が分かれたまま残る
- A24 "AnalysisError::InvalidRules" の中身を StopReason にする
  - why: source-analysis の破壊的変更になるのに、運ぶ種類は設定の誤りしか無い
- A25 "AnalysisError" の前置きを剥がす往復を残す
  - why: 文言を変えたときの二重の前置きが型で捕まらない
- A26 CLI がライブラリの文をそのまま出す（unknown query）
  - why: CLI の出力が変わる。足すだけの API で保てる
- A27 "UnknownQuery" の前置きを剥がす往復を残す
  - why: 文言を変えると前置きが二重になりうる
- A28 誤りの文に型付きの抽出と ast --schema の両方を書く
  - why: エンジンの文に CLI の名前が残る
- A29 type の衝突の文を今のまま残す
  - why: ライブラリだけを使う人に、使っていないサブコマンドの名前で理由が書かれたまま残る
- A30 クレートの外から使われていない公開の項目を全部狭める
  - why: IR の入口（TBL-schema-010、library-io.md）まで仕様を変えることになり、外の利用者が頼る口を消す
- A31 今回は公開 API を狭めない
  - why: 破壊的なリリースを出すこの機会に、外の約束に入らない項目を片付けておく
- A32 A4 のメソッドの名前を query_id にする
  - why: 知らなかったことが名前から読めず、CLI の文（unknown id）とも揃わない
- A33 A4 のメソッドの名前を id にする
  - why: 今は何の ID かが名前から読めない
- A34 9つのクレートの公開の項目を rustdoc の JSON で全部調べる
  - why: 一覧づくりと判断の回、実装と変異テストの量が増える。残りは次の破壊的なリリースで扱える
- A35 保留にしていた候補だけを狭める
  - why: 同じ理由のものが隣に公開のまま残る
- A36 schema の Finding を作る関数を公開のまま残す
  - why: 読み取り専用のつもりの型を外から作れるまま残る
- A37 TestAnalysis.language と SurfaceAnalysis.language を残す
  - why: 外の解析器が意味の無い欄を埋め続ける
- A38 check_documents_with_duplicates と GlossaryDuplicates を公開のまま残す
  - why: 互換性管理の対象に途中の部品が残る
- A39 入口から届かない6つの型の再公開を残す
  - why: [A36 (public-crate-api)](./2026-10-03-public-crate-api.md#A36) の「必要な型を再公開する」より広いまま残る
- A40 Tool を core の mutants モジュールに移す
  - why: core に使われない型が残る
- A41 Tool を core の根に置いたままにする
  - why: core の公開の項目に、core が使わない型が残る
- A42 CLI の早い判定も残して、誤りが重なったときの今の順を保つ
  - why: 同じ条件を2か所で見ることになる（A22 に戻る）
- A43 既定の場所がディレクトリのときも引数の誤り（"InvalidInput"）にする
  - why: 引数を渡していないのに引数の誤りと出て、REQ-core-004 と停止の理由の表も直すことになる
- A44 AnalysisError の文、消した欄、Tool の置き場まで IR に書く
  - why: IR に型の名前が増え、ほかのライブラリの要求と粒度が揃わなくなる
- A45 TBL-schema-009 だけを直し、ほかは変更履歴だけに書く
  - why: A2 と A4 の振る舞いを確かめる要求が無く、再公開の基準も次の整理で忘れられうる
- A46 ディレクトリの文のパスを基準のディレクトリからの相対にする
  - why: 今の出力と変わり、停止の詳細の表の引数の誤りの行（問題の引数の文字）とも食い違う
- A47 kotowari 側と mds 側で計画と実装のブランチを分ける
  - why: 計画とレビューの回が2つになる。片方だけ先に出す理由が無い

## 検査の注意

- check の注意のうち、この壁打ちで触れた文書にかかるのは docs/ir/schema/cli.md の too_many_lines（294行）と too_many_requirements（要求13件）だけで、壁打ちの前から出ていた（残すと決めたのは [A14 (mutants-gaps)](./2026-09-23-mutants-gaps.md#A14)）。この壁打ちは TBL-schema-009 の1つの条件の言い方と出典を直しただけで、行も要求も足していない。中身は mds の CLI という1つの範囲の行で説明でき、責務の混ざりは無いので分けない。
