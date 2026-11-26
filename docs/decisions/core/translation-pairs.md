# Reading pairs and the agreement record

English | [日本語](translation-pairs.ja.md)

Covers, when documents are kept as per-language pairs, how the language list is read, which documents are paired and how the sides of a pair are told apart, missing sides, the agreement record and blob hashes, stale sides, and how pairs are handled by the existing checks. The skeleton that must match between sides, the switcher line and the checks of links are decided in [Pair skeletons, switcher lines and links](translation-structure.md). Keeping decision documents in pairs itself, and their skeleton, are decided in [Decision documents can also be kept as multilingual pairs](languages-and-guides.md#decision-documents-can-also-be-kept-as-multilingual-pairs).

## The language list is decided by the configuration's languages

Status: implemented

### Problem

A place is needed to decide which languages documents are kept in. The languages are a property of the project, not a setting of the overview only. Deciding them from the environment of whoever runs it (the locale or the agent's settings) would make the results change with the person and the environment.

### Decision

The language list is written in the top-level "languages" key of the configuration (for example `[en, ja]`). The first language of the list is the first language, and the file without a suffix is the first language's side. Without the key, it is treated as one language, English only.

"languages" may have one language only. With one language, pairs are not read and the pair checks are not made. A Japanese-only project can also make the overview UI Japanese by writing "languages: [ja]" ([kotowari holds the English UI strings, and other languages are written in the configuration's labels](overview-languages.md#kotowari-holds-the-english-ui-strings-and-other-languages-are-written-in-the-configurations-labels)).

### Rejected options

- Decide the language from the agent's settings or the locale. It may not represent the user's language, as in a C.UTF-8 environment, and it changes from person to person.

### Confirmations

- C-core-pairs-1 When "languages" has one language or the key is absent, check does not read pairs, reports none of translation_missing, translation_record_invalid, translation_stale, translation_structure_mismatch, translation_switcher_invalid and link_language_mismatch, and also reads a file with a language tag suffix (for example "a.en.md") as an ordinary document
- C-core-pairs-2 Without the "languages" key, the language list is the one language English, and with the key it is the list in the order written

## If a language tag has the wrong form or is duplicated, it stops

Status: implemented

### Problem

Language tags are used in file name suffixes and configuration keys. The reason for deciding the rules of their form has not been kept.

### Decision

A language tag consists only of lowercase English letters, digits and "-". When "languages" has an empty string or a tag containing other characters, or the same tag appears twice or more, kotowari stops with a configuration error.

### Confirmations

- C-core-pairs-3 When "languages" has an empty tag, a tag containing uppercase letters, "_" or whitespace, or a duplicated tag, kotowari stops with a configuration error

## Guides, overview source data and the table of contents have a side for every language of the language list

Status: implemented

### Problem

Documents read by outside people need English, but the user cannot read English, so documents can only be kept per language. If only some documents have translations, readers fall into documents in the original language partway. An LLM writes them, so translating everything is not a matter of effort. Pairs that are not complete lead directly to readers falling into the original language.

### Decision

With two or more languages, guides, overview source data and the overview table of contents (the file "overview.toc" points at) are read as pairs. The pairs of decision documents are decided in [Decision documents can also be kept as multilingual pairs](languages-and-guides.md#decision-documents-can-also-be-kept-as-multilingual-pairs), and the pairs of glossaries in [Glossaries are also paired](#glossaries-are-also-paired).

A paired document must have a side for every language of the language list. If a side is missing, a translation_missing error is reported. When the first language's side is absent and only sides of other languages exist, it is also a translation_missing error.

### Rejected options

- Allow missing sides and report only a notice. It allows translating bit by bit, but the problem of readers falling into the original language partway remains.

### Confirmations

- C-core-pairs-4 If the first language's side exists and a side of another language does not, check reports a translation_missing error
- C-core-pairs-5 If only a side of another language exists and the first language's side does not, check reports a translation_missing error

## The sides of a pair are told apart by file names and configuration only

Status: implemented

### Problem

The sides of a pair are to be shown side by side in the same directory, without creating language directories. Which file is which language's side of which pair must be uniquely determined by file names and configuration only.

### Decision

The sides of a pair are placed in the same directory. The first language's side has the name without a suffix ("foo.md"), and another language's side has the name with the language tag put before the extension ("foo.en.md", or "toc.en.yaml" for the table of contents).

A file name suffix is read as a side of a pair only when it is a language tag of the language list other than the first. "foo.ja.md" with the first language's suffix (when the first is ja), "foo.fr.md" for a language not in the list, and "foo.en.md" in a project with one language are all read as ordinary documents. No dedicated finding is made for files left after a language is removed from the list either. Wrong names are found by other findings (for example, translation_missing for a missing side of a pair).

### Confirmations

- C-core-pairs-6 A file with a suffix of the first language's tag, or of a tag not in the language list, is read not as a side of a pair but as part of the name, and if the pair whose first-language side is that file lacks a side of another language, a translation_missing error is reported

## The agreement record holds the blob hash of each side as of the last time they were confirmed to have the same content

Status: implemented

### Problem

That only one side of a pair was changed is to be found by machine. The state as of the last time the sides were confirmed to have the same content must be kept somewhere. Putting the records in one place makes conflicts likely when different pairs are changed on different branches. Keeping git blob hashes lets the text as of the last confirmed agreement be taken out of git, so that only the difference is translated again.

### Decision

The agreement record is placed beside the first language's side as "<stem>.i18n.yaml". Its content is a YAML mapping whose keys are the file names of each language's side (the names without the directory) and whose values are the blob hashes of those sides. Which side a hash belongs to is known from the file name.

The blob hash is the git blob hash (40 hexadecimal characters, the same value "git hash-object" outputs for that file). The current blob hash of each side is output by "kotowari list" ([With two or more languages, the blob hash of each pair is output too](list.md#with-two-or-more-languages-the-blob-hash-of-each-pair-is-output-too)), and the LLM copies it into the agreement record. kotowari has no command that writes the agreement record. This is the same form as list outputting the fingerprints of guide markers and the LLM copying them.

When the agreement record does not exist, cannot be read as YAML, has a set of keys that does not exactly match the file names of each language's side, or has a value that is not 40 hexadecimal characters, a translation_record_invalid error is reported.

### Rejected options

- Put the agreement records in one place. Changing different pairs on different branches makes conflicts likely.

### Confirmations

- C-core-pairs-7 When the values of the agreement record are the same as the current blob hashes of all sides, check reports neither translation_stale nor translation_record_invalid
- C-core-pairs-8 When the agreement record does not exist, cannot be read as YAML, has keys different from the file names of each side, or has a value that is not 40 hexadecimal characters, check reports a translation_record_invalid error
- C-core-pairs-9 The value kotowari uses as the blob hash of a file is the same as the value "git hash-object" outputs for that file
- C-core-pairs-10 (by reading) No kotowari command writes the agreement record

## If a side is changed without updating the agreement record, that side is stale

Status: implemented

### Problem

That only one side of a pair was changed is to be found by machine. Pairs that are not complete lead directly to readers falling into the original language.

### Decision

When the current blob hash of a side differs from the value for that side in the agreement record, a translation_stale error is reported on that side.

### Confirmations

- C-core-pairs-11 If only one side is changed after the agreement record was written, check reports a translation_stale error on that side

## The staleness of guide markers is reported on every side of a guide pair

Status: implemented

### Problem

A pair is treated as the per-language forms of one document. Guide markers are part of the skeleton, so every side has the same ones, but which side guide staleness is reported on needed deciding. The reason for deciding to report it on every side has not been kept.

### Decision

Every side of a guide pair is read as a guide, and the checks of guide markers and the guide_stale notice are reported on every side.

### Confirmations

- C-core-pairs-12 If both sides of a guide pair have the same stale marker, check reports a guide_stale notice on each side

## The other-language sides of overview source data are not counted as separate pages

Status: implemented

### Problem

A pair is treated as the per-language forms of one document. If the other-language sides of overview source data were counted as separate source data, the same document would be subject, as two pages, to the name overlap and the matching against the table of contents. No other reason for deciding not to count them has been kept.

### Decision

The other-language sides of overview source data and of the table of contents are not counted as separate source data. The overlap of page names ([overview build draws everything every time, writes only the files whose contents differ, and deletes files no longer produced](overview-commands.md#overview-build-draws-everything-every-time-writes-only-the-files-whose-contents-differ-and-deletes-files-no-longer-produced)) and the matching of names in the table of contents ([Each overview page is placed exactly once in the table of contents](overview-toc.md#each-overview-page-is-placed-exactly-once-in-the-table-of-contents)) are done on the first language's side only.

### Confirmations

- C-core-pairs-13 When there are overview source data "a.md" and "a.en.md" and the only name item in the table of contents is "a", check reports neither overview_name_conflict nor overview_toc_page_missing

## Glossaries are also paired

Status: implemented

### Problem

The glossary is placed, as a document people read, in the group folder of the decision document location. Documents read by outside people need English, but the user cannot read English, so documents can only be kept per language. The glossary was until now paired as part of the IR. check does not check the contents of the glossary ([Keep the glossary as a document for people, and drop the term check](decision-documents.md#keep-the-glossary-as-a-document-for-people-and-drop-the-term-check)).

### Decision

With two or more languages, the glossary `CONTEXT.md` in a group folder of the decision document location is also read as a pair. Like other pairs, it has a side for every language ("CONTEXT.<tag>.md") and an agreement record, and missing sides, errors of the agreement record and stale sides are reported with the same findings as other pairs.

For glossary pairs, the skeletons of the sides are not compared, and the links inside the sides are not checked. The switcher line right after the title is required, as for pairs of decision documents.

### Confirmations

- C-core-pairs-14 With two or more languages, if `CONTEXT.md` in a group folder of the decision document location lacks a side of another language or an agreement record, check reports a translation_missing or translation_record_invalid error
- C-core-pairs-19 Even if the sides of a glossary pair differ in headings and tables, and another language's side has a link to a document that does not exist, check reports neither translation_structure_mismatch nor a link error

## For pairs of decision documents, counting and output are done on the first language's side only

Status: implemented

### Problem

A pair is treated as the per-language forms of one document. The skeleton is guaranteed to match by [The sides of a pair must match in their skeleton, everything other than natural-language sentences](translation-structure.md#the-sides-of-a-pair-must-match-in-their-skeleton-everything-other-than-natural-language-sentences) and [Decision documents can also be kept as multilingual pairs](languages-and-guides.md#decision-documents-can-also-be-kept-as-multilingual-pairs), so checks that concern the skeleton need only one side. The output of "kotowari query" and "kotowari list" is read by an LLM, which can read any language.

### Decision

For pairs of decision documents, the duplication of confirmation IDs, the judgement of whether a surface is quoted in the specification ([A surface is in the specification when its name is quoted in a decision's "If the name of a surface is quoted in "Decision" or "Confirmations" of a decision, that surface is in the specification" or "Confirmations"](surface.md#if-the-name-of-a-surface-is-quoted-in-decision-or-confirmations-of-a-decision-that-surface-is-in-the-specification)), and the output of "kotowari list" and "kotowari query" are done on the first language's side only. That the matching of confirmation IDs against markers is done on the first language's side only is decided in [Decision documents can also be kept as multilingual pairs](languages-and-guides.md#decision-documents-can-also-be-kept-as-multilingual-pairs). list and query get no option for choosing a language.

### Confirmations

- C-core-pairs-15 Even if both sides of a pair of decision documents have the same confirmation ID, check reports no error for duplicated IDs
- C-core-pairs-16 "kotowari list" and "kotowari query" output, for a pair of decision documents, only the decisions and confirmation rows of the first language's side
- C-core-pairs-18 Whether a surface is quoted in the specification is judged on the first language's side of decision documents only, and a name written only on another language's side is not counted as a quote

## The vague word check is made on "Decision" and "Confirmations" of every side of decision documents

Status: implemented

### Problem

Checks of sentences are needed per side, because the sentences differ by side. Even without a list of vague words per language, English words can be added to the same list if wanted, and running matches for English words over Japanese sentences does no harm.

### Decision

The vague word check ([Keep the glossary as a document for people, and drop the term check](decision-documents.md#keep-the-glossary-as-a-document-for-people-and-drop-the-term-check)) is made on "Decision" and "Confirmations" of every side of decision documents. The list of vague words is not kept per language; the one list of the configuration's "vague_words" is applied to every side.

### Confirmations

- C-core-pairs-17 If "Decision" or "Confirmations" of another language's side of a decision document contains a word of the configuration's "vague_words", check reports a finding on that side
