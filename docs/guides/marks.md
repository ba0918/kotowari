# Marking tests — `@kotowari[...]`

English | [日本語](marks.ja.md)

A mark (`@kotowari[C-shop-discount-1]`) is the only link between a confirmation row of a decision document and a test.
Write a row's ID in the comment just before a test, and kotowari counts that test as confirming that row.
Once you have written the decision documents, add marks to your existing tests, and `check` and `status` will show which rows are confirmed by tests.

## Format

<!-- @kotowari[docs/decisions/core/test-markers.md#a-marker-is-ids-listed-with-commas-and-may-be-anywhere-in-a-line:74293754] -->

```rust
// @kotowari[C-shop-discount-1, C-shop-discount-2]
```

| Part | Form |
|---|---|
| Start | `@kotowari[` |
| Contents | IDs separated by commas, or the word `detail` ([tests that have no row](#tests-that-have-no-row)). Whitespace is allowed before and after a comma |
| End | `]` (on the same line as the start) |

- A single mark may list any number of IDs, from any decision documents.
- If you write two marks on one line, both are picked up (`// @kotowari[C-shop-discount-1] @kotowari[C-shop-discount-2]`).
- The comment syntax is not inspected. `//`, `#` and `///` all work the same.

Marks written in guides (`<!-- @kotowari[target:fingerprint] -->`) are read under different rules. See [Writing guides](writing-guides.md).

## Try marking one test

<!-- @kotowari[docs/decisions/core/test-markers.md#a-marker-is-tied-to-a-test-only-when-it-is-in-the-comment-block-directly-before-that-test:9dac125a, docs/decisions/core/list.md#a-confirmation-row-gets-the-tests-that-point-at-it-and-a-decision-gets-a-fingerprint:64ab7c66, docs/decisions/core/confirmations.md#check-counts-per-confirmation-row-whether-a-test-points-at-it:f6a31de4] -->

As an example, here is a small decision document with two implemented decisions, each with one row.

```markdown
## Members get 10% off their orders

Status: implemented

### Decision

An order placed by a member is priced at 10% off.

### Confirmations

- C-shop-discount-1 A member's order of 1000 is priced at 900

## Orders of non-members are not discounted

Status: implemented

### Decision

An order placed by a non-member is priced at the list price.

### Confirmations

- C-shop-discount-2 A guest's order of 1000 is priced at 1000
```

(This is an excerpt; the title, the lead and the Problem sections are omitted.)

Write the mark as a comment directly above the test function.

```rust
// @kotowari[C-shop-discount-1]
#[test]
fn member_gets_ten_percent_off() {}

// @kotowari[C-shop-discount-2]
#[test]
fn non_member_pays_full_price() {}
```

If `check` finishes silently with exit code 0, the tests are linked.
`list` shows which test is linked to which row.

```console
$ kotowari check --format text
$ echo $?
0
$ kotowari list --format text
docs/decisions/shop/discount.md#members-get-10-off-their-orders - Members get 10% off their orders docs/decisions/shop/discount.md:5 tests=0
C-shop-discount-1 - A member's order of 1000 is priced at 900 docs/decisions/shop/discount.md:19 tests=1
  tests/discount.rs:1 member_gets_ten_percent_off
docs/decisions/shop/discount.md#orders-of-non-members-are-not-discounted - Orders of non-members are not discounted docs/decisions/shop/discount.md:21 tests=0
C-shop-discount-2 - A guest's order of 1000 is priced at 1000 docs/decisions/shop/discount.md:35 tests=1
  tests/discount.rs:5 non_member_pays_full_price
```

The lines starting with a path are the decisions themselves; tests point at rows, so a decision always shows `tests=0`.
The `1` in `tests/discount.rs:1` is the line of the mark, not the line of the test.

This repository's own tests are written like this, for example (from `tests/step23_decision_documents.rs`):

```rust
// @kotowari[C-core-query-14]
#[test]
fn query_of_a_decision_puts_the_whole_decision_in_its_body() {
```

kotowari does not look at test names.

## Where to put marks

<!-- @kotowari[docs/decisions/core/test-markers.md#a-marker-is-tied-to-a-test-only-when-it-is-in-the-comment-block-directly-before-that-test:9dac125a, docs/decisions/core/test-markers.md#the-preceding-comment-block-consists-of-comment-lines-up-to-a-blank-line-and-the-lines-that-may-be-interposed-per-language:88e06f78] -->

A mark is linked to a test only when it is in the test's **preceding comment block**.
The block is the run of comment lines going upward from just above the test's first line until a blank line.

| Where the mark is | What happens |
|---|---|
| In the test's preceding comment block | Linked to that test |
| Inside the test (including a comment at the start of the function body) | Ignored, with no error |
| Not just before any test | Ignored, with no error |

Which lines may sit in the middle of a block depends on the language.

| Language | Lines allowed in between |
|---|---|
| Rust | Attribute lines (`#[...]`), including every line of a multi-line attribute |
| Python | Decorator lines (`@...`), including every line of a multi-line decorator |
| Others | None (comment lines only) |

- A blank line, a line of code, or a comment on the same line as code (`setup(); // note`) ends the block there.
- Marks inside the body of an attribute or decorator (the string in `#[doc = "..."]`) are not read.
- When two or more tests share the same first line, the mark is linked only to the test that starts first on that line.

The full rules are in [the specification of test marks](../decisions/core/test-markers.md).

### Python and TypeScript examples

<!-- @kotowari[docs/decisions/core/test-markers.md#the-preceding-comment-block-consists-of-comment-lines-up-to-a-blank-line-and-the-lines-that-may-be-interposed-per-language:88e06f78, docs/decisions/core/test-discovery.md#a-tests-name-is-taken-from-the-text-the-query-captured:a5dba3c1] -->

In Python, the mark is linked even when decorators sit between it and the test function.

```python
import pytest

# @kotowari[C-shop-discount-1]
@pytest.mark.parametrize("total", [1000])
def test_member_gets_ten_percent_off(total):
    pass
```

In TypeScript, write it just before the `it` inside a `describe`.

```ts
describe('discount', () => {
  // @kotowari[C-shop-discount-2]
  it('non-member pays full price', () => {});
});
```

```console
$ kotowari list --format text | grep -A1 '^C-'
C-shop-discount-1 - A member's order of 1000 is priced at 900 docs/decisions/shop/discount.md:19 tests=1
  tests/test_discount.py:3 test_member_gets_ten_percent_off
C-shop-discount-2 - A guest's order of 1000 is priced at 1000 docs/decisions/shop/discount.md:35 tests=1
  tests/discount.test.ts:2 non-member pays full price
```

The test name is the function name in Python, and in TypeScript the first argument of `it` with its quotes removed.

### Tests inside macros

<!-- @kotowari[docs/decisions/core/test-queries.md#in-rust-functions-whose-attribute-path-ends-in-test-are-always-counted-and-attributes-and-macros-are-added-by-configuration:1e514868] -->

Inside a macro such as `proptest!`, marks are placed the same way.
The following example is from this repository: the explanatory `///` and the mark's `//` are both comment lines and form a single block (from `crates/kotowari-markdown-schema/tests/properties.rs`).

```rust
proptest! {
    /// 一覧のマーカーが "-"、"*"、"+" のどれであっても読み分けは変わらない。
    // @kotowari[C-schema-block-2]
    #[test]
    fn markers_do_not_change_how_list_lines_are_read(
```

For kotowari to read inside a macro, the macro's name must be in the `tests.rust.macros` setting ([pitfalls](#tests-inside-a-macro-are-not-counted)).

## Rows that need no test

<!-- @kotowari[docs/decisions/core/confirmations.md#write-by-reading-at-the-head-of-a-line-not-checked-by-tests:bfbb46c7, docs/decisions/core/test-markers.md#a-marker-that-points-at-a-line-checked-by-reading-is-not-an-error:e433e730, docs/decisions/core/confirmations.md#the-implementing-side-changes-the-status-and-decisions-not-built-yet-stay-proposed:436adde2] -->

`check` asks for a test only for the rows of implemented decisions that do not start with `(by reading)`.

| Row | Without a mark | With a mark |
|---|---|---|
| A row of an implemented decision | `confirmation_without_test` | Counted as having a test |
| A row that starts with `(by reading)` | Nothing | Nothing (not an error, but not needed) |
| A row of a proposed decision | Nothing | Nothing; the test may be written before the decision is turned to implemented |

```markdown
### Confirmations

- C-shop-discount-3 (by reading) The receipt template has a line for the discount
```

A row confirmed by reading is read and confirmed by the implementer and the review; `list` shows `reading` in its second column, and `status` counts it under `by_reading`.

## Tests that have no row

<!-- @kotowari[docs/decisions/core/coverage.md#a-test-that-has-no-row-can-be-declared-with-kotowaridetail-and-a-declared-test-gets-no-test_without_id:5f7fbfcf] -->

Decision documents hold only what a person decided, so a test of a smaller detail, such as the wording of an error message, may have no row to point at.
Declare such a test with `@kotowari[detail]` in the comment just before it.

```rust
// @kotowari[detail]
#[test]
fn empty_name_error_names_the_field() {
    // ...
}
```

- `check` reports no `test_without_id` for a declared test, and no `unresolved_reference` for `detail`.
- `detail` is a fixed word, not an ID. Write it in lowercase; `Detail` is read as an ID and gives `unresolved_reference`.
- A declaration confirms no row. A row of an implemented decision that no mark points at still gives `confirmation_without_test`, however many declared tests there are.
- When the test confirms a row as well, write both in one mark: `// @kotowari[C-shop-discount-1, detail]`.
- When the test confirms a row, write that row's ID instead of the declaration. Do not write the ID of a row the test does not confirm to make `test_without_id` go away.

## Which tests are found

<!-- @kotowari[docs/decisions/core/test-discovery.md#test-files-are-selected-with-the-globs-of-testsfiles:f7efafe7, docs/decisions/core/test-discovery.md#tests-are-found-by-applying-queries-to-the-syntax-tree-not-by-searching-lines-with-regular-expressions:ce30c2d1, docs/decisions/core/test-discovery.md#the-correspondence-between-extensions-and-languages-is-the-same-as-ast-grep-and-is-case-sensitive:0894862d] -->

kotowari finds tests in this order.

1. Read the files matched by the globs in the `tests.files` setting ([Configuration file](config.md#how-globs-are-read)).
2. Decide the language from the extension. Case matters (`.py` is Python; `.PY` maps to no language).
3. If the language has a query (an ast-grep rule that finds tests), read the syntax tree and find the tests.

The full mapping from extensions to languages is in [the specification of test discovery](../decisions/core/test-discovery.md).

### Languages with a bundled query

<!-- @kotowari[docs/decisions/core/test-queries.md#queries-are-bundled-only-for-the-four-families-of-languages-the-user-uses:e806bd53, docs/decisions/core/test-queries.md#in-rust-functions-whose-attribute-path-ends-in-test-are-always-counted-and-attributes-and-macros-are-added-by-configuration:1e514868, docs/decisions/core/test-queries.md#in-typescript-and-javascript-calls-of-it-and-test-are-counted:9a76e5a9, docs/decisions/core/test-queries.md#in-python-functions-and-methods-whose-names-start-with-test-are-counted:8ba7df8f, docs/decisions/core/test-queries.md#in-php-methods-and-calls-of-test-are-counted:a1039155, docs/decisions/core/test-discovery.md#overlapping-matches-on-the-same-node-count-as-one-and-a-test-inside-a-test-is-counted-separately:0ccd8b7a] -->

| Language | What counts as a test | Full rules |
|---|---|---|
| Rust (`.rs`) | Functions whose attribute path ends in `test` (`#[test]`, `#[tokio::test]`) | [Rust](../decisions/core/test-queries.md#in-rust-functions-whose-attribute-path-ends-in-test-are-always-counted-and-attributes-and-macros-are-added-by-configuration) |
| TypeScript, Tsx, JavaScript | `it(...)`, `test(...)`, and forms such as `it.skip(...)` and `it.each(table)(...)`. `describe` is not counted | [TypeScript and JavaScript](../decisions/core/test-queries.md#in-typescript-and-javascript-calls-of-it-and-test-are-counted) |
| Python | Top-level functions and class methods whose names start with `test`. Functions nested inside functions are not counted | [Python](../decisions/core/test-queries.md#in-python-functions-and-methods-whose-names-start-with-test-are-counted) |
| Php | Methods whose names start with `test`, methods with `#[Test]`, methods with an `@test` docblock, and Pest's `test(...)` | [Php](../decisions/core/test-queries.md#in-php-methods-and-calls-of-test-are-counted) |

Tests that are not run, such as `it.skip(...)`, are counted too.
A test inside a test (an `it` inside an `it`) is counted as a separate test.

### Adding more through the configuration

<!-- @kotowari[docs/decisions/core/query-rules.md#queries-are-added-with-testsrules-in-the-configuration-and-the-bundled-queries-cannot-be-removed:ec930518, docs/decisions/core/query-rules.md#a-rule-file-is-read-as-an-ast-grep-rule-yaml-as-it-is:10835ba2] -->

| What to add | Configuration key |
|---|---|
| Other Rust attributes (`#[kani::proof]`) | `tests.rust.attributes` |
| Tests inside Rust macros (`proptest!`) | `tests.rust.macros` |
| Any other form or language | List ast-grep rule files under `tests.rules` |

The bundled queries cannot be removed; configured ones are only added to them.
For how to write them, see [Configuration file](config.md#testsrules).

### Languages without a query

<!-- @kotowari[docs/decisions/core/test-markers.md#in-files-of-a-language-without-a-query-all-markers-in-the-file-are-picked-up:d8565343, docs/decisions/core/coverage.md#markers-in-files-of-a-language-without-a-query-also-count-toward-satisfying-confirmation-rows:e4917759] -->

For languages without a query, such as Go or Java (including files whose extension maps to no language), kotowari does not read the syntax tree.
It picks up every mark in the file, regardless of whether it is in a comment or before which test, and counts the rows as "having tests".
In exchange, it cannot find tests without marks, and `test_without_id` is never reported.

```go
func TestMember(t *testing.T) {
	// @kotowari[C-shop-discount-1]
}

func TestNonMember(t *testing.T) {}
```

```console
$ kotowari check --format text
docs/decisions/shop/discount.md:35 [error] confirmation_without_test C-shop-discount-2
```

Even a mark inside the function body satisfies C-shop-discount-1, and nothing is said about the unmarked `TestNonMember`.
Whether the tests are marked correctly is left for a person to check.

## Findings about marks

<!-- @kotowari[docs/decisions/core/coverage.md#a-test-without-a-marker-gets-a-test_without_id-error:2f609678, docs/decisions/core/test-markers.md#a-malformed-marker-gets-an-invalid_marker-error-pointing-at-the-markers-line:915d8722, docs/decisions/core/test-markers.md#even-a-marker-that-points-only-at-ids-in-no-document-makes-its-test-count-as-one-with-a-marker:5e328dc1, docs/decisions/core/test-discovery.md#a-test-file-with-a-syntax-error-gets-unparsable_file-and-is-skipped:4d52e291, docs/decisions/core/confirmations.md#check-counts-per-confirmation-row-whether-a-test-points-at-it:f6a31de4] -->

| Finding | Severity | When it appears | Line | detail |
|---|---|---|---|---|
| `test_without_id` | error | A test in a language with a query has no mark | The first line of the test's node (in Rust, the `fn` line) | The test name (the test's first line if it has no name) |
| `confirmation_without_test` | error | A row of an implemented decision has no tests ([rows that need no test](#rows-that-need-no-test)) | The row | The row's ID |
| `unresolved_reference` | error | An ID in a mark does not exist, or is not in ID form | The mark's line | That ID |
| `invalid_marker` | error | The mark is empty or only separators, or has no `]` on the same line | The mark's line | The whole line containing the mark |
| `unparsable_file` | error | A test file in a language with a query has a syntax error. That file is skipped | None (`-`) | The file path |

Even a mark pointing only at IDs that do not exist counts the test as "marked".
A mark that is not just before any test produces neither `invalid_marker` nor `unresolved_reference`.
All findings are listed in [Findings](findings.md).

## Common pitfalls

The following are results of breaking the example in [Try marking one test](#try-marking-one-test) in one place at a time and running it.

### A blank line between the mark and the test

<!-- @kotowari[docs/decisions/core/test-markers.md#the-preceding-comment-block-consists-of-comment-lines-up-to-a-blank-line-and-the-lines-that-may-be-interposed-per-language:88e06f78, docs/decisions/core/coverage.md#a-test-without-a-marker-gets-a-test_without_id-error:2f609678] -->

```rust
// @kotowari[C-shop-discount-1]

#[test]
fn member_gets_ten_percent_off() {}
```

```console
$ kotowari check --format text
docs/decisions/shop/discount.md:19 [error] confirmation_without_test C-shop-discount-1
tests/discount.rs:4 [error] test_without_id member_gets_ten_percent_off
```

The blank line ends the block, so the mark attaches to no test and is silently ignored.
As a result, the test is reported as "no mark", and the row as "no tests".
Removing the blank line fixes it.

### Written inside the function body

<!-- @kotowari[docs/decisions/core/test-markers.md#a-marker-is-tied-to-a-test-only-when-it-is-in-the-comment-block-directly-before-that-test:9dac125a] -->

```rust
#[test]
fn member_gets_ten_percent_off() {
    // @kotowari[C-shop-discount-1]
}
```

```console
$ kotowari check --format text
docs/decisions/shop/discount.md:19 [error] confirmation_without_test C-shop-discount-1
tests/discount.rs:2 [error] test_without_id member_gets_ten_percent_off
```

The same two findings as with the blank line. Move the mark above `#[test]`.

### A line of code between the mark and the test

<!-- @kotowari[docs/decisions/core/test-markers.md#the-preceding-comment-block-consists-of-comment-lines-up-to-a-blank-line-and-the-lines-that-may-be-interposed-per-language:88e06f78] -->

```ts
describe('discount', () => {
  // @kotowari[C-shop-discount-2]
  const price = 1000;
  it('non-member pays full price', () => {});
});
```

```console
$ kotowari check --format text
docs/decisions/shop/discount.md:35 [error] confirmation_without_test C-shop-discount-2
tests/discount.test.ts:4 [error] test_without_id non-member pays full price
```

Only Rust attributes and Python decorators may sit in between.
In TypeScript, any line other than a comment line ends the block.

### Written above `describe`

<!-- @kotowari[docs/decisions/core/test-queries.md#in-typescript-and-javascript-calls-of-it-and-test-are-counted:9a76e5a9, docs/decisions/core/test-markers.md#a-marker-is-tied-to-a-test-only-when-it-is-in-the-comment-block-directly-before-that-test:9dac125a] -->

```ts
// @kotowari[C-shop-discount-2]
describe('discount', () => {
  it('non-member pays full price', () => {});
});
```

```console
$ kotowari check --format text
docs/decisions/shop/discount.md:35 [error] confirmation_without_test C-shop-discount-2
tests/discount.test.ts:3 [error] test_without_id non-member pays full price
```

`describe` is not counted as a test, so a mark above it is not just before any test.
Move the mark to just before the `it`.

### A mistyped ID

<!-- @kotowari[docs/decisions/core/test-markers.md#even-a-marker-that-points-only-at-ids-in-no-document-makes-its-test-count-as-one-with-a-marker:5e328dc1, docs/decisions/core/test-markers.md#a-malformed-marker-gets-an-invalid_marker-error-pointing-at-the-markers-line:915d8722] -->

```rust
// @kotowari[C-shop-discount-10]
#[test]
fn member_gets_ten_percent_off() {}

// @kotowari[C-shop-discount]
#[test]
fn non_member_pays_full_price() {}
```

```console
$ kotowari check --format text
docs/decisions/shop/discount.md:19 [error] confirmation_without_test C-shop-discount-1
docs/decisions/shop/discount.md:35 [error] confirmation_without_test C-shop-discount-2
tests/discount.rs:1 [error] unresolved_reference C-shop-discount-10
tests/discount.rs:5 [error] unresolved_reference C-shop-discount
```

Both an ID that does not exist (`C-shop-discount-10`) and one not in ID form (`C-shop-discount`) give `unresolved_reference`.
In this case the test still counts as "marked", so `test_without_id` is not reported.
What needs fixing is the ID inside the mark.

### An unclosed or empty mark

<!-- @kotowari[docs/decisions/core/test-markers.md#a-malformed-marker-gets-an-invalid_marker-error-pointing-at-the-markers-line:915d8722, docs/decisions/core/test-markers.md#a-marker-is-ids-listed-with-commas-and-may-be-anywhere-in-a-line:74293754] -->

```rust
// @kotowari[C-shop-discount-1
#[test]
fn member_gets_ten_percent_off() {}

// @kotowari[]
// @kotowari[C-shop-discount-2]
#[test]
fn non_member_pays_full_price() {}
```

```console
$ kotowari check --format text
docs/decisions/shop/discount.md:19 [error] confirmation_without_test C-shop-discount-1
tests/discount.rs:1 [error] invalid_marker // @kotowari[C-shop-discount-1
tests/discount.rs:3 [error] test_without_id member_gets_ten_percent_off
tests/discount.rs:5 [error] invalid_marker // @kotowari[]
```

The detail of `invalid_marker` shows the line containing the mark as is.
A mark must close within one line. If you push the `]` to the next line, the mark is unclosed.

### Tests inside a macro are not counted

<!-- @kotowari[docs/decisions/core/test-queries.md#in-rust-functions-whose-attribute-path-ends-in-test-are-always-counted-and-attributes-and-macros-are-added-by-configuration:1e514868] -->

Even if you mark a test inside `proptest!`, the inside of the macro is not read without configuration.

```rust
proptest! {
    // @kotowari[C-shop-discount-2]
    #[test]
    fn non_member_pays_full_price(total in 0u32..10_000) {}
}
```

```console
$ kotowari check --format text
docs/decisions/shop/discount.md:35 [error] confirmation_without_test C-shop-discount-2
```

Adding the macro name (without the `!`) to the configuration makes the error go away.

```yaml
tests:
  files:
    - "tests/**/*"
  rust:
    macros: [proptest]
```

### The test file is not covered by `tests.files`

<!-- @kotowari[docs/decisions/core/test-discovery.md#test-files-are-selected-with-the-globs-of-testsfiles:f7efafe7] -->

When the tests live in `spec/` but `tests.files` is only `tests/**/*`:

```console
$ kotowari check --format text
docs/decisions/shop/discount.md:19 [error] confirmation_without_test C-shop-discount-1
docs/decisions/shop/discount.md:35 [error] confirmation_without_test C-shop-discount-2
```

If everything is reported as "no tests" even though the marks are there, suspect the glob first.

### An uppercase extension makes tests slip through

<!-- @kotowari[docs/decisions/core/test-discovery.md#the-correspondence-between-extensions-and-languages-is-the-same-as-ast-grep-and-is-case-sensitive:0894862d] -->

Adding an unmarked `def test_forgotten():` to `test_discount.PY` produces no finding.

```console
$ kotowari check --format text
$ kotowari status --format text | grep '^tests'
tests marks=2 PY=1 ts=1
```

This is because `.PY` is not taken as Python, so the file is treated as a language without a query and only its marks are picked up.
Rename the same file to `test_discount.py`, and the test is found this time.

```console
$ kotowari check --format text
tests/test_discount.py:8 [error] test_without_id test_forgotten
```

This is an example of why the absence of errors is not a reason to relax.
Check the number of files per extension in the `tests` line of `status`.

## Related

- Specification: [Test marks](../decisions/core/test-markers.md), [Test discovery](../decisions/core/test-discovery.md), [Bundled queries](../decisions/core/test-queries.md), [Confirmations and tests](../decisions/core/confirmations.md), [Coverage](../decisions/core/coverage.md), [Queries added by configuration](../decisions/core/query-rules.md)
- Terms: [Glossary](../decisions/core/CONTEXT.md)
- Writing rows: [Writing decision documents](decision-documents.md)
- Configuration keys: [Configuration file](config.md)
- Listing rows and tests: [kotowari list](commands/list.md)
- Checking whether everything is in place: [kotowari status](commands/status.md)
- Kinds of findings: [Findings](findings.md)
