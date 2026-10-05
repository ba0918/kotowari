# The skill references agreeing with the binary

English | [日本語](skill-references.ja.md)

Covers the check that the values and forms of the binary copied into the references of the skills under "agent/skills/" (the documents of "agent/skills/kotowari/references/" and "agent/skills/kotowari-plan/references/") agree with the binary. This check is not a behaviour of kotowari; the tests of this repository perform it. What they are compared with is the values held by the binary's code, not the tables of the IR.

## Requirements

### REQ-core-125: Agreement of the finding kinds

- kind: ubiquitous
- source: docs/decision/records/2026-09-17-check-reach.md#A6, docs/decision/records/2026-09-17-check-reach.md#A7, docs/decision/records/2026-09-23-ir-english-tokens.md#A6, docs/decision/records/2026-09-23-skill-distribution.md#A1
- verification: unit

The tests of this repository always confirm, for the table in "agent/skills/kotowari/references/findings.md" whose header's first column is "Kind", that the set of first columns excluding the header and separator rows equals the set of kinds of `finding` the binary's code raises.

### REQ-core-126: Agreement of the defaults

- kind: ubiquitous
- source: docs/decision/records/2026-09-17-check-reach.md#A6, docs/decision/records/2026-09-17-check-reach.md#A7, docs/decision/records/2026-09-17-check-reach.md#A11, docs/decision/records/2026-09-17-check-reach.md#A27, docs/decision/records/2026-09-17-mutation-tests.md#A59, docs/decision/records/2026-09-23-skill-distribution.md#A1
- verification: unit

The tests of this repository always confirm that the YAML code block in step 1 of setup in "agent/skills/kotowari/references/config.md" contains every key of `TBL-core-004` that has a default, and that the result of reading that block with the binary's code as a `configuration file` equals the default values of the configuration held by the binary's code.

### REQ-core-127: Agreement of the stop wording

- kind: ubiquitous
- source: docs/decision/records/2026-09-17-check-reach.md#A6, docs/decision/records/2026-09-17-check-reach.md#A7, docs/decision/records/2026-09-17-check-reach.md#A12, docs/decision/records/2026-09-23-ir-english-tokens.md#A6, docs/decision/records/2026-09-23-skill-distribution.md#A1
- verification: unit

The tests of this repository always confirm, for the table in "agent/skills/kotowari/references/findings.md" whose header's first column is "Message", that the set of first columns excluding the header and separator rows equals the set of wordings the binary's code writes on the first line of standard error as the reason of a `stop`.

### REQ-core-195: Agreement of the example plan

- kind: ubiquitous
- source: docs/decision/records/2026-09-24-plan-schema.md#A13, docs/decision/records/2026-09-23-skill-distribution.md#A1, docs/decision/records/2026-09-24-plan-schema.md#A34
- verification: unit

The tests of this repository always confirm that when "agent/skills/kotowari-plan/references/plan-example.md" is read with the binary's code as a `plan`, not a single `finding` is raised.
