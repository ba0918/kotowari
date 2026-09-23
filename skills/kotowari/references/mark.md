Based on the kotowari specification (revised 2026-09-23; the version of kotowari itself is not pinned)

## The form of a mark

Write it in the form `@kotowari[ID, ...]`.

| Part | Form |
|---|---|
| Start | @kotowari[ |
| Content | IDs separated by commas. Spaces may be placed before and after a comma |
| End | ] |

Several marks may be written on one line (all are picked up). The comment syntax does not matter, and marks are picked up from any position on the line. If there is no closing bracket on the same line, an invalid_marker error. Content that is empty or only separators is also an invalid_marker error.

Examples:

- `// @kotowari[REQ-001, TBL-002]` — binds to REQ-001 and TBL-002
- `// @kotowari[REQ-001] @kotowari[TBL-002]` — two marks on one line. Both are picked up

## Marks for examples

A mark can hold not only requirement IDs but also the IDs of examples (scenarios). When an example whose @about has a requirement with a verification other than review has no mark containing its ID, a scenario_without_test error is raised (not raised for an example with only review requirements, or one that names no requirement).

- `// @kotowari[REQ-001, EX-201]` — binds to both the requirement REQ-001 and the example EX-201
- `// @kotowari[EX-201, EX-202]` — one mark binding to two examples

A mark for an example also covers the requirements that example lists in `@about`. If there is a test with only `@kotowari[EX-201]`, requirement_without_test is not raised for the requirements in EX-201's `@about`. The reverse does not hold. A mark listing only a requirement's ID does not cover that requirement's examples.

When examples with the same ID are in two or more places, the `@about` of the first one (by byte order of the document path, and within the same document the smaller line) is used.

## Where marks may go

Positions of a mark that bind to a test:

| Position of the mark | Treatment |
|---|---|
| The block of comments directly before a function and its attributes (attributes may come in between; a blank line cuts it) | Bound to that function's test |
| The block of comments at the start of the function body, before any statement | Bound to that function's test |
| Both of the above | The IDs of both are combined and bound |
| In the middle of the function body | Ignored |
| Outside tests | Ignored (neither invalid_marker nor unresolved_reference is raised) |
| Functions inside macros | The same rules as above apply |

If a blank line comes between the mark and the test function, they do not bind. Attributes (such as `#[test]`) may come in the middle of the comment block.

Example:

```rust
// @kotowari[REQ-001]
#[test]
fn req_001_returns_ok() {
    // ...
}
```

The mistake of a blank line in between:

```rust
// @kotowari[REQ-001]

#[test]
fn req_001_returns_ok() {  // becomes test_without_id
    // ...
}
```

## How tests are recognised

What counts as a test in Rust:

| Target | How it is counted |
|---|---|
| A function with an attribute whose path ends in the element test (`#[test]`, `#[tokio::test]` and so on) | Always counted |
| A function with an attribute in tests.rust.attributes | Counted if the attribute path matches exactly |
| A function inside a macro in tests.rust.macros | If the last element of the macro name matches, its content is reread as Rust items, and each top-level function is counted |

Writing attribute paths in the configuration's `tests.rust.attributes` makes functions with an attribute of that path count as tests too. The path is compared on the part without `#[`, `]` and arguments.

Writing macro names (without `!`) in the configuration's `tests.rust.macros` makes the content of macros whose name's last element matches be read as tests.

## Test names

The convention for test names: lowercase the ID being verified, change its hyphens to `_`, and put it at the start (example: `req_001_returns_ok`). kotowari does not check test names.

## Languages other than Rust

For languages other than Rust, kotowari ships no query, so tests are not recognised. Every mark in a test file is picked up and counted toward clearing requirement_without_test and scenario_without_test. test_without_id is not raised. A person verifies that the tests are correct.
