# List of equivalents

English | [日本語](equivalents.ja.md)

Covers the location and form of the `list of equivalents`, how it is matched against each `mutation outcome`, and the `finding` emitted for an entry of the list.

## Requirements

### REQ-core-141: Matching an entry of the list

- kind: ubiquitous
- source: docs/decision/records/2026-09-17-mutation-tests.md#A9, docs/decision/records/2026-09-17-mutation-tests.md#A15, docs/decision/records/2026-09-17-mutation-tests.md#A33, docs/decision/records/2026-09-17-mutation-tests.md#A37, docs/decision/records/2026-09-17-mutation-tests.md#A48, docs/decision/records/2026-09-17-mutation-tests.md#A51, docs/decision/records/2026-09-17-mutation-tests.md#A52, docs/decision/records/2026-09-17-mutation-tests.md#A34, docs/decision/records/2026-09-17-mutation-tests.md#A44
- verification: unit

kotowari always treats a `mutation outcome` whose result is "missed" and a well-formed entry of the `list of equivalents` as matching when "file", after the normalisation of REQ-core-110, is the same string as the file of the `mutation outcome`, "change" is the same string as the change description, and "text" is the same string as the wording of that line in the current content of the file of the `mutation outcome`, both with leading and trailing half-width spaces and tabs removed. Lines are split as in TBL-core-010, and the "\r" of a "\r\n" at the end of a line is not included in the wording. When the file of the `mutation outcome` does not exist, cannot be read, is not UTF-8, or the line exceeds the number of lines in that file, kotowari does not `stop`, and that `mutation outcome` matches no entry. In a file with several lines of the same wording, one entry matches the `mutation outcome` of any of those lines.

### REQ-core-142: An entry whose wording is gone

- kind: event_driven
- source: docs/decision/records/2026-09-17-mutation-tests.md#A20, docs/decision/records/2026-09-17-mutation-tests.md#A31, docs/decision/records/2026-09-17-mutation-tests.md#A40, docs/decision/records/2026-09-17-mutation-tests.md#A48, docs/decision/records/2026-09-17-mutation-tests.md#A52, docs/decision/records/2026-09-17-mutation-tests.md#A53, docs/decision/records/2026-09-17-mutation-tests.md#A57
- verification: unit

In "kotowari mutants", when the file of "file" has no line whose wording is the same as the "text" of a well-formed entry of the `list of equivalents`, both with leading and trailing half-width spaces and tabs removed (including when the file does not exist, cannot be read, or is not UTF-8), kotowari emits, for each such entry, an equivalent_stale `notice` whose "path" is the file of the `list of equivalents`, whose "line" is null, and whose detail is "file" and "change", exactly as written in the list, joined by ": ". Whether that `mutation` appears in the outcomes file is not looked at. It is not emitted for a malformed entry.

### REQ-core-143: A malformed entry

- kind: event_driven
- source: docs/decision/records/2026-09-17-mutation-tests.md#A16, docs/decision/records/2026-09-17-mutation-tests.md#A17, docs/decision/records/2026-09-17-mutation-tests.md#A31, docs/decision/records/2026-09-17-mutation-tests.md#A34, docs/decision/records/2026-09-17-mutation-tests.md#A37, docs/decision/records/2026-09-17-mutation-tests.md#A40, docs/decision/records/2026-09-17-mutation-tests.md#A44, docs/decision/records/2026-09-17-mutation-tests.md#A51, docs/decision/records/2026-09-17-mutation-tests.md#A53, docs/decision/records/2026-09-17-mutation-tests.md#A57
- verification: unit

In "kotowari mutants", when an entry of the `list of equivalents` is not a mapping of keys and values, lacks any of the keys "file", "change", "text", "class" and "why", has a key other than these five, has a value that is not a string, has a "why" that is empty after removing leading and trailing half-width spaces and tabs, has a "class" that is not "equivalent", or has a "file" that is an absolute path or contains a ".." element, kotowari emits, for each such entry, an equivalent_invalid `error` whose "path" is the file of the `list of equivalents`, whose "line" is null, and whose detail is "file" and "change", exactly as written in the list, joined by ": ", and that entry matches no `mutation outcome`. In the detail, "file" or "change" is the empty string if it is absent or not a string. Two or more entries with the same content are not in themselves checked.

### REQ-core-148: Location of the list

