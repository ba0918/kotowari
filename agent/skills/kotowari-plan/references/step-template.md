# Step template

The plan's form is checked by `kotowari plan <plan path>` against the schema bundled in
kotowari. A whole plan that passes it is in [plan-example.md](plan-example.md); copy its shape.

The steps go under one `## Steps` section, after the plan-level sections (listed at the end of
this file). Each step is a `### S<number>: <what this step produces>` heading followed by these
eight field lines, in this order, each exactly once. The order is the order an implementer needs
them.

```markdown
### S1: <what this step produces>

- Purpose: <one sentence>
- Specification: <path>#REQ-nnn, <path>#REQ-nnn
- Prerequisites: <steps that must be complete; environment or data that must exist>
- May change: <files or directories; nothing outside this scope>
- Done when: <observable condition>
- Shown by: test | check | artifact | external — <test names / commands / artifact path / what to observe and where>
- Left to the implementer: <choices where every option keeps approved behavior> (or "none")
- Stop and hand back if: <conditions specific to this step, beyond the four general ones>
```

Form rules the check enforces:

- Write every field as a list line `- Name: value` (`*` or `+` also work) and keep its value,
  commands included, on that one line. A line right after it, or an indented line after a blank
  line, is not part of the value; it is a finding.
- Nothing else goes under a step: no prose, no other list, no table, and nothing between
  `## Steps` and the first step.
- **Shown by** starts with one of the four words `test`, `check`, `artifact`, `external`,
  followed by a space or nothing.
- The step number is `S` and digits; gaps, repeats and the name after `:` are not checked.
- Do not put a frontmatter block at the top of the plan. The check skips one unread, but the plan
  does not name its schema.

Guidance per field:

- **Specification** names requirement IDs as `<document path>#REQ-nnn` (a topic with no IR:
  `<path>#<heading>`), not paraphrases. If a requirement you need does not exist, the
  specification is missing something — hand back to brainstorm rather than inventing the content.
- **Done when** is a condition someone else can observe, not "the feature works".
- **Shown by** picks exactly one kind. *Test* means RED → GREEN → REFACTOR with named tests.
  *Check* lists commands in order. *Artifact* names the file and any format check. *External*
  says what to observe, on what, and what counts as pass; if it is unsafe, privileged, or
  irreversible, say that a human runs or confirms it. Name only tests that meet **Evidence
  conditions**. Do not add tests for conditions already true or match their count to the
  number of Done conditions; use one test per behavior being implemented. If no test qualifies
  and the specification does not require human or platform inspection, hand back to brainstorm.
- **Left to the implementer** holds choices delegated for this step; plan-wide ones go in the
  plan-level section. Naming, internal
  structure, and helper extraction usually qualify; input formats, limits, error behavior, and
  persistence never do.
- **Stop and hand back if** names conditions the implementer could not infer: a dependency that
  may be unavailable, a measurement that may disagree with the specification, an interface that
  may already exist under another name.

Plan-level sections, each exactly once beside `## Steps` (their order is not checked):
**Goal** (one sentence, the result the person gets), **Specification** (the IR store path and
the requirement IDs, or for a topic with no IR the one governing path), **Approach and why**,
**Scope of change**, **Step order and prerequisites**, **Verification map** (which steps prove
which requirements and scenarios), **Left to the implementer**, **Stop conditions**, **Test
command** (optional: only when the project does not fix one), **Out of scope**. No other `## `
section. Under them write prose, `-` lists with their child lists, tables and code blocks;
numbered lists and `### ` or deeper headings are findings. Nothing but blank lines goes between
the title and the first `## ` section.

## Evidence conditions

An oracle — a test, a check, or a fixture — counts as evidence only when the condition it
produces has a named operational producer in a supported environment (untrusted input arriving
at a boundary is one), its subject is the product or a check rather than the oracle itself, the
rule it enforces is stated by the specification, and every wording, file layout, or internal
name it pins is declared there as a contract. An oracle that fails any of these is a cost: do
not add it, keep it in a change under review, or demand it.

A requirement whose only oracle would fail these conditions is not mechanically verifiable:
when it is not code, verify it by a human-run check or by the platform's own checker; when it
is code, drop the requirement and let the failure join a generic error path a reachable
failure already proves — never resolve it by having the implementer build the fixture.

Source: `ba0918-verification`, agentic-rules v0.8.0.
