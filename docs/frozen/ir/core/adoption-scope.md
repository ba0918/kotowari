# Adoption scope and how rows are made

English | [日本語](adoption-scope.ja.md)

Covers how kotowari-adopt decides the range of topics handled in one adoption, and how it reads the `legacy material`, the implementation and the tests to make the rows of the list. Where a row goes is covered by TBL-core-037 in adoption.md, and how many times the person is asked is covered by REQ-core-219 in adoption.md. This document holds only requirements checked by a person or an LLM reading the skill's text.

## Requirements

### REQ-core-215: Adoption scope and preparation

- kind: ubiquitous
- source: docs/decision/records/2026-09-26-adoption.md#A2, docs/decision/records/2026-09-26-adoption.md#A19, docs/decision/records/2026-09-26-adoption.md#A23, docs/decision/records/2026-09-26-adoption.md#A27, docs/decision/records/2026-09-26-adoption.md#A28, docs/decision/records/2026-09-26-adoption.md#A33, docs/decision/records/2026-09-27-adopt-scope.md#A1, docs/decision/records/2026-09-27-adopt-scope.md#A5
- verification: review
- how_to_verify: Read "agent/skills/kotowari-adopt/" and confirm that all of the following are written. First check whether ".kotowari/config.yaml" exists, and if not, name the setup steps in the kotowari skill's "references/config.md" and do them first. One run handles one topic, chosen by the person in the scope round, with the feature planned to change next as the default. In the same round, have the person confirm the topic's user entry points (the commands and subcommands the user touches, the screen operations), the proposed code files and test files of the scope, and the candidate `legacy material` to read. Reading and sorting are done by the main session; if the topic is too large to read in one go, do not have another agent read it, but propose to the person in the scope round to split the topic smaller

kotowari-adopt always confirms that the configuration exists, then has the person confirm in the scope round one topic, its user entry points, the code files and test files of the scope, and the `legacy material` to read, and reads and sorts them in the main session.

### REQ-core-237: Range of behaviors made into rows

- kind: ubiquitous
- source: docs/decision/records/2026-09-27-adopt-scope.md#A1, docs/decision/records/2026-09-27-adopt-scope.md#A2
- verification: review
- how_to_verify: Read "agent/skills/kotowari-adopt/" and confirm that all of the following are written. Only behaviors observable through the topic's user entry points become rows, and a behavior observed through another entry point is outside the scope even when it shares the same code. Files outside the scope may be read for understanding, but a behavior outside the scope is not made into a row and is not written in the `decision record` either. In the list round, show only the number of behaviors found outside the scope and the names of the candidate next topics

kotowari-adopt always makes rows only of the behaviors observable through the topic's user entry points, and for behaviors found outside the scope shows only their number and the names of the candidate next topics.

### REQ-core-238: Granularity of a row

- kind: ubiquitous
- source: docs/decision/records/2026-09-27-adopt-scope.md#A3
- verification: review
- how_to_verify: Read "agent/skills/kotowari-adopt/" and confirm it is written that one row of the list is one candidate `requirement`, that things under the same rule that differ only in a value or a wording are put together in one row as a candidate to be written as a `decision table`, and that the table's contents are listed briefly in the list's column for the implementation's behavior

kotowari-adopt always makes one row of the list one candidate `requirement`, and puts together in one row the things that differ only in a value or a wording.

### REQ-core-239: Review when there are many rows

- kind: event_driven
- source: docs/decision/records/2026-09-27-adopt-scope.md#A4
- verification: review
- how_to_verify: Read "agent/skills/kotowari-adopt/" and confirm it is written that when the number of candidate `requirement` items exceeds "limits.requirements" in the configuration, kotowari-adopt checks whether user entry points are mixed; if they are, it proposes within the list round to split the topic and has the person choose the side for this run; if they are not, it does not split because of the number, and splits the documents by responsibility at the stage of writing the `IR`

When the number of candidate `requirement` items exceeds "limits.requirements", kotowari-adopt checks whether user entry points are mixed, and if they are, proposes within the list round to split the topic.
