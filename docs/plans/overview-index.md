# Plan: arrange the overview index by a table of contents

## Goal

A person whose project sets `overview.files` and `overview.toc` gets an overview index that nests and orders the pages as the table of contents (目次) declares, shows a page count and the unreviewed-section, undecided and planned counts, and gives every page its place in the contents and links to the other pages of its group; a contents file that disagrees with the overview data is a `kotowari check` error that stops `overview build` and `overview serve`.

## Specification

The IR store is `docs/ir/`. Read every requirement with `kotowari query ID`, including its definition table and its scenarios; never work from this plan's paraphrase. The governing decision record is [the overview index record](../decision/records/2026-10-05-overview-index.md) (A1–A36, D1); it builds on [the overview record](../decision/records/2026-10-02-whole-picture.md) and [the record that puts overview on the public crates](../decision/records/2026-10-04-overview-on-public-api.md).

New requirements and scenarios this plan must make tested:

| Document | Items |
|---|---|
| `docs/ir/view/navigation.md` | REQ-view-016 to REQ-view-021, EX-view-011 to EX-view-015 |
| `docs/ir/view/rendering.md` | EX-view-010 |
| `docs/ir/core/overview-toc.md` | REQ-core-325 to REQ-core-332 (TBL-core-043), EX-core-505 to EX-core-510 |

Requirements verified by review: `docs/ir/core/overview-workflow.md#REQ-core-333` (new) and `docs/ir/core/overview-workflow.md#REQ-core-300` (its text gained "revise the table of contents").

Existing items whose text changed in the specification commit, so their tests must follow the new text: REQ-view-001 (the render input gains the table of contents), REQ-view-002 (the index links only to documents named in the contents), REQ-view-005 (rewritten: the index follows the contents instead of name byte order), EX-view-001 (its index-order line was removed; the test `ex_view_001_two_documents_give_four_pages_and_an_ordered_index` in `crates/kotowari-markdown-view/tests/rendering.rs` still asserts the old order), REQ-core-027 (five new kinds with a null line), REQ-core-294 (contents errors also stop build and serve), and the tables TBL-core-001, TBL-core-004 and TBL-core-008. Compare each with `git show` of the commit that adds `docs/decision/records/2026-10-05-overview-index.md`. The new finding kinds reach the test of REQ-core-125, which compares the kinds table in `agent/skills/kotowari/references/findings.md` with the code. No new stop wording is added, so REQ-core-127 is unaffected.

## Approach and why

Build from the bottom of the dependency graph upward, as the overview was built: the view engine first, because both the library and the CLI consume its render input; then the core vocabulary (configuration key and finding kinds); then the contents checks in `kotowari-overview`; then reading the file and integrating it into check, status, build and serve in the `kotowari` library and the CLI; then the skill texts. Each layer is testable before the next one consumes it.

The view stays ignorant of kotowari and never checks its input (REQ-view-003, record A18): it takes the contents as a third part of the render input and draws exactly what it is given, with the defined behaviour for names without a document, documents missing from the contents and repeated names (REQ-view-021). All checking of the contents lives in `kotowari-overview`, next to the overview data checks it mirrors: the form is a JSON Schema checked with the existing `jsonschema` dependency, with the same place notation as `overview_part_invalid` (REQ-core-282, record A34). Reading the file, the overlap check and the stop reasons belong to the `kotowari` library, where `overview.files` is already read and its overlap with `guides.files` and `tests.files` already checked (`crates/kotowari/src/overview.rs`). Core learns only the `overview.toc` key and the five finding kinds in its kind vocabulary (`crates/kotowari-core/src/lib.rs`); as for the existing overview kinds, the null line is set by `kotowari-overview`, which creates these findings, so REQ-core-027 is shown in S4.

The view counts the states itself from the documents it already receives (record A13): the stale flag of each section and the labels of the `status` parts. No new input is added for the counts.

`kotowari-markdown-view` and `kotowari-overview` have not been released yet (the last release, `kotowari-v0.3.0`, does not contain them), so their public types may change shape without a compatibility path.

## Scope of change

