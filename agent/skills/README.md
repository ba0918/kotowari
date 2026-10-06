# kotowari の skill

Claude Code の skill を10個置いている。

| skill | 役目 |
|---|---|
| `kotowari` | IR と判断の記録の書き方、`kotowari check` の結果の読み方、テストへの印の置き方を、場面ごとに reference で読ませる |
| `kotowari-using-workflow` | 新しい依頼をどの工程から始めるかを決める入口 |
| `kotowari-brainstorm` | 壁打ちで決めたことを IR と判断の記録に書き、承認を得る |
| `kotowari-plan` | 承認済みの IR の要求から実装の計画を書く |
| `kotowari-cycle` | 計画を実装・レビュー・修正の繰り返しで回し、終端で `kotowari check` と `kotowari status` を見る |
| `kotowari-implement` | 計画の手順を実行する（cycle から呼ばれる） |
| `kotowari-review` | 差分や文書を別の文脈でレビューする（cycle から呼ばれるか、人が直接呼ぶ） |
| `kotowari-iterate` | 仕様も計画も要らない小さな作業を cycle の繰り返しで回す |
| `kotowari-investigate` | 読むだけの調査。原因と影響を調べて直し方を報告する |
| `kotowari-adopt` | 既存のプロジェクトに話題1つずつ kotowari を入れる。旧資料と実装とテストを仕分けて IR と判断の記録と FLAG に書き、承認を得て、テストの作業の依頼文を渡す |

`kotowari-` で始まる9個を工程の skill と呼ぶ。

## 関係

工程の skill は `kotowari` スキルに依存する。工程の途中で IR を書く、check の結果を読む、印を置くといった場面では、`kotowari` スキルの reference を名指しして読む。

逆向きの依存は無い。`kotowari` スキルは工程の skill を名指ししないので、`kotowari` スキルだけを入れて別の工程（ほかの workflow の skill や手作業）の中で使ってもよい。工程の skill は推奨であって、無くても kotowari は使える。

工程の skill は agentic-workflow の ba0918 の工程の skill を元に写して独立させたもので、ba0918 の skill を読むことも、それに追従することもしない。

## 入れ方

次のどれか1行で、10個をまとめて入れられる。どれも GitHub の ba0918/kotowari から取る。

```sh
apm install ba0918/kotowari/agent --target claude
gh skill install ba0918/kotowari --agent claude-code --all
npx skills add ba0918/kotowari --agent claude-code --skill '*'
```

APM は `agent/` を指定すると、リポジトリ全体ではなく skill の分だけを落とす。入れる先や入れる skill を選ぶ方法は、それぞれの道具の help を読む。

### 版を固定して入れる

skill は kotowari 本体と同じタグ（`kotowari-v0.1.0` の形）で出している。版を固定して入れるときは、`gh skill install` の `--pin` にそのタグを渡す。

```sh
gh skill install ba0918/kotowari --agent claude-code --all --pin kotowari-v0.1.0
```

`kotowari-v0.1.0` は入れたい版のタグに置き換える。入れた kotowari 本体と同じ版にそろえると、skill が教える書き方と `kotowari check` の検査が食い違わない。

## リポジトリを手元に置いている場合の入れ方

`~/.claude/skills/` に、このリポジトリの `agent/skills/` の各ディレクトリへのシンボリックリンクを置く。リンクにしておくと、リポジトリを更新すれば手元の skill も常に最新になる。

```sh
for d in /path/to/kotowari/agent/skills/kotowari*/; do
  ln -s "${d%/}" ~/.claude/skills/
done
```

`/path/to/kotowari` はこのリポジトリを置いた場所に置き換える。同じ名前の実ディレクトリ（以前にコピーで入れたもの）が既にあると `ln` は失敗するので、中身がリポジトリと同じであることを確かめてから消す。

工程の skill を使うには `kotowari` スキルも入れる。`kotowari` スキルだけを使うなら、そのリンクだけでよい。

## 工程の skill を使うプロジェクトの AGENTS.md に足す1行

工程の skill を使う kotowari のプロジェクトでは、`AGENTS.md` に次の1行を足す。

> 新しい依頼をどこから始めるかは kotowari-using-workflow で決める。ba0918-using-workflow は使わない。

`AGENTS.md` にルーティングの表（ba0918-scaffold が作る Rule Routing の表など）があるときは、同じ趣旨の行をその表に足す。例:

```markdown
| deciding where a new request starts (use this, not ba0918-using-workflow) | kotowari-using-workflow |
```

この1行が無いと、入口が ba0918-using-workflow のままになり、ba0918 の工程の skill（出力が IR でないもの）に進む道が残る。`kotowari` スキルの setup が `AGENTS.md` に書くのは「仕様は IR で管理する」の一文だけで、この1行は書かない（`kotowari` スキルは工程の skill を名指ししないため）。

## レビューの任意の席の一覧

`kotowari-review` の full review では、quality の観点のレビュー役に加えて、別のモデルのレビュー役（任意の席）を同じプロンプトで立てられる。どの席を立てるかは、利用者スコープで毎回読まれる指示ファイル（Claude Code なら `~/.claude/CLAUDE.md`）に一覧として書く。使えるモデルは利用者の契約で決まるので、リポジトリには置かない。一覧を書かなければ任意の席は無く、いつものレビュー役だけで回る。席の数は quality のレビュー役と任意の席を数え、conformance のレビュー役は数えない。

1席ぶんに書くもの:

- 名前: 報告で出席・欠席を示すときの呼び名
- 起動の手段: 実行するコマンド、または呼び出す skill。レビュー役のプロンプトを受け取り、finding の JSON を返すもの
- 時間の上限（任意）: 書いたときだけ効く。書かなければ終わるまで待つ

例（`~/.claude/CLAUDE.md` に書く）:

```markdown
## kotowari-review の任意の席

- gpt: コマンド `codex exec -`（プロンプトは標準入力で渡す）。時間の上限 15分
```

任意の席は、起動の直前に作る使い捨ての複製（HEAD に、まだコミットしていない変更と追跡していないファイルを重ねたもの。追跡していないシンボリックリンクは写さない）の中で走り、席が終わったら欠席のときも含めて複製は消される。席がファイルを書き換えても、利用者の worktree には届かない。起動の手段を自分の sandbox で包むのは自由。

一覧のどの席も、落ちたら（利用枠切れ、時間切れ、起動の失敗、出力を読めない）試し直さずに欠席となり、レビューは残りの席で続く。今回だけ席を減らすなら、cycle や review を呼ぶときに「今回は1席で」「gpt だけ」のように一言添える。数だけを言ったときは、一覧に書いた順に任意の席を使う。任意の席は diff review には付かない。
