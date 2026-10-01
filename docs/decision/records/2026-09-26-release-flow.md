# 壁打ちの記録: kotowari の最初のリリースとリリースの流れ

## Context

kotowari はまだ一度もリリースしていない。このリポジトリにタグは無く、変更履歴のファイルも、CI のワークフローも無い。入れ方は README の `cargo install --git` だけである。
版の持ち方とタグの形は [A1（版と CLI の名前）](./2026-09-23-versions-and-cli-name.md#A1) で決めてある（kotowari と mds が別々の版を持ち、タグは `kotowari-v0.1.0` の形）。kotowari-core は kotowari の版に合わせる（[A2（版と CLI の名前）](./2026-09-23-versions-and-cli-name.md#A2)）。
kotowari を 0.1.0 として出すにあたって、何を配るか、変更履歴をどう持つか、どこまでを自動にするかを決める。

Position: 承認済み（2026-09-26）。3ラウンド（A1〜A15）で木が尽きた。実装のレビューで見つかった規則を A16 として足した。IR は変えない（A1）。実装は plan → cycle

## Agreements

- A1 リリースの流れは IR に書かず、この判断の記録と PROJECT.md のリリースの節に書く。版の置き場も PROJECT.md に書く
  - why: リリースの流れは kotowari の CLI の振る舞いではない。版の持ち方も記録だけで決めた（[A1（版と CLI の名前）](./2026-09-23-versions-and-cli-name.md#A1)）
  - decided_by: 利用者（推奨を採用）

- A2 最初のリリースは kotowari（kotowari-core を含む）と "agent/skills/" のスキルを "kotowari-v0.1.0" として出し、mds は今回は出さない。仕組みは mds にもそのまま使える形にする
  - why: スキルはこのリポジトリから入れるので、タグがスキルの版になる。mds は版が別で 0.1.0 は移す前のリポジトリでリリース済みなので、取り込み後の変更を出すかは別に決める
  - decided_by: 利用者（推奨を採用）

- A3 配るのは GitHub Release（リリースノート付き）とビルド済みのバイナリ（Linux x86_64、macOS arm64）で、crates.io には出さない
  - why: "cargo install --git" はビルドに時間がかかり、kotowari を取ってくる CI やバイナリで入れる道具のために速い入れ方が要る。crates.io に出すと kotowari-core の公開と公開する API の約束まで背負う。ほかの OS は使う人が出てから足す
  - decided_by: 利用者（推奨を採用）

- A4 リリースは2段にする。手元のスクリプト（例: "scripts/release.sh kotowari 0.1.0"）が版の書き換え、変更履歴の Unreleased の節を版の見出しにすること、比較のリンクの追加を1つのコミットにし、チェックが通ったらタグを作る。タグの push は人が行い、それを受けて GitHub Actions がチェックをもう一度走らせ、バイナリを作り、変更履歴のその版の節を本文にした GitHub Release を作る
  - why: 公開したタグは取り消せないので、最後の1歩は人が踏む。手で同じ数字を何か所も書き換えるとずれる
  - decided_by: 利用者（推奨を採用）

- A5 版のずれを機械で見張る。kotowari-core の版が kotowari の版と同じことを検査し、CI でタグの版とその製品の Cargo.toml の版が同じことを検査する
  - why: 版の宣言が複数あり、今はずれの検査が無い。リリースの規則は宣言のずれを機械で検査することを求める
  - decided_by: 利用者（推奨を採用）

- A6 変更履歴は製品ごとに、kotowari は根の "CHANGELOG.md"、mds は "crates/kotowari-markdown-schema/CHANGELOG.md" に Keep a Changelog の形で持ち、変更したときに Unreleased の節に書く。0.1.0 の節には最初のリリースで手に入るものを書く。コミットログから生成しない
  - why: コミットログは何をどう変えたかを、変更履歴は上げる人が何をするかを書く場所で、読み手が違う
  - decided_by: 利用者（推奨を採用）

- A7 GitHub Actions のワークフローは自分で書く。ubuntu と macos-14 の2つで "cargo build --release" し、tar.gz と SHA256 を作り、変更履歴のその版の節を切り出して "gh release create" で Release を作る
  - why: 配る先が2つだけで、自分で書いたほうが読めて直しやすい
  - rejected: cargo-dist。専用の設定ファイルと大きなワークフローが生成され、今は要らないインストーラーまで付く
  - decided_by: 利用者（推奨を採用）

- A8 配るファイルは "kotowari-v<版>-<Rust のターゲット名>.tar.gz"（"x86_64-unknown-linux-gnu"、"aarch64-apple-darwin"）で、中身は kotowari のバイナリ、README、LICENSE とし、それぞれに ".sha256" を付ける。"kotowari-mds" のバイナリは入れない
  - why: ターゲット名を Rust の標準の書き方にすると、名前から OS と CPU を見分ける道具（mise の github: など）がそのまま使える。mds は別の製品として出す（A2）
  - decided_by: 利用者（推奨を採用）

- A9 タグを作る前にスクリプトが回すのは、テスト全件、"kotowari check" の終了コード0、版のずれの検査の3つとする。変異テストの全件はタグの push のときに既存の pre-push のフックが回し、落ちたら push は拒まれ、手元だけのタグを消して直してから打ち直す
  - why: pre-push はタグがあると変異テストを全体で回すので、スクリプトで二度回さない。手元だけのタグは公開されていないので消してよい
  - decided_by: 利用者（推奨を採用）

- A10 リリース以外の CI（main への push や PR ごとのもの）は今回作らない
  - why: 頼まれたのはリリースの流れで、同じチェックは手元のフックが回している
  - decided_by: 利用者（推奨を採用）

- A11 PROJECT.md のリリースの節に、利用者に見える変更は同じブランチで CHANGELOG の Unreleased に書くと書き、機械の検査は付けない。工程のスキルには書かない
  - why: 利用者に見えるかは機械で判定できず、検査は誤検知ばかりになる。工程のスキルはほかのプロジェクトにも配るので kotowari 自身の決まりを書かない
  - decided_by: 利用者（推奨を採用）

- A12 README の入れ方に、GitHub Release のバイナリ、mise（github: の書き方）、タグで版を固定した "cargo install --git" の3通りを並べ、"agent/skills/README.md" にタグで版を固定してスキルを入れる方法を足す。mise の書き方は Release ができてから試して動いたものだけを書く
  - why: 速い入れ方と版の固定を利用者に示す。試していない書き方は載せない
  - decided_by: 利用者（推奨を採用）

- A13 版のずれの検査はシェルのスクリプト "scripts/check-versions.sh" にし、リリースのスクリプトと CI の両方から呼ぶ。kotowari と kotowari-core の Cargo.toml の版が同じことと、タグを渡されたときはタグの版と製品の版が同じことを確かめる
  - why: "tests.files" は "tests/**/*.rs" を含み、Rust のテストにすると IR の ID を指す印が要るが、リリースの流れは IR に書かない（A1）ので印の指す先が無く test_without_id になる
  - decided_by: 利用者（推奨を採用）

- A14 mds の "CHANGELOG.md" は mds を次にリリースするときに作る。A6 は置き場を決めたものとする
  - why: 今作ると移す前の 0.1.0 と取り込み後の変更を整理する作業になり、それは mds のリリースを決めるときの仕事
  - decided_by: 利用者（推奨を採用）

- A15 0.1.0 の変更履歴の節は実装の中で草稿を書き、リリースのスクリプトを走らせる前に利用者が確かめる。中身は最初のリリースで手に入るもの（コマンド check、list、query、status、plan、mutants、工程のスキル、入れ方）で、作ってきた経緯は書かない
  - why: 変更履歴の読み手は最初に入れる人で、経緯はコミットログにある
  - decided_by: 利用者（推奨を採用）

- A16 リリースのスクリプトは main ブランチの上でだけ走る
  - why: タグは共有の履歴の上に打つ。main 以外で版上げのコミットとタグを作ると、タグが main に入らないコミットを指しうる
  - decided_by: 利用者（推奨を採用。実装のレビューで仕様に無い規則として見つかった）