- `crates/kotowari-markdown-view/` (sources, style, tests, examples, README)
- `crates/kotowari-overview/` (sources, a contents schema under `schemas/`, tests, examples, README)
- `crates/kotowari-core/src/` and `crates/kotowari-core/tests/` for the `overview.toc` key and the five finding kinds with their line rule
- `crates/kotowari/` for reading the contents, the overlap check, and passing it through check, status, `overview_prepare` and `overview_build`, sync and async
- Root `src/` and `tests/` (`tests/step1_config.rs` for the configuration key, `tests/step18_overview.rs`, `tests/step19_overview_build.rs`, `tests/step20_overview_serve.rs`) for process-level behaviour
- Every existing test that enables `overview` (`tests/step1_config.rs`, `tests/step18_overview.rs`, `tests/step19_overview_build.rs`, `tests/step20_overview_serve.rs`, `crates/kotowari/tests/project.rs`, `crates/kotowari/tests/async.rs`, and the tests and examples of `crates/kotowari-overview/`): it gains an `overview.toc` value in S3 and, from S5 on, a contents file that lists every overview data name, because the key becomes required and the contents is checked
- `agent/skills/kotowari/references/findings.md`, `agent/skills/kotowari/references/overview.md`, `agent/skills/kotowari-brainstorm/SKILL.md`
- `CHANGELOG.md` (Unreleased) and `README.md` where they describe overview configuration
- `.kotowari/changes/implementation.yaml` and `.kotowari/changes/review.yaml`, each by its own role
- `.kotowari/equivalents.yaml` only for equivalents that pass the existing challenge rule

The IR and the decision records are approved and do not change in this plan.

## Step order and prerequisites

The specification commit and this plan are committed on branch `overview-index` before S1. S1 then S2 (both in the view crate). S3 has no prerequisite beyond the commits and may run before or after S1 and S2. S4 needs S1 and S3. S5 needs S4. S6 can run any time after S3. S7 comes last. One writer works the steps in order on branch `overview-index`.

