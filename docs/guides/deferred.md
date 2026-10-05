# 要求を後回しにする — `- deferred:`

<!-- @kotowari[REQ-core-208:223c4219] -->

仕様には書いたが今は作らない、と決めた要求を「後回し」と宣言する書き方です。
後回しの要求と、それだけを指すシナリオには、テストが無くても `requirement_without_test` と `scenario_without_test` が出なくなります。
形と参照の検査と ID の予約はそのまま続きます。

上流の制約で作れない要求が混ざっていると、それらのテストの無さが、作ったのにテストが無い本物の未完と同じ誤りに並び、後者が埋もれます。
後回しを宣言すると、誤りに残るのは本物の未完だけになり、`status` の `complete` もその有無を表すようになります。

## 書式

<!-- @kotowari[REQ-core-208:223c4219, REQ-core-209:9672da47, REQ-core-210:3ab477dc, TBL-core-011:19177a27, REQ-core-046:6346a41e] -->

値は出典です。
後回しにすると決めた決定を、理由と一緒に判断の記録に書き、それを `- source:` と同じ `パス#決定の番号` の形で指します。
コンマで区切って複数書けます。

要求を1つずつ後回しにするときは、要求の見出しの下に書きます。

```markdown
### REQ-greet-003: 英語で挨拶する

- kind: event_driven
- source: docs/decision/records/2026-09-24-greet.md#A4
- verification: unit
- deferred: docs/decision/records/2026-09-24-greet.md#A4

"--lang en" を受けたとき、コマンドは`英語の挨拶文`を出す。
```

文書のすべての要求を後回しにするときは、題名の後、最初の `## ` より前に1行書きます（文書単位の宣言）。

```markdown
# 挨拶の翻訳

英語など、日本語でない挨拶文を扱う。
- deferred: docs/decision/records/2026-09-24-greet.md#A4

## Requirements
```

| 置き場 | 後回しになるもの |
|---|---|
| 要求の見出しの下 | その要求 |
| 話題ごとの文書の、題名の後で最初の `## ` か `### ` より前 | その文書のすべての要求 |

- 両方を書いても指摘は出ません
- 文書単位の宣言は範囲の行に数えません。範囲の文が無く宣言だけの文書は `missing_scope` になります
- 文書単位の宣言を置けるのは話題ごとの文書だけです。`CONTEXT.md` と `FLAGS.md` に書くと `unknown_field` になります
- 決定表、性質、シナリオは、それだけを後回しにはできません。シナリオは指す要求で決まります（次の節）

## 変わること、変わらないこと

<!-- @kotowari[REQ-core-085:9c02a2ea, REQ-core-137:192fc62f, REQ-core-208:223c4219] -->

上の `REQ-greet-003` から `- deferred:` の行を消すと、その要求と、それを指すシナリオ `EX-greet-002` にテストが無いことが誤りになります。

```console
$ kotowari check --format text
docs/ir/greet/greet.md:15 [error] requirement_without_test REQ-greet-003
docs/ir/greet/greet.md:31 [error] scenario_without_test EX-greet-002
$ echo $?
1
```

`- deferred:` の行を戻すと、どちらも出なくなります。

```console
$ kotowari check --format text
$ echo $?
0
```

シナリオにテストを求めないのは、`@about` の要求のうち検証が unit、property、proof、review のものに後回しが1つ以上あり、それらがすべて後回しか review のときです（後回しのシナリオ）。
`- verification:` の行の無い要求は、この判断に数えません。
後回しの要求と、後回しでない unit の要求を両方指すシナリオには、今までどおりテストが要ります。

後回しでも変わらないもの:

- 形の検査（`verification_missing`、`missing_statement` など）
- 参照の検査（`unresolved_reference`）と出典の検査
- ID の予約（`duplicate_id`）

同じ ID が2か所以上にあるときは、1つ目（文書のパスのバイト順、同じ文書では行の小さい方）の要求で後回しかどうかを決めます。
シナリオも1つ目のシナリオの `@about` で決めます。

## status と list と query

