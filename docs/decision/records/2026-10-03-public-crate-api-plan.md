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
  - why: [A46](./2026-10-03-public-crate-api.md#A46) requires actual standalone package builds before publication. Installed Cargo 1.98.1 documents --workspace packaging and lockfile preparation for interdependent packages; cargo vendor and directory sources provide dependency resolution without an extra tool. Python's standard library supplies archive extraction, TOML reading and checksums. This is a local prepublication simulation, not evidence that these versions already exist on crates.io. The package helper must fail if Cargo cannot resolve the staged packages or if a workspace path escapes into a standalone build.
  - rejected: cargo package --list alone; publish --dry-run against unavailable registry versions; patching manifests to point back at the working tree; installing an unapproved registry tool.
  - decided_by: Plan author under [D1](./2026-10-03-public-crate-api.md#D1)
- A4 Move the four embedded YAML schemas into crates/kotowari-core/schemas and retain their former .kotowari/schemas paths as relative symlinks to the canonical files.
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
