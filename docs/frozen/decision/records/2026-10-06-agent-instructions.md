# エージェントの指示のルーティング表

## Context

AGENTS.md のルーティング表は手で書いた行のまま、入れてあるルールのスキルが増えても追従していなかった（ci、document、mutation、rust、worktree の行が無かった）。kotowari と kotowari-using-workflow の行は、このリポジトリ固有の内容が AGENTS.md に入っていた。

Position: 利用者が ba0918-scaffold の案を kemi で確かめ、gui の行を外す指摘をした（2026-10-06）。

## Agreements

- A1 AGENTS.md のルーティング表は、入れてある ba0918-* のスキルの ba0918-routing から作り直す。ただし gui（ba0918-gui-structure）の行は置かない。
  - why: 手で書いた表は気づかないうちに古くなる。このリポジトリには GUI が無い
  - decided_by: 利用者（kemi のレビューで gui の行を外す指摘）
- A2 kotowari のスキルを読む場面と、新しい依頼の入口を kotowari-using-workflow で決めることは、AGENTS.md の表でなく PROJECT.md の Specification の節に書く。
  - why: AGENTS.md はスキルの設定から作り直す薄い入口で、このリポジトリ固有の内容は PROJECT.md に置く
  - decided_by: 利用者（kemi のレビューで指摘なし）
