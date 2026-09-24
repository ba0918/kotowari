# Plan: report the words a document uses too often

## Goal

A person who runs `wordcount check <path>` sees each word used more than the configured limit, with the line of its first use.

## Specification

The IR is in `docs/ir/`. This plan covers:

- `docs/ir/core/count.md#REQ-core-010`, `#REQ-core-011`
- `docs/ir/core/output.md#REQ-core-020`
- Examples: EX-core-030, EX-core-031

## Approach and why

Counting and reporting are split: a pure function counts words per document, and the command only reads the file and prints. The existing `read_text` helper already stops on unreadable and non-UTF-8 files, so the command reuses it instead of opening the file itself.

## Scope of change

- `src/count.rs` (new)
- `src/main.rs`
  - the `check` arm only
- `tests/count.rs` (new)

## Step order and prerequisites

S1 before S2: S2 prints what S1 counts.

## Verification map

| Step | Requirements | Examples |
|---|---|---|
| S1 | REQ-core-010, REQ-core-011 | EX-core-030 |
| S2 | REQ-core-020 | EX-core-031 |

## Left to the implementer

- The names of the counting function and its result type

## Stop conditions

- The limit turns out to need a per-word setting, which the specification does not define
- An existing test fails for a reason other than the new command

## Test command

```sh
cargo test --workspace
```

## Out of scope

- Stemming or case folding beyond what REQ-core-011 states

## Steps

### S1: count the words of one document

- Purpose: count each word of a document and remember the line of its first use
- Specification: `docs/ir/core/count.md#REQ-core-010`, `docs/ir/core/count.md#REQ-core-011`
- Prerequisites: none
- May change: `src/count.rs`, `tests/count.rs`
- Done when: counting a document with "a" three times and the limit 2 returns "a" with the line of its first use, and a document within the limit returns nothing
- Shown by: test — EX-core-030, one test per rule of REQ-core-011
- Left to the implementer: none
- Stop and hand back if: REQ-core-011 and EX-core-030 disagree on what counts as a word

### S2: print the words over the limit

- Purpose: wire the count into `wordcount check` and print one line per word over the limit
- Specification: `docs/ir/core/output.md#REQ-core-020`
- Prerequisites: S1
- May change: `src/main.rs`, `tests/count.rs`
- Done when: `wordcount check a.md` prints `a.md:3 a` for EX-core-031 and exits with 1, and exits with 0 when nothing is over the limit
- Shown by: test — EX-core-031
- Left to the implementer: none
- Stop and hand back if: the exit code for a document with no words is not decided by REQ-core-020
