# kotowari

English | [日本語](README.ja.md)

kotowari is a CLI tool that checks specifications written in Markdown in a fixed format. Beyond the format of the documents, it also mechanically checks the links between specifications, decisions and tests.

When you have an LLM write a specification and then implement it, gaps like these tend to creep in:

- The specification contains decisions with no basis
- Requirements have no tests
- It is unclear which requirement a test is checking

With kotowari, you write the specification in normalized Markdown called the "IR". Running `kotowari check` confirms three things:

- Each requirement states where in the decision records the decision behind it is
- That decision actually exists
- Which tests check that requirement

kotowari is designed first and foremost for LLMs, so it outputs results as JSON by default. For people, switch to text output with `--format text`.

## Installation

Release tags have the form `kotowari-v0.1.0`.  
The commands below use version `0.1.0`; replace it with the version you want to install.

### mise

With [mise](https://mise.jdx.dev/), you can install the binary from the GitHub Release using `github:`.
Specifying `version_prefix=kotowari-v` strips the leading `kotowari-v` from the tag, so you can give the version as just `0.1.0`.

```console
$ mise use -g 'github:ba0918/kotowari[version_prefix=kotowari-v]@0.1.0'
$ kotowari --version
```

### Prebuilt binaries

Prebuilt binaries for Linux x86_64 and macOS arm64 are available on [GitHub Releases](https://github.com/ba0918/kotowari/releases).

The archives are named `kotowari-v<version>-<target>.tar.gz`, where `<target>` is `x86_64-unknown-linux-gnu` for Linux and `aarch64-apple-darwin` for macOS.
Each archive contains the `kotowari` binary, the README and the licenses, and comes with a `.sha256` file holding its SHA256 checksum.

```console
$ curl -LO https://github.com/ba0918/kotowari/releases/download/kotowari-v0.1.0/kotowari-v0.1.0-x86_64-unknown-linux-gnu.tar.gz
$ curl -LO https://github.com/ba0918/kotowari/releases/download/kotowari-v0.1.0/kotowari-v0.1.0-x86_64-unknown-linux-gnu.tar.gz.sha256
$ shasum -a 256 -c kotowari-v0.1.0-x86_64-unknown-linux-gnu.tar.gz.sha256
$ tar -xzf kotowari-v0.1.0-x86_64-unknown-linux-gnu.tar.gz
$ install kotowari-v0.1.0-x86_64-unknown-linux-gnu/kotowari ~/.local/bin/
$ kotowari --version
```

You don't have to install into `~/.local/bin/`; any directory on your `PATH` will do.

### Building from source

If you have a Rust toolchain, you can build and install from source with `cargo install`. Pass a release tag with `--tag` to pin the version. Since it builds locally, installation takes a while.

```console
$ cargo install --git https://github.com/ba0918/kotowari --tag kotowari-v0.1.0 kotowari
$ kotowari --version
```

## A minimal example
In the current source tree, the CLI packages are `kotowari-cli` and `kotowari-mds`.
To install from a checkout, use `cargo install --path . --bin kotowari` and `cargo install --path crates/kotowari-mds --bin kotowari-mds`.
The old-tag example above uses the old package name found at that tag.
To call kotowari from Rust, see the [public crate API](docs/guides/public-crate-api.md). The executable's commands, output and exit codes are unchanged.


By default, the configuration, the specification and the decision records live here:

```
.kotowari/config.yaml        configuration (may be empty or absent; defaults apply)
docs/ir/                     IR (the specification)
docs/ir/CONTEXT.md           glossary
docs/decision/records/       decision records
docs/decision/adr/           ADRs (may be empty)
```

If a requirement's text contains 「適切に」 (appropriately), 「必要に応じて」 (as needed), 「通常は」 (usually) or 「など」 (etc.), it is reported as a vague word.
These four words are checked by default; you can change them with `vague_words` in `.kotowari/config.yaml`.

First, write what you decided in a decision record, one decision per line. Here it is saved as `docs/decision/records/2026-01-01-login.md`.

```markdown
# Brainstorm record: login

## Context

Decide how to handle failed logins.

## Agreements

- A1 Lock an account for 15 minutes after five consecutive wrong passwords
  - why: slow down brute-force attacks
  - decided_by: user
```

Next, write the requirement in `docs/ir/login.md`. For its source, point at the decision you just recorded.

```markdown
# Login

Covers how failed logins are handled.

## Requirements

### REQ-001: Lock after consecutive failures

- kind: event_driven
- source: docs/decision/records/2026-01-01-login.md#A1
- verification: unit

When the password is entered wrongly five times in a row, the system locks that account for 15 minutes.
```

Finally, put an `@kotowari[ID]` mark on the test that checks the requirement.

```rust
// @kotowari[REQ-001]
#[test]
fn req_001_locks_after_five_failures() { /* ... */ }
```

With all of this in place, `kotowari check` prints nothing and exits with code 0.
Remove the mark from the test and run it again, and it reports these two findings:

```console
$ kotowari check --format text
docs/ir/login.md:7 [error] requirement_without_test REQ-001
src/lib.rs:2 [error] test_without_id req_001_locks_after_five_failures
```

Deleting or mistyping the decision number in the source produces a finding in the same way.

## Reference

For detailed usage, start from the [reference index](docs/guides/index.md). It covers the [configuration file](docs/guides/config.md), [how to mark tests](docs/guides/marks.md), [finding kinds and how to fix them](docs/guides/findings.md), and each command.

## Commands

| Command | What it does |
|---|---|
| [`kotowari check`](docs/guides/commands/check.md) | Checks the IR's format, that sources exist, and the marks on tests. Exit code 0 means no findings, 1 means there are errors, and 2 means the check could not start |
| [`kotowari list`](docs/guides/commands/list.md) | Lists the IR items and the marked tests |
| [`kotowari query <ID>`](docs/guides/commands/query.md) | Shows one item's text, its tests, and what refers to it |
| [`kotowari status`](docs/guides/commands/status.md) | Tallies how complete things are and prints `complete true` or `complete false` on the last line |
| [`kotowari mutants --tool cargo-mutants <results-file>`](docs/guides/commands/mutants.md) | Reports misses from the results of mutation testing (cargo-mutants) |
| [`kotowari plan <plan-file>`](docs/guides/commands/plan.md) | Checks, against the bundled schema, that an implementation plan file is written in the fixed format |
| `kotowari overview build` | Checks the overview data matched by the configuration's `overview.files` and the table of contents that `overview.toc` points at, and if there are no errors, writes under `.kotowari/cache/overview/` an index ordered and nested as in the table of contents, plus one HTML page per overview. Only changed files are written, and pages no longer produced are deleted. Available only when `overview` is set in the configuration |
| `kotowari overview serve [--port <PORT>]` | Writes the same pages as build, then serves them at `http://127.0.0.1:<PORT>/` (default 4590) until Ctrl-C |
| [`kotowari changes --base <REV> (--head <REV> \| --staged) --phase <implementation\|review>`](docs/guides/commands/changes.md) | Compares the Git diff between the base and the target against the change records, and reports changes that were not reconciled and records that have gone stale. Available only when `changes` is set in the configuration. |

Currently, marks on tests can be read only from Rust. The IR checks, however, work in projects that use other languages too.

## Using it with an LLM

`agent/skills/` provides Claude Code skills for using kotowari together with an LLM.

- `kotowari` teaches the LLM how to write the IR and decision records, how to fix findings, and where to put marks
- `kotowari-*` are optional skills that use kotowari to drive the workflow from brainstorming through planning, implementation and review

Installation instructions are in [agent/skills/README.md](agent/skills/README.md).

## Bundled tool

`crates/kotowari-mds` contains `kotowari-mds`, a general-purpose CLI that checks Markdown documents against a YAML schema.
kotowari reads the IR through this mechanism, but `kotowari-mds` can also be used on its own. See its [README](crates/kotowari-markdown-schema/README.md) for details.

## License

Licensed under either [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your option.
