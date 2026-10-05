# Plan: hold the IR, guides and overview as multilingual pairs

## Goal

A project that lists several languages in its configuration keeps each IR document, glossary, problem record, guide, overview data file and table of contents as a pair of files (one per language) with a consistency record, `kotowari check` reports missing sides, stale sides, skeleton mismatches, wrong switcher lines and links that leave the reader's language, `kotowari list` prints the blob hashes to record, and `kotowari overview build` writes one set of overview pages per language whose UI text comes from the configuration.

## Specification

The IR store is `docs/ir/`. Read every requirement with `kotowari query ID`, including its scenarios; never work from this plan's paraphrase. The governing decision record is [the localization record](../decision/records/2026-10-05-localization.md) (A1–A40, D1–D3; A1, A4, A6, A7, A21 are superseded and say so). The specification commit is `458eae6` on branch `spec/localization`.

New requirements and scenarios this plan must make tested:

| Document | Items |
|---|---|
| `docs/ir/core/translation-pairs.md` | REQ-core-334 to REQ-core-343, EX-core-512 to EX-core-525 |
| `docs/ir/core/translation-structure.md` | REQ-core-344 to REQ-core-349, TBL-core-044, TBL-core-045, EX-core-526 to EX-core-535 |
| `docs/ir/core/overview-languages.md` | REQ-core-351 to REQ-core-355, TBL-core-046, EX-core-536 to EX-core-542 |
| `docs/ir/view/languages.md` | REQ-view-026 to REQ-view-030, TBL-view-002, EX-view-018 to EX-view-023 |

