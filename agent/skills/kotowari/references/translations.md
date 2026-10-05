Based on the kotowari specification (revised 2026-10-05; the version of kotowari itself is not pinned)

# Pairs: one document in several languages

A project that lists two or more language tags in the configuration's `languages` (for example `[ja, en]`) keeps each IR document (glossaries `CONTEXT.md` and flag records `FLAGS.md` included), guide, overview data file and table of contents as a pair: one file per language, all equal in standing. Decision records, ADRs and other files in the places for records are never paired. With one language, or without the key, nothing here applies and a file such as `a.en.md` is just a document of that name.

## Names and the consistency record

- The first language in `languages` owns the file without a suffix: `a.md`, `toc.yaml`.
- Every other language's side sits in the same directory with the tag before the extension: `a.en.md`, `toc.en.yaml`, `CONTEXT.en.md`. kotowari finds it by name, whether or not a glob of `guides.files` or `overview.files` matches it.
- `<stem>.i18n.yaml` beside the first-language side is the consistency record: a YAML map from each side's file name (no directory) to the git blob hash of that side when the sides were last confirmed to say the same thing. It must have exactly one key per language, each value 40 lowercase hexadecimal characters.
- A suffix of the first language (`a.ja.md` with `[ja, en]`) or of a language not in the list (`a.fr.md`) is part of an ordinary name; such a file needs its own sides (`a.ja.en.md`).

```yaml
# docs/ir/core/a.i18n.yaml (values copied from kotowari list; these are examples)
a.md: 78981922613b2afb6025042ff6bd878ac1994e85
a.en.md: 9c1e0f0d6f1b4e2a8c3d5b7e9f1a2c4d6e8f0a1b
```

## What must match

kotowari reads items, IDs, sources, fingerprints, `list` items and `query` output from the first-language side only. On the other sides it checks only terms (against that language's glossary chain, `CONTEXT.<tag>.md`), vague words, document-name references, unclosed backticks and the glossary's form. Everything except natural-language sentences is the skeleton and must be the same on every side; a difference is translation_structure_mismatch:

| Document | Must match |
|---|---|
| Topic document | `## ` and `### ` headings (and the ID of each `### `), the `- kind:`, `- source:`, `- verification:`, `- definition:`, `- deferred:`, `- related:` lines and whether `- how_to_verify:` is there, table shapes and the IDs in their cells, scenario tag lines and the keyword starting each step, other code blocks |
| Glossary | the number of rows and each row's Source cell |
| Flag record | `### FLAG-` IDs and their `- kind:`, `- related:`, `- source:` |
| Guide | headings, guide marks, code blocks, table shapes, link destinations |
| Overview data | frontmatter, headings, part kinds and every non-sentence field, guide marks, table shapes, link destinations |
| Table of contents | the nesting of groups, the page names and whether each group has a `note` |

The sentence fields of overview parts, which are translated: lead `conclusion` and `points`; flow box `title` and `body`; steps `title` and `body`; cards `title` and `items`; status `text`; compare `before`, `after` and `why`; decisions `text` and `by`; quiz `q` and `a`. `state`, `tone`, `refs`, `ref` and `width` stay as they are.

A link destination that points at another language's side is compared as the first-language side, and only the part before `#` counts, since anchors made from headings change in translation.

## The switcher line and links

Every side of an IR or guide pair has, as its first non-blank line after the title, the switcher line: each language's `language_name` (from its UI text) in `languages` order, separated by ` | `, its own language as plain text and every other as `[name](that side's file name)`:

```markdown
# 対の読み方

日本語 | [English](translation-pairs.en.md)
```

Only a line exactly equal to it is the switcher line; it is not scope text and takes no text checks. Overview data and tables of contents have none.

Inside a side, a link (or image, or link reference definition) that does not start with a scheme or `#` must point at the same language: a link to another language's side is link_language_mismatch, and a link to a file in the places for decision records or ADRs is link_to_record — a reader reaches the decisions through the IR's sources, and records are not translated.

## The procedure for keeping a pair aligned

1. Change one side of a pair only together with every other side, in the same change. A commit that edits `a.md` edits `a.en.md` too.
2. To bring another side up to date, take the text the record last confirmed from git with the recorded blob hash of the side you changed (`git cat-file -p <hash>`), compare it with the current text, and translate only that difference into the other sides. If git cannot give that text (the hash was never committed), translate the other sides again from the whole current text.
3. Keep the skeleton identical and translate only the sentences; check the meaning by reading, since kotowari does not compare meanings.
4. After the sides say the same thing, run `kotowari list` and copy each side's `blob` from its `translations` entry into `<stem>.i18n.yaml`. Never compute or invent the hashes.
5. When a language other than English is added to `languages`, write `labels.<tag>` in the configuration with every UI text key (config.md, "UI text keys": all 17 keys, and exactly one `{n}` in the keys that take a number), write every pair's new side with its switcher line, and add the new file name to every consistency record.

## Findings

| Kind | Fix |
|---|---|
| translation_missing | Write the missing side as a translation, or remove a side that should not exist |
| translation_record_invalid | Rewrite the record from `kotowari list` (step 4) |
| translation_stale | Bring the sides to the same content (step 2), then update the record (step 4) |
| translation_structure_mismatch | Make that side's skeleton equal to the first-language side's, at the part named in detail |
| translation_switcher_invalid | Write the line in detail right after the title |
| link_language_mismatch | Point the link at the side in the same language |
| link_to_record | Remove the link |

`kotowari overview build` and `serve` write nothing while an IR, overview data or table of contents pair has translation_missing or translation_structure_mismatch.
