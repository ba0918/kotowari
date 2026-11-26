# Pair skeletons, switcher lines and links

English | [日本語](translation-structure.ja.md)

Covers the skeleton that must match between the sides of a pair, the switcher line placed on each side of a pair, the checks of links inside the sides of a pair, and the LLM's procedure for aligning a pair. The language list, how the sides of a pair are told apart, and the agreement record are decided in [Reading pairs and the agreement record](translation-pairs.md).

## The sides of a pair must match in their skeleton, everything other than natural-language sentences

Status: implemented

### Problem

Every language of a pair was made equal, and writing in any language first was allowed. That requires what does not change under translation (everything other than natural-language sentences) to be the same on every side. Everything other than sentences does not change under translation, so it can be confirmed by machine. Whether the meaning is the same cannot be confirmed by machine, so it is confirmed in review.

The part of a link destination after "#" is often made from the text of a heading, and changes under translation. The table of contents has sentences, the titles of groups, and they are needed for the index page of each language's overview.

### Decision

Between the sides of a pair, everything other than natural-language sentences must match. If it does not match, a translation_structure_mismatch error is reported on the side that does not match. Whether the meaning is the same is not checked.

For each kind of document, what must match and the sentences that may be translated are as follows. The skeleton of decision documents is decided in [Decision documents can also be kept as multilingual pairs](languages-and-guides.md#decision-documents-can-also-be-kept-as-multilingual-pairs).

| Document | What must match | Sentences that may be translated |
|---|---|---|
| Guide | The depth and order of headings, guide markers, the contents of code blocks, the numbers of table rows and columns, link destinations | The sentences of headings and body, the sentences of table cells |
| Overview source data | The kinds and order of parts, the non-sentence fields of parts, guide markers | The sentence fields of parts |
| Overview table of contents | The nesting shape of groups, and the document each item points at | The titles of groups and the title of the table of contents |

Which fields of overview parts are sentences is decided in advance per kind of part ([The kinds of parts are a fixed list of eight held by view](../view/parts.md#the-kinds-of-parts-are-a-fixed-list-of-eight-held-by-view)).

A link destination that points at a side of another language is read as pointing at the first language's side of that pair before comparing. Only the path of the destination is compared, not the part after "#". Whether the anchor is correct is looked at in review.

The switcher line is not part of the skeleton.

### Confirmations

- C-core-pairstruct-1 When the sides of a guide pair differ only in the sentences of headings, body and table cells, check does not report translation_structure_mismatch
- C-core-pairstruct-2 If the sides of a guide pair differ in any of the depth or order of headings, guide markers, the contents of code blocks, or the number of table rows or columns, check reports a translation_structure_mismatch error
- C-core-pairstruct-3 Between the sides of a pair of overview source data, translation_structure_mismatch is not reported if only the sentence fields of parts differ, and is reported if a non-sentence field of a part (for example "state" of the status part) differs
- C-core-pairstruct-4 Between the sides of a pair of the overview table of contents, translation_structure_mismatch is not reported if only the titles of groups and the title of the table of contents differ, and is reported if the nesting of groups or the document an item points at differs
- C-core-pairstruct-5 When a link destination points, on the other side, at that language's side and only the part after "#" differs, translation_structure_mismatch is not reported, and it is reported if the path of the destination differs

## Each side of a guide pair has a switcher line right after the title

Status: implemented

### Problem

What GitHub opens is the first language's side without a suffix. Without links to the sides of other languages, readers of other languages cannot reach their own language's side. Even a reader who lands on a document in a language they cannot read can find their own language's name. Overview source data can have only parts and HTML comments after the title, and the table of contents is YAML with no title. Neither is something people open on GitHub, and the drawn pages carry language links.

### Decision

Each side of a guide pair has, on the line right after the title, a switcher line that lists all languages. Its own language is text only, not a link, and other languages are links to that language's side. The name of a language is written in that language itself, taken from the configured UI strings ("language_name" of "labels.<tag>"; [kotowari holds the English UI strings, and other languages are written in the configuration's labels](overview-languages.md#kotowari-holds-the-english-ui-strings-and-other-languages-are-written-in-the-configurations-labels)). Even in projects that do not use the overview, "labels.<tag>" is written for the languages of the language list other than English.

If the switcher line is absent, has the wrong form, or points at the wrong targets, a translation_switcher_invalid error is reported. Pairs of overview source data and of the table of contents do not have switcher lines.

The switcher line is excluded from the skeleton comparison and from the [check of link languages](#links-inside-guides-and-overview-source-data-point-at-the-side-of-the-same-language).

### Rejected options

- Hold only the language names on the "languages" side, and keep the UI strings under the overview configuration. The place of the UI strings would split in two.

### Confirmations

- C-core-pairstruct-6 If the line after the title of each side of a guide pair is a correct switcher line, check does not report translation_switcher_invalid
- C-core-pairstruct-7 If the line after the title of a side of a guide pair is not a switcher line, check reports a translation_switcher_invalid error on that side
- C-core-pairstruct-8 For a pair of overview source data, check does not report translation_switcher_invalid even if what follows the title is not a switcher line
- C-core-pairstruct-9 Links inside a switcher line do not cause translation_structure_mismatch even if their number differs between sides, and do not cause link_language_mismatch even if they point at sides of other languages

## Each side of a pair of decision documents also has a switcher line right after the title

Status: implemented

### Problem

For decision documents too, what GitHub opens is the first language's side without a suffix. Without links to the sides of other languages, readers of other languages cannot reach their own language's side.

A topic document starts with the title and a paragraph on what the topic covers ([Write one decision as text in Problem, Decision, Rejected options and Confirmations](decision-documents.md#write-one-decision-as-text-in-problem-decision-rejected-options-and-confirmations)). If the switcher line were counted as that paragraph, a document without the paragraph would not be an error.

### Decision

Each side of a pair of decision documents also has, right after the title, [the same switcher line as guide pairs](#each-side-of-a-guide-pair-has-a-switcher-line-right-after-the-title). If it is absent, has the wrong form, or points at the wrong targets, a translation_switcher_invalid error is reported.

The switcher line is placed between the title and the topic paragraph. The check of the paragraph after the title skips the switcher line and looks at the next paragraph. If there is only the switcher line and no topic paragraph, it is the error of a missing topic paragraph (topic_lead_missing).

### Confirmations

- C-core-pairstruct-10 If the line after the title of a side of a pair of decision documents is not a correct switcher line, check reports a translation_switcher_invalid error on that side
- C-core-pairstruct-16 If both sides of a pair of decision documents have no topic paragraph after the title and the switcher line, check reports a topic_lead_missing error, and does not count the switcher line as the topic paragraph

## Links inside guides and overview source data point at the side of the same language

Status: implemented

### Problem

A link to a side of another language drops the reader into a document in a language they cannot read. Links are written by an LLM, so merely writing down a rule brings the same links back.

### Decision

On each side of pairs of guides and of overview source data, the Markdown links outside the switcher line are checked. Outside URLs and links within the page (those starting with "#") are not checked. When a link destination is a side of a pair whose language differs from the language of the side holding the link, a link_language_mismatch error is reported.

### Confirmations

- C-core-pairstruct-11 If the English side of a guide has a link to the Japanese side of another pair, check reports a link_language_mismatch error
- C-core-pairstruct-12 For links to a side of the same language, links to outside URLs, and links within the page starting with "#", check does not report link_language_mismatch

## Links inside decision documents also point at the side of the same language

Status: implemented

### Problem

For decision documents too, a link to a side of another language drops the reader into a document in a language they cannot read.

### Decision

The links on each side of pairs of decision documents also get [the same check of link languages as guides](#links-inside-guides-and-overview-source-data-point-at-the-side-of-the-same-language). A link from a guide or overview source data to a side of a pair of decision documents is also a link_language_mismatch error if it does not point at the side of the same language.

### Confirmations

- C-core-pairstruct-13 If a side of a pair of decision documents has a link to a side of another language of the same pair, check reports a link_language_mismatch error on that side
- C-core-pairstruct-14 If the English side of a guide has a link to the Japanese side of a decision document, check reports a link_language_mismatch error

## Links from decision documents and their paired sides to the frozen tree are errors

Status: implemented

### Problem

Documents in the old form from before the switch (including decision records and ADRs) move to the frozen tree ("docs/frozen/"), and are kept there as something not read as grounds for the current rules ([Put decision documents in docs/decisions, and move documents in the old form to the frozen tree](decision-documents.md#put-decision-documents-in-docsdecisions-and-move-documents-in-the-old-form-to-the-frozen-tree)). Until now, links from documents to the locations of decision records and ADRs were made link_to_record errors. No other reason for making links to the frozen tree errors is on record.

### Decision

A link pointing under the frozen tree ("docs/frozen/") from either the first language's side or another language's side of a decision document is a check error.

### Confirmations

- C-core-pairstruct-17 If the first language's side or another language's side of a decision document has a link to a document under "docs/frozen/", check reports an error on that side

## The procedure for aligning pairs is held by kotowari's skill

Status: implemented

### Problem

The sides of a pair are written and revised by an LLM. The blob hash of the agreement record is a git blob hash, so if that content has never been in git, the previous text cannot be taken out.

### Decision

kotowari's skill describes the procedure for aligning pairs. When any side of a pair is changed, all other sides are changed in the same change. When changing them, the previous text is taken out of git from the blob hash in the agreement record, and only the difference is translated. If it cannot be taken out, the other side is translated again from the full text. After changing them, the agreement record is rewritten with the values of "translations" from "kotowari list". When a language other than English is added to "languages", "labels.<tag>" is written.

### Confirmations

- C-core-pairstruct-15 (by reading) kotowari's skill says that when any side of a pair is changed all other sides are changed in the same change, that the previous text is taken out of git from the blob hash of the agreement record and only the difference is translated, and if it cannot be taken out the full text is translated again, that after changing them the agreement record is rewritten with the values of "translations" from "kotowari list", and that when a language other than English is added to "languages", "labels.<tag>" is written