<!-- @kotowari[TBL-core-028:669e5402, TBL-core-026:05d8938e, REQ-core-155:ec783479, TBL-core-027:62b02eff, REQ-core-161:bf9675f4] -->

`status` は、後回しの要求の数を `requirements` の `deferred` に、後回しのシナリオの数を `scenarios` の `deferred` に出します。
後回しは `with_tests` にも `without_tests` にも数えないので、`without_tests` は本物の未完の数になります。
検証の値ごとの数（`unit` など）と `without_examples` には、後回しも数えます。

```console
$ kotowari status --format text
documents files=2 lines=42
items requirement=2 table=0 property=0 scenario=2 flag=0
requirements unit=2 property=0 proof=0 review=0 with_tests=1 without_tests=0 review_with_how_to_verify=0 review_without_how_to_verify=0 without_examples=0 deferred=1
scenarios with_tests=1 without_tests=0 deferred=1
tests marks=1 rs=1
guides files=0 marks=0
overview files=0 marks=0
surface total=0 specified=0 unspecified=0
findings error=0 notice=0
complete true
```

`list` と `query` の JSON では、すべての項目とシナリオが真偽値の `deferred` を持ちます。
後回しの要求と後回しのシナリオは true、ほかは false です。
text では、後回しの1件の行の末尾に ` deferred` が付きます。

```console
$ kotowari list --format text
REQ-greet-001 unit 名前を入れて挨拶する docs/ir/greet/greet.md:7 tests=0
REQ-greet-003 unit 英語で挨拶する docs/ir/greet/greet.md:15 tests=0 deferred
EX-greet-001 - 名前を挨拶文に入れる docs/ir/greet/greet.md:28 tests=1
  tests/greet.rs:1 greets_with_name
EX-greet-002 - 英語で挨拶する docs/ir/greet/greet.md:33 tests=0 deferred
```

テストがまだ無い項目を探すときは、後回しを外します。

```console
$ kotowari list | jq -r '.items[] | select(.tests == [] and .verification != "review" and (.deferred | not)) | .id'
```

## 後回しとの食い違いの注意

<!-- @kotowari[REQ-core-211:d7879896, REQ-core-212:e533fb7e, TBL-core-009:e42a4a68] -->

後回しと、テストの印や参照が食い違うと、2種類の注意が出ます。
注意なので、終了コードも `status` の `complete` も変えません。

```console
$ kotowari check --format text
docs/ir/greet/greet.md:13 [notice] depends_on_deferred REQ-greet-001 REQ-greet-003
docs/ir/greet/greet.md:15 [notice] deferred_with_test REQ-greet-003
```

| 種類 | 出るとき | detail | 行 |
|---|---|---|---|
| `deferred_with_test` | 後回しの要求か後回しのシナリオの ID を含む印がある（問い合わせの無い言語の印も数える）。印が何本あっても ID ごとに1件 | その ID | 要求の見出しの行。シナリオはタグの行 |
| `depends_on_deferred` | 後回しでない要求か性質が、`- definition:` の行か文の中のバッククォートで囲んだ ID で後回しの要求を指す。後回しのシナリオでないシナリオが、`@about` かステップの中のバッククォートで囲んだ ID で後回しの要求を指す。参照1件ごとに1件 | 参照元の ID と参照先の ID を半角空白で区切ったもの | 参照を書いた行 |

後回しの要求と後回しのシナリオからの参照、後回しのシナリオへの参照、問題の記録の `- related:` からの参照には出ません。

`deferred_with_test` は、作り終えたのに後回しを外し忘れたか、印の付け間違いです。
作り終えたなら、決定を判断の記録に書いてから `- deferred:` の行を消します。
まだ作らないなら、印を消します。

`depends_on_deferred` は、これから作る項目が、作らない項目に頼っている印です。
参照元も後回しにするか、要求を後回しから戻すか、参照を外すかを決めます。
どれも仕様の判断です。

## 宣言の誤り

<!-- @kotowari[REQ-core-209:9672da47, REQ-core-210:3ab477dc, REQ-core-115:16c7309b, TBL-core-008:11d73b51, TBL-core-019:67506434] -->

