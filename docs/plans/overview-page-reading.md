# Plan: make the overview document pages easier to read

## Goal

A person reading an overview document page sees an outline of its sections, which stays beside the text on a wide screen and sits under the title on a narrow one, marks the sections not reviewed since the IR changed, and links to each section; flow parts read as columns of parallel boxes and stack vertically on a narrow screen; and a part whose YAML cannot be read is reported at the line of the YAML error instead of the opening line of its fence.

## Specification

The IR store is `docs/ir/`. Read every requirement with `kotowari query ID`, including its scenarios; never work from this plan's paraphrase. The governing decision record is [the overview page reading record](../decision/records/2026-10-05-overview-page-reading.md) (A1–A18, D1, D2); it builds on [the overview index record](../decision/records/2026-10-05-overview-index.md) and [the overview record](../decision/records/2026-10-02-whole-picture.md).

New requirements and scenarios this plan must make tested:

| Document | Items |
|---|---|
| `docs/ir/view/navigation.md` | REQ-view-022, EX-view-016, EX-view-017 |
| `docs/ir/core/overview-data.md` | EX-core-511 (the revised REQ-core-282) |

Requirements verified by review, by a person looking at rendered pages (record A15): `docs/ir/view/navigation.md#REQ-view-023`, `docs/ir/view/parts.md#REQ-view-024`, `docs/ir/view/parts.md#REQ-view-025`. Their `how_to_verify` lines say what to look at.

Existing item whose text changed in the specification commit: `docs/ir/core/overview-data.md#REQ-core-282` gained one sentence (the line of an unreadable YAML part). Compare it with `git show a2e1426 -- docs/ir/core/overview-data.md`. Its existing tests (for example the one marked for EX-core-466) must keep passing unchanged.

## Approach and why

Three independent changes share one branch because they come from one approved specification. They are ordered from the most code to the least: the outline in the view engine first, then the flow drawing, then the YAML line in `kotowari-overview`, then the texts that describe the YAML line, then the change records.

The outline lives entirely in `kotowari-markdown-view`: the view already knows every section's heading text and stale flag when it draws a document page (`crates/kotowari-markdown-view/src/lib.rs`, where each section is written as `<section class="section">` with an `<h2>`). Each section gains an anchor, and the page gains a navigation list built from the same section list, so the outline cannot disagree with the sections. "Always visible on a wide screen" is done with CSS (`position: sticky`) and a media query in `crates/kotowari-markdown-view/src/style.css`; no script is added, because every page must stay without one (`docs/ir/view/navigation.md#REQ-view-018`). The stale mark in the outline uses the same visible wording or symbol as the mark next to a stale section heading (`docs/ir/view/rendering.md#REQ-view-009`), so a test can find it the same way.

The flow change is markup and style in `crates/kotowari-markdown-view/src/parts.rs` (function `flow`) and `style.css`: today each column is already a `flow-column` element and the arrow is a CSS pseudo-element between columns; the column gains a visible enclosure, and a narrow-screen media query stacks the columns and turns the arrow downward.

The YAML line is in `crates/kotowari-overview/src/parts.rs` (`check`, which returns `Checked::Invalid(vec!["(yaml)"])` and discards the error) and the caller in `crates/kotowari-overview/src/lib.rs` (around the line that sets `let line = Some(part.line)`). The YAML reader `serde_saphyr` returns an error with `location()`, an `Option<Location>` whose `line()` is the line within the part's content; the reported line is that line translated to a line of the overview data file. Whether `location()` is present for a syntax error such as an unclosed bracket was not verified while writing this plan.

`kotowari-markdown-view` and `kotowari-overview` are unreleased, so their internal types may change freely.

## Scope of change

