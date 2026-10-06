# Changelog

Changes to kotowari that its users can notice. The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and versions follow [Semantic Versioning](https://semver.org/).
The skills in `agent/skills/` ship under the same tags as kotowari, so changes to the skills are written here too. Changes to the bundled `kotowari-mds` are not written here.

## [Unreleased]

### Added

- When the improvement loop stops making progress, `kotowari-cycle` now questions the premise before it stops. If fixes keep raising new findings next to the ones they closed — or a finding stays present, a closed cause returns, or new findings do not shrink — the cycle writes down the premises its fixes shared, from the most specific up, replaces the top one and tries once more: through the fixer when the premise lies in the implementation, through the consistency phase when it lies in the IR or a decision record and grounds can settle it. Only a premise no grounds can settle goes to you, with the list of premises in the final report. A second firing after the attempt stops the loop as before. A new stop condition joins the others: a new finding overlapping, in the same file and lines, the finding the fix before it addressed, for two fixes in a row. Each attempt is recorded in the findings file (`premise_attempts`), so a resumed cycle does not try the same firing twice. `kotowari-iterate` and a direct fix in the main session take the same step, and `kotowari-implement` replaces its own premise once before handing back to the cycle.

### Changed

- The workflow skills are proofread: rules written in several places now live in one and are pointed to from the others, descriptions say only when to use each skill, and text a caller follows is kept apart from text pasted into a delegate's prompt. What you can notice: the contract the cycle pastes into its fixer and consistency phase is `kotowari-cycle/references/editing-contract.md`; the Evidence conditions live only in `kotowari-review/references/oracle-evidence.md`, with their four conditions numbered; how a caller launches optional seats is in `kotowari-review/references/optional-seats.md`; and `kotowari-brainstorm/references/records.md` is removed, its record kinds now in the skill's body. A finding raised by a diff review is now matched against closed findings like any other, and a finding kept only for the record no longer counts toward the "still present twice" stop.

## [0.5.0] - 2026-10-06

### Removed

- **BREAKING** The `kotowari changes` command and change records are removed. kotowari now has seven commands. The `changes` section of the configuration (`changes.files`, `changes.exclude`, `changes.records`) is gone, and a configuration that still has it stops with a configuration error as an unknown key: delete the `changes` section from `.kotowari/config.yaml` and delete `.kotowari/changes/`. `check` and `status` no longer read change records, the finding kinds `change_uncovered`, `change_stale`, `change_ir_stale`, `change_conclusion_conflict`, `change_deferred` and `change_record_invalid` are gone, and so is the `git error` stop.
- **BREAKING (Rust API)** The public API that served only `changes` is removed. From `kotowari`: `Project::changes`, `AsyncProject::changes`, `ChangesOptions`, `ChangesReport`, `Target`, `Phase`, `ChangesConfig`, `Comparison` and `ErrorKind::GitFailure`. From `kotowari-core`: the modules `changes`, `change_records` and `comparison` (with `Comparison`), `ChangesConfig` and the field `changes` of `Config`, `CheckInputs` and `RepositoryCheckInputs`, `RepositoryInspectionPreparation::changes`, the `FindingKind` variants `ChangeStale`, `ChangeIrStale`, `ChangeUncovered`, `ChangeDeferred`, `ChangeConclusionConflict` and `ChangeRecordInvalid`, and `StopReason::GitError`. A repository inspection now starts at the guides phase. The glob set builder that `check` still uses is `kotowari_core::config::glob_set`.

### Added

- The `kotowari-cycle` skill runs a consistency phase: once after implementation and once after the quality review's fixes converge, a separate agent reads the code the diff changed against the IR, finds behaviour the IR does not state, behaviour different from it and gaps or contradictions in the IR around it, and resolves each on grounds and measurements into a state where the IR and the code agree. It fixes and commits the IR, the decision record, the flag record and the code itself, and writes what it decided to a new decision record. Its findings go into the cycle's findings file beside the review's, with the perspective `consistency`, and the phase is rerun and stopped by the same rules as the review loop. A question of meaning the grounds cannot settle continues with a default and stays open for the final report with the word that reverses it; the phase never stops to hand it back to a person. A run is skipped only when the diff adds or changes no behaviour a user can observe (skill text counts as behaviour), with the reason in the final report. `kotowari-iterate` runs it through the cycle, and a direct fix in the main session runs it by the same measure.

### Changed

- `kotowari-review` launches reviewers with the quality perspective only. There is no conformance reviewer comparing code with the specification any more; that is the consistency phase's. Whether a plan or a document contradicts the specification is still read within the quality perspective.
- The skills no longer write or ask for change records. The implementer and the fixer leave IR-side findings to the consistency phase instead of adding IR themselves.
- Workflow skills share the editing and review hand-off rules instead of repeating conflicting fixer instructions in the consistency phase. Delegates read writing and marker references only when needed, and direct edits use the same finding and stopping rules as cycle.

## [0.4.0] - 2026-10-06

### Changed

- The minimum supported Rust version (`rust-version`) of every crate is now 1.99. Crates that declared 1.89 have a higher floor, and `kotowari`, `kotowari-core`, `kotowari-source-analysis`, `kotowari-overview` and `kotowari-cli`, which declared none, now declare 1.99.

- The README, the guides and the IR are now pairs of English and Japanese. `README.md` and `docs/guides/*.md` without a suffix are English, `*.ja.md` is Japanese, and the line under each page's title switches between them. The guides no longer have the "why it is built this way" sections that explained the history of decisions (the history is in the decision records).

- The `kotowari` skill now states more clearly how guides and overviews divide the work: a guide explains usage to the people who use the product, and an overview explains decisions, reasons, plans and open questions to the people who build it. An overview shows behaviour briefly as the result of a decision, and commands, configuration and usage steps go in the guides.

- **BREAKING (Rust API and package layout)** The root package is now `kotowari-cli`, and `kotowari` is split out as a library operated from an explicit starting point. In-memory computation uses `kotowari-core`, and source analysis uses `kotowari-source-analysis`. Code that called the old core's CLI or acquisition API needs to migrate. The binary's commands, output and exit codes are unchanged.

- The `kotowari` skill has a new "What the IR holds" section stating that the IR holds only behaviour a user of the product can observe. CI and workflows, hooks, release steps, build configuration, the repository's own data, and the project's own tests and checks are not written in the IR; they go in the decision records and carry neither requirements nor tests. When in doubt, leave it out, and adding to the IR to justify a test is forbidden. Brainstorm, plan, implement, cycle, review and iterate refer to this definition.
- The change conformance reference now states that a change to CI, hooks, release or build configuration alone is recorded as a `new` entry citing a decision, with `ir` and `requirements` empty, and is not a `missing_spec` gap.
- An element of `tests.files`, `guides.files`, `surface.files` or `overview.files` that starts with `/` now stops with a configuration error, as an absolute-path value. Such an element never matched anything, since globs are matched against paths relative to the base directory.

### Added

- `kotowari check --allow-test-findings`, a flag without a value. It leaves test-side findings out of the exit code, so the commit that approves a specification ahead of its tests can pass a pre-commit hook: `requirement_without_test`, `scenario_without_test` and `test_without_id`; `invalid_marker` and `unresolved_reference` on a test file; and `unparsable_file` on a test file that is not a surface file. Every other error still exits with 1, and the findings and output are the same as without the flag. Run it with the flag in the pre-commit hook and without it in the pre-push hook and in CI; the guide to check, the `kotowari` skill's findings reference and the `kotowari-adopt` skill now say so. Only `check` takes the flag; any other command stops with an argument error.

- An introduction page, in English and Japanese, explaining how specifications, decision records and tests connect and what the mechanical checks cover.

- `kotowari overview build` and `kotowari overview serve`. They check the overview data matched by `overview.files` in the configuration (frontmatter `ir`, a title, a leading `lead` part, and Markdown with sections and parts), and when there are no errors write an index and one HTML page per overview under `.kotowari/cache/overview/`. serve serves them at `http://127.0.0.1:<port>/` (4590 by default, changed with `--port`) and ends on Ctrl-C. Errors in the overview data stop with `overview error` without writing anything; a port that cannot be used, or a failure to accept connections while serving, stops with `port error`. When `.kotowari`, `.kotowari/cache` or `.kotowari/cache/overview` is a symbolic link or not a directory, they stop with `cache error` without writing or deleting anything, and so does a failure to create, write or delete in that location.
- With the `overview` key in the configuration, `kotowari check` and `kotowari status` read the overview data and report errors in its form, part contents, leading `lead`, IR documents covered, references inside parts and overlapping page names (`overview_form_invalid`, `overview_part_unknown`, `overview_part_invalid`, `overview_lead_missing`, `overview_ir_missing`, `overview_ir_shared`, `overview_ref_unresolved`, `overview_name_conflict`), along with stale guide marks in sections of the overview data. The JSON of check and status always has the `overview` group (`files` and `marks`), both 0 without the key.
- The overview index is drawn from a table of contents. `overview` in the configuration now also requires `overview.toc`, pointing at the table of contents YAML file. The index uses the table of contents' title as its heading and draws its groups (`title`, an optional one-line `note`, and `items`) in the written order and nesting, as collapsible blocks. Cards show the number of unreviewed sections, open questions and plans, and group headings show the page count and the totals of unreviewed sections and open questions. Every page gets its position in the table of contents and links to the other pages in the same group. Errors in the table of contents' form, pages missing from it, names without overview data, second and later occurrences of a name, and empty groups are `kotowari check` and `kotowari status` errors (`overview_toc_invalid`, `overview_toc_page_missing`, `overview_toc_page_unknown`, `overview_toc_page_duplicate`, `overview_toc_group_empty`) and stop build and serve. A missing or unreadable table of contents stops, and a table of contents matched by the scan of `overview.files`, `guides.files` or `tests.files` stops with a configuration error. With no overview data at all, no table of contents can pass, so write the `overview` key together with the first overview data and the table of contents.
- `overview_prepare`, which writes nothing, and `overview_build`, which then writes, on `Project` and `AsyncProject`. `check`, `status` and `inspect` include the findings on the overview data and the `overview` group. A new `kotowari-overview` crate performs the overview checks.
- A synchronous `Project`, `ReadModel` and `Inspection` that are kept and reused, and `AsyncProject` behind the `tokio` feature, disabled by default.
- A procedure for verifying the actual release archives of the nine crates in an independent offline environment. The version numbers and the release names of the two products are unchanged.
- IR, guides and overviews can be kept as multilingual pairs. Listing two or more language tags in `languages` in the configuration keeps the IR (glossaries and flag records included), guides, overview data and tables of contents as pairs `foo.md` and `foo.<language tag>.md` in the same directory, with the git blob hash of each side recorded in `foo.i18n.yaml` beside them. `kotowari check` and `kotowari status` report a missing side (`translation_missing`), an error in the consistency record (`translation_record_invalid`), a hash differing from the record (`translation_stale`), a mismatch in the non-sentence skeleton (`translation_structure_mismatch`), an error in the switcher line after the title (`translation_switcher_invalid`), a link to another language's side (`link_language_mismatch`) and a link to a decision record (`link_to_record`) as errors. The other language sides of the IR are checked for terms, vague words, document name references, unclosed backticks and the glossary's form against that language's glossary. `kotowari list` outputs the blob hash of every side of each pair in the top-level `translations`. Overviews are drawn per language: pages of languages other than the first are written under `.kotowari/cache/overview/<language tag>/` and link to each other, and a missing side or a skeleton mismatch in a pair of IR, overview data or table of contents stops without writing. The overview UI text is held in English by kotowari itself and written for other languages in `labels` in the configuration (an error in its form stops with a configuration error), and the labels of the `status` part are now `decided`, `planned`, `open` and `dropped`. The `kotowari` skill has a new procedure for keeping pairs aligned. Without `languages`, everything stays English only, as before, and no pairs are read.

### Fixed

- `kotowari overview serve` returns a 403 with no body to requests whose Host header is not `127.0.0.1:<port>` or `localhost:<port>` (including requests without Host). An outside site could point its own name at 127.0.0.1 (DNS rebinding) and read the overview pages through the browser.
- In a repository with two or more languages in `languages`, `kotowari changes` also read the other language sides (`foo.ja.md` and the like) as IR, and treated a record naming the first language's side as `change_record_invalid`. It now reads only the first language's side as IR, the same as `kotowari check`.
- In multilingual pairs, reference-style links and images are also checked on the line where they are used, and a mismatch in the sequence of link destinations is detected. A configuration where only the translated side of an overview overlaps the test glob also stops with a configuration error.
- The setup steps of the `kotowari` skill now say to create a `.gitkeep` in the directory for decision records and include it in the commit. Otherwise the empty directory is lost in a clone or worktree and `kotowari check` fails.
- Pages from `kotowari overview build` no longer draw GFM footnote syntax (`[^1]`) as footnotes. Drawing them put the English heading "Footnotes" and back-link characters even on Japanese pages.
- `kotowari overview build` now removes only HTML comments from headings and titles. Before, parts of a line read as HTML, such as `<String>` in `Vec<String>`, were removed too, and the heading became "Vec".
- `kotowari changes` no longer reads hidden directories under the places for the IR and decision records (such as `docs/ir/.drafts/`), the same as `kotowari check`. Before, it judged a change record citing IR there as valid, while `check` reported the same record as `change_record_invalid`.
- When the same requirement ID is in two IR documents, `kotowari check` now takes the document earlier in path order as the IR defining it for change records (the same as `kotowari changes`). Before, it took the later document and reported records citing the earlier one as `change_record_invalid`.
- Fixed the library's in-memory entry points (`ReadModel::build`, `Inspection::build`) failing to find the title and the language switcher line of a document starting with a BOM, which reported the switcher line as `translation_switcher_invalid` and also counted it as a scope line of the document. `kotowari check`, which reads from files, has always skipped the BOM.
- `kotowari changes` now starts the detail of an error in the target configuration file with the configuration file's relative path, the same as `kotowari check` (`config error: .kotowari/config.yaml: ...`).
- `kotowari changes` with `--port` now stops with an argument error, like the other commands. Before, it silently ignored it and ran the check.
- Fixed a panic inside the library when `changes.records` or `overview.files` had a brace glob spanning the `/` separator (such as `{docs/.changes,docs/changes}/*.yaml`).
- `kotowari overview build` and `kotowari overview serve` now stop with `cache error` when they cannot enumerate the files under `.kotowari/cache/overview`, like the other failures in that location. Before, they stopped with `unreadable file`.

## [0.3.0] - 2026-10-01

### Added

- The distributed skills now write back to the decision records, keep YAML conformance records for both roles, check the whole branch after an independent review, and say where gaps in the specification go. Delegated work may add concrete IR within the scope that keeps approved requirements; changes, deletions and judgments of meaning without grounds go back to a person.
- `kotowari changes` enumerates the changes between a given Git base and a commit or the index, and checks the implementation and review conformance records and the freshness of their contents. `changes` in the configuration is optional; once adopted, check and status also check the records' form and valid references.

## [0.2.0] - 2026-09-27

### Added

- The README's installation section has a way to install with mise's `github:` backend (`mise use -g 'github:ba0918/kotowari[version_prefix=kotowari-v]@0.1.0'`).
- Surface checks. `surface.files` and `surface.rules` (ast-grep rules) in the configuration pick out the surfaces of the code that users see (CLI subcommands, flags and the like), and `kotowari check` reports a `surface_without_spec` error when a name is not quoted in a requirement's statement, a decision table cell or a scenario step of the IR. Only files in the rules' languages are read among the surface files, so a broad `src/**` matching images and the like does not stop. Without rules, nothing happens.
- A surface not yet in the IR can be excluded by listing it, with a reason, in the list `surface.unspecified` points at. A malformed entry is a `surface_unspecified_invalid` error, and an entry no longer needed is a `surface_unspecified_stale` notice. The number excluded appears on the last line of `check` as `surface: unspecified=<n>` (`surface` in JSON), and the number of surfaces on the `surface` line of `status`.
- The `kotowari` skill has a new reference `surface.md` for surface checks, and `kotowari-adopt` explains how to shrink the list. Only `kotowari-brainstorm` and `kotowari-adopt` add to the list; the implementer sends a surface missing from the IR back to the brainstorm.

### Changed

- `kotowari-adopt` also confirms the topic's user entry points (commands and screen operations) when confirming the scope, and lists only behaviour observable from those entry points. Behaviour found outside the scope is shown only as a count and candidates for the next topic. One row is one candidate requirement; rows differing only in values or wording are merged into one row as a candidate decision table, and when the candidates exceed `limits.requirements`, it checks whether entry points are mixed.
- The output of `kotowari status` always has the `surface` group (`total`, `specified`, `unspecified`). Without surface rules, all three are 0.

## [0.1.0] - 2026-09-26

### Added

- `kotowari`, a CLI for writing a specification as Markdown in a fixed form called the "IR" and checking it mechanically. Output is JSON by default; add `--format text` for people to read.
- `kotowari check`: checks the IR's form, whether the decision records cited as sources of requirements exist, and which tests verify which requirements, and reports findings. The exit code is 0 for no findings, 1 for errors, and 2 when the check could not start. It also checks whether the marks placed in user guides disagree with the current IR.
- `kotowari list`: lists the IR's items and scenarios together with the marked tests.
- `kotowari query <ID>`: outputs the body of one item, the tests verifying it, and the items pointing at it.
- `kotowari status`: counts whether everything is in place and answers with `complete true` or `complete false` on the last line.
- `kotowari plan <file>`: checks the form of an implementation plan file against the bundled schema.
- `kotowari mutants --tool cargo-mutants <results file>`: reads mutation test (cargo-mutants) results and reports missed mutants as findings.
- Test marks `@kotowari[ID]` are read with bundled rules in Rust, TypeScript, JavaScript, Python and PHP tests. Other languages that ast-grep (tree-sitter) handles can be read by writing rules in `tests.rules` in the configuration.
- Ten Claude Code skills in `agent/skills/`. `kotowari` teaches how to write the IR and decision records, fix findings and place marks. The nine starting with `kotowari-` run the workflow from brainstorm through plan, implementation and review on top of kotowari (optional).
- Two ways to install: prebuilt binaries on GitHub Releases (Linux x86_64 and macOS arm64, with SHA256), and `cargo install --git https://github.com/ba0918/kotowari --tag kotowari-v0.1.0 kotowari` with the version pinned by tag. Passing the tag to `--pin` of `gh skill install` installs the skills at a pinned version.

[Unreleased]: https://github.com/ba0918/kotowari/compare/kotowari-v0.5.0...HEAD
[0.5.0]: https://github.com/ba0918/kotowari/compare/kotowari-v0.4.0...kotowari-v0.5.0
[0.4.0]: https://github.com/ba0918/kotowari/compare/kotowari-v0.3.0...kotowari-v0.4.0
[0.3.0]: https://github.com/ba0918/kotowari/compare/kotowari-v0.2.0...kotowari-v0.3.0
[0.2.0]: https://github.com/ba0918/kotowari/compare/kotowari-v0.1.0...kotowari-v0.2.0
[0.1.0]: https://github.com/ba0918/kotowari/releases/tag/kotowari-v0.1.0
