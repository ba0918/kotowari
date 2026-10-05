# Marking tests — `@kotowari[...]`

English | [日本語](marks.ja.md)

A mark (`@kotowari[REQ-001]`) is the only link between an IR item and a test.
Write an ID in the comment just before a test, and kotowari counts that test as backing that item.
Once you have written the IR, add marks to your existing tests, and `check` and `status` will show which requirements are protected by tests.

## Format

<!-- @kotowari[REQ-core-071:2eb26299, TBL-core-015:3a4c51ae, REQ-core-073:2133d014, REQ-core-074:e7275c62] -->

```rust
// @kotowari[REQ-001, EX-001]
```

| Part | Form |
|---|---|
| Start | `@kotowari[` |
| Contents | IDs separated by commas. Whitespace is allowed before and after a comma |
| End | `]` (on the same line as the start) |

- A single mark may list any number of IDs. You may mix IDs of requirements, decision tables, properties and scenarios.
- If you write two marks on one line, both are picked up (`// @kotowari[REQ-001] @kotowari[EX-001]`).
- The comment syntax is not inspected. `//`, `#` and `///` all work the same.

Marks written in guides (`<!-- @kotowari[ID:指紋] -->`) are read under different rules. See [Writing guides](writing-guides.md).

## Try marking one test

<!-- @kotowari[REQ-core-075:c5389c2a, TBL-core-026:382b0b95] -->

As an example, here is a small IR with just two requirements and one scenario.

```markdown
### REQ-001: 会員の割引
- verification: unit
システムは常に、会員の注文に1割の割引を付ける。

### REQ-002: 会員でない注文
- verification: unit
システムは常に、会員でない注文に割引を付けない。

@id=EX-001 @about=REQ-001 …
Scenario: 1000円の注文は900円になる
```

(This is an excerpt; the `kind` and `source` lines are omitted.)

Write the mark as a comment directly above the test function.

```rust
// @kotowari[REQ-001, EX-001]
#[test]
fn member_gets_ten_percent_off() {}

// @kotowari[REQ-002]
#[test]
fn non_member_pays_full_price() {}
```

If `check` finishes silently with exit code 0, the tests are linked.
`list` shows which test is linked to which item.

```console
$ kotowari check --format text
$ echo $?
0
$ kotowari list --format text
REQ-001 unit 会員の割引 docs/ir/discount.md:7 tests=1
  tests/discount.rs:1 member_gets_ten_percent_off
REQ-002 unit 会員でない注文 docs/ir/discount.md:15 tests=1
  tests/discount.rs:5 non_member_pays_full_price
EX-001 - 1000円の注文は900円になる docs/ir/discount.md:27 tests=1
  tests/discount.rs:1 member_gets_ten_percent_off
```

The `1` in `tests/discount.rs:1` is the line of the mark, not the line of the test.

This repository's own tests are written like this, for example (from `tests/step11_query.rs`):

```rust
// @kotowari[REQ-core-156, REQ-core-159, TBL-core-027, EX-core-250]
#[test]
fn req_156_item_has_body_and_referenced_by() {
```

Starting a test name with an ID is just this repository's convention; kotowari does not look at test names.

## Where to put marks

<!-- @kotowari[REQ-core-075:c5389c2a, TBL-core-016:d4d7ced2, TBL-core-035:e8943c7c] -->

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

