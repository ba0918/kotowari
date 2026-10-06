# Configuration

English | [日本語](config.ja.md)

Covers the location of the configuration file, its keys and default values, and configuration errors.

## Requirements

### REQ-core-011: Location of the configuration file

- kind: event_driven
- source: docs/decision/records/records.md#A2, docs/decision/records/records.md#A37, docs/decision/records/2026-09-24-plan-schema.md#A15
- verification: unit

When a command other than "kotowari plan" does not receive "--config", kotowari reads ".kotowari/config.yaml" in the `base directory` as the `configuration file`.

### REQ-core-012: When there is no configuration file

- kind: event_driven
- source: docs/decision/records/records.md#A12, docs/decision/records/records.md#A60, docs/decision/records/records.md#A105, docs/decision/records/records.md#A135
- verification: unit

When "--config" is not received and there is no `configuration file`, kotowari checks with the default values. When the `configuration file` is empty (0 bytes or only comments), kotowari checks with the default values, even if it is the one pointed to by "--config".

### REQ-core-013: Keys and default values

- kind: algorithm
- source: docs/decision/records/records.md#A12, docs/decision/records/records.md#A23, docs/decision/records/records.md#A36, docs/decision/records/records.md#A41, docs/decision/records/records.md#A47, docs/decision/records/records.md#A48, docs/decision/records/records.md#A49, docs/decision/records/2026-09-16-notice.md#A5, docs/decision/records/2026-09-27-surface-check.md#A1, docs/decision/records/2026-09-27-surface-check.md#A6
- definition: TBL-core-004
- verification: unit

### REQ-core-014: Configuration errors

- kind: event_driven
- source: docs/decision/records/records.md#A12, docs/decision/records/records.md#A41, docs/decision/records/records.md#A20, docs/decision/records/records.md#A44, docs/decision/records/records.md#A93, docs/decision/records/records.md#A105, docs/decision/records/records.md#A135, docs/decision/records/records.md#A161, docs/decision/records/2026-09-24-doc-marks.md#A16, docs/decision/records/2026-09-24-guide-gaps.md#A7, docs/decision/records/2026-09-27-surface-check.md#A1, docs/decision/records/2026-09-27-surface-check.md#A20, docs/decision/records/2026-10-02-whole-picture.md#A17, docs/decision/records/2026-10-02-whole-picture.md#A32, docs/decision/records/2026-10-02-whole-picture.md#A71, docs/decision/records/2026-10-02-whole-picture.md#A76, docs/decision/records/2026-10-06-todo-zero.md#A3
- verification: unit

When the `configuration file` cannot be read as YAML, or when the `configuration file` contains any of the following: an unknown key, a second occurrence of the same key, a key whose value is null (a line with only "ir:"; an empty list is accepted as in REQ-core-016), a value of the wrong type, a negative number, 0, an absolute-path value (one starting with "/", including an element of "tests.files", "guides.files", "surface.files" or "overview.files"), an empty-string element or a second occurrence of the same word in "vague_words", or an element of any of "tests.files", "guides.files", "surface.files" or "overview.files" that cannot be read as a glob, kotowari will `stop` on the grounds of a configuration error.

### REQ-core-015: A list replaces the default

- kind: ubiquitous
- source: docs/decision/records/records.md#A41
- verification: unit

kotowari always accepts only a list for a list key, and replaces the default list with the list written.

### REQ-core-016: Empty lists

- kind: event_driven
- source: docs/decision/records/2026-10-01-change-details.md#A2, docs/decision/records/records.md#A59
- verification: unit

When an empty list is written for a list key, kotowari treats that key as a list with no elements. However, an empty list for changes.files or changes.records stops the run as a configuration error.

### REQ-core-017: Nested keys

- kind: ubiquitous
- source: docs/decision/records/records.md#A59
- verification: unit

kotowari always reads the keys of the `configuration file` in nested form ("records:" under "decisions:").

### REQ-core-018: When a location is missing

