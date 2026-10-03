# Public-crate API implementation planning

## Context

The public-crate API specification was committed at 922c19bb128d5e8e0d4b0e83398c4d53a1e05a4d.
The [approved decisions](./2026-10-03-public-crate-api.md#A34) define seven packages and preserve both CLI contracts.
The [implementation plan](../../plans/public-crate-api.md) needs an order that keeps callers buildable and a package check that does not require publishing workspace dependencies.
These are planning choices within the approved behavior; implementation and independent review start only after plan approval.

Position: Plan draft prepared for parent-owned independent review and approval. No implementation or release has run.

## Agreements

- A1 Migrate the schema API first, separate its I/O and binary next, introduce core-owned memory inputs and comparison types, then move kotowari orchestration, source analysis and CLI code together. Add optional async adapters only after both synchronous entry points pass their contract tests.
  - why: Current core calls schema APIs directly, while source-analysis must depend on core rather than the reverse. Introducing shared input types before moving their producers avoids a dependency cycle. The existing CLI tests remain the compatibility oracle throughout.
  - decided_by: Plan author under [D1](./2026-10-03-public-crate-api.md#D1)
- A2 Preserve the root Cargo.toml package version as the kotowari-series canonical value and crates/kotowari-markdown-schema/Cargo.toml as the schema-series canonical value. Keep 0.3.0 and 0.1.0 respectively. Update every following package declaration, internal dependency version and lockfile entry when its dependency's series changes.
  - why: [A15](./2026-10-03-public-crate-api.md#A15), [A28](./2026-10-03-public-crate-api.md#A28), [A34](./2026-10-03-public-crate-api.md#A34) and [A48](./2026-10-03-public-crate-api.md#A48) require independent series and updated automation, but do not require changing the established canonical locations or making a release.
  - decided_by: Plan author under [D1](./2026-10-03-public-crate-api.md#D1)
- A3 Use Cargo's multi-package packaging support to produce original .crate artifacts, then build their unmodified extracted manifests in an isolated, offline directory-source simulation containing those artifacts and Cargo-vendored external dependencies. Record artifact checksums and compare all resolved versions with the locked workspace graph. Run external consumers against the same artifact source.
  - superseded_by: [A8 operational staging correction](#A8)
  - why: [A46](./2026-10-03-public-crate-api.md#A46) requires actual standalone package builds before publication. Installed Cargo 1.98.1 documents --workspace packaging and lockfile preparation for interdependent packages; cargo vendor and directory sources provide dependency resolution without an extra tool. Python's standard library supplies archive extraction, TOML reading and checksums. This is a local prepublication simulation, not evidence that these versions already exist on crates.io. The package helper must fail if Cargo cannot resolve the staged packages or if a workspace path escapes into a standalone build.
  - rejected: cargo package --list alone; publish --dry-run against unavailable registry versions; patching manifests to point back at the working tree; installing an unapproved registry tool.
  - decided_by: Plan author under [D1](./2026-10-03-public-crate-api.md#D1)
- A4 Move the four embedded YAML schemas into crates/kotowari-core/schemas and retain their former .kotowari/schemas paths as relative symlinks to the canonical files.
  - superseded_by: [A11 canonical review paths without legacy links](#A11)
  - why: [A46](./2026-10-03-public-crate-api.md#A46) forbids package-external embedded assets and duplicate content. Existing review instructions in [the form contract](../../ir/core/form-contract.md) and [IR documents](../../ir/core/ir-document.md) name the old paths. Links preserve those read paths without copying bytes or revising approved IR. The package contains regular canonical files and does not rely on the links.
  - decided_by: Plan author under [D1](./2026-10-03-public-crate-api.md#D1)
- A5 Split existing mixed test files by responsibility. Keep process-level CLI tests with their binary package; move library tests to their owning package and keep private-helper tests inside private modules. Make hooks and automation select the full workspace explicitly.
  - why: [A47](./2026-10-03-public-crate-api.md#A47) requires package-level library tests. Root tests currently import core and also run kotowari, while schema integration tests locate kotowari-mds via cargo_bin. Leaving them in place would either miss tests or require exposing private APIs.
  - decided_by: Plan author under [D1](./2026-10-03-public-crate-api.md#D1)
- A6 Capture the approved-plan HEAD once when starting branch public-crate-api as the branch-wide comparison base. Use an immutable final HEAD and separately authored implementer and reviewer records for final conformance.
  - why: The specification commit is an approval anchor, not the implementation branch base. [Change conformance](../../guides/change-conformance.md) requires the entire branch comparison and independent review of the same bytes. Existing unrelated specification gaps can block integration without extending this plan.
  - decided_by: Plan author applying PROJECT.md change-conformance rules
- A7 The parent session orchestrates implementation and fixing with openai/gpt-6.1-sol after approval and obtains separate-context review. The plan author does not implement or self-approve.
  - why: This is the user's execution instruction for this task, not a product requirement or a permanent executor assignment. No model variant was requested.
  - decided_by: User through the parent task instruction
- A8 Vendor locked external dependencies first, create original archives with native cached offline Cargo workspace staging under a separate configuration without source replacement, then apply the isolated directory-source replacement for standalone builds, tests and consumers.
  - why: Cargo 1.98.1 rejects the initial replacement-source staging command. Naming crates-io explicitly gets past that rejection but fails to resolve the staged schema dependency through the vendor directory. The corresponding native offline staging probe produced all seven original archives with exit 0. Delaying replacement preserves original artifact bytes, locked external version/checksum checks, offline standalone resolution and the no-upload/no-patch guarantees of [A46](./2026-10-03-public-crate-api.md#A46). This corrects only the verification method in [the implementation plan](../../plans/public-crate-api.md#verification-map); the original approved bytes did not contain it.
  - rejected: treating the diagnostic archives as already independently verified; copying working-tree packages into the source simulation; editing shipped manifests or regenerating a failing shipped lockfile.
  - decided_by: Parent adjudication after observing both staging failures and the successful native offline probe, within approved [D1](./2026-10-03-public-crate-api.md#D1).

## Agreements

- A9 Keep native filesystem spellings at the acquisition boundary and use canonical logical paths only for caller-provided memory input. Use private iterator boundaries to feed acquired source spellings to the shared calculations and rule matching without a public normalization switch.
  - why: Existing walkers have different separator behavior. Canonicalizing a literal Unix backslash before source matching changes the existing source_invalid result and configured test-rule matching. The retained CLI regression and source/guide/change-record tests demonstrate that keeping each walker's established spelling preserves behavior, while memory-input tests continue to require canonical SourceText paths.
  - rejected: imposing a new global normalization policy; duplicating validation; adding a public normalization mode.
  - decided_by: Implementer within [D1](./2026-10-03-public-crate-api.md#D1).
- A10 Keep a small private bounded blocking scheduler in each of the two async adapters, sharing a semaphore across clones and carrying its owned permit into the worker. Keep validated schema deserialization private and expose retained calculation results through read-only wrappers.
  - why: A shared scheduler crate would violate the seven-package graph. Moving the permit into the worker preserves the bound after caller cancellation. Semaphore-queued runtime shutdown supplies an observable task submission failure without fabricating an unreachable error. Private RawSchema validation prevents unchecked deserialization; read-only wrappers reuse existing algorithms and preserve lower-level type identity without exposing third-party parser types.
  - decided_by: Implementer within [D1](./2026-10-03-public-crate-api.md#D1).

## Agreements

- A11 Remove the four legacy schema symlinks and update only the schema-location text in the verification references of [REQ-core-179](../../ir/core/form-contract.md#REQ-core-179) and [REQ-core-041](../../ir/core/ir-document.md#REQ-core-041) to the canonical crates/kotowari-core/schemas paths.
  - why: The selected legacy symlinks make the required branch snapshot stop under the existing [REQ-core-266](../../ir/core/changes-inputs.md#REQ-core-266) unsupported-mode rule. Removing them keeps one canonical copy of the original schema bytes and preserves symlink rejection, change selection, exclusions and all gates. The user expressly authorized only these two verification-reference path updates, without changing requirement behavior or unrelated overview.yaml references.
  - rejected: accepting changed selected symlinks; adding change exclusions; maintaining duplicate schema contents.
  - decided_by: User, accepting the parent's narrow legacy-path correction request.

## Agreements

- A12 Replace cross-crate result-assembly constructors with core-owned operations over acquired text and analysis facts. Preserve native acquisition spellings in explicit RepositoryReadInputs and RepositoryCheckInputs; derive documents, diagnostics and retained results inside core. Keep the parsed schema document and public diagnostic/result fields read-only, and preserve CLI JSON through CLI-owned conversion.
  - why: Hidden public constructors still let external callers combine unchecked internal documents with unrelated results. Computing from acquired input protects construction without changing the native-path compatibility demonstrated by A9. Private schema implementation modules remove the old bypasses; read-only accessors preserve inspection and lower-level type identity. Existing private-helper tests run as core unit modules rather than requiring public mutable internals. The same ownership moves change-record, plan and mutation-result assembly into pure core operations while the high-level package continues to own I/O.
  - rejected: treating doc-hidden signatures as private; exposing a replacement result constructor; changing path normalization or JSON output contracts; removing Serialize from legitimate public values solely because the CLI uses its own conversion.
  - decided_by: Implementer within [D1](./2026-10-03-public-crate-api.md#D1), applying [A40](./2026-10-03-public-crate-api.md#A40) and [V3](./2026-10-03-public-crate-api.md#V3).

## Revisions

The user adopted A11 to supersede A4's legacy-link layout after the whole-branch snapshot stopped on selected symlink modes. Only the two named verification-method path references are relaxed from the earlier approved-IR edit prohibition; all requirement behavior and the fixed comparison base remain unchanged.

The parent adopted A8 after implementation reached the package gate. A8 supersedes A3's operational staging prescription, while retaining its artifact and consumer verification obligations. The fixed branch-wide comparison base from A6 remains unchanged. Standalone validation is still mandatory; archive creation alone is not a completion claim.

## Reuse decisions

| Layer | Choice and ground |
|---|---|
| Schema parsing, validation and extraction | Adopt existing schema.rs, document.rs, validate.rs and extract.rs; change the public boundary around the existing algorithms. |
| IR, source references and result assembly | Adopt existing core modules and test fixtures; split acquisition from calculation instead of duplicating validators. |
| Source analysis | Adopt existing ast-grep queries, parsing and marker helpers; move them together behind Analyzer. |
| Filesystem, Git and HTTP | Adopt the existing readers, Git snapshot code, ureq client and cache rules; relocate them without replacing their behavior. |
| Private Markdown inspection in core | Adopt the existing markdown dependency and a private parser helper for guides and table inspection; a third-party AST need not cross the schema crate's public boundary. |
| Async scheduling | Adopt approved optional Tokio spawn_blocking and semaphore facilities, plus standard channels for deterministic tests; no new scheduler package. |
| Contract verification | Adopt existing Rust integration tests, proptest, tempfile, assert_cmd and loopback HTTP helpers; split ownership and extend only missing behavior. |
| Packaging | Adopt Cargo package/vendor and Python standard-library tarfile, hashlib, json, tomllib and subprocess; build a small repository helper to connect them and verify resolution evidence. |
| Version and release preparation | Extend existing check-versions.sh, release.sh, mutants.sh and workflow mappings; preserve the two existing version authorities. |
