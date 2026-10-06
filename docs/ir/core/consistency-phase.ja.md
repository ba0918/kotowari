# 整合のフェーズ

[English](consistency-phase.md) | 日本語

実装した、または変えたコードと`IR`の食い違い、`IR`そのものの隙間と矛盾を、cycle の中の専用の検証で LLM が見つけて片付けることを扱う。人は最後の手段としてだけ関わる。

## Requirements

### REQ-core-359: 整合のフェーズを走らせる場面

- kind: ubiquitous
- source: docs/decision/records/2026-10-06-changes-rethink.md#A3, docs/decision/records/2026-10-06-changes-rethink.md#A5, docs/decision/records/2026-10-06-changes-rethink.md#A11, docs/decision/records/2026-10-06-changes-rethink.md#A16, docs/decision/records/2026-10-06-changes-rethink.md#A17, docs/decision/records/2026-10-06-changes-rethink.md#A19, docs/decision/records/2026-10-06-changes-rethink.md#A27, docs/decision/records/2026-10-06-changes-rethink.md#A28, docs/decision/records/2026-10-06-changes-rethink.md#A29
- verification: review
- how_to_verify: "agent/skills/kotowari-cycle/"、"agent/skills/kotowari-iterate/" と "agent/skills/kotowari-using-workflow/" を読み、次のすべてが書いてあることを確かめる。cycle の工程が、実装、`整合のフェーズ`、品質のレビューの順であること。`整合のフェーズ`は実装の後に1回と、品質のレビューの直しが収束した後に直し全体の差分に対して1回走り、フェーズ自身の直しの後は新しい指摘が出なくなるまで走ること。走らせるかは cycle（オーケストレータ）が決め、メインセッションでの直接の修正ではメインセッションのエージェントが同じ目安で決めること。省いてよいのは差分が利用者から見える振る舞いを足しも変えもしないとき（ガイドなどの文書だけ、テストだけ、振る舞いを変えないリファクタ）だけで、スキルの文章は振る舞いとして扱って省かず、迷ったら走らせ、省いたときは最後の報告に1行の理由を書くこと。フェーズ役に渡す指示が "agent/skills/kotowari-cycle/" の reference に1つあること。新しいスキルを足していないこと。kotowari-iterate が cycle の流れでフェーズを通すこと

kotowari-cycle は常に、実装の後と、品質のレビューの直しが収束した後に、品質のレビューとは別の工程として`整合のフェーズ`を走らせ、差分が利用者から見える振る舞いを足しも変えもしないときだけ理由を残して省く。メインセッションでの直接の修正にも同じ目安で走らせる。

### REQ-core-360: 整合のフェーズが見る範囲

- kind: ubiquitous
- source: docs/decision/records/2026-10-06-changes-rethink.md#A2, docs/decision/records/2026-10-06-changes-rethink.md#A10, docs/decision/records/2026-10-06-changes-rethink.md#A12, docs/decision/records/2026-10-06-changes-rethink.md#A14, docs/decision/records/2026-10-06-changes-rethink.md#A15, docs/decision/records/2026-10-06-changes-rethink.md#A24, docs/decision/records/2026-10-06-changes-rethink.md#A25, docs/decision/records/2026-10-06-changes-rethink.md#A29
- verification: review
- how_to_verify: "agent/skills/kotowari-cycle/" の`整合のフェーズ`の reference を読み、次のすべてが書いてあることを確かめる。見るのは記録ではなくコードそのもので、コードにはスキルの文章のような製品として配る文章を含むこと。範囲は、差分で変わったコードとそれが呼ぶ・呼ばれる直接の周辺、差分が使う値の作り元を1段、範囲の`項目`（差分が触った`項目`と、計画が扱う ID と差分が付けた`印`の ID の`項目`）、その`項目`が使う用語、`項目`どうしの関係（定義の表、@about、参照）、範囲の`項目`と用語が引く`判断の記録`であること。`IR`の無い話題では、承認済みの仕様の文書（無ければ依頼の文）を相手にすること。見つけるものは、`IR`に無い振る舞い（隙間）、`IR`と違う振る舞い、範囲の`IR`の中の隙間と矛盾であること。コードベース全体と`IR`全体の突き合わせは、別に頼まれたときだけ行うこと

`整合のフェーズ`は常に、差分で変わったコードとその直接の周辺と作り元を1段、範囲の`項目`とその用語と関係と引く`判断の記録`を読み、`IR`に無い振る舞い、`IR`と違う振る舞い、範囲の`IR`の隙間と矛盾を見つける。

### REQ-core-361: 見つけたものの片付け方

