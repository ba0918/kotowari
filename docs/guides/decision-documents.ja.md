# 決定の文書を書く

[English](decision-documents.md) | 日本語

<!-- @kotowari[docs/decisions/core/decision-documents.md#the-specification-lives-only-in-decision-documents-one-per-topic-holding-the-decisions-now-in-force:d2ea663b] -->

kotowari は仕様を決定の文書から読みます。話題ごとに1つの Markdown のファイルで、その話題の今有効な決定を並べます。
1つの決定には、何が困っていたか、何を決めたか、採らなかった案、決定が守られていることを示す結果を書きます。
`kotowari check` はこの文書の形を検査し、結果の行ごとにテストが確かめているかを数えます。

決定の文書には、人が決めたことだけを書きます。
実装する側が根拠を持って決めた細部（上限の数、出力の順）は書きません。細部はコードとテストが持ち、その理由はコミットとプルリクエストの本文が持ちます。
経緯（「以前は」「〜に変えた」）も書きません。経緯は git が持ちます。

## 決定の文書の置き場

<!-- @kotowari[docs/decisions/core/decision-documents.md#put-decision-documents-in-docsdecisions-and-move-documents-in-the-old-form-to-the-frozen-tree:f9400721, docs/decisions/core/config.md#the-location-of-decision-documents-is-pointed-at-by-decisionsdocuments-with-docsdecisions-as-the-default-and-the-keys-for-the-locations-of-the-old-form-are-removed:4a2eacb8, docs/decisions/core/ir-document.md#the-decision-document-location-is-traversed-to-any-depth-and-only-files-with-a-lowercase-md-are-read:4a6f0eb6, docs/decisions/core/decision-documents.md#keep-the-glossary-as-a-document-for-people-and-drop-the-term-check:270bb62b] -->

文書は、設定の `decisions.documents` が指すディレクトリ（既定は `docs/decisions`）の下に置き、クレートや製品のようなプロジェクトの群ごとにフォルダを作ります。

```text
docs/decisions/
  shop/
    CONTEXT.md      the glossary of the group
    cart.md         one topic
    price.md        another topic
```

置き場の下の `.md` のファイルは、深さによらず、どれも話題の文書として読みます。例外は、対のほかの言語の側と、群の用語集 `CONTEXT.md` です。
用語集は人が読む文書で、check は中身を読まず、用語の検査もしません。
目次やメモのような、ほかのものは置き場に置かないでください。

`docs/frozen/` という名前の木は読みません。
古い形の文書だけを置く所で、決定の文書からそこへリンクを張ると `link_to_record` の誤りになります。

## 話題の文書の形

<!-- @kotowari[docs/decisions/core/decision-documents.md#write-one-decision-as-text-in-problem-decision-rejected-options-and-confirmations:3990395b] -->

```markdown
# Cart

This topic decides how the cart computes its total.

## The total is the sum of the line prices

Status: implemented

### Problem

Customers could not see what they would pay before checkout.

### Decision

The cart shows the sum of the prices of its lines as the total.

### Rejected options

- Show the total only at checkout. Customers left the cart to find out the price.

### Confirmations

- C-shop-cart-1 A cart with two lines of 100 and 250 shows a total of 350
- C-shop-cart-2 (by reading) The help page of the cart says how the total is computed

## Open

- Whether a discount code can be combined with another. The shop owner decides before the campaign.
```

| 部分 | check の読み方 |
|---|---|
| 題名 | 最初の `#` の見出し。無ければ `missing_title`。2つ目以降の `#` の見出しは `multiple_titles` |
| 導入 | 題名と最初の決定のあいだの、話題が何を扱うかを書いた段落が1つ以上。無ければ `topic_lead_missing`。見るのは段落があるかだけ |
| 決定 | `## 未決` のほかの `##` の見出しが、次の `##` の見出しまでの1つの決定を始める |
| 状態 | 決定の見出しの下の最初の段落。`状態: 提案` か `状態: 実装済み`。ほかは `decision_state_invalid` |
| 節 | `### 問題` と `### 決めたこと` は必須（`decision_section_missing`）。`### 見送った案` と `### 確認すること` は任意。ほかの名前の `###` の見出しは `unknown_heading`。`####` 以下の見出しは、それがある節に属する |
| 行 | `### 確認すること` の下の箇条書きの項目が1つの行で、最初の語がその ID |
| 未決の節 | 最後に置く `## 未決`。箇条書きの項目が1つの未決の問い。決定ではない |

どのキーワードにも日本語と英語の綴りがあります（`状態: 提案` と `Status: proposed`、`### 決めたこと` と `### Decision`、`（読んで確認）` と `(by reading)`、`## 未決` と `## Open`）。
check はどの文書でもどちらの綴りも受けます。1つの文書は、その文書の言語の綴りで書いてください。

## 1つの決定の書き方

<!-- @kotowari[docs/decisions/core/decision-documents.md#write-one-decision-as-text-in-problem-decision-rejected-options-and-confirmations:3990395b, docs/decisions/core/decision-documents.md#when-the-same-decision-appears-in-two-topics-write-it-on-the-side-where-the-behaviour-is-visible:c0eeb81f] -->

- 見出しは、決定を言う1つの文にします。1つの見出しには1つの決定だけを書き、いくつも束ねません。
- 問題には、何が困っていたかを、答えを知らなくても読める言葉で書きます。決定の理由は、問題と決めたことの文に書きます。
- 決めたことには、今有効な規則を普通の文で書きます。決定表（条件と結果の組）は、その中の Markdown の表にします。
- 見送った案には、実際に検討した案だけを、採らなかった理由と一緒に挙げます。無ければ節ごと省きます。
- テストで見るものの無い、工程やビルドやリリースの決定は、`### 確認すること` を付けずに書きます。
- 2つの話題にかかる決定は、振る舞いが見える側の話題に書き、もう一方からリンクを張ります。

設定の `vague_words` の語（既定は「適切に」「必要に応じて」「通常は」「など」）は、決めたことと確認することの節では `vague_word` の誤りになります。
代わりに、何が起きるかを具体的に書いてください。

## 提案と実装済み

<!-- @kotowari[docs/decisions/core/confirmations.md#the-implementing-side-changes-the-status-and-decisions-not-built-yet-stay-proposed:436adde2, docs/decisions/core/confirmations.md#check-counts-per-confirmation-row-whether-a-test-points-at-it:f6a31de4] -->

| 状態 | 意味 | check が求めること |
|---|---|---|
| `状態: 提案` | 決めたが、まだ作っていない | 行にテストを求めない。その行を指す印があっても何も出さない |
| `状態: 実装済み` | 作った。今の製品の振る舞い | 読んで確認する行のほかの各行にテスト（`confirmation_without_test`） |

新しい決定は提案で書きます。そうすれば、テストが無いうちに仕様だけを先にコミットできます。
すべての行を満たしたら、実装済みに変えます。テストで確かめる行には、その行の ID を印に持つテストがあり、読んで確認する行は読んで正しいと確かめた状態です。
行の無い決定は、決めたことを実際に済ませたら変えます。
今は誰も作っていない提案の決定は、提案のまま置きます。`kotowari status` は両方の状態の数を出します。

実装済みの決定に行を足しても、状態は実装済みのままです。足した行のテストが無いことは、テストを書くまで誤りとして出ます。

## 確認することの行と ID

<!-- @kotowari[docs/decisions/core/confirmations.md#check-counts-per-confirmation-row-whether-a-test-points-at-it:f6a31de4, docs/decisions/core/confirmations.md#write-by-reading-at-the-head-of-a-line-not-checked-by-tests:bfbb46c7, docs/decisions/core/findings.md#if-the-group-name-of-a-confirmation-id-does-not-match-the-group-folder-it-is-placed-in-it-is-an-id_domain_mismatch-error:cf755b08] -->

1つの行には、見て確かめられる結果を1つ、承認する人が何を確かめるのか分かる言葉で書きます。
入力の値や出力の細かい形は、行ではなく、その行を指すテストに書きます。

ID は `C-<名前>-<番号>` です。

- 名前は小文字の英字、数字、`-` で、群と話題から作ります（`C-shop-cart-3`）。群のフォルダの下の文書では、ID の群の名前をそのフォルダの名前にします。違えば、その行は `id_domain_mismatch` です。
- 番号は1から始め、先頭に0を付けません。新しい行には、その話題で使った最大の番号の次を付けます。消した行の番号を詰めたり、別の行に使い回したりしません。

形の正しい ID の無い行は `invalid_id`、同じ ID の2つ目の行は `duplicate_id` です。
テストは印 `@kotowari[C-shop-cart-1]` で行を指します（[marks.md](marks.ja.md)）。

テストでは確かめられない行（ヘルプのページに何かが書いてある、文書がある）は、ID のすぐ後を `（読んで確認）` で始めます。
check はその行にテストを求めません。実装する側とレビューが読んで確かめるので、行の文には何が見つかればよいかを書きます。

## 決定を変える

<!-- @kotowari[docs/decisions/core/decision-documents.md#rewrite-when-a-fact-changes-when-overturning-a-decision-write-the-old-decision-in-rejected-options:371ec613, docs/decisions/core/decision-documents.md#to-refer-to-another-decision-link-to-its-heading:41ba3d75] -->

- 決定の中の事実（値、名前、パス）が変わったら、その決定をその場で書き直します。
- 作ってみて文がそのままでは成り立たないと分かり、向きは変わらないときも、同じように文を直します。
- 決定を覆すときは、新しい決定を書きます。その見送った案に、古い決定と捨てた理由を `以前の決定:` で始めて書きます。古い理由と見送った案のうち、まだ意味のあるものは新しい決定に移し、古い決定の文は消します。古い行を指していたテストは新しい行を指すように直し、確かめることが残らなければ消します。

覆した決定に「覆した」と書いて文書に残すことはしません。

ほかの決定を指すときは、文から見出しへリンクを張ります。`[合計は行の値段の和にする](cart.ja.md#合計は行の値段の和にする)` のように書きます。
行き先の名前は GitHub の見出しの規則に従います。小文字にし、英数字、`_`、`-`、空白のほかの文字を除き、空白を `-` にし、同じ見出しには `-1`、`-2` を足します。日本語の文字は文字として残り、日本語の句読点は除かれます。
置き場の中に無い文書や見出しへのリンクは `link_target_missing` なので、見出しの文を変えたら、そこを指すリンクとガイドの印を同じ変更で直してください。

## 未決と `complete`

<!-- @kotowari[docs/decisions/core/decision-documents.md#write-questions-handed-back-to-people-in-the-open-section-of-the-topic-document:8286069a] -->

まだ決めていないことと、人に戻す問いは、関わる話題の文書の最後の `## 未決` に箇条書きの項目で書きます。何が決まっていないか、誰がいつ決めるかを書きます。
書くのは人が決めることだけです。実装する側が決めてよい細部は書きません。

`kotowari status` の `complete` は、check に誤りが無く、どの話題の文書の `## 未決` にも項目が無いときだけ true です。
提案のまま置いた決定は、これを変えません。

## 2つの言語で持つ決定の文書

<!-- @kotowari[docs/decisions/core/languages-and-guides.md#decision-documents-can-also-be-kept-as-multilingual-pairs:535e227a, docs/decisions/core/translation-structure.md#each-side-of-a-pair-of-decision-documents-also-has-a-switcher-line-right-after-the-title:3056a209, docs/decisions/core/translation-pairs.md#for-pairs-of-decision-documents-counting-and-output-are-done-on-the-first-languages-side-only:f5bf6692, docs/decisions/core/translation-structure.md#links-inside-decision-documents-also-point-at-the-side-of-the-same-language:361842e7] -->

設定の `languages` に2つ以上のタグを書くと、決定の文書はどれもガイドと同じく対になります。`cart.md` が先頭の言語の側、`cart.ja.md` が日本語の側、`cart.i18n.yaml` が一致の記録です（[言語と対](config.ja.md#言語と対--languages-と-labels)）。

- どの側も自分の言語のキーワードを使い、題名の直後に切り替えの行を置きます。話題の段落は切り替えの行の後に書きます。
- 決定、行の ID、そのテスト、指紋、ガイドの印の行き先は、先頭の言語の側だけから読みます。ほかの側では、見出し、リンク（同じ言語の側を指す）、曖昧語だけを検査します。
- 側どうしの骨組みは揃えます。見出し、各 `###` がどの節か、各決定の状態、各行の ID と読んで確認する行かどうかです。違えば、ほかの側が `translation_structure_mismatch` です。
- 状態を変えることも含めて、両方の側を同じ変更で直し、`kotowari list` の各側の `blob` を一致の記録に写します。

## 文書を検査する

<!-- @kotowari[docs/decisions/core/status.md#the-number-of-confirmation-rows-is-output-split-into-those-with-tests-those-without-and-those-by-reading:4b92a0ae, docs/decisions/core/decision-documents.md#write-questions-handed-back-to-people-in-the-open-section-of-the-topic-document:8286069a] -->

書いたら `kotowari check` を実行します。
上のかごの文書と、`@kotowari[C-shop-cart-1]` の印を付けたテストがあれば、check は何も出さず、`kotowari status` が決定と行を数えます。

```console
$ kotowari check --format text
$ kotowari status --format text | grep -E '^(decisions|confirmations|complete)'
decisions proposed=0 implemented=1
confirmations with_tests=1 without_tests=0 by_reading=1
complete false
```

ここで `complete` が false なのは、文書の `## 未決` に項目があるからです。
指摘の種類ごとの直し方は [指摘の種類](findings.ja.md#誤り-決定の文書) にあります。

## 関連

- 仕様: [決定の文書](../decisions/core/decision-documents.ja.md)、[確認することとテスト](../decisions/core/confirmations.ja.md)
- 指摘の種類: [findings.md](findings.ja.md)
- テストに印を付ける: [marks.md](marks.ja.md)
- 設定: [config.md](config.ja.md)