- kind: event_driven
- source: docs/decision/records/records.md#A41, docs/decision/records/records.md#A47, docs/decision/records/records.md#A66, docs/decision/records/records.md#A95, docs/decision/records/records.md#A96, docs/decision/records/records.md#A124, docs/decision/records/records.md#A146, docs/decision/records/2026-09-16-ir-tree.md#A16, docs/decision/records/2026-09-17-mutation-tests.md#A55, docs/decision/records/2026-09-24-doc-marks.md#A16, docs/decision/records/2026-09-27-surface-check.md#A20, docs/decision/records/2026-10-02-whole-picture.md#A17, docs/decision/records/2026-10-02-whole-picture.md#A32, docs/decision/records/2026-10-02-whole-picture.md#A71, docs/decision/records/2026-10-02-whole-picture.md#A76
- verification: unit

In "kotowari check", when the target of "ir", "decisions.records" or "decisions.adr" does not exist, is not a directory, or cannot be read, kotowari will `stop` on the grounds of an unreadable file. It will also `stop` for the same reason when a directory under "ir", "decisions.records" or "decisions.adr" cannot be read, when a directory cannot be read while scanning "tests.files", "guides.files", "surface.files" or "overview.files", and when a scan meets a symbolic link with no target.

### REQ-core-019: How globs are read

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-current-change-records.md#A8, docs/decision/records/2026-10-02-whole-picture.md#A17, docs/decision/records/2026-10-02-whole-picture.md#A32, docs/decision/records/2026-10-02-whole-picture.md#A71, docs/decision/records/2026-10-06-todo-zero.md#A4, docs/decision/records/records.md#A59, docs/decision/records/records.md#A102, docs/decision/records/2026-10-01-change-details.md#A4
- verification: unit

kotowari always reads "**" in a glob as recursion. Except for the enumeration of Git targets in changes, the search for conformance records under hidden directories that changes.records names explicitly by a path component in check/status, and the search for `overview data` under hidden directories that overview.files names explicitly by a path component, it does not include hidden directories even when a glob names them. A broad "**" alone does not enter hidden directories that are not specified. It reads hidden files when a glob matches them, and does not follow symbolic links to directories. A hidden directory named only inside braces (as in "{.kotowari/changes,other}/*.yaml") is not counted as named by a path component.

### REQ-core-020: kotowari.toml at the root is not read

- kind: prohibition
- source: docs/decision/records/records.md#R6, docs/decision/records/2026-09-17-check-reach.md#A3, docs/decision/records/2026-09-17-check-reach.md#A4
- verification: unit

kotowari shall not read "kotowari.toml" directly under the repository root as the `configuration file`.

## Decision tables

### TBL-core-004: Keys and default values

- source: docs/decision/records/2026-10-01-change-details.md#A2, docs/decision/records/records.md#A12, docs/decision/records/records.md#A23, docs/decision/records/records.md#A36, docs/decision/records/records.md#A41, docs/decision/records/records.md#A47, docs/decision/records/records.md#A48, docs/decision/records/records.md#A49, docs/decision/records/records.md#A62, docs/decision/records/records.md#A69, docs/decision/records/2026-09-16-notice.md#A5, docs/decision/records/2026-09-17-mutation-tests.md#A36, docs/decision/records/2026-09-17-mutation-tests.md#A43, docs/decision/records/2026-09-17-mutation-tests.md#A16, docs/decision/records/2026-09-24-multi-language-tests.md#A9, docs/decision/records/2026-09-24-multi-language-tests.md#A28, docs/decision/records/2026-09-24-doc-marks.md#A4, docs/decision/records/2026-09-27-surface-check.md#A1, docs/decision/records/2026-09-27-surface-check.md#A3, docs/decision/records/2026-09-27-surface-check.md#A6, docs/decision/records/2026-09-27-surface-check.md#A20, docs/decision/records/2026-09-27-surface-check.md#A22, docs/decision/records/2026-10-02-whole-picture.md#A17, docs/decision/records/2026-10-02-whole-picture.md#A32, docs/decision/records/2026-10-02-whole-picture.md#A71, docs/decision/records/2026-10-05-overview-index.md#A10, docs/decision/records/2026-10-05-overview-index.md#A11, docs/decision/records/2026-10-05-localization.md#A16, docs/decision/records/2026-10-05-localization.md#A40, docs/decision/records/2026-10-05-localization.md#A21, docs/decision/records/2026-10-05-localization.md#A7

