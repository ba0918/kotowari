Based on the kotowari specification (revised 2026-09-24; the version of kotowari itself is not pinned)

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

A mark binds only from the comment block directly before a test. The block starts on the line just above the first line of the test's node and runs upward, line by line, until a blank line or a line of code. When two or more tests start on the same first line, the marks bind only to the test that starts first on that line; the others have no marks and each raises test_without_id:

| Position of the mark | Treatment |
|---|---|
| The comment block directly before a test | Bound to that test |
| Inside the test's node (including the start of a function body), except the comment block directly before another test nested in it | Ignored (neither invalid_marker nor unresolved_reference is raised) |
| The comment block directly before a test nested inside another test | Bound to the nested test |
| Not in the comment block of any test | Ignored (neither invalid_marker nor unresolved_reference is raised) |
| Functions inside macros in tests.rust.macros | The same rules as above apply |

Lines that belong to the block:

| Line | In the block |
|---|---|
| A comment-only line: after trimming, every character is part of a comment. Lines in the middle of a multi-line comment count | Yes |
| A Rust attribute line (`#[...]`), all lines of a multi-line attribute | Yes, may come between comment lines |
| A Python decorator line (`@...`), all lines of a multi-line decorator | Yes, may come between comment lines |
| A line with code and a comment (`setup(); // note`) | No, it ends the block |
| A blank line (empty or only whitespace) | No, it ends the block. A blank line in the middle of a multi-line comment, attribute or decorator does not end it |

In other languages only comment-only lines may form the block. Within the block, marks are read only from the text of comments: a mark inside an attribute or a decorator (such as the string of `#[doc = "@kotowari[REQ-001]"]`) is not read, and raises neither invalid_marker nor unresolved_reference. A mark at the start of a function body does not bind; move it above the test.

Example:

```rust
// @kotowari[REQ-001]
#[test]
fn req_001_returns_ok() {
    // ...
}
```

```python
# @kotowari[REQ-001]
@pytest.mark.parametrize("a", [1])
def test_req_001_returns_ok(a):
    ...
```

The mistake of a blank line in between:

```rust
// @kotowari[REQ-001]

#[test]
fn req_001_returns_ok() {  // becomes test_without_id
    // ...
}
```

The mistake of a mark at the start of the body:

```rust
#[test]
fn req_001_returns_ok() {  // becomes test_without_id
    // @kotowari[REQ-001]
}
```

## How tests are recognised

The language of a test file is decided by its extension (case-sensitive): `rs` is Rust; `ts`, `mts`, `cts` TypeScript; `tsx` Tsx; `js`, `jsx`, `mjs`, `cjs` JavaScript; `py`, `py3`, `pyi`, `bzl`, `bazel` Python; `php` Php. Other extensions that ast-grep knows (`go`, `java`, `rb` and so on) decide a language with no bundled query. A language with a query is one that has at least one bundled rule or rule from `tests.rules`. kotowari parses its files with tree-sitter, applies every rule, and counts each matched node as one test (a node matched by several rules is one test; a test nested inside another test is counted too). A file of a language with a query that has a syntax error raises unparsable_file.

What counts as a test in Rust:

| Target | How it is counted |
|---|---|
| A function with an attribute whose path ends in the element test (`#[test]`, `#[tokio::test]` and so on) | Always counted |
| A function with an attribute in tests.rust.attributes | Counted if the attribute path matches exactly |
| A function inside a macro in tests.rust.macros | If the last element of the macro name matches, its content is reread as Rust items, and each top-level function is counted |
| The functions and macros above inside another function's body | Counted wherever they are |

Writing attribute paths in the configuration's `tests.rust.attributes` makes functions with an attribute of that path count as tests too. The path is compared on the part without `#[`, `]` and arguments.

Writing macro names (without `!`) in the configuration's `tests.rust.macros` makes the content of macros whose name's last element matches be read as tests.

What counts as a test in TypeScript, Tsx and JavaScript:

| Target | How it is counted |
|---|---|
| A call of `it(...)` or `test(...)` | Counted. The name is the first argument |
| A call of `it` or `test` followed by `.` and names (`it.only`, `it.skip`, `it.todo`, `test.concurrent`, `it.only.each`) | Counted. The name is the first argument |
| A call of the result of `...each(table)` (`it.each(table)(name, fn)`) | The outer call is counted with its first argument; the inner `it.each(table)` is not |
| `describe(...)` and calls whose caller is not `it` or `test` (`regex.test(s)`) | Not counted. No describe name is prefixed to the name |

What counts as a test in Python: a function whose name starts with `test` at the top level of the file, and a method whose name starts with `test` in a class, with or without decorators. A function nested in a function is not counted.

What counts as a test in Php: a method whose name starts with `test`; a method with an attribute whose path ends in `Test` (`#[Test]`, `#[\PHPUnit\Framework\Attributes\Test]`); a method whose preceding comment starts with `/**` and contains `@test`; a call of `test(...)` or `it(...)` (Pest), named by its first argument.

The name of a test is the text captured by `$NAME`. If it starts and ends with the same quote (`'`, `"` or a backquote), one quote is removed from each end. The line of test_without_id is the first line of the test's node: the `fn` or `def` line for Rust and Python (attributes and decorators are outside the node), the `#[Test]` line for such a Php method.

## Test names

The convention for test names: lowercase the ID being verified, change its hyphens to `_`, and put it at the start (example: `req_001_returns_ok`). kotowari does not check test names.

## Adding rules and languages without a query

Rules for more tests are added with `tests.rules` in the configuration: a list of paths, relative to the base directory, of ast-grep rule YAML files (several rules may be separated by `---`). Each rule is added to the queries of its `language` (matched without case; aliases such as `ts` and `py` are accepted). The bundled queries cannot be removed. A rule matches like `ast-grep scan -r <file>`: the matched node is the test, and `$NAME` is its name. When the rule does not capture `$NAME`, the name is null, and test_without_id uses the whole first line of the node, trimmed, as its detail. `files` and `ignores` are read as ast-grep reads them, against the path relative to the base directory. `fix`, `message`, `severity`, `note` and `metadata` are not used; a rule with `severity: off` still applies. A missing, unreadable, non-UTF-8 or malformed rule file, the same path listed twice, or an unknown `language` stops with a config error.

```yaml
# rules/bench.yml
id: bench
language: typescript
rule:
  pattern: bench($NAME, $$$)
```

Adding a rule for a language with no bundled query (Go, Java and so on) makes it a language with a query. Only comment-only lines may form its comment block.

For a language without a query, kotowari does not parse the file. Every mark in the text of the file is picked up and counted toward clearing requirement_without_test and scenario_without_test, and test_without_id is not raised. A person verifies that the tests are correct.