- kind: ubiquitous
- source: docs/decision/records/2026-10-06-changes-rethink.md#A6, docs/decision/records/2026-10-06-changes-rethink.md#A8, docs/decision/records/2026-10-06-changes-rethink.md#A9, docs/decision/records/2026-10-06-changes-rethink.md#A13, docs/decision/records/2026-10-06-changes-rethink.md#A26, docs/decision/records/2026-10-06-changes-rethink.md#A29, docs/decision/records/2026-10-06-changes-rethink.md#A32
- verification: review
- how_to_verify: "agent/skills/kotowari-cycle/" の`整合のフェーズ`の reference を読み、次のすべてが書いてあることを確かめる。見つけたものは必ず`IR`とコードが整合した状態に片付け、`IR`に穴があるのにコードを`IR`に合わせて戻すことはしないこと。観測で決まるものは実測や再現で確かめて決めること。既存の`IR`と整合する具体化は根拠を示して`IR`と`判断の記録`に足すこと。用語や表の定義より広げる・狭める読みは具体化ではなく食い違いとして扱うこと。既存の`IR`と食い違うものは、その`項目`が引く`判断の記録`の理由を制約として読み、`IR`の誤りを示せたときだけ`IR`を改めて理由を残し、示せなければコードを`IR`に合わせること。`IR`の誤りを示すのは`IR`どうしの矛盾か実現できないことの再現だけで、テストが通ることは根拠にしないこと。実験で決まらない意味の決めごとは、`IR`と矛盾しない既定値で整合させて進め、`問題の記録`に残し、その指摘を人の判断を待つもの（human_judgment）として開いたまま最後の報告に回し、既定値とそれを覆す一言を載せること。人の判断は最後の手段で極力減らし、人の判断を待つものにする指摘には、試した根拠と実測と、それで決められなかった理由を書くこと。フェーズが人に戻して止まる道が無いこと。仕様との整合については、品質のレビューの直しよりフェーズの決定を優先すること。cycle の「IR は直し役に任せ、矛盾は人に戻す」という扱いがこれに置き換わっていること

`整合のフェーズ`は常に、見つけたものを根拠と実測に基づいて`IR`とコードが整合した状態に片付け、根拠から決められない意味の決めごとだけを既定値で進めて、人の判断を待つものとして最後の報告に回す。

### REQ-core-363: 整合のフェーズの直しと終わり

- kind: ubiquitous
- source: docs/decision/records/2026-10-06-changes-rethink.md#A13, docs/decision/records/2026-10-06-changes-rethink.md#A28, docs/decision/records/2026-10-06-changes-rethink.md#A29, docs/decision/records/2026-10-06-changes-rethink.md#A31, docs/decision/records/2026-10-06-changes-rethink.md#A33, docs/decision/records/2026-10-06-changes-rethink.md#A34, docs/decision/records/2026-10-06-consistency-phase-findings-loop.md#A1
- verification: review
- how_to_verify: "agent/skills/kotowari-cycle/" の`整合のフェーズ`の reference と cycle の手順を読み、次のすべてが書いてあることを確かめる。フェーズ自身が`IR`、`判断の記録`、`問題の記録`、コードを直してコミットし、cycle はそのコミットを記録するだけであること。決めたことは、その実行の新しい`判断の記録`に、決めたのが`整合のフェーズ`であることと、選択、理由、根拠のポインタ（実測のコマンドと結果、ファイルと行）を書くこと。`IR`を直したときは英日の対と一致の記録をそろえること。フェーズが見つけたものは cycle の指摘のファイルにレビューの指摘と同じ形で観点を consistency として載り、毎回の実行で前の回までのフェーズの指摘のうち開いていて直しを待つものごとに still_present か no_longer_visible を返し（人の判断を待つ指摘は最後の報告まで開いたまま残すため、再評価も上げ直しもしない）、新しく見つけたものと直したもののコミットを返すこと。止め方は cycle の既存の決まり（2回続けて still_present、閉じた原因が戻る、利用者の上限、再開）に従い、フェーズ用の独自の状態を持たないこと。指摘のファイルが持つフェーズの状態は、最初の品質のレビューが見た head の1つだけであること。1回の実行は、その実行の範囲の`IR`側の指摘が無い状態で終え、テスト側の指摘と範囲の外のものは含めないこと。直し役の契約の「フックが`IR`側の指摘で止めたら`IR`を触らずに戻す」をフェーズに当てはめないこと

`整合のフェーズ`は常に、自分で直してコミットし、決めたことを根拠とともに新しい`判断の記録`に残し、見つけたものを cycle の指摘のファイルに載せて、cycle の既存の止め方に従う。

### REQ-core-362: レビューは品質の観点だけ

- kind: ubiquitous
- source: docs/decision/records/2026-10-06-changes-rethink.md#A18, docs/decision/records/2026-10-06-changes-rethink.md#A29
- verification: review
- how_to_verify: "agent/skills/kotowari-review/"、"agent/skills/kotowari-cycle/" と "agent/skills/kotowari-iterate/" を読み、レビュー役は品質の観点だけで立ち、コードと仕様の整合を見る適合の観点の席も、それを足す決まりも無いこと、計画や文書が仕様と矛盾していないかは品質の観点の中で見ることが書いてあることを確かめる

kotowari-review は常に、品質の観点だけでレビュー役を立て、コードと仕様の整合は`整合のフェーズ`に任せる。
