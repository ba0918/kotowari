# 壁打ちの記録: mds の置き場を ".kotowari/" に寄せる

## Context

mds の CLI は "kotowari-mds" に名前を変えていて（[A4（versions-and-cli-name）](./2026-09-23-versions-and-cli-name.md#A4)）、kotowari から派生した道具の1つと見なせる。それでも mds は ".mds/" で基準のディレクトリを決め、スキーマとキャッシュの置き場も ".mds/" にしていて（[A20（mds-spec）](./2026-09-21-mds-spec.md#A20)）、kotowari の ".kotowari/" と並んで隠しディレクトリが2つになっていた。kotowari 自身が IR を読むスキーマ3つも ".mds/schemas/" に置いていた。利用者の提案で ".kotowari/" に統合する。

Position: A1〜A5 を決めた。

## Agreements

- A1 mds の基準のディレクトリは、カレントディレクトリから上に向かって探し、最初に見つかった ".kotowari/" のあるディレクトリとする。見つからなければカレントディレクトリ。".mds/" は見ず、互換のために探すこともしない
  - why: kotowari と mds で基準のディレクトリの目印がそろい、乱立しがちな隠しディレクトリが1つ減る（利用者の言葉）。mds が kotowari に依存しないという方針はコードの依存の向きの話で、置き場の名前には及ばない。まだ使っている人がほぼいない段階なので、互換は持たない
  - rejected: ".mds/" も探す互換を持つ。探し方が2通りになり、両方あるときの優先を決める必要が出る
  - decided_by: 利用者
- A2 スキーマの推奨の置き場を ".kotowari/schemas/" とし、kotowari 自身が IR を読むスキーマ3つ（ir.yaml、context.yaml、flags.yaml）もそこに置く
  - why: kotowari の持ち物（設定、等価の一覧、スキーマ）が ".kotowari/" にそろう
  - decided_by: 利用者（推奨を採用）
- A3 URL のスキーマのキャッシュは、基準のディレクトリの ".kotowari/cache/schemas/" に置く。".kotowari/cache/" は消しても作り直せるものの置き場で、その直下にはファイルを置かず、キャッシュは用途ごとのサブディレクトリに分ける。README に .gitignore の1行（"/.kotowari/cache/"）を書く
  - why: ".kotowari/cache/" だけだと何のキャッシュか分からない（利用者の言葉）。用途ごとのサブディレクトリにすれば、後でキャッシュが増えても責務が混ざらず、1つだけ消すこともでき、.gitignore は1行のまま
  - rejected: ".kotowari/schema-cache/" のように用途ごとに直下に置く。キャッシュが増えるたびに直下と .gitignore の行が増える
  - decided_by: 利用者
- A4 mds のディレクトリの検査で ".mds/" を辿らない決まりは、".kotowari/" に置き換わる。名前が "." で始まるディレクトリはもともと辿らないので、振る舞いは変わらない
  - why: 置き場の名前が変わるだけで、[REQ-schema-044](../../ir/schema/cli.md#REQ-schema-044) の「スキーマとキャッシュの置き場を辿らない」はそのまま効く
  - decided_by: 利用者（推奨を採用）
- A5 記録と IR（mds の用語集、キャッシュの要求と具体例、kotowari の how_to_verify）を直して参照を張り替え、mds の README とテストの fixture も直し、このリポジトリの ".mds/" は無くす。過去の判断の記録の ".mds/" は履歴なので書き換えない
  - why: 実質の作業は参照の張り替えで、影響は小さい（利用者の言葉）
  - decided_by: 利用者（推奨を採用）

## Revisions

- A1 は [A9（mutants-gaps）](./2026-09-23-mutants-gaps.md#A9)、[A12（mutants-gaps）](./2026-09-23-mutants-gaps.md#A12) の ".mds/" を ".kotowari/" に改める
- A2 と A3 は [A20（mds-spec）](./2026-09-21-mds-spec.md#A20) の ".mds/schemas/" と ".mds/cache/" を改める
