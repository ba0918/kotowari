# Plan: kotowari のリリースの流れを作る

## Goal

人が `scripts/release.sh kotowari 0.1.0` を走らせるとチェックを通った版上げのコミットとタグが手元にでき、そのタグを push すると GitHub Actions が Linux x86_64 と macOS arm64 のバイナリを付けた GitHub Release を作る。

## Specification

IR は無い。仕様はコミット済みの判断の記録 `docs/decision/records/2026-09-26-release-flow.md`（A1〜A15）で、この計画はそのすべてを扱う。版とタグの形はその前の `docs/decision/records/2026-09-23-versions-and-cli-name.md` の A1、A2（kotowari と mds は別々の版、タグは `kotowari-v0.1.0` と `kotowari-mds-v0.2.0` の形、kotowari-core は kotowari の版に合わせる）。

リリースの規則は次のとおりで、実装の判断の基準になる。版の置き場は製品ごとに1か所で PROJECT.md に書く。ほかの宣言はそれに従い、ずれは機械で検査する。タグ、版の見出し、比較のリンクは1つの操作で出す。タグはチェックを通った状態にだけ打つ。push したタグは動かさない。手元だけのタグは消してよい。

## Approach and why

- 版の置き場は、kotowari が根の `Cargo.toml` の `[package]` の `version`、mds が `crates/kotowari-markdown-schema/Cargo.toml` の `version` である。`crates/kotowari-core/Cargo.toml` の版は kotowari に従う宣言で、`Cargo.lock` の workspace のパッケージの版も従う側になる。
- 版のずれの検査は `scripts/check-versions.sh` にする（A13）。Rust のテストにしないのは、`tests.files` が `tests/**/*.rs` を含み、IR の ID を指す印の無いテストが test_without_id になるため。
- `scripts/release.sh <製品> <版>` が受け付ける製品は `kotowari` と `kotowari-mds` である。製品から、版の置き場、従う宣言、変更履歴の場所、タグの接頭辞を引く。mds の変更履歴はまだ無い（A14）ので、`kotowari-mds` を指定すると「変更履歴が無い」と言って止まる。
- スクリプトは、ファイルを書き換えてからチェックを回す。書き換えは、版、`Cargo.lock`、変更履歴の Unreleased の節を版の見出しにすること、比較のリンクである。チェックが通ったら1つのコミットにし、注釈付きのタグを作る。落ちたら書き換えたファイルを元に戻して止まる。コミットの前にチェックを回すのは、落ちたときに、タグの無い版上げのコミットを履歴に残さないためである。タグが指すコミットの中身は、チェックを通った作業ツリーと同じになる。
- スクリプトは push しない（A4）。最後に、人が打つ push のコマンドを表示して終わる。タグの push で pre-push のフックが変異テストを全体で回す（A9）。落ちたら push は拒まれるので、手元のタグを消して直し、打ち直すことを表示に含める。
- ワークフローは `.github/workflows/release.yml` に自分で書く（A7）。起動は `kotowari-v*` と `kotowari-mds-v*` のタグの push である。タグから製品、バイナリの名前、パッケージ、変更履歴の場所を引く。これは A2 の「仕組みは mds にもそのまま使える形にする」に当たる。mds のタグはまだ打たれないので、mds の側は変更履歴が無ければ止まってよい。
- 配るファイルは `<製品>-v<版>-<ターゲット>.tar.gz` と、それぞれの `.sha256` である（A8）。中身はバイナリ、`README.md`、`LICENSE-MIT`、`LICENSE-APACHE` とする。A8 の「LICENSE」は、このリポジトリでは2つのファイルに分かれている。
- 0.1.0 の変更履歴の節は草稿を書くまでにとどめ、利用者が確かめる（A15）。このブランチではスクリプトを本物のリポジトリで走らせない。版上げとタグは、マージの後に人が見てから行う。

## Scope of change

- `scripts/check-versions.sh`（新設）
- `scripts/release.sh`（新設）
- `.github/workflows/release.yml`（新設）
- `CHANGELOG.md`（新設）
- `PROJECT.md`
  - 新しいリリースの節だけ
- `README.md`
  - 「入れ方」の節だけ
- `agent/skills/README.md`
  - 入れ方の節だけ

## Step order and prerequisites

S1 の検査を S2 と S3 が呼ぶので、S1 が先になる。S2 と S3 は互いに依らない。S4 の文書は、S2 のスクリプトが読み書きする変更履歴の形に合わせるので、S2 の後にする。S5 で全体を確かめる。

## Verification map