- `crates/kotowari-markdown-view/src/` (`lib.rs`, `parts.rs`, `style.css`, and a new module if the implementer wants one) and `crates/kotowari-markdown-view/tests/` (`navigation.rs`)
- `crates/kotowari-overview/src/` (`parts.rs`, `lib.rs`) and `crates/kotowari-overview/tests/overview.rs`
- `tests/step18_overview.rs` for the process-level scenario EX-core-511
- `docs/guides/findings.md` (the `overview_part_invalid` row's line column), `docs/guides/commands/check.md` (the sentence about the line of part findings), `agent/skills/kotowari/references/findings.md` (the `overview_part_invalid` row), refreshing any guide mark fingerprints those sections carry
- `.kotowari/changes/implementation.yaml` and `.kotowari/changes/review.yaml`, each by its own role
- `.kotowari/equivalents.yaml` only for equivalents that pass the existing challenge rule

The IR and the decision records are approved and do not change in this plan. `CHANGELOG.md` is not changed: the overview feature is unreleased and its Unreleased entry describes behavior at a level these changes do not alter.

## Step order and prerequisites

The specification commit `a2e1426` and this plan are committed on branch `spec/overview-page-reading` before S1. S1 then S2 (both in the view crate and its style). S3 is independent of S1 and S2. S4 needs S3. S5 comes last. One writer works the steps in order on that branch.

Before S1, capture the branch-wide comparison base once with `git merge-base origin/main HEAD` (it was `ed9cc5be7392b730083d66ddfde8c6ed71eb03ab` when this plan was written) and keep it in session-local evidence; the final change records cover the specification commit, this plan and the implementation against that base. Run the baseline: `CARGO_BUILD_JOBS=4 cargo test --workspace --all-features --locked` must pass, and `kotowari check --format json` must report exactly 1 requirement_without_test (REQ-view-022) and 3 scenario_without_test (EX-view-016, EX-view-017, EX-core-511) errors and 9 size notices, and nothing else.

## Verification map

| Step | Requirements | Examples |
|---|---|---|
| S1 | REQ-view-022, REQ-view-023 (review) | EX-view-016, EX-view-017 |
| S2 | REQ-view-024 (review), REQ-view-025 (review) | none |
| S3 | REQ-core-282 | EX-core-511 |
| S4 | REQ-core-282 (texts describing it) | none |
| S5 | all of the above | all of the above |

## Left to the implementer

- The form of each section's anchor id, within record D1: unique within one page even for two sections with the same heading, the same for the same render input (`docs/ir/view/rendering.md#REQ-view-004`), and following an outline item reaches its section.
- The breakpoint between wide and narrow screens, the look of the outline, of the flow column enclosure and of the arrows (record D2).
- The markup element of the outline (for example a `nav` with a list), and module and helper names.
- Whether the stale mark in the outline repeats the exact text of the section-heading mark or a shorter form, as long as a reader can tell it means the same thing (record A5).

## Stop conditions

- A requirement cannot be met without adding a script to a page, without loading anything from outside the pages (`docs/ir/view/rendering.md#REQ-view-010`), or without changing the render input (`docs/ir/view/rendering.md#REQ-view-001`).
- `serde_saphyr` does not report a location for the YAML error that EX-core-511 needs, so the scenario cannot pass: hand back; do not switch YAML libraries or parse the error text.
- An existing test fails for a reason other than markup it pinned and this plan changes.
- The four general conditions: missing meaning or departure from approved content; an irreversible, privileged or dangerous operation; a spreading accident; no progress after a changed approach.

## Test command

`CARGO_BUILD_JOBS=4 cargo test --workspace --all-features --locked`. Format only the Rust files you touched with `rustfmt --edition 2024 <file>`; never run `cargo fmt --all`. Pass `< /dev/null` when running the pre-push hook by hand.

## Out of scope

- Localization and a switch between light and dark colours (record U1, U2).
- Highlighting the section being read while scrolling (record A6).
- The line of `overview_toc_invalid` (record A16).
- Any change to the index page beyond what the shared style needs to keep it as it is.

## Steps

### S1: an outline of sections on each document page

- Purpose: draw the outline that lists a document's sections in order, marks stale ones and links to each section, kept beside the text on a wide screen and under the title on a narrow one.
- Specification: docs/ir/view/navigation.md#REQ-view-022, docs/ir/view/navigation.md#REQ-view-023
- Prerequisites: the baseline above passes
- May change: crates/kotowari-markdown-view/src/, crates/kotowari-markdown-view/tests/navigation.rs
- Done when: the tests marked for REQ-view-022, EX-view-016 and EX-view-017 pass, every existing view test passes, and no page contains a script element
- Shown by: test — new tests in crates/kotowari-markdown-view/tests/navigation.rs marked `@kotowari[REQ-view-022, EX-view-016]` and `@kotowari[REQ-view-022, EX-view-017]`, written first and seen failing, then passing; compare links by their target and visible text, not by their markup
- Left to the implementer: anchor id form within record D1, outline markup, breakpoint and look (record D2)
- Stop and hand back if: drawing the outline needs any input the render input does not already carry

### S2: flow columns as parallel groups that stack on a narrow screen

- Purpose: draw each flow column as one enclosure with arrows only between columns, and stack the columns with downward arrows on a narrow screen.
- Specification: docs/ir/view/parts.md#REQ-view-024, docs/ir/view/parts.md#REQ-view-025
- Prerequisites: S1
- May change: crates/kotowari-markdown-view/src/parts.rs, crates/kotowari-markdown-view/src/style.css
- Done when: every existing view test passes and a rendered page with a flow of two or more columns, one holding two or more boxes, exists for the review in S5
- Shown by: artifact — pages rendered with `cargo run -p kotowari-markdown-view --example render` (or an overview build in a scratch copy of the repository) under a scratch directory, named in the implementer's report for the person to open at a wide and a narrow width
- Left to the implementer: the enclosure and arrow look, the breakpoint (record D2)
- Stop and hand back if: stacking the columns needs a change to the flow part schema

### S3: the line of an unreadable YAML part

- Purpose: report overview_part_invalid with detail "<kind> (yaml)" at the line of the YAML error in the overview data file when the reader gives a location, and at the opening line of the fence otherwise.
- Specification: docs/ir/core/overview-data.md#REQ-core-282
- Prerequisites: the baseline above passes
- May change: crates/kotowari-overview/src/parts.rs, crates/kotowari-overview/src/lib.rs, crates/kotowari-overview/tests/overview.rs, tests/step18_overview.rs
- Done when: the test marked for EX-core-511 passes and the existing tests for REQ-core-282 (EX-core-466 and others) pass unchanged
- Shown by: test — a new process-level test in tests/step18_overview.rs marked `@kotowari[EX-core-511]` running `kotowari check --format json` on overview data whose part has a YAML error a few lines after its fence, written first and seen failing, then passing
- Left to the implementer: how the location is carried from parts.rs to the caller
- Stop and hand back if: serde_saphyr gives no location for that YAML error, or its line numbering cannot be mapped to the content's lines without parsing the error text

### S4: texts that describe the line of overview_part_invalid

- Purpose: make the user guides and the skill reference say that an unreadable YAML part is reported at the line of the YAML error when it is known.
- Specification: docs/ir/core/overview-data.md#REQ-core-282
- Prerequisites: S3
- May change: docs/guides/findings.md, docs/guides/commands/check.md, agent/skills/kotowari/references/findings.md
- Done when: the three texts state the new line rule and `kotowari check` reports no guide_stale or invalid_marker on these files
- Shown by: check — `kotowari check --format json | jq '[.findings[] | select(.path | test("docs/guides/|agent/skills/"))]'` prints an empty list, and the test of REQ-core-125 passes
- Left to the implementer: wording
- Stop and hand back if: a guide mark in these sections points at an item this plan does not cover and would go stale

### S5: change records and the final checks

- Purpose: record and independently review the branch's changes against the comparison base, and confirm the plan's own items are done.
- Specification: docs/ir/view/navigation.md#REQ-view-022, docs/ir/view/navigation.md#REQ-view-023, docs/ir/view/parts.md#REQ-view-024, docs/ir/view/parts.md#REQ-view-025, docs/ir/core/overview-data.md#REQ-core-282
- Prerequisites: S1 to S4
- May change: .kotowari/changes/implementation.yaml, .kotowari/changes/review.yaml, .kotowari/equivalents.yaml
- Done when: `kotowari query` shows a nonempty tests list for REQ-view-022, EX-view-016, EX-view-017 and EX-core-511; `kotowari check` reports no error on a file the branch changed or on those IDs; both records are written by their own roles and `kotowari changes --base <base> --head <head> --phase review` exits 0 at the committed head; the person has looked at the pages from S2 and a long document page at a wide and a narrow width for REQ-view-023, REQ-view-024 and REQ-view-025
- Shown by: check — the test command, then `kotowari check --format json`, then `kotowari changes --base <base> --head <head> --phase review --format text`
- Left to the implementer: none
- Stop and hand back if: the person's look at the pages finds a behavior the requirements do not describe
