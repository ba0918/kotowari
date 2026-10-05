# Users and construction

English | [日本語](cli-scope.ja.md)

Covers who uses kotowari, where it writes, and where its code lives.

## Requirements

### REQ-core-101: Users

- kind: ubiquitous
- source: docs/decision/records/records.md#A1, docs/decision/records/records.md#A98
- verification: review
- how_to_verify: The CLI output is JSON/text in a form an LLM reads easily. Check `src/main.rs`

kotowari is always, first of all, a CLI used by an LLM, which a human may also run to check things. It is used in the repository of a developer who runs a specification-driven flow (brainstorm, `decision record`, `IR`, implementation).

### REQ-core-102: No saved state

- kind: prohibition
- source: docs/decision/records/records.md#A75, docs/decision/records/2026-09-17-check-reach.md#A3, docs/decision/records/2026-09-17-check-reach.md#A4, docs/decision/records/2026-09-17-check-reach.md#A21, docs/decision/records/2026-10-02-whole-picture.md#A28, docs/decision/records/2026-10-02-whole-picture.md#A80, docs/decision/records/2026-10-02-whole-picture.md#A72
- verification: unit

kotowari shall not save state, nor write anywhere other than standard output and standard error. The only exception is that "kotowari overview build" and "kotowari overview serve" write under ".kotowari/cache/overview/" of the `base directory` and delete files under it (REQ-core-296).

### REQ-core-105: Where the crates and the CLI live

- kind: ubiquitous
- source: docs/decision/records/records.md#A8, docs/decision/records/2026-10-03-public-crate-api.md#A14, docs/decision/records/2026-10-03-public-crate-api.md#A34, docs/decision/records/2026-10-03-public-crate-api.md#A35
- verification: review
- how_to_verify: Confirm that the root kotowari-cli package provides the kotowari binary in src/main.rs, and that the library is split under crates according to the responsibilities and dependencies of TBL-core-040.

kotowari's code always adds a crate each time a layer is added, and manages the CLI in "src/" directly under the root.

## Examples

```gherkin
@id=EX-core-042 @about=REQ-core-102 @source=docs/decision/records/2026-09-17-check-reach.md#A21,docs/decision/records/2026-09-17-check-reach.md#A28
Scenario: Nothing is written to the home or temporary directory either
  Given the environment variables "HOME" and "TMPDIR" point to an empty temporary directory
  When "kotowari check" is run
  Then the list and contents of all files in that temporary directory and in the `base directory` are equal before and after the run
```