Requirement verified by review: `docs/ir/core/translation-structure.md#REQ-core-350` (the kotowari skill's procedure for keeping pairs aligned). Its `how_to_verify` line says what to look at.

Existing items whose text changed in the specification commit (compare with `git show 458eae6 -- <file>`); their existing tests must keep passing, adjusted only where they pinned text the specification now replaces:

- `docs/ir/core/config.md#TBL-core-004` (keys `languages`, `labels`), `docs/ir/core/findings.md#TBL-core-008` (seven new error kinds), `docs/ir/core/finding-order.md#REQ-core-027` and `docs/ir/core/finding-order.md#TBL-core-019` (lines of the new kinds), `docs/ir/core/ir-document.md#REQ-core-033` (glossary and problem-record sides)
- `docs/ir/core/list.md#REQ-core-152` and `docs/ir/core/list.md#REQ-core-155` (the `translations` output)
- `docs/ir/core/overview-commands.md#REQ-core-291`, `docs/ir/core/overview-commands.md#REQ-core-293`, `docs/ir/core/overview-commands.md#REQ-core-294`, `docs/ir/core/overview-commands.md#REQ-core-305`, `docs/ir/core/overview-toc.md#REQ-core-328`, `docs/ir/core/overview-toc.md#REQ-core-329`
- `docs/ir/view/rendering.md#REQ-view-001`, `docs/ir/view/rendering.md#REQ-view-008`, `docs/ir/view/rendering.md#REQ-view-009`, `docs/ir/view/navigation.md#REQ-view-016`, `docs/ir/view/navigation.md#REQ-view-017`, `docs/ir/view/navigation.md#REQ-view-019`, `docs/ir/view/navigation.md#REQ-view-022` and their scenarios EX-view-011, EX-view-012, EX-view-015 (the strings now come from the UI text in the render input), and `docs/ir/view/parts.md#TBL-view-001` (status tags become `decided`, `planned`, `open`, `dropped`)

## Approach and why

The work is ordered so that each step stands on settled input: the configuration first (every later step reads the language list), then finding the pairs and their consistency records, then how the existing checks treat a pair, then the skeleton comparison, then switcher lines and links, then `list`, then the view engine, then the per-language overview build that joins core and view, then the texts, then the change records.

The layering stays as it is (`docs/ir/core/library-crates.md#REQ-core-307`, `docs/ir/core/library-crates.md#REQ-core-308`): `kotowari-core` decides everything from in-memory texts; the `kotowari` library (`crates/kotowari/src/`, for example `acquisition.rs`, `ir.rs`, `guides.rs`, `overview.rs`) finds and reads files, including looking up the other-language sides by name next to the first-language side and reading `<stem>.i18n.yaml`; `kotowari-overview` builds the per-language render inputs; `kotowari-markdown-view` holds no language text at all (`docs/ir/view/languages.md#REQ-view-027`). The CLI in `src/` only formats.

The configuration keys are parsed in `crates/kotowari-core/src/config.rs`, which already rejects unknown keys; `languages` and `labels` join it, and the English UI text of `docs/ir/core/overview-languages.md#TBL-core-046` lives in kotowari, not in the view.

The blob hash (`docs/ir/core/translation-pairs.md#REQ-core-340`) needs SHA-1. Add the `sha1` crate at version 0.10 to `kotowari-core`: it is from the same RustCrypto family and version line as the `sha2` 0.10 already used for fingerprints, so the digest API is the same; do not shell out to git. Links (`docs/ir/core/translation-structure.md#REQ-core-347`) are read with the `markdown` crate already used for guides, whose mdast gives link, image and definition nodes with their destination and position; do not parse links with a regular expression.

The view gains three inputs (`docs/ir/view/rendering.md#REQ-view-001`): the language tag, the UI text, and the list of other languages; a reference may lack a body. `kotowari-markdown-view` and `kotowari-overview` are unreleased (the CHANGELOG lists them under Unreleased), so their types may change freely. The status part's schema in `crates/kotowari-markdown-view/schemas/status.json` changes its `state` enum to the four English words.

Every test fixture whose language list holds a language other than `en` must write a complete `labels.<tag>` with every key of `docs/ir/core/overview-languages.md#TBL-core-046`, in addition to whatever its scenario names; otherwise the configuration stops (`docs/ir/core/overview-languages.md#REQ-core-352`) before the assertion is reached. An empty `languages` list means English only (record D3 and the glossary term 言語の一覧), not a stop.

The test `tests/step7_skill_references.rs` (marked for REQ-core-125) requires the finding kinds in code to equal the Kind column of `agent/skills/kotowari/references/findings.md`, so every step that adds a finding kind adds its row to that table in the same step.

The repository itself keeps one language: its `.kotowari/config.yaml` gets no `languages` key in this plan, so `kotowari check` on this repository must behave as before apart from the new texts. Translating the repository's own documents is a later, separate piece of work (record A13).

## Scope of change

- `crates/kotowari-core/` (`Cargo.toml` for `sha1`, `src/`, `tests/`), `Cargo.lock`
- `crates/kotowari/src/` and `crates/kotowari/tests/`
- `crates/kotowari-overview/src/`, `crates/kotowari-overview/tests/overview.rs` and `crates/kotowari-overview/examples/overview.rs`
- `crates/kotowari-markdown-view/src/`, `crates/kotowari-markdown-view/schemas/status.json`, `crates/kotowari-markdown-view/tests/`, `crates/kotowari-markdown-view/examples/render.rs`, `crates/kotowari-markdown-view/README.md`
- `src/cli.rs` and `src/cli/` only where `list` text output or new findings need formatting
- `tests/step1_config.rs`, `tests/step10_list.rs`, `tests/step15_guides.rs`, `tests/step18_overview.rs`, `tests/step19_overview_build.rs`, and one new process-level test file `tests/step21_translations.rs`
- `docs/guides/` (the five guides with stale marks today: `commands/check.md`, `commands/list.md`, `config.md`, `deferred.md`, `findings.md`, plus any guide that describes overview status tags) and their guide-mark fingerprints
- `agent/skills/kotowari/` (`SKILL.md` and `references/`: `config.md`, `findings.md`, `overview.md`, and the pair procedure of REQ-core-350 in whichever file the skill's own structure puts it)
- `CHANGELOG.md` (one Unreleased entry under Added)
- `.kotowari/changes/implementation.yaml` and `.kotowari/changes/review.yaml`, each by its own role
- `.kotowari/equivalents.yaml` only for equivalents that pass the existing challenge rule

The IR and the decision records are approved and do not change in this plan.

## Step order and prerequisites

The specification commit `458eae6` and this plan are committed on branch `spec/localization` before S1. S1 first; S2 needs S1; S3 and S4 need S2; S5 needs S2 and S4 (it adds the switcher exclusion to the skeleton); S6 needs S2; S7 is independent of S1 to S6 but adapts kotowari-overview minimally so the workspace still builds; S8 needs S1, S3, S4 and S7; S9 needs S1 to S8; S10 comes last. One writer works the steps in order on that branch.

Before S1, capture the branch-wide comparison base once with `git merge-base origin/main HEAD` (it was `25008aed476ffedb4f43711ac7b54e682376e164` when this plan was written) and keep it in session-local evidence; the final change records cover the specification commit, this plan and the implementation against that base. Run the baseline: the test command must pass, and `kotowari check --format json` must report exactly 26 requirement_without_test and 37 scenario_without_test errors (all on the IDs listed in Specification), 16 guide_stale notices (all in `docs/guides/`), 9 size notices, and nothing else.

## Verification map

| Step | Requirements | Examples |
|---|---|---|
| S1 | REQ-core-334 (language list), REQ-core-335, REQ-core-351, REQ-core-352, TBL-core-046 | EX-core-513, EX-core-537, EX-core-538, EX-core-539 |
| S2 | REQ-core-334, REQ-core-336, REQ-core-337, REQ-core-338, REQ-core-339, REQ-core-340, REQ-core-341 | EX-core-512, EX-core-514 to EX-core-521 |
| S3 | REQ-core-342, REQ-core-343, REQ-core-033, REQ-core-305, REQ-core-328, REQ-core-329 | EX-core-522 to EX-core-525 |
| S4 | REQ-core-344, REQ-core-345, TBL-core-044 (all parts except the guide and overview-data link parts), TBL-core-045 | EX-core-526 to EX-core-529 |
| S5 | REQ-core-346, REQ-core-347, REQ-core-348, REQ-core-349, REQ-core-027, TBL-core-019, TBL-core-044 (the link parts and the switcher exclusion) | EX-core-530 to EX-core-535 |
| S6 | REQ-core-152, REQ-core-155 | none new |
| S7 | REQ-view-026 to REQ-view-030, TBL-view-002, REQ-view-001, REQ-view-008, REQ-view-009, REQ-view-016, REQ-view-017, REQ-view-019, REQ-view-022, TBL-view-001 | EX-view-018 to EX-view-023, EX-view-011, EX-view-012, EX-view-015 |
| S8 | REQ-core-353, REQ-core-354, REQ-core-355, REQ-core-291, REQ-core-293, REQ-core-294 | EX-core-536, EX-core-540, EX-core-541, EX-core-542 |
| S9 | REQ-core-350 (review), TBL-core-008 (texts describing it) | none |
| S10 | all of the above | all of the above |

## Left to the implementer

- Module and type names, and which existing module each piece joins, within the layering above.
- The internal representation of a pair, of the language list and of the UI text, as long as the view receives exactly the inputs of `docs/ir/view/rendering.md#REQ-view-001`.
- Where the language links sit on a page and how they look, as long as every HTML page has them (`docs/ir/view/languages.md#REQ-view-029`) and no page gets a script.
- The CHANGELOG wording.

## Stop conditions

- A requirement cannot be met without a script in a page, without loading anything from outside the pages, or without the view holding language text of its own.
- The `markdown` crate does not give the position of a link, image or definition destination needed for the line of `link_language_mismatch` or `link_to_record`: hand back; do not switch parsers or match links with a regular expression.
- The `sha1` crate cannot be added (registry or licence problem): hand back; do not implement SHA-1 by hand.
- An existing test fails for a reason other than text, status tags or `list` output that the specification commit replaced.
- `kotowari check` on this repository reports a new finding kind of this plan although the repository has no `languages` key.
- The four general conditions: missing meaning or departure from approved content; an irreversible, privileged or dangerous operation; a spreading accident; no progress after a changed approach.

## Test command

`CARGO_BUILD_JOBS=4 cargo test --workspace --all-features --locked`. Format only the Rust files you touched with `rustfmt --edition 2024 <file>`; never run `cargo fmt --all`. Tests must not wait on real time. Pass `< /dev/null` when running the pre-push hook by hand.

## Out of scope

- Translating this repository's own README, guides and IR, removing the guides' links to decision records, and deciding the overlap between guides and overviews (record A13, U1).
- A reader-side switch between light and dark colours (record A2).
- A per-language list of vague words (record A23) and a language option for `query` and `list` (record A24).
- A command that writes the consistency record (record A18).

## Steps

### S1: the language list and the UI text from the configuration

- Purpose: read `languages` and `labels` from the configuration, stop on malformed tags, duplicates and bad UI text, and resolve each language's UI text from the built-in English text and `labels`.
- Specification: docs/ir/core/translation-pairs.md#REQ-core-334, docs/ir/core/translation-pairs.md#REQ-core-335, docs/ir/core/overview-languages.md#REQ-core-351, docs/ir/core/overview-languages.md#REQ-core-352, docs/ir/core/config.md#TBL-core-004
- Prerequisites: the baseline above passes
- May change: crates/kotowari-core/src/config.rs, crates/kotowari-core/src/ (a new module for UI text if wanted), crates/kotowari-core/tests/, tests/step1_config.rs
- Done when: the tests marked for EX-core-513, EX-core-537, EX-core-538, EX-core-539, REQ-core-351 (with TBL-core-046) and REQ-core-352 pass, and every existing configuration test passes unchanged
- Shown by: test — process-level tests in tests/step1_config.rs marked `@kotowari[EX-core-513]`, `@kotowari[REQ-core-352, EX-core-537]`, `@kotowari[EX-core-538]`, `@kotowari[EX-core-539]`, and a core test marked `@kotowari[REQ-core-351, TBL-core-046]` that resolves English with one `labels.en` override and a complete `labels.ja`, each written first and seen failing, then passing
- Left to the implementer: where the English table lives in kotowari-core, the type of the resolved UI text
- Stop and hand back if: an existing configuration test pins that an unknown top-level key named `languages` or `labels` stops

### S2: pairs, missing sides and consistency records

- Purpose: find the pairs among the IR documents, guides, overview data and the table of contents, report missing sides, invalid consistency records and stale sides, using the git blob hash.
- Specification: docs/ir/core/translation-pairs.md#REQ-core-334, docs/ir/core/translation-pairs.md#REQ-core-336, docs/ir/core/translation-pairs.md#REQ-core-337, docs/ir/core/translation-pairs.md#REQ-core-338, docs/ir/core/translation-pairs.md#REQ-core-339, docs/ir/core/translation-pairs.md#REQ-core-340, docs/ir/core/translation-pairs.md#REQ-core-341
- Prerequisites: S1
- May change: crates/kotowari-core/Cargo.toml, Cargo.lock, crates/kotowari-core/src/, crates/kotowari-core/tests/, crates/kotowari/src/, crates/kotowari/tests/, tests/step21_translations.rs, agent/skills/kotowari/references/findings.md (Kind rows of the new kinds only)
- Done when: the tests marked for REQ-core-340 and EX-core-512, EX-core-514 to EX-core-521 pass, the same file pair read from two places yields its pair findings once, and `kotowari check` on this repository reports none of the new kinds
- Shown by: test — a core test marked `@kotowari[REQ-core-340]` hashing "a\n" and the empty text to the values in EX-core-518 and EX-core-519, and process-level tests in tests/step21_translations.rs marked with each scenario ID (and its requirement IDs), each written first and seen failing, then passing
- Left to the implementer: how the library hands the other-language sides and the consistency records to the core
- Stop and hand back if: looking up sides by name next to the first-language side (REQ-core-337) conflicts with the glob reading of `guides.files` or `overview.files` in a way the specification does not settle

### S3: a pair as one document in the existing checks

- Purpose: read items, IDs, sources, fingerprints, list items and surface from the first-language side only, run the text checks and glossary checks on every side with that language's glossary chain, count every side in files and lines, and apply the overview and table-of-contents checks per side or first side as specified.
- Specification: docs/ir/core/translation-pairs.md#REQ-core-342, docs/ir/core/translation-pairs.md#REQ-core-343, docs/ir/core/ir-document.md#REQ-core-033, docs/ir/core/overview-commands.md#REQ-core-305, docs/ir/core/overview-toc.md#REQ-core-328, docs/ir/core/overview-toc.md#REQ-core-329
- Prerequisites: S2
- May change: crates/kotowari-core/src/, crates/kotowari-core/tests/, crates/kotowari/src/, crates/kotowari-overview/src/, crates/kotowari-overview/tests/overview.rs, tests/step21_translations.rs
- Done when: the tests marked for EX-core-522 to EX-core-525 pass, a test marked for REQ-core-342 shows that "files" and "lines" in the check and status JSON count the other-language side, and every existing IR, guide, overview and table-of-contents test passes unchanged
- Shown by: test — process-level tests in tests/step21_translations.rs marked `@kotowari[REQ-core-342, EX-core-522]`, `@kotowari[EX-core-523]`, `@kotowari[REQ-core-343, EX-core-524]`, `@kotowari[EX-core-525]`, each written first and seen failing, then passing
- Left to the implementer: how the glossary chain is chosen per language inside the term check
- Stop and hand back if: an existing check not named in REQ-core-342 or REQ-core-343 would need to run on other-language sides to keep an existing test passing

### S4: the skeleton comparison

- Purpose: compare each other-language side's skeleton with the first-language side's by document kind and report the first mismatching part with its line.
- Specification: docs/ir/core/translation-structure.md#REQ-core-344, docs/ir/core/translation-structure.md#REQ-core-345, docs/ir/core/translation-structure.md#TBL-core-044, docs/ir/core/translation-structure.md#TBL-core-045
- Prerequisites: S2
- May change: crates/kotowari-core/src/, crates/kotowari-core/tests/, crates/kotowari-overview/src/, crates/kotowari/src/, tests/step21_translations.rs, agent/skills/kotowari/references/findings.md (Kind row of the new kind only)
- Done when: the tests marked for EX-core-526 to EX-core-529 pass, and a core test per document kind in TBL-core-044 shows one differing part reported with its part name, for every part except the guide and overview-data link parts, which S5 adds
- Shown by: test — process-level tests in tests/step21_translations.rs marked `@kotowari[REQ-core-344, EX-core-526]`, `@kotowari[REQ-core-345, EX-core-527]`, `@kotowari[TBL-core-045, EX-core-528]`, `@kotowari[EX-core-529]`, and core tests marked `@kotowari[TBL-core-044]` covering each document kind's parts, each written first and seen failing, then passing
- Left to the implementer: how each document kind's skeleton is extracted, as long as natural-language text never enters it
- Stop and hand back if: a part of TBL-core-044 cannot be extracted from what the existing readers already parse without a new Markdown reader

### S5: switcher lines and links

- Purpose: check the switcher line of IR and guide sides, and report links from paired sides to another language's side or to the decision record and ADR places, with their lines.
- Specification: docs/ir/core/translation-structure.md#REQ-core-346, docs/ir/core/translation-structure.md#REQ-core-347, docs/ir/core/translation-structure.md#REQ-core-348, docs/ir/core/translation-structure.md#REQ-core-349, docs/ir/core/finding-order.md#REQ-core-027, docs/ir/core/finding-order.md#TBL-core-019
- Prerequisites: S2, S4
- May change: crates/kotowari-core/src/, crates/kotowari-core/tests/, crates/kotowari/src/, crates/kotowari-overview/src/, tests/step21_translations.rs, agent/skills/kotowari/references/findings.md (Kind rows of the new kinds only)
- Done when: the tests marked for EX-core-530 to EX-core-535 pass, a correct switcher line raises neither missing_scope nor term findings, the guide and overview-data link parts and the switcher exclusion of TBL-core-044 are compared with a core test each, and the existing finding-order tests pass
- Shown by: test — process-level tests in tests/step21_translations.rs marked `@kotowari[REQ-core-346, EX-core-530]`, `@kotowari[EX-core-531]`, `@kotowari[EX-core-532]`, `@kotowari[REQ-core-347, REQ-core-348, EX-core-533]`, `@kotowari[EX-core-534]`, `@kotowari[REQ-core-349, EX-core-535]`, and a core test marked `@kotowari[REQ-core-027, TBL-core-019]` for the lines of the new kinds, each written first and seen failing, then passing
- Left to the implementer: how a destination is resolved to a pair side
- Stop and hand back if: the `markdown` crate gives no position for a destination (see Stop conditions)

### S6: blob hashes in list

- Purpose: make `kotowari list` walk the guide, overview data and table-of-contents places when there are two or more languages and print `translations` in JSON and text.
- Specification: docs/ir/core/list.md#REQ-core-152, docs/ir/core/list.md#REQ-core-155
- Prerequisites: S2
- May change: crates/kotowari-core/src/list.rs, crates/kotowari/src/, src/cli.rs, src/cli/, tests/step10_list.rs
- Done when: a test marked for REQ-core-155 shows `translations` with `path` and `sides` (null blob for a missing side) in JSON and the one-line text form, a project without `languages` prints no `translations`, and every existing list test passes unchanged
- Shown by: test — process-level tests in tests/step10_list.rs marked `@kotowari[REQ-core-152, REQ-core-155]`, written first and seen failing, then passing
- Left to the implementer: none
- Stop and hand back if: an existing list test pins that the JSON top level has only `items` for a project with one language

### S7: the view draws in the language it is given

- Purpose: take the language tag, the UI text and the other languages as render input, set lang, draw every view-written string from the UI text with "{n}" replaced, link to the same page in the other languages, draw body-less references as names only, and use the English status tags.
- Specification: docs/ir/view/languages.md#REQ-view-026, docs/ir/view/languages.md#REQ-view-027, docs/ir/view/languages.md#REQ-view-028, docs/ir/view/languages.md#REQ-view-029, docs/ir/view/languages.md#REQ-view-030, docs/ir/view/languages.md#TBL-view-002, docs/ir/view/rendering.md#REQ-view-001, docs/ir/view/rendering.md#REQ-view-008, docs/ir/view/rendering.md#REQ-view-009, docs/ir/view/navigation.md#REQ-view-016, docs/ir/view/navigation.md#REQ-view-017, docs/ir/view/navigation.md#REQ-view-019, docs/ir/view/navigation.md#REQ-view-022, docs/ir/view/parts.md#TBL-view-001
- Prerequisites: the baseline above passes
- May change: crates/kotowari-markdown-view/src/, crates/kotowari-markdown-view/schemas/status.json, crates/kotowari-markdown-view/tests/, crates/kotowari-markdown-view/examples/render.rs, crates/kotowari-markdown-view/README.md, and in crates/kotowari-overview/src/, crates/kotowari-overview/tests/overview.rs and crates/kotowari-overview/examples/overview.rs only the minimal adaptation that keeps the workspace building (English UI text, no other languages, every reference with a body)
- Done when: the tests marked for EX-view-018 to EX-view-023 pass, the tests of EX-view-011, EX-view-012 and EX-view-015 pass with the UI text their scenarios now give, and no string a reader sees in a page comes from a literal in the view's source
- Shown by: test — new tests in crates/kotowari-markdown-view/tests/ (for example a new languages.rs) marked with each of EX-view-018 to EX-view-023 and its requirement IDs, and the existing navigation and rendering tests updated to pass UI text, each new test written first and seen failing, then passing
- Left to the implementer: the shape of the new input fields, the placement and look of the language links
- Stop and hand back if: a page needs a string that none of the TBL-view-002 keys covers

### S8: one set of overview pages per language

- Purpose: build the render input per language from that language's sides, with per-language reference bodies, UI text and other-language links, write the first language at the cache root and the others under "<tag>/", and stop the build on missing sides or skeleton mismatches.
- Specification: docs/ir/core/overview-languages.md#REQ-core-353, docs/ir/core/overview-languages.md#REQ-core-354, docs/ir/core/overview-languages.md#REQ-core-355, docs/ir/core/overview-commands.md#REQ-core-291, docs/ir/core/overview-commands.md#REQ-core-293, docs/ir/core/overview-commands.md#REQ-core-294
- Prerequisites: S1, S3, S4, S7
- May change: crates/kotowari-overview/src/, crates/kotowari-overview/tests/overview.rs, crates/kotowari-overview/examples/overview.rs, crates/kotowari/src/overview.rs, crates/kotowari/src/, tests/step19_overview_build.rs, tests/step21_translations.rs
- Done when: the tests marked for EX-core-536, EX-core-540, EX-core-541 and EX-core-542 pass, a build with a missing other-language side of an IR document stops without writing, and every existing build and serve test passes with the files they expect
- Shown by: test — process-level tests marked `@kotowari[REQ-core-351, EX-core-536]`, `@kotowari[REQ-core-353, REQ-core-355, EX-core-540]`, `@kotowari[EX-core-541]`, `@kotowari[REQ-core-354, EX-core-542]`, and `@kotowari[REQ-core-294]` for the stop, each written first and seen failing, then passing
- Left to the implementer: how the per-language inputs are assembled and passed through kotowari-overview
- Stop and hand back if: writing "<tag>/" pages conflicts with the deletion rule of REQ-core-293 for files left from an earlier build with other languages

### S9: guides, skill references and changelog

- Purpose: make the user guides, the kotowari skill and the changelog describe `languages`, `labels`, the new findings, `translations` in `list`, per-language overviews and the English status tags, and give the skill the pair procedure.
- Specification: docs/ir/core/translation-structure.md#REQ-core-350, docs/ir/core/findings.md#TBL-core-008, docs/ir/core/config.md#TBL-core-004, docs/ir/core/list.md#REQ-core-155
- Prerequisites: S1 to S8
- May change: docs/guides/, agent/skills/kotowari/, CHANGELOG.md
- Done when: the texts state the behavior of S1 to S8, the skill holds every point listed in REQ-core-350's how_to_verify, and `kotowari check` reports no guide_stale or invalid_marker
- Shown by: check — `kotowari check --format json | jq '[.findings[] | select(.path | test("docs/guides/|agent/skills/"))]'` prints an empty list, and the test of REQ-core-125 passes
- Left to the implementer: wording, and where in the skill the pair procedure sits
- Stop and hand back if: a guide mark in these texts points at an item this plan does not cover and its section does not change

### S10: change records and the final checks

- Purpose: record and independently review the branch's changes against the comparison base, and confirm the plan's own items are done.
- Specification: docs/ir/core/translation-pairs.md#REQ-core-334, docs/ir/core/translation-structure.md#REQ-core-344, docs/ir/core/translation-structure.md#REQ-core-350, docs/ir/core/overview-languages.md#REQ-core-353, docs/ir/view/languages.md#REQ-view-026
- Prerequisites: S1 to S9
- May change: .kotowari/changes/implementation.yaml, .kotowari/changes/review.yaml, .kotowari/equivalents.yaml
- Done when: `kotowari query` shows a nonempty tests list for every requirement and scenario in the Verification map except REQ-core-350; `kotowari check` reports no error on a file the branch changed or on those IDs; a reviewer has confirmed REQ-core-350 by its how_to_verify; both records are written by their own roles and `kotowari changes --base <base> --head <head> --phase review` exits 0 at the committed head
- Shown by: check — the test command, then `kotowari check --format json`, then `kotowari changes --base <base> --head <head> --phase review --format text`
- Left to the implementer: none
- Stop and hand back if: mutation testing in the pre-push hook finds survivors that no test of this plan's requirements can catch without pinning markup or wording the specification does not declare