| Step | Requirements | Examples |
|---|---|---|
| S1 | A5、A13 | なし |
| S2 | A4、A6、A9 | なし |
| S3 | A3、A7、A8、A2 | なし |
| S4 | A1、A6、A11、A12、A15 | なし |
| S5 | この表のすべて | なし |

## Left to the implementer

- スクリプトの内部の書き方（関数の分け方、Cargo.toml の版の読み方）。ただし外の道具を新しく要求しない（bash、git、sed、awk、grep と cargo で書く）
- ワークフローで使う GitHub Actions の既製のアクションの選び方（ツールチェーンの用意、成果物の受け渡し）
- 変更履歴の 0.1.0 の節の文面（中身は A15 のとおり）

## Stop conditions

- タグの形や版の置き場が、判断の記録と食い違って見える
- `gh skill install`、`apm install`、`npx skills add` のどれも、タグで版を固定する方法を持たない。または help で確かめられない
- ワークフローの検査に、GitHub 上で走らせないと分からない前提が要る（actionlint で分からない秘密の値や権限など）

## Test command

```sh
shellcheck scripts/check-versions.sh scripts/release.sh
actionlint .github/workflows/release.yml
CARGO_BUILD_JOBS=4 cargo test --workspace
CARGO_BUILD_JOBS=4 cargo run -q -p kotowari -- check --format text
```

## Out of scope

- 本物の 0.1.0 の版上げ、タグ、push、Release（マージの後に人が確かめてから行う）
- README の mise の入れ方の行（Release ができてから試して、動いたものだけを書く。A12）
- mds の `CHANGELOG.md` と mds のリリース（A14）
- リリース以外の CI（A10）

## Steps

### S1: 版のずれの検査を書く

- Purpose: kotowari と kotowari-core の版のずれと、タグと製品の版のずれを機械で見つける
- Specification: `docs/decision/records/2026-09-26-release-flow.md#A5`, `docs/decision/records/2026-09-26-release-flow.md#A13`
- Prerequisites: none
- May change: `scripts/check-versions.sh`
- Done when: 引数なしでは kotowari と kotowari-core の Cargo.toml の版と `Cargo.lock` の kotowari と kotowari-core の版がすべて同じなら終了コード0、1つでも違えば違う箇所を出して1で終わる。引数にタグ（`kotowari-v0.1.0` か `kotowari-mds-v0.1.0`）を渡すと、加えてタグの版と製品の版が同じかを確かめ、形の違うタグは2で止まる
- Shown by: check — `shellcheck scripts/check-versions.sh` の後に、使い捨ての写し（`git worktree add` か `git clone` の一時ディレクトリ）で kotowari-core の版だけを書き換えて終了コード1、元のままで0、`kotowari-v9.9.9` を渡して1、`kotowari-v0.1.0` で0、`foo` で2になることを実行して示す（写しは後で消し、コミットに含めない）
- Left to the implementer: 版の読み方
- Stop and hand back if: `Cargo.lock` に workspace のパッケージの版が載っていない、または読み方が決められない

### S2: リリースのスクリプトを書く

- Purpose: 版の書き換え、変更履歴の昇格、比較のリンク、チェック、コミット、タグを1つの操作にする
- Specification: `docs/decision/records/2026-09-26-release-flow.md#A4`, `docs/decision/records/2026-09-26-release-flow.md#A6`, `docs/decision/records/2026-09-26-release-flow.md#A9`
- Prerequisites: S1
- May change: `scripts/release.sh`
- Done when: `scripts/release.sh kotowari <版>` が、次の場合は何も変えずに止まる：main 以外のブランチ、作業ツリーが汚れている、同じ名前のタグがある、変更履歴の Unreleased の節が空。止まらない場合は、根と kotowari-core の Cargo.toml と Cargo.lock の版を書き換え、Unreleased の節を `## [<版>] - <日付>` にして空の Unreleased を上に作り、比較のリンクを足す（前のタグがあれば前のタグとの compare、無ければ releases/tag のリンク。Unreleased のリンクは新しいタグからの compare）。そのうえでテスト全件、`kotowari check` の終了コード0、`scripts/check-versions.sh <タグ>` を回し、通れば1つのコミットと注釈付きのタグ `kotowari-v<版>` を作り、人が打つ push のコマンドと、pre-push で落ちたときの手順を表示する。落ちれば書き換えを戻して止まる。`kotowari-mds` は変更履歴が無いので止まる
- Shown by: check — `shellcheck scripts/release.sh` の後に、使い捨ての写し（origin を外した `git clone`）で、Unreleased に1行ある CHANGELOG.md を置いたコミットの上から `scripts/release.sh kotowari 0.1.0` を走らせる。`git show kotowari-v0.1.0` に版の見出し、比較のリンク、3か所の版が同じコミットで入っていること、`scripts/check-versions.sh kotowari-v0.1.0` が0になることを示す。わざとチェックを落とした写し（例：IR に誤りを1つ入れる）では、コミットもタグもできず、作業ツリーが元に戻ることを示す。写しは消し、コミットに含めない
- Left to the implementer: 日付の取り方と、比較のリンクの行の置き場（ファイルの末尾に Keep a Changelog の形で並べる）
- Stop and hand back if: チェックに時間がかかりすぎて、写しでの確認が現実的でない（テスト全件が10分を超えるなど）