- kind: event_driven
- source: docs/decision/records/2026-09-17-mutation-tests.md#A16, docs/decision/records/2026-09-17-mutation-tests.md#A34, docs/decision/records/2026-09-17-mutation-tests.md#A36, docs/decision/records/2026-09-17-mutation-tests.md#A44, docs/decision/records/2026-09-17-mutation-tests.md#A45, docs/decision/records/2026-09-17-mutation-tests.md#A49, docs/decision/records/2026-09-17-mutation-tests.md#A55
- verification: unit

When the configuration has no "mutants.equivalents" key, or its target is empty (0 bytes or only comments), kotowari continues "kotowari mutants" with a `list of equivalents` of 0 entries. When the target of the key does not exist or cannot be read, kotowari will `stop` on the grounds of an unreadable file. When the target is not UTF-8, kotowari will `stop` on the grounds of a file that is not UTF-8. When the target cannot be read as YAML, or its top level is not a sequence, kotowari will `stop` on the grounds of a configuration error and puts the relative path of the file of the `list of equivalents` in the detail. "kotowari check" only checks the value of the key as in REQ-core-014; it does not read the target and does not look at whether it exists.

## Examples

```gherkin
@id=EX-core-211 @about=REQ-core-139,REQ-core-141,REQ-core-145 @source=docs/decision/records/2026-09-17-mutation-tests.md#A9,docs/decision/records/2026-09-17-mutation-tests.md#A11,docs/decision/records/2026-09-17-mutation-tests.md#A15,docs/decision/records/2026-09-17-mutation-tests.md#A31,docs/decision/records/2026-09-17-mutation-tests.md#A37,docs/decision/records/2026-09-17-mutation-tests.md#A38,docs/decision/records/2026-09-17-mutation-tests.md#A52
Scenario: A miss on the list is not a finding and counts as equivalent
  Given line 3 of "src/a.rs" is "    if a == b {", and the outcomes file has a `miss` on line 3 of "src/a.rs" whose change description is "replace == with != in f"
  And the `list of equivalents` has an entry whose "file" is "src/a.rs", "change" is "replace == with != in f", "text" is "if a == b {", "class" is "equivalent", and "why" is not empty
  When "kotowari mutants --tool cargo-mutants outcomes.json" is run
  Then no mutant_survived error is emitted, and "survived" of "mutants" is 0 and "equivalent" is 1
  And the exit code is 0

@id=EX-core-212 @about=REQ-core-141 @source=docs/decision/records/2026-09-17-mutation-tests.md#A11,docs/decision/records/2026-09-17-mutation-tests.md#A15,docs/decision/records/2026-09-17-mutation-tests.md#A31
Scenario: If the line only moved, it still matches
  Given there is the `list of equivalents` of EX-core-211, the line "    if a == b {" of "src/a.rs" has moved to line 7, and the line of the `miss` in the outcomes file is also 7
  When "kotowari mutants --tool cargo-mutants outcomes.json" is run
  Then no mutant_survived error is emitted

@id=EX-core-213 @about=REQ-core-141,REQ-core-142 @source=docs/decision/records/2026-09-17-mutation-tests.md#A11,docs/decision/records/2026-09-17-mutation-tests.md#A15,docs/decision/records/2026-09-17-mutation-tests.md#A20,docs/decision/records/2026-09-17-mutation-tests.md#A31,docs/decision/records/2026-09-17-mutation-tests.md#A40
Scenario: If the line is rewritten, it does not match and the entry becomes stale
  Given there is the `list of equivalents` of EX-core-211, line 3 of "src/a.rs" has changed to "    if a == c {", there is no line "if a == b {" anywhere, and the outcomes file has a `miss` on line 3 with the same change description
  When "kotowari mutants --tool cargo-mutants outcomes.json" is run
  Then a mutant_survived error is emitted on line 3 of "src/a.rs"
  And an equivalent_stale notice is emitted whose "path" is the file of the `list of equivalents`, whose "line" is null, and whose detail is "src/a.rs: replace == with != in f"

@id=EX-core-214 @about=REQ-core-141 @source=docs/decision/records/2026-09-17-mutation-tests.md#A11,docs/decision/records/2026-09-17-mutation-tests.md#A31,docs/decision/records/2026-09-17-mutation-tests.md#A33,docs/decision/records/2026-09-17-mutation-tests.md#A40
Scenario: A miss whose line exceeds the file's line count does not match and becomes a finding
  Given "src/a.rs" has 5 lines, and the outcomes file has a `miss` on line 9 of "src/a.rs"
  When "kotowari mutants --tool cargo-mutants outcomes.json" is run
  Then a mutant_survived error whose "line" is 9 is emitted
  And the exit code is 1

@id=EX-core-215 @about=REQ-core-143 @source=docs/decision/records/2026-09-17-mutation-tests.md#A11,docs/decision/records/2026-09-17-mutation-tests.md#A16,docs/decision/records/2026-09-17-mutation-tests.md#A31,docs/decision/records/2026-09-17-mutation-tests.md#A34,docs/decision/records/2026-09-17-mutation-tests.md#A40,docs/decision/records/2026-09-17-mutation-tests.md#A53
Scenario: An entry whose reason is only whitespace is an error and does not remove the miss
  Given in the situation of EX-core-211, the "why" of the entry of the `list of equivalents` is only half-width spaces
  When "kotowari mutants --tool cargo-mutants outcomes.json" is run
  Then an equivalent_invalid error with detail "src/a.rs: replace == with != in f" is emitted
  And a mutant_survived error is emitted on line 3 of "src/a.rs"
  And no equivalent_stale notice is emitted

@id=EX-core-216 @about=REQ-core-143 @source=docs/decision/records/2026-09-17-mutation-tests.md#A17,docs/decision/records/2026-09-17-mutation-tests.md#A31,docs/decision/records/2026-09-17-mutation-tests.md#A34
Scenario: A class other than equivalent cannot be written
  Given the "class" of an entry of the `list of equivalents` is "untested"
  When "kotowari mutants --tool cargo-mutants outcomes.json" is run
  Then an equivalent_invalid error is emitted

@id=EX-core-217 @about=REQ-core-148 @source=docs/decision/records/2026-09-17-mutation-tests.md#A34,docs/decision/records/2026-09-17-mutation-tests.md#A36
Scenario: The run stops when the target of the list does not exist
  Given "mutants.equivalents" in the configuration is "docs/equivalents.yaml", and that file does not exist
  When "kotowari mutants --tool cargo-mutants outcomes.json" is run
  Then the exit code is 2
  And the first line of standard error starts with "unreadable file: "

@id=EX-core-220 @about=REQ-core-143 @source=docs/decision/records/2026-09-17-mutation-tests.md#A31,docs/decision/records/2026-09-17-mutation-tests.md#A44
Scenario: For an entry without file, the first half of the detail is empty
  Given an entry of the `list of equivalents` has no "file" key, and its "change" is "replace f with ()"
  When "kotowari mutants --tool cargo-mutants outcomes.json" is run
  Then an equivalent_invalid error with detail ": replace f with ()" is emitted

@id=EX-core-221 @about=REQ-core-148 @source=docs/decision/records/2026-09-17-mutation-tests.md#A45
Scenario: check does not stop even if the target of the list does not exist
  Given "mutants.equivalents" in the configuration is "docs/equivalents.yaml", and that file does not exist
  When "kotowari check" is run
  Then the exit code is not 2

@id=EX-core-232 @about=REQ-core-141 @source=docs/decision/records/2026-09-17-mutation-tests.md#A11,docs/decision/records/2026-09-17-mutation-tests.md#A15,docs/decision/records/2026-09-17-mutation-tests.md#A31,docs/decision/records/2026-09-17-mutation-tests.md#A38
Scenario: If two lines have the same wording, one entry applies to both
  Given lines 3 and 8 of "src/a.rs" are both "    if a == b {", the outcomes file has a `miss` with the same change description on both lines, and there is the `list of equivalents` of EX-core-211
  When "kotowari mutants --tool cargo-mutants outcomes.json" is run
  Then no mutant_survived error is emitted, and "equivalent" of "mutants" is 2

@id=EX-core-233 @about=REQ-core-141 @source=docs/decision/records/2026-09-17-mutation-tests.md#A11,docs/decision/records/2026-09-17-mutation-tests.md#A15,docs/decision/records/2026-09-17-mutation-tests.md#A31,docs/decision/records/2026-09-17-mutation-tests.md#A37
Scenario: An entry with a different file does not match
  Given in the situation of EX-core-211, the "file" of the entry of the `list of equivalents` is "src/b.rs"
  When "kotowari mutants --tool cargo-mutants outcomes.json" is run
  Then a mutant_survived error is emitted on line 3 of "src/a.rs"

@id=EX-core-234 @about=REQ-core-141 @source=docs/decision/records/2026-09-17-mutation-tests.md#A15,docs/decision/records/2026-09-17-mutation-tests.md#A51,docs/decision/records/2026-09-17-mutation-tests.md#A52
Scenario: A path written differently and tab indentation still match
  Given in the situation of EX-core-211, the "file" of the entry of the `list of equivalents` is "./src/a.rs", and line 3 of "src/a.rs" is indented with a tab and ends with "\r\n"
  When "kotowari mutants --tool cargo-mutants outcomes.json" is run
  Then no mutant_survived error is emitted

@id=EX-core-235 @about=REQ-core-141,REQ-core-142 @source=docs/decision/records/2026-09-17-mutation-tests.md#A11,docs/decision/records/2026-09-17-mutation-tests.md#A20,docs/decision/records/2026-09-17-mutation-tests.md#A31,docs/decision/records/2026-09-17-mutation-tests.md#A48
Scenario: A source that is not UTF-8 does not stop the run and falls on the non-matching side
  Given in the situation of EX-core-211, "src/a.rs" is not UTF-8
  When "kotowari mutants --tool cargo-mutants outcomes.json" is run
  Then the exit code is 1, and a mutant_survived error and an equivalent_stale notice are emitted

@id=EX-core-236 @about=REQ-core-143 @source=docs/decision/records/2026-09-17-mutation-tests.md#A31,docs/decision/records/2026-09-17-mutation-tests.md#A51
Scenario: An entry pointing outside the base directory is an error
  Given the "file" of an entry of the `list of equivalents` is "../x/src/a.rs"
  When "kotowari mutants --tool cargo-mutants outcomes.json" is run
  Then an equivalent_invalid error is emitted

@id=EX-core-237 @about=REQ-core-143 @source=docs/decision/records/2026-09-17-mutation-tests.md#A31,docs/decision/records/2026-09-17-mutation-tests.md#A34,docs/decision/records/2026-09-17-mutation-tests.md#A53
Scenario: Two malformed entries with the same content give two errors
  Given the `list of equivalents` has two entries with the same content and an empty "why"
  When "kotowari mutants --tool cargo-mutants outcomes.json" is run
  Then two equivalent_invalid errors are emitted

@id=EX-core-238 @about=REQ-core-148 @source=docs/decision/records/2026-09-17-mutation-tests.md#A16,docs/decision/records/2026-09-17-mutation-tests.md#A36,docs/decision/records/2026-09-17-mutation-tests.md#A49
Scenario: An empty list continues as 0 entries
  Given the target of "mutants.equivalents" in the configuration is 0 bytes, and the outcomes file has one entry whose "summary" is "CaughtMutant"
  When "kotowari mutants --tool cargo-mutants outcomes.json" is run
  Then the exit code is 0

@id=EX-core-239 @about=REQ-core-148 @source=docs/decision/records/2026-09-17-mutation-tests.md#A34,docs/decision/records/2026-09-17-mutation-tests.md#A44,docs/decision/records/2026-09-17-mutation-tests.md#A49
Scenario: A list whose top level is not a sequence stops the run with the list's path
  Given "mutants.equivalents" in the configuration is "docs/equivalents.yaml", and its content is the single line "file: src/a.rs"
  When "kotowari mutants --tool cargo-mutants outcomes.json" is run
  Then the exit code is 2
  And the first line of standard error starts with "config error: docs/equivalents.yaml"

@id=EX-core-243 @about=REQ-core-142 @source=docs/decision/records/2026-09-17-mutation-tests.md#A20,docs/decision/records/2026-09-17-mutation-tests.md#A31,docs/decision/records/2026-09-17-mutation-tests.md#A40,docs/decision/records/2026-09-17-mutation-tests.md#A57
Scenario: The detail of a stale entry uses the path as written
  Given the `list of equivalents` has a well-formed entry whose "file" is "./src/a.rs" and "change" is "replace f with ()", and the line of its "text" is not in "src/a.rs"
  When "kotowari mutants --tool cargo-mutants outcomes.json" is run
  Then an equivalent_stale notice with detail "./src/a.rs: replace f with ()" is emitted
```
