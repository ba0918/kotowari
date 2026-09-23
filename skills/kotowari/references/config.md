Based on the kotowari specification (revised 2026-09-23; the version of kotowari itself is not pinned)

The configuration file is `.kotowari/config.yaml`, directly under the base directory. An empty configuration file (0 bytes, or comments only) checks with the default values. When there is no configuration file, the default values are used too.

| Key | Value | Default |
|---|---|---|
| ir | Path of a directory (one string) | docs/ir |
| decisions.records | Path of a directory (one string). Decisions in the files under it can be cited as sources. Markdown that is not a decision record (form contracts, supplementary documents) can also be placed there | docs/decision/records |
| decisions.adr | Path of a directory (one string) | docs/decision/adr |
| tests.files | List of globs | src/\*\*/\*.rs, tests/\*\*/\*.rs |
| tests.rust.attributes | List of attribute paths added to "#[test]" | Empty list |
| tests.rust.macros | List of macro names | Empty list |
| mutants.equivalents | Path of a file (one string). Points at the list of equivalents (mutants.md) | None |
| limits.lines | Number (negative numbers and 0 are not allowed) | 200 |
| limits.requirements | Number (negative numbers and 0 are not allowed) | 10 |
| vague_words | List of words | The four words 「適切に」「必要に応じて」「通常は」「など」 |

The default column is a copy of the defaults of kotowari itself (as of 2026-09-17), and `setup` writes to the configuration file only the keys that have a default. `mutants.equivalents` has no default, and without the key the list of equivalents behaves as empty, so it is not written in the YAML of step 1. Add it when you decide to keep a list of equivalents. The guidance for writing the IR is held as kinds of findings, not as numbers (ir-form.md, "Limits and the unit of splitting").

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
limits:
  lines: 200
  requirements: 10
vague_words:
  - "適切に"
  - "必要に応じて"
  - "通常は"
  - "など"
```

2. Create the directories for the files: `docs/ir/`, `docs/decision/records/`, `docs/decision/adr/`

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
