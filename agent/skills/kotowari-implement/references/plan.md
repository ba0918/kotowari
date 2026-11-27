# The form of a plan

A plan is one Markdown file per branch, `docs/plans/<branch>.md`. `kotowari plan <path>` checks
its form against the schema bundled in kotowari and reports each departure as invalid_plan (the
kotowari skill's `references/findings.md`). The plan is in form when that command exits with 0.

## Sections

One `# ` title, with only blank lines between it and the first `## ` section. Then each of these
`## ` sections exactly once, in any order, and no other `## ` section:

`## Goal`, `## Specification`, `## Approach and why`, `## Scope of change`,
`## Step order and prerequisites`, `## Verification map`, `## Left to the implementer`,
`## Stop conditions`, `## Out of scope`, `## Steps`, and `## Test command` zero times or once.

Under every section but `## Steps` write sentences, `-` lists with their child lists, tables and
code blocks. A numbered list and a `### ` or deeper heading there are findings.

## Steps

`## Steps` holds only steps, with nothing before the first. Each step is a heading
`### S<digits>: <what this step produces>` followed by these eight field lines, each once, in this
order, as `- Name: value` (`*` or `+` also work), each value on its one line:

```markdown
### S1: <what this step produces>

- Purpose: <one sentence>
- Specification: <path>#<anchor> of the decisions, and the row IDs this step meets
- Prerequisites: <steps that must be done first, or none>
- May change: <files or directories>
- Done when: <a condition someone else can observe>
- Shown by: test | check | artifact | external — <test names, commands, artifact path, or what to observe>
- Left to the implementer: <choices left open, or none>
- Stop and hand back if: <conditions, or none>
```

Nothing else goes under a step: no prose, no other list, no table, no continuation line.
`Shown by` starts with one of the words `test`, `check`, `artifact` or `external`, followed by a
space or nothing. Gaps or repeats in step numbers and the name after `:` are not checked, nor is
what a field says.

## What a plan is for here

A plan holds the procedure so that an implementer who takes over can find what remains. It holds
no progress: no step is marked done. Its `## Stop conditions` section and `Stop and hand back if`
fields add no condition for stopping, its `## Test command` section does not decide the test
command, and `May change` does not bind the files you touch (the kotowari-implement skill's
**Plans**). Write them all the same, since the form requires them.
