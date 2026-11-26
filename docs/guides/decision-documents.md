# Writing decision documents

English | [日本語](decision-documents.ja.md)

<!-- @kotowari[docs/decisions/core/decision-documents.md#the-specification-lives-only-in-decision-documents-one-per-topic-holding-the-decisions-now-in-force:d2ea663b] -->

kotowari reads the specification from decision documents: one Markdown file per topic, holding the decisions of that topic that hold now.
Each decision says what the trouble was, what was decided, which options were not taken, and which results show that the decision is kept.
`kotowari check` checks the form of these documents and counts, row by row, whether a test confirms each result.

A decision document holds only what a person decided.
A detail the implementer decided with its grounds (a limit, the order of an output) is not written there: the code and the tests hold it, and the commit and pull request bodies hold its reason.
History is not written either ("previously", "changed to"); git holds it.

## Where decision documents live

<!-- @kotowari[docs/decisions/core/decision-documents.md#put-decision-documents-in-docsdecisions-and-move-documents-in-the-old-form-to-the-frozen-tree:f9400721, docs/decisions/core/config.md#the-location-of-decision-documents-is-pointed-at-by-decisionsdocuments-with-docsdecisions-as-the-default-and-the-keys-for-the-locations-of-the-old-form-are-removed:4a2eacb8, docs/decisions/core/ir-document.md#the-decision-document-location-is-traversed-to-any-depth-and-only-files-with-a-lowercase-md-are-read:4a6f0eb6, docs/decisions/core/decision-documents.md#keep-the-glossary-as-a-document-for-people-and-drop-the-term-check:270bb62b] -->

The documents live under the directory that `decisions.documents` in the configuration names (`docs/decisions` by default), with a folder for each group of the project, such as a crate or a product:

```text
docs/decisions/
  shop/
    CONTEXT.md      the glossary of the group
    cart.md         one topic
    price.md        another topic
```

Every `.md` file under the location is read as a topic document, at any depth, except the other-language sides of a pair and the glossary `CONTEXT.md` of a group.
The glossary is a document for people: check does not read its content and does not check terms.
Put nothing else in the location, such as an index or notes.

A tree named `docs/frozen/` is not read.
It holds old-form documents only, and a link to it from a decision document is a `link_to_record` error.

## The form of a topic document

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

| Part | How check reads it |
|---|---|
| Title | The first `#` heading. Without one, `missing_title`; each later `#` heading is `multiple_titles` |
| Lead | At least one paragraph between the title and the first decision, saying what the topic covers. Without it, `topic_lead_missing`. Only its presence is checked |
| Decision | Every `##` heading other than `## Open` starts one decision, up to the next `##` heading |
| State | The first paragraph under the decision's heading: `Status: proposed` or `Status: implemented`. Anything else is `decision_state_invalid` |
| Sections | `### Problem` and `### Decision` are required (`decision_section_missing`); `### Rejected options` and `### Confirmations` are optional. Any other `###` heading is `unknown_heading`. `####` and deeper headings belong to the section they are in |
| Rows | Each item of the list under `### Confirmations` is one row, and its first word is its ID |
| Open section | `## Open`, placed last. Each list item is one open question. It is not a decision |

Every keyword has an English and a Japanese spelling (`Status: proposed` and `状態: 提案`, `### Decision` and `### 決めたこと`, `(by reading)` and `（読んで確認）`, `## Open` and `## 未決`).
check accepts either in any document; write one document in the spelling of its language.

## Writing one decision

<!-- @kotowari[docs/decisions/core/decision-documents.md#write-one-decision-as-text-in-problem-decision-rejected-options-and-confirmations:3990395b, docs/decisions/core/decision-documents.md#when-the-same-decision-appears-in-two-topics-write-it-on-the-side-where-the-behaviour-is-visible:c0eeb81f] -->

- The heading is one sentence that states the decision. One heading holds one decision; do not bind several into one.
- The problem says what the trouble was, in words that need no knowledge of the answer. The reason for the decision goes in the problem and in the sentences of the decision.
- The decision states the rule that holds now, in ordinary sentences. A decision table (pairs of condition and result) is a Markdown table inside it.
- The rejected options list only options that were actually considered, each with why it was not taken. When there were none, leave the section out.
- A decision about the workflow, the build or the release, with nothing a test observes, is written without `### Confirmations`.
- A decision that concerns two topics is written in the topic where the behaviour is visible, and the other topic links to it.

The words of `vague_words` in the configuration ("適切に", "必要に応じて", "通常は", "など" by default) are `vague_word` errors in the Decision and Confirmations sections.
Write concretely what happens instead.

## Proposed and implemented

<!-- @kotowari[docs/decisions/core/confirmations.md#the-implementing-side-changes-the-status-and-decisions-not-built-yet-stay-proposed:436adde2, docs/decisions/core/confirmations.md#check-counts-per-confirmation-row-whether-a-test-points-at-it:f6a31de4] -->

| State | Meaning | What check asks |
|---|---|---|
| `Status: proposed` | Decided, not built yet | No test for its rows. A mark pointing at one of its rows raises nothing |
| `Status: implemented` | Built; the behaviour of the product now | A test for each of its rows not confirmed by reading (`confirmation_without_test`) |

Write a new decision as proposed, so the specification can be committed before its tests exist.
Turn it to implemented when every row is met: each row checked by a test has a test whose mark carries the row's ID, and each row confirmed by reading has been read and found true.
A decision with no rows is turned when what it decides has been done.
A proposed decision nobody is building now stays proposed; `kotowari status` counts both states.

When rows are added to an implemented decision, it stays implemented, and the new rows without tests are errors until tests are written.

## Confirmation rows and their IDs

<!-- @kotowari[docs/decisions/core/confirmations.md#check-counts-per-confirmation-row-whether-a-test-points-at-it:f6a31de4, docs/decisions/core/confirmations.md#write-by-reading-at-the-head-of-a-line-not-checked-by-tests:bfbb46c7, docs/decisions/core/findings.md#if-the-group-name-of-a-confirmation-id-does-not-match-the-group-folder-it-is-placed-in-it-is-an-id_domain_mismatch-error:cf755b08] -->

A row states one result that can be seen, worded so that a person approving it can tell what is to be confirmed.
Exact inputs and the exact shape of an output belong in the test that points at the row, not in the row.

The ID is `C-<name>-<number>`:

- The name is lowercase letters, digits and `-`, built from the group and the topic (`C-shop-cart-3`). In a document under a group folder, the group in the ID must be that folder's name, or the row is `id_domain_mismatch`.
- The number starts at 1, without a leading zero. Give a new row the number after the largest one the topic has used. Never close up the numbers of deleted rows, and never reuse a number for another row.

A row without a well-formed ID is `invalid_id`, and a second row with the same ID is `duplicate_id`.
A test points at a row with its mark, `@kotowari[C-shop-cart-1]` ([marks.md](marks.md)).

A row that a test cannot confirm (a help page says something, a document exists) starts with `(by reading)` right after its ID.
check asks for no test for such a row; the implementer and the review read it and confirm it, so the row's text says what must be found.

## Changing a decision

<!-- @kotowari[docs/decisions/core/decision-documents.md#rewrite-when-a-fact-changes-when-overturning-a-decision-write-the-old-decision-in-rejected-options:371ec613, docs/decisions/core/decision-documents.md#to-refer-to-another-decision-link-to-its-heading:41ba3d75] -->

- When a fact inside a decision changes (a value, a name, a path), rewrite the decision in place.
- When building it shows that a sentence does not hold literally and the direction does not change, correct the sentence the same way.
- To overturn a decision, write a new decision. In its rejected options, write the old decision and why it was dropped, starting the item with `Previous decision:`. Move the old reasons and rejected options that still mean something into the new decision, then delete the old decision's text. Point the tests of the old rows at the new rows, or delete them when nothing is left for them to confirm.

Do not keep an overturned decision in the document marked as overturned.

To point at another decision, link from the sentence to its heading: `[the total is the sum of the line prices](cart.md#the-total-is-the-sum-of-the-line-prices)`.
The anchor follows GitHub's rule for headings: lower-cased, characters other than letters, digits, `_`, `-` and spaces removed, spaces turned into `-`, and `-1`, `-2` added to repeated headings.
A link to a document or heading that does not exist inside the location is `link_target_missing`, so when a heading's words change, fix every link and guide mark pointing at it in the same change.

## Open questions and `complete`

<!-- @kotowari[docs/decisions/core/decision-documents.md#write-questions-handed-back-to-people-in-the-open-section-of-the-topic-document:8286069a] -->

What is not decided yet, and a question handed back to a person, go as list items of `## Open` at the end of the topic document it concerns: what is not decided, and who decides it and when.
Only what a person decides goes there; a detail the implementer may decide does not.

`complete` in `kotowari status` is true only when check has no error and no topic document has an item under `## Open`.
Decisions left proposed do not change it.

## Decision documents in two languages

<!-- @kotowari[docs/decisions/core/languages-and-guides.md#decision-documents-can-also-be-kept-as-multilingual-pairs:535e227a, docs/decisions/core/translation-structure.md#each-side-of-a-pair-of-decision-documents-also-has-a-switcher-line-right-after-the-title:3056a209, docs/decisions/core/translation-pairs.md#for-pairs-of-decision-documents-counting-and-output-are-done-on-the-first-languages-side-only:f5bf6692, docs/decisions/core/translation-structure.md#links-inside-decision-documents-also-point-at-the-side-of-the-same-language:361842e7] -->

When `languages` in the configuration lists two or more tags, every decision document is a pair, like a guide: `cart.md` is the first language's side, `cart.ja.md` the Japanese side, and `cart.i18n.yaml` the agreement record ([languages and pairs](config.md#languages-and-pairs--languages-and-labels)).

- Each side uses the keywords of its own language and has the switcher line right after the title. The topic paragraph comes after the switcher line.
- The decisions, the row IDs, their tests, the fingerprints and the targets of guide marks are read from the first language's side only. On another side, only its headings, its links (they point at the same language's side) and its vague words are checked.
- The skeleton of the sides must match: the headings, which section each `###` is, the state of each decision, and each row's ID and whether it is confirmed by reading. Otherwise the other side is `translation_structure_mismatch`.
- Change both sides in the same change, including turning a state, then copy each side's `blob` from `kotowari list` into the agreement record.

## Checking the documents

<!-- @kotowari[docs/decisions/core/status.md#the-number-of-confirmation-rows-is-output-split-into-those-with-tests-those-without-and-those-by-reading:4b92a0ae, docs/decisions/core/decision-documents.md#write-questions-handed-back-to-people-in-the-open-section-of-the-topic-document:8286069a] -->

Run `kotowari check` after writing.
For the cart document above with a test marked `@kotowari[C-shop-cart-1]`, check reports nothing, and `kotowari status` counts the decisions and rows:

```console
$ kotowari check --format text
$ kotowari status --format text | grep -E '^(decisions|confirmations|complete)'
decisions proposed=0 implemented=1
confirmations with_tests=1 without_tests=0 by_reading=1
complete false
```

`complete` is false here because the document has an item under `## Open`.
Each finding kind and its fix is in [Finding kinds](findings.md#errors-decision-documents).

## Related

- Specification: [decision documents](../decisions/core/decision-documents.md), [confirmations and tests](../decisions/core/confirmations.md)
- Finding kinds: [findings.md](findings.md)
- Marking tests: [marks.md](marks.md)
- Configuration: [config.md](config.md)
