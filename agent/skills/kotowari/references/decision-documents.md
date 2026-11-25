Based on the kotowari specification (revised 2026-11-26; the version of kotowari itself is not pinned)

How to write a decision document: the form check reads, what goes in it and what does not, the
two states, the rows to confirm and their IDs, the rows confirmed by reading, how a decision is
rewritten or overturned, the open section, and links. The findings check raises on these
documents are in findings.md; the key that names their place is `decisions.documents` in
config.md.

## What a decision document is

The specification is held in decision documents only: one Markdown file per topic, holding the
decisions of that topic that hold now. There is no record added per discussion, no requirement
written in a fixed sentence pattern, no scenario, no source line and no collation.

A decision document holds only what a person decided. A detail the implementer decided with its
grounds (a limit, the order of an output, the line a finding points at) is not written in it: the
code and the tests are where that detail is, and its reason is in the commit body and the pull
request body.

"What was decided" always states the present rule. Do not write history into it ("previously",
"changed to"); git holds the history. Only the history needed to say why the decision was made
goes in the problem and the rejected options.

## Place

The documents live under the directory `decisions.documents` names, one topic per file, with at
most one level of directories for the groups of a project (a crate, a product):
`docs/decisions/<group>/<topic>.md`. Every `.md` file under that place is read as a topic
document, so put nothing else there — no glossary, no index, no notes. A tree named `docs/frozen/`
is not read; it holds documents in the old form only and nothing is added to it.

## The form of a topic document

```markdown
# <Title of the topic>

<A paragraph saying what the topic covers, and links to the neighbouring topics.>

## <One sentence that states the decision>

状態: 提案

### 問題

<What was the trouble. Readable without knowing what was decided.>

### 決めたこと

<The rule that holds now, in ordinary sentences.>

### 見送った案

- <An option considered and not taken>. <Why it was not taken.>

### 確認すること

- C-<name>-1 <A result someone can see that shows the decision is kept>
- C-<name>-2 （読んで確認）<A result a person or an LLM confirms by reading>

## 未決

- <What is not decided>. <Who decides it, and when.>
```

The strings check matches are Japanese and literal: `状態:`, `提案`, `実装済み`, `問題`,
`決めたこと`, `確認すること`, `未決` and `（読んで確認）` (full-width parentheses).

| Part | How check reads it |
|---|---|
| Title | The first `#` heading. A document without one is missing_title |
| Lead | At least one paragraph between the title and the first decision; without it, topic_lead_missing |
| Decision | Every `##` heading other than `## 未決` starts one decision, up to the next `##` heading |
| State | The first paragraph under the decision's heading, `状態: 提案` or `状態: 実装済み`; anything else, or no such paragraph, is decision_state_invalid |
| Sections | `###` headings. `### 問題` and `### 決めたこと` are required (decision_section_missing); `### 見送った案` and `### 確認すること` are not |
| Rows | Each item of a list under `### 確認すること` is one row; its first word is its ID |
| Open section | `## 未決`, placed last; each item of a list under it is one open item. It is not a decision |

### Writing one decision

- The heading is one sentence that says the decision. One heading holds one decision; do not bind
  several into one.
- The problem says what was the trouble, in words that need no knowledge of the answer. The reason
  for the decision is written here and in the sentences of "what was decided".
- The rejected options list only options that were actually considered, each with why it lost.
  When there were none, leave the section out; never invent one. When no reason survives, write
  in the problem that the reason is not recorded.
- A decision table (pairs of condition and result) is a Markdown table inside "what was decided".
  A property (what holds for every input) is a row to confirm. There is no separate section or ID
  kind for either.
- A decision about the process, the build or the release, which has nothing a test observes, is
  written in the same kind of document without `### 確認すること`.

## States

| State | Meaning |
|---|---|
| `提案` (proposed) | Decided, not built yet. check asks for no test for its rows |
| `実装済み` (implemented) | Built; the behavior of the product now. check asks for a test for each of its rows not confirmed by reading |

A decision is written as proposed. The implementer turns it to implemented when every row of it
is met: each row checked by a test has a test whose mark carries the row's ID, and each row
confirmed by reading has been read and found true. A decision with no rows is turned to
implemented when what it decides has been done (the skill text written, the setting changed).
A proposed decision nobody is building now stays proposed; `kotowari status` counts both states.
A mark that points at a row of a proposed decision raises nothing, because a test is often written
before its decision is turned.

## Rows to confirm and their IDs

A row states one result that can be seen, worded so that a person approving it can tell what is
to be confirmed. Exact inputs and the exact shape of an output belong in the test that points at
the row, not in the row.

The ID is `C-<name>-<number>`: the name in lowercase letters, digits and `-`, built from the group
and the topic (`C-core-docs-3`), and the number from 1 without a leading zero. Give a new row the
number after the largest one the topic has used. Never close up the numbers of deleted rows and
never reuse a number for another row. A row without a well-formed ID is invalid_id; a second row
with the same ID is duplicate_id.

A test points at a row by its mark, `@kotowari[C-core-docs-3]` (the form of a mark is in
mark.md). A mark that points at an ID no document has is an error. A row of an implemented
decision that no mark points at is confirmation_without_test. `kotowari query <row ID>` returns
the row and the file, line and name of each test that points at it.

### Rows confirmed by reading

A row that a test cannot confirm — a skill's text says something, a document exists — starts
with `（読んで確認）` right after its ID. check asks for no test for such a row. The implementer
and the review read it and confirm it; the row's own text says what must be found.

## Changing a decision

- When a fact inside a decision changes (a value, a name, a path), rewrite the decision in place.
- When building it shows that a sentence of a decision does not match the facts (it cannot hold
  literally), and the direction decided does not change, correct the sentence as a rewrite of a
  fact, and say so in the pull request body.
- To overturn a decision, write a new decision. In its rejected options, write the old decision
  and why it was dropped, starting the item with `以前の決定:` (previous decision). Move the old
  decision's reasons and rejected options that still mean something into the new one. Then delete
  the old decision's text. The tests that pointed at the old decision's rows are made to point at
  the new rows, or deleted when nothing is left for them to confirm.

Do not leave an overturned decision in the document marked as overturned, and do not move it to
the frozen tree.

## The open section

What is not decided yet, and a question handed back to a person, go as items of `## 未決` at the
end of the topic document it concerns: what is not decided, and who decides it and when. Only what
a person decides goes there; a detail the implementer may decide does not. There is no separate
document of flags.

`complete` in `kotowari status` is true only when check has no error and no topic document has an
item under `## 未決`. Decisions left proposed do not change it.

## Links

Point at another decision by a Markdown link from the sentence to that decision's heading, not by
an ID: `[the heading's words](confirmations.md#<anchor>)`. The anchor follows GitHub's rule for
headings: lower-cased, every character other than letters, digits, `_`, `-` and spaces removed,
spaces turned into `-`, and `-1`, `-2` added to repeated headings. Japanese letters are letters,
and Japanese punctuation is removed: `## 朝は「おはよう」と言う` is `#朝はおはようと言う`, and
`## 決定の文書は docs/decisions に置く` is `#決定の文書は-docsdecisions-に置く`. The href is relative to the
document holding the link. A link to a document or heading that does not exist inside the place is
link_target_missing. When a heading's words change, fix every link to it in the same change.

A definition used by several decisions is written inside the decision that settled it, and the
others link to it; there is no section for definitions only. The meaning of a term goes in the
project's glossary.
