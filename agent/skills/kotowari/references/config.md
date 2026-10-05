Based on the kotowari specification (revised 2026-10-05; the version of kotowari itself is not pinned)

The configuration file is `.kotowari/config.yaml`, directly under the base directory. An empty configuration file (0 bytes, or comments only) checks with the default values. When there is no configuration file, the default values are used too.

| Key | Value | Default |
|---|---|---|
| ir | Path of a directory (one string) | docs/ir |
| decisions.records | Path of a directory (one string). Decisions in the files under it can be cited as sources. Markdown that is not a decision record (form contracts, supplementary documents) can also be placed there | docs/decision/records |
| decisions.adr | Path of a directory (one string) | docs/decision/adr |
| tests.files | List of globs | src/\*\*/\*.rs, tests/\*\*/\*.rs |
| guides.files | List of globs. The places of the guides, the user-facing documents whose guide marks are compared with the IR (guides.md) | Empty list |
| tests.rust.attributes | List of attribute paths added to "#[test]" | Empty list |
| tests.rust.macros | List of macro names | Empty list |
| tests.rules | List of paths of ast-grep rule YAML files, relative to the base directory. Globs are not allowed. The rules are added to the bundled queries of their `language` (mark.md) | Empty list |
| surface.files | List of globs. The surface files, the code from which surfaces are taken (surface.md) | Empty list |
| surface.rules | List of paths of ast-grep rule YAML files, relative to the base directory. Globs are not allowed. When not empty, check and status take surfaces with these rules and check them against the IR (surface.md). They are not added to the queries for tests | Empty list |
| surface.unspecified | Path of a file (one string). Points at the list of unspecified surfaces (surface.md) | None |
| mutants.equivalents | Path of a file (one string). Points at the list of equivalents (mutants.md) | None |
| limits.lines | Number (negative numbers and 0 are not allowed) | 200 |
| limits.requirements | Number (negative numbers and 0 are not allowed) | 10 |
| vague_words | List of words | The four words 「適切に」「必要に応じて」「通常は」「など」 |
| languages | List of language tags (lowercase letters, digits and `-`; no duplicates). The first is the first language, whose files have no suffix. With two or more, the IR, guides, overview data and table of contents are kept as pairs (translations.md) | None (English `en` only) |
| labels | Map from a language tag in `languages` to a map from UI text keys to strings. English has built-in text and `labels.en` overrides only the keys written; every other language in `languages` needs all keys ("UI text keys" below) | None |

The default column copies the defaults of kotowari itself, and `setup` writes to the configuration file only the keys that have a default. `mutants.equivalents` has no default, and without the key the list of equivalents behaves as empty, so it is not written in the YAML of step 1. Add it when you decide to keep a list of equivalents. `surface.unspecified` has no default for the same reason. `surface.files` and `surface.rules` must be written together: one without the other, or `surface.unspecified` without `surface.rules`, stops every command that reads the configuration with a config error. The guidance for writing the IR is held as kinds of findings, not as numbers (ir-form.md, "Limits and the unit of splitting").

`languages` and `labels` have no default and are not written in step 1. Add them only when the project keeps its documents in more than one language, or wants the overview pages in a language other than English; follow translations.md. Whenever a language other than `en` is in `languages`, write `labels.<tag>` with every key, or every command that reads the configuration stops with a config error.

### UI text keys

The UI text is the text the overview pages write themselves. English is the table below with the keys written in `labels.en` replacing theirs; every other language is exactly its `labels.<tag>`. `{n}` is replaced by a number when a page is drawn.

| Key | Where it appears | Takes `{n}` | English text |
|---|---|---|---|
| language_name | The language's name in switcher lines and in an overview page's links to other languages | No | English |
| index_link | The link back to the index from an overview page the table of contents does not name | No | Overview |
| pages | The number of overviews in a table-of-contents group on the index | Yes | {n} pages |
| stale_sections | The number of stale sections on the index | Yes | {n} sections to review |
| open_items | The number of items with status open on the index | Yes | {n} open |
| planned_items | The number of items with status planned on the index | Yes | {n} planned |
| stale_mark | The mark near a stale section's heading | No | Not reviewed since the IR changed |
| outline_stale | The mark on a stale section in the outline | No | not reviewed |
| superseded | The mark on a superseded reference | No | (superseded) |
| deferred | The mark on a deferred reference | No | (deferred) |
| compare_before | The heading of the before column of a compare part | No | Before |
| compare_after | The heading of the after column of a compare part | No | After |
| compare_why | The heading of the reason column of a compare part | No | Why |
| state_decided | The status label decided | No | Decided |
| state_planned | The status label planned | No | Planned |
| state_open | The status label open | No | Open |
| state_dropped | The status label dropped | No | Dropped |

Every command that reads the configuration stops with a config error when `labels` has a tag that is not in `languages`, when `labels.<tag>` has a key not in this table, when a language other than `en` in `languages` has no `labels.<tag>` or lacks one of these keys, or when the value of a key that takes `{n}` does not contain `{n}` exactly once.

A list written for a list key replaces the default list. An empty list is treated as a list with no elements. Nested keys are written in YAML's nested form.

The base directory: search upward from the current directory for a directory that holds a `.kotowari/` directory. The first one found is the base; if none is found, the current directory is the base. A file named `.kotowari` is ignored and the search continues upward.

The steps of setup: do the following in order. Do not overwrite files or directories that already exist; tell the person they exist (only adding a section to `AGENTS.md` is done, since it appends rather than overwrites).

1. Write `.kotowari/config.yaml` with the default values. Change a value from its default only when changing the test globs or the places for files.

```yaml
ir: docs/ir
decisions:
  records: docs/decision/records
  adr: docs/decision/adr
tests:
  files:
    - "src/**/*.rs"
    - "tests/**/*.rs"
  rust:
    attributes: []
    macros: []
  rules: []
guides:
  files: []
surface:
  files: []
  rules: []
limits:
  lines: 200
  requirements: 10
vague_words:
  - "適切に"
  - "必要に応じて"
  - "通常は"
  - "など"
```

2. Create the directories configured by `ir`, `decisions.records` and `decisions.adr` (defaults: `docs/ir/`, `docs/decision/records/`, `docs/decision/adr/`). In each newly created decision directory, also create an empty `.gitkeep`. Git does not retain empty directories, so include these files when committing the setup files to preserve the directories in clones and worktrees. Do not stage or commit them without the person's approval. The IR directory receives `CONTEXT.md` in step 3 and needs no `.gitkeep`.

3. Create `docs/ir/CONTEXT.md` with four lines:

```
# Glossary

| Term | Meaning | Source |
|---|---|---|
```

4. Add a `## kotowari` section to `AGENTS.md`. If there is no `AGENTS.md`, create it; if there is, add the section at the end. If a `## kotowari` heading already exists, do not add it. Template:

```markdown
## kotowari

This project manages its specification as an IR (`docs/ir/`).
```