### S3: リリースのワークフローを書く

- Purpose: タグの push を受けて、チェック、ビルド、GitHub Release の作成を行う
- Specification: `docs/decision/records/2026-09-26-release-flow.md#A2`, `docs/decision/records/2026-09-26-release-flow.md#A3`, `docs/decision/records/2026-09-26-release-flow.md#A7`, `docs/decision/records/2026-09-26-release-flow.md#A8`
- Prerequisites: S1
- May change: `.github/workflows/release.yml`
- Done when: `kotowari-v*` と `kotowari-mds-v*` のタグの push で起動する。まず ubuntu でテスト全件、`kotowari check`、`scripts/check-versions.sh <タグ>` を走らせる。通ったら ubuntu（x86_64-unknown-linux-gnu）と macos-14（aarch64-apple-darwin）で製品のバイナリを `--release` でビルドし、`<製品>-v<版>-<ターゲット>.tar.gz`（バイナリ、README.md、LICENSE-MIT、LICENSE-APACHE）とその `.sha256` を作る。最後に変更履歴からその版の節を切り出し、それを本文にして `gh release create` で Release を作り、ファイルを付ける。権限は contents: write だけにする
- Shown by: check — `actionlint .github/workflows/release.yml` が指摘0件。加えて、変更履歴からその版の節を切り出す処理を手元で同じコマンドで走らせ、S4 の CHANGELOG.md の 0.1.0 の節だけが出ることを示す
- Left to the implementer: 既製のアクションの選び方と版の固定のしかた
- Stop and hand back if: ワークフローが GitHub のリポジトリの設定（秘密の値、保護されたタグ）を新しく要る

### S4: 変更履歴と文書を書く

- Purpose: 0.1.0 の変更履歴の草稿、版の置き場とリリースの手順、入れ方を文書に書く
- Specification: `docs/decision/records/2026-09-26-release-flow.md#A1`, `docs/decision/records/2026-09-26-release-flow.md#A6`, `docs/decision/records/2026-09-26-release-flow.md#A11`, `docs/decision/records/2026-09-26-release-flow.md#A12`, `docs/decision/records/2026-09-26-release-flow.md#A15`
- Prerequisites: S2
- May change: `CHANGELOG.md`, `PROJECT.md`, `README.md`, `agent/skills/README.md`
- Done when: CHANGELOG.md が Keep a Changelog の形で、Unreleased の節に 0.1.0 で手に入るもの（check、list、query、status、plan、mutants のコマンド、工程のスキル、入れ方）を経緯なしで書いている。PROJECT.md のリリースの節が、版の置き場（製品ごと）、従う宣言とずれの検査、`scripts/release.sh` から push と Release までの手順、利用者に見える変更は同じブランチで Unreleased に書くことを書いている。README の入れ方が GitHub Release のバイナリとタグで固定した `cargo install --git` の2通りを書いている（mise は書かない）。`agent/skills/README.md` が、タグで版を固定してスキルを入れる方法を、help で確かめた書き方で書いている
- Shown by: artifact — 4つのファイル。`agent/skills/README.md` に書いた固定の書き方ごとに、確かめた help の出力の該当行を報告に挙げる
- Left to the implementer: 文面
- Stop and hand back if: 3つの入れ方の道具のどれもタグでの固定を持たない

### S5: 全体を確かめる

- Purpose: 検査とテストがこの変更で壊れていないことを確かめる
- Specification: `docs/decision/records/2026-09-26-release-flow.md#A9`
- Prerequisites: S1, S2, S3, S4
- May change: none
- Done when: Test command の4つがすべて通る
- Shown by: check — `shellcheck scripts/check-versions.sh scripts/release.sh`、`actionlint .github/workflows/release.yml`、`CARGO_BUILD_JOBS=4 cargo test --workspace`、`CARGO_BUILD_JOBS=4 cargo run -q -p kotowari -- check --format text` の順
- Left to the implementer: none
- Stop and hand back if: cargo test がこの変更と関係の無い理由で落ちる
