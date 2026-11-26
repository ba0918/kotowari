# Rust の関門と CI の整え方

## Context

ルールのスキル ba0918-ci と ba0918-rust に照らすと、このリポジトリには次の外れがあった（2026-10-06 の点検）。

- ツールチェーンが固定されておらず、CI は "stable" を入れていた。edition と最小対応 Rust バージョン（MSRV）はクレートごとに書かれ、4クレートは 1.89、残りは宣言が無かった。lint の設定が無かった。
- 整形、lint、テストを回して main を守る CI が無かった（必須のチェックは mutants だけで、clippy には16件の警告が残っていた）。
- change-conformance.yml のアクションが commit hash で固定されておらず、資格情報を残し、時間の上限が無かった。ほかのワークフローにも、時間の上限、pipefail、同時実行の扱い、名前、権限を広げる理由の欠けがあった。release.yml と mutants-run.yml に分岐する手順が直接書かれていた。固定した hash を更新する仕組みが無かった。

Position: 利用者が第1ラウンドで、進め方（スクリプト化も含めて1本の PR）、MSRV（固定する版と同じ）、unwrap（禁じてテストのファイルは理由付きで外す）を決めた（2026-10-06）。

## Agreements

第1ラウンド（2026-10-06）

- A1 rust-toolchain.toml でツールチェーンを 1.99.0 に固定し、rustfmt と clippy を入れる。CI は "stable" でなくこのファイルの版を入れる。
  - why: 2026-10-06 に rustup check で確かめた最新の安定版が 1.99.0。固定しないと、CI と手元で違う版の lint が出る
  - decided_by: Claude（利用者の「最新ルールに従っているか検証して改善」の指示のもとで ba0918-rust に従った）
- A2 edition（2024）と MSRV を root の Cargo.toml の [workspace.package] で1回だけ宣言し、すべてのパッケージが継承する。MSRV は固定する版と同じ 1.99 とする。
  - why: ルールは1回の宣言と継承を求める。MSRV を固定の版と同じにすれば、MSRV で確かめる別のジョブが要らない
  - rejected: すべて 1.89 にそろえる（1.89 でビルドする CI のジョブが要り、宣言の無かったクレートが 1.89 で通るかの確認も要る）。今のまま残す（ルールから外れる）
  - decided_by: 利用者
- A3 lint は [workspace.lints] に1つずつ選んで書き、すべてのパッケージが継承する。rust の unsafe_code（forbid）と unsafe_op_in_unsafe_fn（deny）、clippy の unwrap_used、allow_attributes、allow_attributes_without_reason、undocumented_unsafe_blocks、todo、unimplemented、dbg_macro、needless_pass_by_value、wildcard_enum_match_arm（warn。関門で誤りにする）。ライブラリのクレートの根には print_stdout と print_stderr の deny を置く。抑えるときは expect と理由を書く。
  - why: 守ることを lint に任せられるものは lint で守る。wildcard_enum_match_arm は数えると自分の enum の分が多かった（59件のうち約35件）ので入れ、外の enum の分だけ理由を付けて抑える
  - decided_by: Claude（ba0918-rust の lints の参照に従った）
- A4 unwrap_used は禁じ、テストの関数の中は clippy.toml の allow-unwrap-in-tests で許す。統合テストと例のファイルは、#[test] の外の補助関数の分を、ファイルの先頭の #![expect(clippy::unwrap_used, reason = ...)] で外す。製品のコードの unwrap は、起きうる失敗なら ? で返し、起きない理由があれば expect にその理由を書く。
  - why: 製品のコードは12か所だけだが、テストの補助関数に378か所あった。補助関数の準備の失敗は panic で知らせてよい
  - rejected: 製品の12か所だけ直して lint は入れない（新しい unwrap を機械で止められない）
  - decided_by: 利用者
- A5 scripts/gates.sh を Rust の関門の入口にし（整形の検査、warnings を誤りにした clippy を既定の feature と全 feature で、テスト全件。どれも --locked）、新しい CI のワークフロー ci.yml の "Rust gates" のジョブが PR と main で回す。リリースのチェックも同じスクリプトを回す。change-conformance.yml はテストを回さなくなる。main のブランチ保護で "Rust gates" を必須にするのは、最初に通った後に利用者が決める。
  - why: ルールは整形、lint、テストを CI の必須のチェックにすることを求める。同じテストを2つのワークフローで回すのは無駄で、手元と CI で同じ入口を使える
  - decided_by: Claude（ba0918-rust と ba0918-ci に従った。必須にする操作は外に見えるので利用者に残す）
- A6 すべてのワークフローで、外のアクションを commit hash で固定し、actions/checkout で資格情報を残さず、すべてのジョブに timeout-minutes を、ワークフローに defaults.run.shell: bash（pipefail）を置き、ランナーを ubuntu-24.04 にし、ジョブとステップに名前を付け、広げた権限に理由のコメントを書く。PR のチェックは新しい実行で古い実行を止め、リリースは "release" の群で1つずつ途中で止めずに回す。必須のチェックの "mutants" のジョブの名前は変えない。
  - why: ba0918-ci の固定、最小権限、上限のある実行、構造の決まり。macos-15 は版の付いた名前なのでそのまま
  - decided_by: Claude（ba0918-ci に従った）
- A7 ワークフローに直接書いていた分岐する手順をスクリプトに移す。タグから製品を引く scripts/release-meta.sh、変更履歴から版の節を切り出す scripts/release-notes.sh、配るファイルを固める scripts/package-release.sh、変異テストの分担を回す scripts/mutants-shard.sh。
  - why: スクリプトは手元で動かして確かめられ、プラットフォームに縛られない
  - decided_by: 利用者（同じ PR に入れることを選んだ）
- A8 .github/dependabot.yml で github-actions の固定を週に1回確かめ、公開から7日たった版だけを提案させる。
  - why: 固定した hash を新しく保つ仕組みが無かった。公開直後の版は、乗っ取られていても数日のうちに気づかれて取り下げられることが多い
  - decided_by: Claude（ba0918-ci に従った）
- A9 zizmor の self-repository の help（"./" の代わりに "$/" で同じリポジトリのワークフローを呼ぶ）は直さずに残す。
  - why: ba0918-ci の決まりに無く、低い重さの提案で、呼び出しは同じ commit のワークフローを指していて固定の問題が無い
  - decided_by: Claude
