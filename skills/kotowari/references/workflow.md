kotowari の仕様に基づく（改訂 2026-09-16。本体の版は固定しない）

## brainstorm

既存の brainstorm スキルの手順のうち、次を置き換える。

出力: 既存の brainstorm が作る `docs/spec/<name>.md` ではなく、IR の置き場の話題ごとの文書（新規か既存への追記）と、用語集 `CONTEXT.md`、問題の記録があるときは `FLAGS.md` を作る。

判断の記録: 進捗ファイルを `.agents/tmp/` に置かず、最初から `docs/decision/brainstorm/YYYY-MM-DD-<name>.md` に書く。ファイル名は最初に決めて改名しない（出典のパスが壊れるため）。決定の節の見出し（`## Agreements`、`## Prohibitions`、`## Delegated`、`## Rejected`）を1つ以上置き、その下に `- A1 本文` の形で並べる。番号は使い回さない。意味が変わったら `Revisions` に改訂を書く。承認後も消さない。`Undecided` と `Revisions` の番号は出典に使えない（決定の節ではない）。

再開: `.agents/tmp/brainstorm-*.md` ではなく、`docs/decision/brainstorm/` の未コミットの記録から再開する。

成功の条件と反例: 各要求に求める観測できる成功の条件と反例は、IR の `## 具体例` のシナリオとして書く。成功の条件は通る場面、反例は指摘か停止が出る場面で書く。要求の見出しの下には持てる行しか置けない。

禁止・却下・未決・委譲: IR の話題ごとの文書には要求・決定表・性質・具体例しか置けないので、これらは判断の記録の節に置く。矛盾・欠落・曖昧は `FLAGS.md` の `### FLAG-nnn: 名前` に `- 種類:`、`- 関係:`、`- 出典:` を付けて書く。

用語集: brainstorm の「リポジトリの CONTEXT.md」は IR の置き場の `CONTEXT.md` のこと。用語集はどのディレクトリにも置け、文書から見えるのは自分のディレクトリから置き場の根までの `CONTEXT.md` の用語（連鎖）。用語は連鎖の中で1つのファイルにだけ置く。

終わりのレビュー: 「記録への適合」のレビューは下の照合レビューが置き換える。「仕様の品質」のレビュー1本は IR に対して残す。

承認時に stage するもの: IR の文書、用語集、問題の記録、判断の記録。

承認の手順:

1. `kotowari check --format json` を走らせる。終了コード2なら停止として findings.md のとおり扱う。`findings` のうち `path` が IR の置き場のファイルである誤りが0になるまで直す。テスト側の指摘（requirement_without_test と、テストのファイルへの指摘）は承認の時点では残ってよく、cycle の終端で0にする。注意（too_many_lines、too_many_requirements）は残してよい。残すときは ir-form.md の「上限と分ける単位」のとおりに読み直し、残す理由を判断の記録に書く。

2. 照合レビュー: collate.md の指示で別セッションの LLM に IR の各項目とその出典を渡し、出典の決定が項目の内容を裏付けていないものを挙げさせる。返る JSON の `unsupported` の中に挙がったものは、出典を足すか、記録に決定を足すか、項目を直す。IR か記録を変えたら手順1（check）に戻ってから再び照合する。3回目の照合でも残るものは問題の記録（FLAG）にして人に返す。

3. 人に見せるもの: 判断の記録の差分（1行1決定で人が読む対象）、check の出力（テスト側の指摘の件数と、注意を残した文書とその理由）、照合レビューの結果、承認の対象のパスと内容の識別子（IR の文書、用語集、問題の記録、判断の記録）。IR の差分を読むことは求めない。

## plan

既存の plan スキルの手順のうち、次を置き換える。

入力: IR の置き場のパスと、この計画が対象にする要求の ID の一覧。

承認済みの判定: IR の文書、用語集、問題の記録、判断の記録がすべてコミット済みで、`kotowari check` の誤りがテスト側の指摘（requirement_without_test と、テストのファイルへの指摘）だけ。

要求の参照: 計画の中で要求を `文書のパス#REQ-nnn` の形で参照する。参照先の実在は、その文書に `### REQ-nnn:` で始まる見出しがあることで確かめる。

確認コマンド: 最後のステップの確認コマンドに `kotowari check`（終了コード0）を列挙する。

plan 自身は `kotowari check` を走らせない。

## cycle

既存の cycle スキルの手順のうち、次を置き換える。

review に渡す仕様のパス: IR の置き場のパス（レビュー役は置き場の文書すべてを読む）。

implementer と fixer のプロンプト: mark.md の内容を貼る（委譲先はスキルを読まない）。

終端報告の直前: `kotowari check` を走らせ、出力を終端報告に載せる。

テスト側の指摘（requirement_without_test、test_without_id、invalid_marker、unparsable_file、印からの unresolved_reference）: fixer への指摘として扱う。直らなければ既存の「進捗なし」の終わり方にする。

IR 側の指摘: cycle では直さない。人の判断として終端報告に載せ、brainstorm に戻す。

## implement

既存の implement スキルの手順のうち、次を置き換える。

plan が列挙した `kotowari check` を確認コマンドとして走らせる。

終了コード1のとき:

- テスト側の指摘（requirement_without_test、test_without_id、invalid_marker、unparsable_file、印からの unresolved_reference）は自分で直す
- IR 側の指摘は仕様の問題として差し戻す
