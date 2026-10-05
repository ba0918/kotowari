# The workflow that makes the overview

English | [日本語](overview-workflow.ja.md)

Covers the steps by which the kotowari skills under "agent/skills/" make or revise an `overview` at a brainstorm approval and show it. This document holds only requirements that a person or an LLM verifies by reading the text of the skills; the behaviour of kotowari itself is covered by overview-data.md and overview-commands.md.

## Requirements

### REQ-core-300: Making and showing the overview at approval

- kind: ubiquitous
- source: docs/decision/records/2026-10-02-whole-picture.md#A2, docs/decision/records/2026-10-02-whole-picture.md#A8, docs/decision/records/2026-10-02-whole-picture.md#A34, docs/decision/records/2026-10-02-whole-picture.md#A52, docs/decision/records/2026-10-02-whole-picture.md#A84, docs/decision/records/2026-10-05-overview-index.md#A19
- verification: review
- how_to_verify: Read the approval steps of "agent/skills/kotowari-brainstorm/SKILL.md" and confirm that, after check and the collation review, they say in this order: decide the `overview` entries involved by the "ir" lists; make or revise the `overview data`; revise the `table of contents`; run "kotowari overview build" and fix the errors; start serve and show the URL for reference, or show the command if it cannot be started in the background; and confirm that they do not change what is approved or the material for the approval

kotowari-brainstorm always, at the approval in a project that has the "overview" key, after the current check and the collation review, decides from each `topic document` of the `IR` that was changed the `overview` entries involved, by the "ir" list of the `overview data`, makes or revises that `overview data`, revises the `table of contents`, runs "kotowari overview build" and fixes the errors, starts "kotowari overview serve" and shows the URL for reference. The `overview` is not made something to approve, and whether to look at it is left to the person.

### REQ-core-301: Deciding the unit of an overview

- kind: ubiquitous
- source: docs/decision/records/2026-10-02-whole-picture.md#A18, docs/decision/records/2026-10-02-whole-picture.md#A20, docs/decision/records/2026-10-02-whole-picture.md#A35
- verification: review
- how_to_verify: Read "agent/skills/kotowari-brainstorm/SKILL.md" or the references of the kotowari skills and confirm that they say: when a `topic document` that is in no "ir" of any `overview data` is changed, propose either adding it to an existing `overview` or making a new one, and let the person decide; the guide for the unit is something the user can call by name as one feature, and something cross-cutting is not made one on its own; a decided unit is not changed unless a brainstorm decides to; and an `overview` for topics not touched is not made all at once

The kotowari skills always, when a brainstorm changes a `topic document` that is in no "ir" of any `overview data`, propose either adding it to an existing `overview` or making a new one and let the person decide, make the unit something the user can call by name as one feature, do not change a decided unit unless a brainstorm decides to, and do not make an `overview` for topics the brainstorm did not touch.

### REQ-core-302: How to write the data

- kind: ubiquitous
- source: docs/decision/records/2026-10-02-whole-picture.md#A1, docs/decision/records/2026-10-02-whole-picture.md#A6, docs/decision/records/2026-10-02-whole-picture.md#A7, docs/decision/records/2026-10-02-whole-picture.md#A8, docs/decision/records/2026-10-02-whole-picture.md#A9, docs/decision/records/2026-10-02-whole-picture.md#A14, docs/decision/records/2026-10-02-whole-picture.md#A15, docs/decision/records/2026-10-02-whole-picture.md#A54, docs/decision/records/2026-10-02-whole-picture.md#A38, docs/decision/records/2026-10-02-whole-picture.md#A66, docs/decision/records/2026-10-02-whole-picture.md#A53, docs/decision/records/2026-10-05-guide-overview-roles.md#A4
- verification: review
- how_to_verify: Confirm that the references of the kotowari skills have a scene for writing the `overview data`, and that it says: put a lead with the conclusion and a summary at the opening; choose the kinds and order of each `part` to fit the subject; show both what the result looks like and how the decisions connect; show the current state and what is planned under `deferral`; write by revising the previous `overview data`; put a `guide mark` next to each section heading; and write the references of a `part` in the "refs" or "ref" field

The kotowari skills always have a scene for writing the `overview data`, and in that scene require putting the conclusion and a summary at the opening, choosing and ordering each `part` of diagrams, tables and decoration to fit the subject, showing both what the result looks like and how the decisions connect, as well as the current state and the plans, writing by revising the previous `overview data`, and putting a `guide mark` on each section.

### REQ-core-303: cycle does not revise the overview

- kind: ubiquitous
- source: docs/decision/records/2026-10-02-whole-picture.md#A39
- verification: review
- how_to_verify: Read "agent/skills/kotowari-cycle/" and "agent/skills/kotowari-implement/" and confirm that there is no step that revises the `overview data`, and that the references of the kotowari skills say that guide_stale on the `overview data` is fixed by the next brainstorm that touches that topic

The kotowari skills always leave the `overview data` unrevised in the cycle and implementation stations, and fix guide_stale on the `overview data` in the next brainstorm that touches that topic.

### REQ-core-333: How the table of contents is arranged

- kind: ubiquitous
- source: docs/decision/records/2026-10-05-overview-index.md#A4, docs/decision/records/2026-10-05-overview-index.md#A6, docs/decision/records/2026-10-05-overview-index.md#A19
- verification: review
- how_to_verify: Read the references of the kotowari skills and confirm that they say: at approval, the LLM decides where in the `table of contents` to place a new `overview` and whether to split a `contents group` or make a new one, and revises the `table of contents` without asking the person; the `table of contents` is not something to approve; a `contents group` is split only once it has grown large; and within one level, entries are ordered in an order meaningful to the reader (the order of use or of the workflow)

The kotowari skills always require that, at approval, the LLM decides where in the `table of contents` to place a new `overview` and whether to split a `contents group` or make a new one, and revises the `table of contents`; they do not make the `table of contents` something to approve, split a `contents group` only once it has grown large, and require ordering within one level in an order meaningful to the reader.
