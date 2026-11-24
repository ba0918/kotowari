# 人を待たずにマージする工程の skill を IR と読み合わせる

## Context

[仕様の承認からマージまでを人なしで通す決定](./2026-11-24-unattended-merge.md)を skill の本文に書いたあと、その差分を整合のフェーズとして IR と読み合わせた。
skill の本文は製品の振る舞いなので、コードとして読む。
IR に書かれていない振る舞いと、IR の中で食い違っている文が見つかったので、根拠と実測で決められるものをここで決める。
決められなかった意味の問いは、既定を置いて進め、問題の記録に残す。

Position: 整合のフェーズの1回の実行で決めた（2026-11-24）。

## Agreements

- A1 見届けは、プルリクエストで止まる回では必須の検査が通ったときにも終わる。マージ先に必須の検査が1つも無いときは、見届けるものが無いので、すぐに終わる。[REQ-core-366](../../ir/core/unattended-merge.md#REQ-core-366) にこの2つを書き足す
  - why: REQ-core-366 と [A43](./2026-11-24-unattended-merge.md#A43) は見届けの終わりを3つ挙げるが、3つとも、自動のマージを頼んだ回か検査が落ちた回のものだった。プルリクエストで止まる回も検査を見届ける（[A44](./2026-11-24-unattended-merge.md#A44)）ので、検査が通ったあとの終わりが無いと、進む手段の無い待ちになる。cycle は見届けに時間切れを持たず、人があとでマージした回は片付けない（[REQ-core-369](../../ir/core/unattended-merge.md#REQ-core-369)）ので、人のマージを待ち続ける読みは取れない。必須の検査が無いマージ先は、自動のマージを使えずプルリクエストで止まる場合（[A35](./2026-11-24-unattended-merge.md#A35)）として想定されていて、そこでは検査の結果が来ない
  - decided_by: 整合のフェーズ（根拠: agent/skills/kotowari-cycle/references/merge.md の Watching の終わりの一覧に4つ目があり、IR に無かった。A43、A44、A35 と矛盾しない具体化）

- A2 [REQ-core-213](../../ir/core/deferred.md#REQ-core-213) の文から「計画」を外す。skill が書いているのは、実装のループ、レビュー、壁打ちでの後回しの扱いとする
  - why: IR どうしが食い違う。[REQ-core-364](../../ir/core/unattended-merge.md#REQ-core-364) は計画を書く段が無いとし、REQ-core-213 の how_to_verify は [A38](./2026-11-24-unattended-merge.md#A38) で計画の skill を挙げる部分を cycle に書き換え済みなのに、文だけが計画での扱いを skill が書いていると言い続けていた。A38 は計画の段の名前を挙げている文を意味を変えずに書き換えると決めていて、この文はその書き換えの取りこぼしにあたる。要求の意味（後回しの ID を実装に渡す前に外す）は how_to_verify のとおりで変わらない
  - decided_by: 整合のフェーズ（根拠: docs/ir/core/deferred.md の REQ-core-213 の文と how_to_verify、REQ-core-364 の how_to_verify、A37 と A38。agent/skills/ の下に計画を書く skill が無いことは `ls agent/skills` で確かめた）

- A3 security の指摘で止まって人に渡したとき、人が「いまは直さず、開いたまま残す」と答えたら、cycle はその指摘のアクションを human_judgment にする。IR は変えず、skill の本文を IR に合わせる
  - why: [REQ-core-367](../../ir/core/unattended-merge.md#REQ-core-367) と [A32](./2026-11-24-unattended-merge.md#A32) は、人の判断を待つ指摘の出どころに「人が残すと決めたもの」を挙げ、それが開いていればマージを頼まないとする。工程の中で人が指摘を開いたまま残すと決める場面は、security の指摘で止まったときの答えだけだった。ところが skill は、その指摘のアクションを変えず、マージを頼まない条件を「human_judgment で開いている指摘」と書いていたので、人が残すと決めた security の指摘が開いたまま、人なしでマージを頼む読みになっていた。アクションが fix_and_verify のままなら見える指摘として残り、収束もしない
  - decided_by: 整合のフェーズ（根拠: agent/skills/kotowari-cycle/SKILL.md の Stopping inside the loop の1項目めと Loop の見える指摘の定義、agent/skills/kotowari-cycle/references/merge.md の When a person confirms の2項目め、REQ-core-367 の how_to_verify）