| Key | Value | Default |
|---|---|---|
| ir | A directory path (one string) | docs/ir |
| decisions.records | A directory path (one string). Decisions in the files under it can be cited as sources. Markdown that is not a decision record (form contracts, supplementary documents) can also be placed there | docs/decision/records |
| decisions.adr | A directory path (one string) | docs/decision/adr |
| tests.files | A list of globs | src/\*\*/\*.rs, tests/\*\*/\*.rs |
| guides.files | A list of globs. The location of the `guide` files (guides.md) | An empty list |
| tests.rust.attributes | A list of attribute paths added to "#[test]" | An empty list |
| tests.rust.macros | A list of macro names | An empty list |
| tests.rules | A list of paths of ast-grep rule YAML files. Relative to the base directory; globs cannot be used (query-rules.md) | An empty list |
| mutants.equivalents | A file path (one string). Points to the list of equivalents | None (without the key the list of equivalents has 0 entries) |
| surface.files | A list of globs. The location of each `surface file` (surface.md) | An empty list |
| surface.rules | A list of paths of ast-grep rule YAML files. Relative to the base directory; globs cannot be used. If not an empty list, the surface check is performed (surface.md) | An empty list |
| surface.unspecified | A file path (one string). Points to the list of unspecified surfaces (surface-unspecified.md) | None (without the key the list of unspecified surfaces has 0 entries) |
| limits.lines | A number (negative numbers and 0 are not allowed) | 200 |
| limits.requirements | A number (negative numbers and 0 are not allowed) | 10 |
| vague_words | A list of words | The four words 「適切に」「必要に応じて」「通常は」「など」 |
| changes.files | A list of relative globs for the targets of change conformance. Required and non-empty when changes is written | When changes is omitted, there is no check of conformance records |
| changes.exclude | A list of relative globs removed from the targets of change conformance | An empty list |
| changes.records | A list of relative globs for conformance records. Required and non-empty when changes is written | When changes is omitted, there is no check of conformance records |
| overview.files | A list of globs. The location of the `overview data` (overview-data.md). Required when overview is written | When overview is omitted, the `overview data` is not read |
| overview.toc | A file path (one string). Points to the `table of contents` (overview-toc.md). Required when overview is written | When overview is omitted, the `table of contents` is not read |
| languages | A list of language tags. The `language list` (translation-pairs.md) | None (treated as a `language list` of only "en") |
| labels | A map from language tags to maps from `UI text` keys to strings (overview-languages.md) | None (kotowari holds the English `UI text`) |

## Examples

```gherkin
@id=EX-core-003 @about=REQ-core-014 @source=docs/decision/records/records.md#A12,docs/decision/records/records.md#A20,docs/decision/records/records.md#A41
Scenario: An unknown key stops the run
  Given the configuration file has an unknown key "limit:"
  When "kotowari check" is run
  Then the exit code is 2
@id=EX-core-383 @about=REQ-core-014 @source=docs/decision/records/2026-09-24-guide-gaps.md#A7,docs/decision/records/ir-form.md#出力
Scenario: When a duplicate key stops the run, that key is shown in one line
  Given the `configuration file` has the line "ir: docs/ir" twice
  When "kotowari check" is run
  Then the exit code is 2 and standard error is one line that starts with "config error: " and contains "duplicate key: ir"

@id=EX-core-544 @about=REQ-core-014 @source=docs/decision/records/2026-10-06-todo-zero.md#A3,docs/decision/records/records.md#A20,docs/decision/records/ir-form.md#出力
Scenario: A glob element starting with "/" stops as a configuration error
  Given the "tests.files" of the `configuration file` has the element "/tests/**/*.rs"
  When "kotowari check" is run
  Then the exit code is 2 and standard error starts with "config error: "

@id=EX-core-545 @about=REQ-core-019 @source=docs/decision/records/2026-10-06-todo-zero.md#A4
Scenario: A hidden directory named only inside braces is not entered
  Given "changes.records" is "{.records,other}/*.yaml", and ".records/a.yaml" exists
  When "kotowari check" is run
  Then ".records/a.yaml" is not read as a conformance record
```