The full rules are in [TBL-core-016](../ir/core/test-markers.md#TBL-core-016) and [TBL-core-035](../ir/core/test-markers.md#TBL-core-035).

### Python and TypeScript examples

<!-- @kotowari[EX-core-307:eb880f50, REQ-core-180:c597decc, EX-core-296:c2f1fd62] -->

In Python, the mark is linked even when decorators sit between it and the test function.

```python
# @kotowari[REQ-001, EX-001]
@pytest.mark.parametrize("total", [1000])
def test_member_gets_ten_percent_off(total):
    pass
```

In TypeScript, write it just before the `it` inside a `describe`.

```ts
describe('discount', () => {
  // @kotowari[REQ-002]
  it('non-member pays full price', () => {});
});
```

```console
$ kotowari list --format text
REQ-001 unit 会員の割引 docs/ir/discount.md:7 tests=1
  tests/test_discount.py:3 test_member_gets_ten_percent_off
REQ-002 unit 会員でない注文 docs/ir/discount.md:15 tests=1
  tests/discount.test.ts:2 non-member pays full price
EX-001 - 1000円の注文は900円になる docs/ir/discount.md:27 tests=1
  tests/test_discount.py:3 test_member_gets_ten_percent_off
```

The test name is the function name in Python, and in TypeScript the first argument of `it` with its quotes removed.

### Tests inside macros

<!-- @kotowari[TBL-core-017:aff9f804, EX-core-018:0b4c0d06] -->

Inside a macro such as `proptest!`, marks are placed the same way.
The following example is from this repository: the explanatory `///` and the mark's `//` are both comment lines and form a single block (from `crates/kotowari-markdown-schema/tests/properties.rs`).

```rust
proptest! {
    /// REQ-schema-028: 一覧のマーカーが "-"、"*"、"+" のどれであっても読み分けは変わらない。
    // @kotowari[REQ-schema-028]
    #[test]
    fn markers_do_not_change_how_list_lines_are_read(
```

For kotowari to read inside a macro, the macro's name must be in the `tests.rust.macros` setting ([pitfalls](#tests-inside-a-macro-are-not-counted)).

## Marking the requirement or the scenario

<!-- @kotowari[REQ-core-085:288046ea, REQ-core-137:cb66f5a8, EX-core-121:252c9340, EX-core-122:7a95532e] -->

The ID in a mark may be either a requirement (`REQ-`) or a scenario (`EX-`).
`check` looks at this in two steps.

| Target | Condition for an error | Finding |
|---|---|---|
| Requirement | There is no mark containing the requirement's ID, and no mark containing the ID of a scenario whose `@about` names that requirement | `requirement_without_test` |
| Scenario | There is no mark containing its ID | `scenario_without_test` |

In other words, a scenario's mark also covers the requirement named in its `@about`, but not the other way round.

With only the scenario's ID written (`// @kotowari[EX-001]`):

```console
$ kotowari check --format text
$ echo $?
0
```

With only the requirement's ID written (`// @kotowari[REQ-001]`):

```console
$ kotowari check --format text
docs/ir/discount.md:26 [error] scenario_without_test EX-001
```

For a requirement that has scenarios, writing the scenario's ID is the shortcut.
For a requirement without scenarios (REQ-002 in the example), write the requirement's ID directly.

A [deferred](deferred.md) requirement produces neither error.
The same goes for a scenario whose `@about` names only deferred requirements (and review requirements).
If you write a deferred ID in a mark, you get a `deferred_with_test` notice.

### Requirements verified by review

<!-- @kotowari[REQ-core-078:aec849b3, EX-core-123:18e5541b] -->

A requirement whose verification is `review` (one that a person or an LLM checks by reading) does not require a test.
Pointing at it from a mark is not an error, but you do not need to.
A scenario whose `@about` names only `review` requirements does not require a test either.

### `tests=0` in `list` does not always mean "no tests"

<!-- @kotowari[TBL-core-026:382b0b95, EX-core-122:7a95532e] -->

The `tests` in `list` shows only the marks that write that ID **directly**.
If you write only the scenario's ID, the requirement appears as `tests=0` in `list`, but `check` and `status` count it as having tests.

```console
$ kotowari list --format text
REQ-001 unit 会員の割引 docs/ir/discount.md:7 tests=0
REQ-002 unit 会員でない注文 docs/ir/discount.md:15 tests=1
  tests/discount.rs:5 non_member_pays_full_price
EX-001 - 1000円の注文は900円になる docs/ir/discount.md:27 tests=1
  tests/discount.rs:1 member_gets_ten_percent_off
$ kotowari status --format text | grep requirements
requirements unit=2 property=0 proof=0 review=0 with_tests=2 without_tests=0 review_with_how_to_verify=0 review_without_how_to_verify=0 without_examples=1 deferred=0
```

When you look for requirements without tests, also check the scenarios of the `tests=0` requirements.

## Which tests are found

<!-- @kotowari[REQ-core-079:589c548b, REQ-core-080:35cc39ab, REQ-core-081:d861bda1, TBL-core-031:aab33120] -->

kotowari finds tests in this order.

1. Read the files matched by the globs in the `tests.files` setting ([Configuration file](config.md#how-globs-are-read)).
2. Decide the language from the extension. Case matters (`.py` is Python; `.PY` maps to no language).
3. If the language has a query (an ast-grep rule that finds tests), read the syntax tree and find the tests.

The full mapping from extensions to languages is in [TBL-core-031](../ir/core/test-discovery.md#TBL-core-031).

### Languages with a bundled query

<!-- @kotowari[REQ-core-182:8ad61d2f, TBL-core-017:aff9f804, TBL-core-032:93a48eea, TBL-core-033:091add60, TBL-core-034:bf49690a, REQ-core-181:5c809bd5] -->

| Language | What counts as a test | Full rules |
|---|---|---|
| Rust (`.rs`) | Functions whose attribute path ends in `test` (`#[test]`, `#[tokio::test]`) | [TBL-core-017](../ir/core/test-queries.md#TBL-core-017) |
| TypeScript, Tsx, JavaScript | `it(...)`, `test(...)`, and forms such as `it.skip(...)` and `it.each(table)(...)`. `describe` is not counted | [TBL-core-032](../ir/core/test-queries.md#TBL-core-032) |
| Python | Top-level functions and class methods whose names start with `test`. Functions nested inside functions are not counted | [TBL-core-033](../ir/core/test-queries.md#TBL-core-033) |
| Php | Methods whose names start with `test`, methods with `#[Test]`, methods with an `@test` docblock, and Pest's `test(...)` | [TBL-core-034](../ir/core/test-queries.md#TBL-core-034) |

Tests that are not run, such as `it.skip(...)`, are counted too.
A test inside a test (an `it` inside an `it`) is counted as a separate test.

### Adding more through the configuration

<!-- @kotowari[REQ-core-121:6502b5d5, REQ-core-186:cfb141ec] -->

| What to add | Configuration key |
|---|---|
| Other Rust attributes (`#[kani::proof]`) | `tests.rust.attributes` |
| Tests inside Rust macros (`proptest!`) | `tests.rust.macros` |
| Any other form or language | List ast-grep rule files under `tests.rules` |

The bundled queries cannot be removed; configured ones are only added to them.
For how to write them, see [Configuration file](config.md#testsrules).

### Languages without a query

<!-- @kotowari[REQ-core-076:9413bda6, REQ-core-087:2c99751b, EX-core-124:5c88c586] -->

For languages without a query, such as Go or Java (including files whose extension maps to no language), kotowari does not read the syntax tree.
It picks up every mark in the file, regardless of whether it is in a comment or before which test, and counts the requirements and scenarios as "having tests".
In exchange, it cannot find tests without marks, and `test_without_id` is never reported.

```go
func TestMember(t *testing.T) {
	// @kotowari[REQ-001, EX-001]
}

func TestNonMember(t *testing.T) {}
```

```console
$ kotowari check --format text
docs/ir/discount.md:15 [error] requirement_without_test REQ-002
```

Even a mark inside the function body satisfies REQ-001 and EX-001, and nothing is said about the unmarked `TestNonMember`.
Whether the tests are marked correctly is left for a person to check.

## Findings about marks

<!-- @kotowari[REQ-core-086:8035b3f8, REQ-core-072:0d066a71, REQ-core-077:082e5fe4, REQ-core-083:c11fae0c, REQ-core-118:2e6e1171] -->

| Finding | Severity | When it appears | Line | detail |
|---|---|---|---|---|
| `test_without_id` | error | A test in a language with a query has no mark | The first line of the test's node (in Rust, the `fn` line) | The test name (the test's first line if it has no name) |
| `requirement_without_test` | error | A requirement has no tests ([table above](#marking-the-requirement-or-the-scenario)) | The requirement's heading | The requirement's ID |
| `scenario_without_test` | error | A scenario has no tests | The scenario's tag line | The scenario's ID |
| `unresolved_reference` | error | An ID in a mark does not exist, or is not in ID form | The mark's line | That ID |
| `invalid_marker` | error | The mark is empty or only separators, or has no `]` on the same line | The mark's line | The whole line containing the mark |
| `unparsable_file` | error | A test file in a language with a query has a syntax error. That file is skipped | None (`-`) | The file path |

Even a mark pointing only at IDs that do not exist counts the test as "marked".
A mark that is not just before any test produces neither `invalid_marker` nor `unresolved_reference`.
All findings are listed in [Findings](findings.md).

## Common pitfalls

The following are results of breaking the example in [Try marking one test](#try-marking-one-test) in one place at a time and running it.

### A blank line between the mark and the test

<!-- @kotowari[EX-core-016:e63e6fd1, REQ-core-086:8035b3f8] -->

```rust
// @kotowari[REQ-001, EX-001]

#[test]
fn member_gets_ten_percent_off() {}
```

```console
$ kotowari check --format text
docs/ir/discount.md:7 [error] requirement_without_test REQ-001
docs/ir/discount.md:26 [error] scenario_without_test EX-001
tests/discount.rs:4 [error] test_without_id member_gets_ten_percent_off
```

The blank line ends the block, so the mark attaches to no test and is silently ignored.
As a result, the test is reported as "no mark", and the requirement and scenario as "no tests".
Removing the blank line fixes it.

### Written inside the function body

<!-- @kotowari[EX-core-306:e057840d, TBL-core-016:d4d7ced2] -->

```rust
#[test]
fn member_gets_ten_percent_off() {
    // @kotowari[REQ-001, EX-001]
}
```

```console
$ kotowari check --format text
docs/ir/discount.md:7 [error] requirement_without_test REQ-001
docs/ir/discount.md:26 [error] scenario_without_test EX-001
tests/discount.rs:2 [error] test_without_id member_gets_ten_percent_off
```

The same three findings as with the blank line. Move the mark above `#[test]`.

### A line of code between the mark and the test

<!-- @kotowari[EX-core-309:549f0665, TBL-core-035:e8943c7c] -->

```ts
describe('discount', () => {
  // @kotowari[REQ-002]
  const price = 1000;
  it('non-member pays full price', () => {});
});
```

```console
$ kotowari check --format text
docs/ir/discount.md:15 [error] requirement_without_test REQ-002
tests/discount.test.ts:4 [error] test_without_id non-member pays full price
```

Only Rust attributes and Python decorators may sit in between.
In TypeScript, any line other than a comment line ends the block.

### Written above `describe`

<!-- @kotowari[TBL-core-032:93a48eea, TBL-core-016:d4d7ced2] -->

```ts
// @kotowari[REQ-002]
describe('discount', () => {
  it('non-member pays full price', () => {});
});
```

```console
$ kotowari check --format text
docs/ir/discount.md:15 [error] requirement_without_test REQ-002
tests/discount.test.ts:3 [error] test_without_id non-member pays full price
```

`describe` is not counted as a test, so a mark above it is not just before any test.
Move the mark to just before the `it`.

### A mistyped ID

<!-- @kotowari[REQ-core-077:082e5fe4, REQ-core-118:2e6e1171] -->

```rust
// @kotowari[REQ-010, EX-001]
#[test]
fn member_gets_ten_percent_off() {}

// @kotowari[REQ002]
#[test]
fn non_member_pays_full_price() {}
```

```console
$ kotowari check --format text
docs/ir/discount.md:15 [error] requirement_without_test REQ-002
tests/discount.rs:1 [error] unresolved_reference REQ-010
tests/discount.rs:5 [error] unresolved_reference REQ002
```

Both an ID that does not exist (`REQ-010`) and one not in ID form (`REQ002`) give `unresolved_reference`.
In this case the test still counts as "marked", so `test_without_id` is not reported.
What needs fixing is the ID inside the mark.

### An unclosed or empty mark

<!-- @kotowari[REQ-core-072:0d066a71, TBL-core-015:3a4c51ae] -->

```rust
// @kotowari[REQ-001, EX-001
#[test]
fn member_gets_ten_percent_off() {}

// @kotowari[]
// @kotowari[REQ-002]
#[test]
fn non_member_pays_full_price() {}
```

```console
$ kotowari check --format text
docs/ir/discount.md:7 [error] requirement_without_test REQ-001
docs/ir/discount.md:26 [error] scenario_without_test EX-001
tests/discount.rs:1 [error] invalid_marker // @kotowari[REQ-001, EX-001
tests/discount.rs:3 [error] test_without_id member_gets_ten_percent_off
tests/discount.rs:5 [error] invalid_marker // @kotowari[]
```

The detail of `invalid_marker` shows the line containing the mark as is.
A mark must close within one line. If you push the `]` to the next line, the mark is unclosed.

### Tests inside a macro are not counted

<!-- @kotowari[REQ-core-082:2cab52bb, EX-core-018:0b4c0d06] -->

Even if you mark a test inside `proptest!`, the inside of the macro is not read without configuration.

```rust
proptest! {
    // @kotowari[REQ-002]
    #[test]
    fn non_member_pays_full_price(total in 0u32..10_000) {}
}
```

```console
$ kotowari check --format text
docs/ir/discount.md:15 [error] requirement_without_test REQ-002
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

<!-- @kotowari[REQ-core-079:589c548b] -->

When the tests live in `spec/` but `tests.files` is only `tests/**/*`:

```console
$ kotowari check --format text
docs/ir/discount.md:7 [error] requirement_without_test REQ-001
docs/ir/discount.md:15 [error] requirement_without_test REQ-002
docs/ir/discount.md:26 [error] scenario_without_test EX-001
```

If everything is reported as "no tests" even though the marks are there, suspect the glob first.

### An uppercase extension makes tests slip through

<!-- @kotowari[REQ-core-081:d861bda1, EX-core-295:ac47cbef] -->

Adding an unmarked `def test_forgotten():` to `test_discount.PY` produces no finding.

```console
$ kotowari check --format text
$ kotowari status --format text | grep '^tests'
tests marks=3 PY=1 ts=1
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

- Specification: [Test marks](../ir/core/test-markers.md), [Test discovery](../ir/core/test-discovery.md), [Bundled queries](../ir/core/test-queries.md), [Requirement–test coverage](../ir/core/coverage.md), [Queries added by configuration](../ir/core/query-rules.md)
- Terms: [Glossary](../ir/core/CONTEXT.md) (mark, test, preceding comment block, language with a query)
- Configuration keys: [Configuration file](config.md)
- Listing items and tests: [kotowari list](commands/list.md)
- Checking whether everything is in place: [kotowari status](commands/status.md)
- Kinds of findings: [Findings](findings.md)
