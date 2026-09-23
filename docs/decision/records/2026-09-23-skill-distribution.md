# 壁打ちの記録: kotowari の skill を他の人へ配る

## Context

kotowari の skill 9つはリポジトリの `skills/` に置き、手元へはシンボリックリンクで入れている（[A7（workflow-split）](./2026-09-23-workflow-split.md#A7)）。他の人へ配る仕組みは決めていなかった。

skill の入れ方には APM（`apm install`）、GitHub CLI（`gh skill install`）、`npx skills add` がある。2026-09-23 に作業用のディレクトリで試すと、今の置き方でも3つとも入った。ただし APM はリポジトリの根を指定するしかなく、リポジトリ全体（2.9MB）を落としてくる。`skills/` を指定すると、その中に `skills/<名前>/SKILL.md` の形が無いので入らない。置き方を1段深くすれば APM も skill の分だけを落とせるので、置き場と入れ方を決める。

Position: A1〜A3 を決めた。記録の本文は利用者の確認待ち（2026-09-23）。

## Agreements

- A1 skill 9つの置き場を `agent/skills/<名前>/` にする。`skills/README.md` も `agent/skills/README.md` に移す
  - why: APM は指定したディレクトリの直下に `skills/<名前>/SKILL.md` があれば、apm.yml が無くても skill の束として受け付け、指定したディレクトリの分だけを落とす（`skills/kotowari` を指定したとき 124KB、根を指定したとき 2.9MB。受け付けの条件は APM 0.31.0 の `apm_cli/models/validation.py` から読んだ）。gh skill と npx skills は、根を指定しても1段深い `agent/skills/` を探して9つとも入れた（手元の複製で実測）
  - rejected: 今の `skills/` のまま根を指定させる。3つとも入るが、APM ではリポジトリ全体を落とす
  - decided_by: 利用者

- A2 他の人への入れ方として、`apm install ba0918/kotowari/agent`、`gh skill install ba0918/kotowari`、`npx skills add ba0918/kotowari` の3つを `agent/skills/README.md` に書く。手元へのシンボリックリンクの入れ方（[A7（workflow-split）](./2026-09-23-workflow-split.md#A7)）はそのまま残し、リンク先を `agent/skills/` にする
  - why: 利用者によって使う道具が違う。どれも1行で9つを入れられる
  - decided_by: 利用者（推奨を採用）

- A3 レビューの観点を変えたパスから選ぶとき、`skills/` という名前のディレクトリを含むパスを Skill の観点にする（深さは問わない）
  - why: 置き場が `agent/skills/` になると、根の `skills/` だけを見る判定では skill の変更が Code の観点に回る。工程の skill は他のプロジェクトでも使うので、特定の置き場の名前を書かない
  - decided_by: 主セッション（利用者が A1 を決めたことに伴う直し）