Before S1, capture the branch-wide comparison base once with `git merge-base origin/main HEAD` and keep it in session-local evidence; the final change records cover the specification commit, this plan and the implementation against that base. Run the baseline: `CARGO_BUILD_JOBS=4 cargo test --workspace --all-features --locked` must pass, and `kotowari check --format json` must report exactly 14 requirement_without_test and 12 scenario_without_test errors (all belonging to this plan's items), 8 guide_stale notices and 9 size notices.

## Verification map

| Step | Requirements | Examples |
|---|---|---|
| S1 | REQ-view-001, REQ-view-002, REQ-view-005, REQ-view-016, REQ-view-017, REQ-view-018, REQ-view-021 | EX-view-001, EX-view-010, EX-view-011, EX-view-012, EX-view-013 |
| S2 | REQ-view-019, REQ-view-020 | EX-view-014, EX-view-015 |
| S3 | REQ-core-013 (TBL-core-004), REQ-core-014, REQ-core-125, TBL-core-008 | none new |
| S4 | REQ-core-027, REQ-core-327 to REQ-core-332 (TBL-core-043) | EX-core-507, EX-core-509, and the check half of EX-core-508 |
| S5 | REQ-core-325, REQ-core-326, REQ-core-294 (TBL-core-001) | EX-core-505, EX-core-506, EX-core-510, and the build half of EX-core-508 |
| S6 | REQ-core-300, REQ-core-333 (review) | none |
| S7 | all of the above, as the branch-wide gate | all of the above |

## Left to the implementer

- Type, field and function names of the contents in the render input and in `kotowari-overview`, as long as REQ-view-001 holds
- The HTML and CSS of the index and the page navigation within REQ-view-005 and REQ-view-016 to REQ-view-021, and record D1 (groups start open, can be folded, nothing is stored, no script); the exact wording of the count labels is fixed by REQ-view-016 and REQ-view-017
- The anchor scheme that lets a page link to a group's place in the index
- How the overlap of REQ-core-326 compares the contents path with the walked files, as long as it uses the same path normalisation the existing overlap checks use
- Where the contents schema file sits under `crates/kotowari-overview/schemas/` and how it is embedded
- Which fixture files the tests use

## Stop conditions

- A requirement cannot be met without the view checking its input or knowing kotowari, or without core learning the meaning of the contents
- The contents form cannot be expressed with `jsonschema` (default features off) as it is used for the parts, or its place notation cannot match REQ-core-282
- An existing marked test fails for a reason other than a text change listed under Specification
- Making `overview.toc` required breaks a test or fixture outside the overview tests listed in Scope of change
- A step needs a new dependency

## Test command

PROJECT.md fixes the ordinary commands. The final gate of S7 runs, in order, with `CARGO_BUILD_JOBS=4` on every Cargo command:

```sh
cargo fmt --all -- --check
cargo test --workspace --locked
cargo test --workspace --all-features --locked
cargo test -p kotowari-markdown-view --locked
cargo test -p kotowari-overview --locked
cargo doc --workspace --all-features --no-deps --locked
cargo run -q -p kotowari-cli --bin kotowari -- check --format json
cargo run -q -p kotowari-cli --bin kotowari -- changes --base "$BASE" --head "$HEAD" --phase review --format json
```

`BASE` is the base captured before S1 and `HEAD` the final commit. Do not run `cargo fmt --all` to rewrite files. Do not write tests that wait on real time.

## Out of scope

- The narrow-screen overlap of flow boxes, the meaning of vertically stacked flow boxes, and the place of a YAML error in overview data (record U1)
- Turning on `overview` in this repository's own `.kotowari/config.yaml` and writing overview data or a contents file for kotowari itself
- Refreshing the 8 guide_stale notices in `docs/guides/` caused by the table rows the specification added
- The compiler warnings left by the public-crate split, unless a step touches the same file
- Pushing, opening the pull request and merging; the mutation tests run in the pull request CI

## Steps

### S1: draw the index from the table of contents

- Purpose: make the view take the table of contents as part of its render input and draw the index as nested, ordered, foldable groups with the page and state counts
- Specification: docs/ir/view/rendering.md#REQ-view-001, docs/ir/view/rendering.md#REQ-view-002, docs/ir/view/rendering.md#REQ-view-005, docs/ir/view/navigation.md#REQ-view-016, docs/ir/view/navigation.md#REQ-view-017, docs/ir/view/navigation.md#REQ-view-018, docs/ir/view/navigation.md#REQ-view-021
- Prerequisites: the specification commit and this plan are committed; the baseline passes
- May change: `crates/kotowari-markdown-view/`, and the callers of `RenderInput` in `crates/kotowari-overview/src/` only as far as needed to keep the workspace compiling (passing a contents value built from the existing name order is enough until S4)
- Done when: `kotowari query` shows a marked, passing test for REQ-view-016, REQ-view-017, REQ-view-018, REQ-view-021 and for EX-view-010 to EX-view-013; the tests of REQ-view-001, REQ-view-002, REQ-view-005 and EX-view-001 follow their new text, and no test still asserts index order by name; the workspace builds
- Shown by: test — one test per scenario EX-view-010 to EX-view-013, a test for the mismatched-input cases of REQ-view-021, the existing property test of REQ-view-004 extended to inputs with contents, and the EX-view-001 test rewritten without the order assertion
- Left to the implementer: the markup of the groups, cards and counts within the requirements; how the property test generates contents
- Stop and hand back if: the foldable groups of REQ-view-018 cannot be drawn without a script, or the counts of REQ-view-016 need a value the document does not carry

### S2: place each page in the table of contents

- Purpose: draw, on each document page, its place in the contents above the title and the links to the other pages of its group after the last section
- Specification: docs/ir/view/navigation.md#REQ-view-019, docs/ir/view/navigation.md#REQ-view-020
- Prerequisites: S1
- May change: `crates/kotowari-markdown-view/`
- Done when: `kotowari query` shows a marked, passing test for REQ-view-019, REQ-view-020, EX-view-014 and EX-view-015
- Shown by: test — one test per scenario EX-view-014 and EX-view-015, plus a test that a page placed directly under the outermost group shows only the outermost title in its place line
- Left to the implementer: the markup of the place line and the group links
- Stop and hand back if: the place line cannot sit above the title without changing what REQ-view-006 puts right after the title

### S3: add the contents key and finding kinds to core

- Purpose: give core the `overview.toc` configuration key (required when `overview` is present) and the five `overview_toc_*` finding kinds in its kind vocabulary, and list the kinds in the skill's findings reference
- Specification: docs/ir/core/config.md#REQ-core-013, docs/ir/core/config.md#REQ-core-014, docs/ir/core/skill-references.md#REQ-core-125
- Prerequisites: the specification commit and this plan are committed
- May change: `crates/kotowari-core/src/`, `crates/kotowari-core/tests/`, `tests/step1_config.rs`, `agent/skills/kotowari/references/findings.md`, and the configuration strings of the existing tests listed in Scope of change that enable `overview` (add an `overview.toc` value so they keep passing)
- Done when: a configuration with `overview` but no `overview.toc` stops with `config error: `; a configuration with both reads the path; a null `overview.toc` and an absolute `overview.toc` stop with `config error: ` as REQ-core-014 says; the five kinds exist in core's kind vocabulary; the REQ-core-125 test passes with the five kinds in the findings reference; the workspace tests pass
- Shown by: test — key tests for `overview.toc` in `tests/step1_config.rs` beside the existing `overview.files` ones (present, missing under `overview`, null, absolute), and the existing `req_125_findings_reference_kinds_match_the_code`
- Left to the implementer: the field name in the configuration type; the wording after `config error: ` for the missing key, following the existing `overview.files is required`
- Stop and hand back if: making the key required breaks a test outside those listed in Scope of change

### S4: check the table of contents in kotowari-overview

- Purpose: check the contents' form and its agreement with the overview data, and pass the contents to the view
- Specification: docs/ir/core/finding-order.md#REQ-core-027, docs/ir/core/overview-toc.md#REQ-core-327, docs/ir/core/overview-toc.md#REQ-core-328, docs/ir/core/overview-toc.md#REQ-core-329, docs/ir/core/overview-toc.md#REQ-core-330, docs/ir/core/overview-toc.md#REQ-core-331, docs/ir/core/overview-toc.md#REQ-core-332
- Prerequisites: S1, S3
- May change: `crates/kotowari-overview/`, and `crates/kotowari/src/acquisition.rs` and `crates/kotowari/src/overview.rs` only as far as needed to keep the workspace compiling (until S5 they may pass a contents that lists every overview data name in name order)
- Done when: `kotowari query` shows a marked, passing test for REQ-core-327 to REQ-core-332, TBL-core-043, EX-core-507 and EX-core-509, and EX-core-508 has a test of its check half; a test shows the five `overview_toc_*` kinds with a null line (REQ-core-027); `kotowari-overview` still touches no file, network or environment; the workspace builds and its tests pass
- Shown by: test — one test per scenario EX-core-507, EX-core-509 and the check half of EX-core-508, a form test per row of TBL-core-043 (unknown key, missing `title` or `items`, empty `title`, a `note` with a line break, an empty name, unreadable YAML as `(yaml)`, a non-mapping top as `(root)`), a test that an unknown name repeated gives only overview_toc_page_unknown, and a test that the render input carries the contents in written order
- Left to the implementer: how the contents text reaches `inspect` (a new parameter or a field of an input type), as long as the crate reads no file
- Stop and hand back if: the JSON Schema checker cannot report a key name for an unknown or missing key in the same way it does for parts

### S5: read the contents and stop on its errors

- Purpose: read `overview.toc` in check, status, build and serve, check its overlap with the globs before reading it, and stop build and serve on contents errors
- Specification: docs/ir/core/overview-toc.md#REQ-core-325, docs/ir/core/overview-toc.md#REQ-core-326, docs/ir/core/overview-commands.md#REQ-core-294
- Prerequisites: S4
- May change: `crates/kotowari/`, root `src/`, `tests/step18_overview.rs`, `tests/step19_overview_build.rs`, `tests/step20_overview_serve.rs` and their fixtures, `README.md`, `CHANGELOG.md`
- Done when: `kotowari query` shows a marked, passing test for REQ-core-325, REQ-core-326, EX-core-505, EX-core-506, EX-core-510 and the build half of EX-core-508; the tests of REQ-core-294 and TBL-core-001 cover the contents cases; sync and async library operations give the same result; every existing test that enables `overview` has a contents file listing every overview data name and passes
- Shown by: test — process-level tests for EX-core-505 (missing key, missing file), a non-UTF-8 contents file, EX-core-506, EX-core-508 (build exits 2 and writes nothing), EX-core-510 (index order follows the contents), and one serve test that a contents error stops serve before it binds
- Left to the implementer: where in `crates/kotowari/src/overview.rs` the overlap check of the contents sits, as long as it runs after the existing overlap checks and before the contents is read
- Stop and hand back if: the overlap of REQ-core-326 cannot be decided from the files each walk reads without walking a second time in a way that changes the existing stop order

### S6: teach the skills the table of contents

- Purpose: make the brainstorm approval revise the contents, and the overview reference say how to write the contents and where to place a page
- Specification: docs/ir/core/overview-workflow.md#REQ-core-300, docs/ir/core/overview-workflow.md#REQ-core-333
- Prerequisites: S3
- May change: `agent/skills/kotowari/references/overview.md`, `agent/skills/kotowari-brainstorm/SKILL.md`
- Done when: the brainstorm approval steps revise the contents after the overview data; the overview reference documents the `overview.toc` key, the contents form (`title`, `note`, `items`, names), the new finding kinds, that the LLM places new pages and splits a group only once it grows, that the order inside a level is the reader's order, and that the contents is not part of what the person approves
- Shown by: external — a reviewer reads the two files against the `how_to_verify` lines of REQ-core-300 and REQ-core-333 and records the result in the review change record of S7
- Left to the implementer: the wording and the example contents in the reference
- Stop and hand back if: describing placement needs a rule the decision record does not contain

### S7: reconcile the whole branch and hand it back

- Purpose: run the final gate on a fixed head and author both change records for the whole branch
- Specification: docs/ir/core/overview-workflow.md#REQ-core-333
- Prerequisites: S1 to S6
- May change: `.kotowari/changes/implementation.yaml`, `.kotowari/changes/review.yaml`, `.kotowari/equivalents.yaml`
- Done when: every command of the Test command section exits 0 on the fixed head, except that `kotowari check` may still report the 8 guide_stale and 9 size notices of the baseline; every item in the Verification map has a marked test or a passed review; the implementer record and a separately authored reviewer record cover every changed file since the base captured before S1; `changes --phase review` reports no finding
- Shown by: check — the commands of the Test command section in order, then `kotowari query ID` for each item of the Verification map
- Left to the implementer: none
- Stop and hand back if: the previous branch's records in `.kotowari/changes/` cannot be replaced without losing a review conclusion that the new reviewer record does not repeat
