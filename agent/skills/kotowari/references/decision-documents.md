Based on the kotowari specification (revised 2026-11-26; the version of kotowari itself is not pinned)

How to write a decision document: the form check reads and its keywords in Japanese and English,
the pair of language sides, what goes in it and what does not, the two states, the rows to confirm and their IDs, the rows confirmed by reading, how a decision is
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

The documents live under the directory `decisions.documents` names, one topic per file, with a
directory below it for each group of a project (a crate, a product):
`docs/decisions/<group>/<topic>.md`. Every `.md` file under that place is read as a topic
document, with two exceptions: the other-language sides of a pair (below), and the glossary of a
group, `docs/decisions/<group>/CONTEXT.md`. check does not read the glossary as a topic document
and checks nothing inside it; links to it and its headings are checked like links to any
document. A `CONTEXT.md` directly in the place, or deeper than a group folder, is a topic document.
Put nothing else there — no index, no notes. A tree named `docs/frozen/` is not read; it holds
documents in the old form only and nothing is added to it.

## The form of a topic document

```markdown
# <Title of the topic>

<The switcher line, when `languages` lists two or more tags>

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

The keywords check matches are literal, and each has a Japanese and an English spelling. check
accepts either spelling of every keyword in any document; write a document in one language's
spelling throughout — the language of that side of the pair (below).

| Keyword | Japanese | English |
|---|---|---|
| State line | `状態: 提案` / `状態: 実装済み` | `Status: proposed` / `Status: implemented` |
| Problem | `### 問題` | `### Problem` |
| What was decided | `### 決めたこと` | `### Decision` |
| Rejected options | `### 見送った案` | `### Rejected options` |
| Rows to confirm | `### 確認すること` | `### Confirmations` |
| Row confirmed by reading | `（読んで確認）` (full-width parentheses) | `(by reading)` |
| Open section | `## 未決` | `## Open` |
| A previous decision in the rejected options | `以前の決定:` | `Previous decision:` |

Below, the Japanese spelling stands for both. `state` in `kotowari list` and `query` is
`proposed` or `implemented` whichever spelling the document uses, and decision_section_missing
names the missing section in the spelling of the decision's state line.

| Part | How check reads it |
|---|---|
| Title | The first `#` heading. A document without one is missing_title; each later `#` heading is multiple_titles |
| Lead | At least one paragraph between the title and the first decision, the switcher line not counted; without it, topic_lead_missing. More paragraphs, lists and tables there are part of the lead; a `###` or deeper heading there is unknown_heading |
| Decision | Every `##` heading other than `## 未決` starts one decision, up to the next `##` heading |
| State | The first paragraph under the decision's heading, `状態: 提案` or `状態: 実装済み`; anything else, or no such paragraph, is decision_state_invalid. Any other block before the first `###` section is unknown_line (a heading, unknown_heading); only a link reference definition may stand there |
| Sections | `###` headings. `### 問題` and `### 決めたこと` are required (decision_section_missing); `### 見送った案` and `### 確認すること` are not; a `###` heading with any other name is unknown_heading |
| Rows | Each item of a list under `### 確認すること` is one row; its first word is its ID |
| Open section | `## 未決`, placed last; each item of a list under it is one open item. Any other block under it (a paragraph, a heading) is unknown_line or unknown_heading, so write every open question as a list item. It is not a decision |
| Code blocks | A fenced code block never closed swallows what follows it: unclosed_code_block at its opening line |

### Languages: a pair of sides

When the configuration's `languages` lists two or more tags, every decision document is a pair,
as the IR and the guides are (translations.md): the file without a suffix is the side of the first
language, `<topic>.<tag>.md` beside it is each other language's side, and `<topic>.i18n.yaml` is
the agreement record. The first language is one per repository and holds for the decision
documents, the guides and the overview alike. In kotowari itself it is English: `x.md` is English
and `x.ja.md` is the Japanese side, which is the side the person reads and approves.

- Each side uses the keywords of its own language: the English side `Status: proposed`,
  `### Decision`; the Japanese side `状態: 提案`, `### 決めたこと`.
- Every side has the switcher line as the first non-blank line after the title (translations.md);
  the topic paragraph is the paragraph after it.
- check reads the decisions, the row IDs, the marks they are matched with, the fingerprints, the
  targets of guide marks and the quoted surfaces from the first-language side only. Another side
  is read only for its headings and links: a link there must point at a document and heading that
  exist, in its own language (`confirmations.ja.md#<Japanese anchor>`), or it is
  link_target_missing or link_language_mismatch. Errors that leave content unread
  (unclosed_code_block, multiple_titles, unknown_heading, unknown_line) are raised on each side
  itself, since the skeleton below does not compare paragraphs or lists.
- The skeleton of the sides must match, or the other side is translation_structure_mismatch: the
  headings (their levels and order, and which keyword section each `###` is, not their words), the
  state of each decision, and each row's ID with whether it is confirmed by reading.
- A missing side is translation_missing; a side changed after the agreement record was written is
  translation_stale. Change both sides in the same change, then copy each side's `blob` from
  `kotowari list` into the agreement record (translations.md, **The procedure for keeping a pair
  aligned**). This includes turning a state.
- A group's `CONTEXT.md` is paired the same way (sides, switcher line, agreement record), but its
  content and links are not compared.

## Writing one decision

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
  and why it was dropped, starting the item with `以前の決定:` (`Previous decision:` in English). Move the old
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
project's glossary (`CONTEXT.md` of the group).

In a pair, each side links to the headings of the same language: the English side to
`confirmations.md#<English anchor>`, the Japanese side to `confirmations.ja.md#<Japanese anchor>`.

## Guides and surfaces

A guide mark points at a decision by `<path of the document>#<anchor of its heading>`, the path and
heading of the first-language side as written, and its
fingerprint covers only "what was decided" and "rows to confirm"; `kotowari list` lists each
decision as kind `decision` with that target as `id` and its `fingerprint` (guides.md). A
surface counts as specified when its name is the whole content of a pair of double quotes or
backticks in "what was decided" or "rows to confirm" of any decision, in either state (a table
cell by cell, code blocks not read); the problem, the rejected options and the open section do
not count (surface.md).