`- deferred:` の値は `- source:` と同じ規則で検査します。
誤りの行は、その `- deferred:` の行です。

```console
$ kotowari check --format text      # 値を空にした
docs/ir/greet/i18n.md:4 [error] missing_source deferred
$ kotowari check --format text      # 無い決定の番号を書いた
docs/ir/greet/i18n.md:4 [error] source_invalid docs/decision/records/2026-09-24-greet.md#A9
```

| 種類 | よくある原因 | 直し方 |
|---|---|---|
| `missing_source` | `- deferred:` の値が空。detail は `deferred` | 後回しにすると決めた決定を書く |
| `source_invalid` | 出典の先に、その決定の番号の行が無い | 判断の記録に実在する決定を指す |
| `duplicate_field` | 文書単位の `- deferred:` の行が2つある（要求の下で2つ書いたときも同じ）。detail は `deferred` | 1行にまとめる。読まれるのは1つ目の行の値だけ |
| `unknown_field` | `CONTEXT.md` か `FLAGS.md` の題名の後に書いた | 話題ごとの文書に書く |
| `missing_scope` | 題名の後に `- deferred:` の行しか無い | 文書が扱う範囲の文を書く |

値が空でも出典が誤っていても、要求は後回しのままです。
誤りを直すまで、テストの無さの誤りの代わりに出典の誤りが出ます。

## 指紋とガイド

<!-- @kotowari[REQ-core-203:e195dec1] -->

要求の見出しの下の `- deferred:` の行は、`- source:` と違って要求の指紋に入ります。
文書単位の宣言を持つ文書では、その1つ目の行を、その文書の要求の指紋の先頭に加えます。
決定表とシナリオの指紋には入りません。

そのため、後回しを宣言したときと外したときに、その要求を印に書いたガイドの節に `guide_stale` が出ます。
後回しを外すことはその機能を出すことなので、ガイドの見直しを促すためです。
指紋の扱いは [writing-guides.md](writing-guides.md#指紋) にあります。

## なぜこういう作りか

- **要求ごとの宣言を基本にし、文書単位の宣言を省略形にする。**
  食い違いの検査は要求ごとに判断するので、要求が単位になります。
  上流の制約で止まるのは文書ごとのことが多いので、1行で済む省略形を足しました。
  （[2026-09-25-deferred-items.md A1](../decision/records/2026-09-25-deferred-items.md#A1)）
- **値は出典にする。**
  理由の無い後回しを作れないようにするためです。
  検査は `- source:` と同じで、新しい誤りの種類は作っていません。
  （[2026-09-25-deferred-items.md A2](../decision/records/2026-09-25-deferred-items.md#A2)）
- **シナリオには宣言を書かない。**
  シナリオのタグは3つのままにし、指す要求で決めます。
  後回しでない要求も指すシナリオは、その要求を確かめる例でもあるので、テストを求め続けます。
  （[2026-09-25-deferred-items.md A3](../decision/records/2026-09-25-deferred-items.md#A3)）
- **後回しのテストの無さは、注意としても出さない。**
  注意にすると後回しの件数だけ並び、本物の未完がまた埋もれます。
  件数は `status` の `deferred` に出るので、黙って増え続けることはありません。
  （[2026-09-25-deferred-items.md A4](../decision/records/2026-09-25-deferred-items.md#A4)、[A8](../decision/records/2026-09-25-deferred-items.md#A8)）
- **食い違いは注意で知らせる。**
  後回しにテストの印がある、作る項目が後回しに頼っている、のどちらも人が気付くべき状態ですが、CI を落とすほどではありません。
  （[2026-09-25-deferred-items.md A11](../decision/records/2026-09-25-deferred-items.md#A11)、[A12](../decision/records/2026-09-25-deferred-items.md#A12)）

## 関連

- 仕様: [後回し](../ir/core/deferred.md)、[テストの無さの検査](../ir/core/coverage.md)
- 指摘の一覧: [findings.md](findings.md)
- 数と一覧: [status](commands/status.md)、[list](commands/list.md)、[query](commands/query.md)
- テストの印: [marks.md](marks.md)
