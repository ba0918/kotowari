# 変更履歴

kotowari の利用者に見える変更を書く。形は [Keep a Changelog](https://keepachangelog.com/ja/1.1.0/) に、版の付け方は [Semantic Versioning](https://semver.org/lang/ja/) に従う。
`agent/skills/` の skill は kotowari と同じタグで出すので、skill の変更もここに書く。同梱の `kotowari-mds` の変更はここに書かない。

## [Unreleased]

### Changed

- すべてのクレートの最小対応 Rust バージョン（`rust-version`）を 1.99 にした。これまで 1.89 を宣言していたクレートは下限が上がり、宣言の無かった `kotowari`、`kotowari-core`、`kotowari-source-analysis`、`kotowari-overview` と `kotowari-cli` も 1.99 を宣言する。

- README、ガイド、IR を英語と日本語の対にした。接尾辞の無い `README.md` や `docs/guides/*.md` が英語、`*.ja.md` が日本語で、各ページの題名の下の行で切り替えられる。ガイドからは決定の経緯を説明していた「なぜこういう作りか」の節を除いた（経緯は判断の記録にある）。

- `kotowari` スキルで、ガイドは製品を使う人向けの使い方、全体像は製品を作る人向けの決定・理由・予定・未決事項を説明するよう書き分ける指針を明確にした。全体像では振る舞いを判断の結果として短く示し、コマンドや設定、使い方の手順はガイドに置く。

- **BREAKING（Rust API・パッケージ構成）** 根を`kotowari-cli`に変更し、`kotowari`を明示した開始点から操作するライブラリに分離した。メモリ計算は`kotowari-core`、ソース解析は`kotowari-source-analysis`を使う。旧coreのCLI・取得APIを呼ぶコードは移行が必要。バイナリのコマンド、出力、終了コードは変わらない。

- `kotowari` スキルに「What the IR holds」を足し、IR に書くのは製品の利用者が観測できる振る舞いだけと定めた。CI とワークフロー、フック、リリース手順、ビルド設定、リポジトリ自身のデータ、プロジェクト自身のテストと検査は IR に書かず、判断の記録に書いて要求もテストも持たせない。迷ったら書かない側に倒し、テストを正当にするために IR に書き足すことは禁じた。壁打ち、計画、実装、cycle、review、iterate はこの定義を参照する。
- 変更照合の reference で、CI・フック・リリース・ビルド設定だけの変更は、決定を引いた `new` で `ir` と `requirements` を空にし、`missing_spec` の穴にしないと明記した。

### Added

- 仕様・判断の記録・テストのつながりと、機械検査の範囲を説明する日英の紹介ページを追加した。

- `kotowari overview build`と`kotowari overview serve`を追加した。設定の`overview.files`に当たる全体像の元データ（frontmatterの`ir`、題名、冒頭の`lead`の部品、節と部品を持つMarkdown）を検査し、誤りが無ければ`.kotowari/cache/overview/`の下に一覧と全体像ごとのHTMLのページを書く。serveはそれを`http://127.0.0.1:<port>/`（既定4590、`--port`で変更）で配り、Ctrl-Cで終わる。元データに誤りがあれば何も書かずに`overview error`で、ポートを使えないか配っている間に接続を受け付けられなければ`port error`で止まる。`.kotowari`、`.kotowari/cache`、`.kotowari/cache/overview`のどれかがシンボリックリンクかディレクトリでなければ何も書かず消さずに、置き場の作成、書き込み、削除に失敗したときもそこで、`cache error`で止まる。
- 設定に`overview`の鍵を書くと、`kotowari check`と`kotowari status`が全体像の元データを読み、形、部品の中身、冒頭の`lead`、扱うIRの文書、部品の中の参照、ページの名前の重なりの誤り（`overview_form_invalid`、`overview_part_unknown`、`overview_part_invalid`、`overview_lead_missing`、`overview_ir_missing`、`overview_ir_shared`、`overview_ref_unresolved`、`overview_name_conflict`）と、元データの節のガイドの印の古さを報告する。checkのJSONとstatusにはいつも`overview`の群（`files`と`marks`）が入り、鍵が無ければ両方0。
- 全体像の一覧を目次で描く。設定の`overview`には目次のYAMLファイルを指す`overview.toc`も必須になり、一覧は目次の題名を見出しに、目次の群（`title`、省いてよい一行の`note`、`items`）を書かれた順と入れ子で、畳める形で描く。カードには見直していない節・未決・予定の数、群の見出しにはページ数と見直していない節・未決の合計を添える。各ページには目次の中の位置と、同じ群のほかのページへのリンクが付く。目次の形の誤り、目次に無いページ、元データの無い名前、2回目以降の名前、空の群は`kotowari check`と`kotowari status`の誤り（`overview_toc_invalid`、`overview_toc_page_missing`、`overview_toc_page_unknown`、`overview_toc_page_duplicate`、`overview_toc_group_empty`）になり、buildとserveを止める。目次のファイルが無いか読めなければ止まり、`overview.files`、`guides.files`、`tests.files`の走査で読むファイルに当たれば設定の誤りで止まる。全体像の元データが1つも無いと通る目次は書けないので、`overview`の鍵は最初の全体像の元データと目次と一緒に書く。
- `Project`と`AsyncProject`に、何も書かない`overview_prepare`と、続けて書く`overview_build`を追加した。`check`、`status`、`inspect`は全体像の元データの指摘と`overview`の群を含む。全体像の検査を行う`kotowari-overview`クレートを追加した。
- 同期`Project`、保持して再利用する`ReadModel`・`Inspection`と、既定で無効な`tokio` featureの`AsyncProject`を追加した。
- 9クレートの実際の配布アーカイブを独立したオフライン環境で検証する手順を追加した。版の番号と2製品の配布名は変更していない。
- IR、ガイド、全体像を多言語の対で持てるようにした。設定の`languages`（言語タグの並び）に2つ以上の言語を書くと、IR（用語集と問題の記録を含む）、ガイド、全体像の元データ、目次を、同じディレクトリの`foo.md`と`foo.<言語タグ>.md`の対で持ち、横の`foo.i18n.yaml`に各側のgitのblob hashを記録する。`kotowari check`と`kotowari status`は、欠けた側（`translation_missing`）、一致の記録の誤り（`translation_record_invalid`）、記録と違うhash（`translation_stale`）、文以外の骨組みの食い違い（`translation_structure_mismatch`）、題名の後の切り替えの行の誤り（`translation_switcher_invalid`）、ほかの言語の側へのリンク（`link_language_mismatch`）、判断の記録へのリンク（`link_to_record`）を誤りにする。ほかの言語のIRの側には、用語、曖昧語、文書名の参照、閉じないバッククォート、用語集の形の検査をその言語の用語集で行う。`kotowari list`は最上位の`translations`に対ごとの各側のblob hashを出す。全体像は言語ごとに描き、先頭以外の言語のページを`.kotowari/cache/overview/<言語タグ>/`の下に書いて互いにリンクし、IR、全体像の元データ、目次の対に欠けた側か骨組みの食い違いがあれば書かずに止まる。全体像のUIの文字は英語を本体が持ち、ほかの言語は設定の`labels`に書き（形の誤りは設定の誤りで止まる）、`status`の部品の札は`decided`、`planned`、`open`、`dropped`にした。`kotowari`スキルに対を揃える手順を追加した。`languages`が無ければ今までどおり英語だけで、対を読まない。

### Fixed

- `kotowari overview serve`が、Hostヘッダーが`127.0.0.1:<port>`か`localhost:<port>`でないリクエスト（Hostの無いものを含む）に、本文の無い403を返すようにした。外のサイトが自分の名前を127.0.0.1に向け直して（DNSリバインディング）、ブラウザ経由で全体像のページを読み出せていた。
- `languages`に2つ以上の言語を書いたリポジトリで、`kotowari changes`がほかの言語の側（`foo.ja.md`など）もIRとして読み、照合記録に先頭の言語の側を書くと`change_record_invalid`にしていた。`kotowari check`と同じく先頭の言語の側だけをIRとして読む。
- 多言語の対で、参照形式のリンクと画像も利用箇所の行で検査し、リンク先の並びの食い違いを検出する。全体像の翻訳側だけがテストの glob と重なる場合も設定の誤りで止める。
- `kotowari` スキルのセットアップ手順に、判断の記録用ディレクトリへの `.gitkeep` 作成とコミット対象に含める指示を追加した。空ディレクトリが clone／worktree で失われ、`kotowari check` がエラーになるのを防ぐ。
- `kotowari overview build`のページで、GFM の脚注の記法（`[^1]`）を脚注として描かないようにした。描くと日本語のページにも英語の見出し「Footnotes」と戻りリンクの文字が入っていた。
- `kotowari overview build`で、見出しと題名から HTML のコメントだけを除くようにした。これまでは`Vec<String>`の`<String>`のように行の中の HTML と読まれる部分も消え、見出しが「Vec」になっていた。


## [0.3.0] - 2026-10-01

### Added

- 配布スキルに、判断の記録への書き戻しと両役の YAML 照合記録、独立 review 後のブランチ全体検査、仕様の穴の処理先を追加した。承認済み要求を保つ委譲範囲の具体的な IR 追加を認め、変更・削除や根拠の無い意味の判断は人へ戻す。
- `kotowari changes` で指定した Git 比較元と commit または index の変更を列挙し、実装・review の照合記録と内容の鮮度を検査できる。設定の `changes` は省略可能で、導入した場合は check/status でも記録の形式と有効な参照を検査する。

## [0.2.0] - 2026-09-27

### Added

- README の入れ方に mise の `github:` で入れる方法（`mise use -g 'github:ba0918/kotowari[version_prefix=kotowari-v]@0.1.0'`）を足した。
- 面の検査。設定の `surface.files` と `surface.rules`（ast-grep の規則）でコードから利用者に見える面（CLI のサブコマンドやフラグなど）を取り出し、名前が IR の要求の文、決定表のセル、シナリオのステップに引用されていなければ `kotowari check` が `surface_without_spec` の誤りにする。面のファイルのうち読むのは規則の言語のものだけで、`src/**` のように広く書いて画像などに当たっても止まらない。規則を書かなければ何も起きない。
- まだ IR にしない面は、`surface.unspecified` が指す一覧に理由付きで載せると外せる。形の誤った1件は `surface_unspecified_invalid` の誤り、要らなくなった1件は `surface_unspecified_stale` の注意になる。外した数は `check` の最後の行 `surface: unspecified=数`（JSON は `surface`）に、面の数は `status` の `surface` の行に出る。
- `kotowari` skill に面の検査の reference `surface.md` を足し、`kotowari-adopt` に一覧の減らし方を書いた。一覧に足すのは `kotowari-brainstorm` と `kotowari-adopt` だけで、実装役は IR に無い面を壁打ちに戻す。

### Changed

- `kotowari-adopt` は範囲の確認で話題の利用者の入口（コマンドや画面の操作）も確かめ、その入口から観測できる振る舞いだけを一覧の行にする。範囲の外で見つけた振る舞いは件数と次の話題の候補だけを見せる。1行は要求1つの候補で、値や文言だけが違うものは1行にまとめて決定表の候補にし、候補が `limits.requirements` を超えたら入口が混ざっていないかを見直す。
- `kotowari status` の出力に `surface` の群（`total`、`specified`、`unspecified`）がいつも入る。面の規則を書いていなければ3つとも 0。

## [0.1.0] - 2026-09-26

### Added

- 仕様を「IR」と呼ぶ決まった形の Markdown に書き、機械で検査する CLI `kotowari`。出力は既定で JSON で、人が読むときは `--format text` を付ける。
- `kotowari check`：IR の形、要求の出典にした判断の記録が実在するか、どのテストがどの要求を確かめているかを検査し、指摘を出す。終了コードは 0 が指摘なし、1 が誤りあり、2 が検査に入れなかった。利用者向けのガイドに付けた印が今の IR と食い違っていないかも見る。
- `kotowari list`：IR の項目とシナリオを、印の付いたテストと一緒に一覧にする。
- `kotowari query <ID>`：1件の本文と、それを確かめるテスト、それを指す項目を出す。
- `kotowari status`：揃っているかを数で集計し、最後の行の `complete true` か `complete false` で答える。
- `kotowari plan <ファイル>`：実装の計画のファイルの形を、同梱のスキーマで検査する。
- `kotowari mutants --tool cargo-mutants <結果のファイル>`：変異テスト（cargo-mutants）の結果を読み、見逃しを指摘にする。
- テストの印 `@kotowari[ID]` は、Rust、TypeScript、JavaScript、Python、PHP のテストなら同梱の規則で読む。ほかの言語でも、ast-grep（tree-sitter）が扱える言語なら、設定の `tests.rules` に規則を書けば読める。
- Claude Code の skill を `agent/skills/` に10個。`kotowari` は IR と判断の記録の書き方、指摘の直し方、印の置き方を教える。`kotowari-` で始まる9個は、壁打ちから計画、実装、レビューまでの工程を kotowari の上で回す（任意）。
- 入れ方は2通り。GitHub Release のビルド済みのバイナリ（Linux x86_64、macOS arm64。SHA256 付き）と、タグで版を固定した `cargo install --git https://github.com/ba0918/kotowari --tag kotowari-v0.1.0 kotowari`。skill は `gh skill install` の `--pin` にタグを渡すと版を固定して入れられる。

[Unreleased]: https://github.com/ba0918/kotowari/compare/kotowari-v0.3.0...HEAD
[0.3.0]: https://github.com/ba0918/kotowari/compare/kotowari-v0.2.0...kotowari-v0.3.0
[0.2.0]: https://github.com/ba0918/kotowari/compare/kotowari-v0.1.0...kotowari-v0.2.0
[0.1.0]: https://github.com/ba0918/kotowari/releases/tag/kotowari-v0.1.0
