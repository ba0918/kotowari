# Deferring requirements — `- deferred:`

English | [日本語](deferred.ja.md)

<!-- @kotowari[REQ-core-208:eb40fd95] -->

This is how you declare a deferral: a requirement you wrote into the specification but decided not to build yet.
For a requirement under deferral, and for a scenario that points only at such requirements, `requirement_without_test` and `scenario_without_test` no longer appear even when there are no tests.
The form and reference checks and the reservation of IDs carry on as before.

When requirements that cannot be built because of upstream constraints are mixed in, their missing tests are listed as the same errors as genuinely unfinished work (built, but without tests), and the latter gets buried.
Once you declare the deferral, only the genuinely unfinished work remains among the errors, and `complete` in `status` reflects exactly whether any is left.

## Format

<!-- @kotowari[REQ-core-208:eb40fd95, REQ-core-209:bc5f958e, REQ-core-210:0772cc2a, TBL-core-011:245eca98, REQ-core-046:0ee3c10b] -->

The value is a source.
Write the decision to defer, together with its reason, in a decision record, and point at it in the same `path#decision-number` form as `- source:`.
You can give several, separated by commas.

To defer requirements one at a time, write the line under the requirement's heading.

```markdown
### REQ-greet-003: 英語で挨拶する

- kind: event_driven
- source: docs/decision/records/2026-09-24-greet.md#A4
- verification: unit
- deferred: docs/decision/records/2026-09-24-greet.md#A4

"--lang en" を受けたとき、コマンドは`英語の挨拶文`を出す。
```

To defer every requirement in a document, write one line after the title and before the first `## ` (a document-level declaration).

```markdown
# 挨拶の翻訳

英語など、日本語でない挨拶文を扱う。
- deferred: docs/decision/records/2026-09-24-greet.md#A4

## Requirements
```

| Where | What is deferred |
|---|---|
| Under a requirement's heading | That requirement |
| In a topic document, after the title and before the first `## ` or `### ` | Every requirement in that document |

- Writing both raises no finding
- A document-level declaration does not count as a scope line. A document with only the declaration and no scope sentence gets `missing_scope`
- Only topic documents can carry a document-level declaration. Writing one in `CONTEXT.md` or `FLAGS.md` gives `unknown_field`
- Decision tables, properties and scenarios cannot be deferred on their own. A scenario follows the requirements it points at (next section)

## What changes and what does not

<!-- @kotowari[REQ-core-085:288046ea, REQ-core-137:cb66f5a8, REQ-core-208:eb40fd95] -->

If you remove the `- deferred:` line from `REQ-greet-003` above, the missing tests for that requirement and for the scenario `EX-greet-002` that points at it become errors.

```console
$ kotowari check --format text
docs/ir/greet/greet.md:15 [error] requirement_without_test REQ-greet-003
docs/ir/greet/greet.md:31 [error] scenario_without_test EX-greet-002
$ echo $?
1
```

Put the `- deferred:` line back and both disappear.

```console
$ kotowari check --format text
$ echo $?
0
```

A scenario is not asked for tests when, among its `@about` requirements whose verification is unit, property, proof or review, at least one is under deferral and all of them are either under deferral or verified by review (a deferred scenario).
Requirements without a `- verification:` line do not count toward this judgement.
A scenario that points at both a deferred requirement and a non-deferred unit requirement still needs tests, as before.

What stays the same under deferral:

- Form checks (`verification_missing`, `missing_statement` and so on)
- Reference checks (`unresolved_reference`) and source checks
- Reservation of IDs (`duplicate_id`)

When the same ID appears in two or more places, the first requirement (in byte order of document path, and the lower line within the same document) decides whether it is deferred.
For scenarios, likewise, the `@about` of the first scenario decides.

## status, list and query

<!-- @kotowari[TBL-core-028:9645c008, TBL-core-026:382b0b95, REQ-core-155:586b389e, TBL-core-027:24f0de66, REQ-core-161:4580b6c0] -->

`status` reports the number of deferred requirements as `deferred` under `requirements`, and the number of deferred scenarios as `deferred` under `scenarios`.
Deferred ones count toward neither `with_tests` nor `without_tests`, so `without_tests` is the number of genuinely unfinished items.
The per-verification counts (`unit` and so on) and `without_examples` do include deferred ones.

```console
$ kotowari status --format text
documents files=2 lines=42
items requirement=2 table=0 property=0 scenario=2 flag=0
requirements unit=2 property=0 proof=0 review=0 with_tests=1 without_tests=0 review_with_how_to_verify=0 review_without_how_to_verify=0 without_examples=0 deferred=1
scenarios with_tests=1 without_tests=0 deferred=1
tests marks=1 rs=1
guides files=0 marks=0
overview files=0 marks=0
surface total=0 specified=0 unspecified=0
findings error=0 notice=0
complete true
```

In the JSON of `list` and `query`, every item and scenario has a boolean `deferred`.
It is true for deferred requirements and deferred scenarios, and false for everything else.
In text, ` deferred` is appended to the end of the line of each deferred entry.

```console
$ kotowari list --format text
REQ-greet-001 unit 名前を入れて挨拶する docs/ir/greet/greet.md:7 tests=0
REQ-greet-003 unit 英語で挨拶する docs/ir/greet/greet.md:15 tests=0 deferred
EX-greet-001 - 名前を挨拶文に入れる docs/ir/greet/greet.md:28 tests=1
  tests/greet.rs:1 greets_with_name
EX-greet-002 - 英語で挨拶する docs/ir/greet/greet.md:33 tests=0 deferred
```

When looking for items that still have no tests, leave out the deferred ones.

```console
$ kotowari list | jq -r '.items[] | select(.tests == [] and .verification != "review" and (.deferred | not)) | .id'
```

## Notices for mismatches with deferral

<!-- @kotowari[REQ-core-211:e38f935b, REQ-core-212:070f1cfc, TBL-core-009:e3c60d7c] -->

When a deferral disagrees with test marks or references, two kinds of notice appear.
Being notices, they change neither the exit code nor `complete` in `status`.

```console
$ kotowari check --format text
docs/ir/greet/greet.md:13 [notice] depends_on_deferred REQ-greet-001 REQ-greet-003
docs/ir/greet/greet.md:15 [notice] deferred_with_test REQ-greet-003
```

| Kind | When it appears | detail | Line |
|---|---|---|---|
| `deferred_with_test` | A mark contains the ID of a deferred requirement or deferred scenario (marks in a language without a query count too). One per ID, however many marks there are | That ID | The heading line of the requirement; for a scenario, its tag line |
| `depends_on_deferred` | A non-deferred requirement or property points at a deferred requirement through its `- definition:` line or a backquoted ID in a statement. A scenario that is not a deferred scenario points at a deferred requirement through `@about` or a backquoted ID in a step. One per reference | The referring ID and the referenced ID, separated by a single space | The line where the reference is written |

They do not appear for references from deferred requirements or deferred scenarios, for references to deferred scenarios, or for references from `- related:` in a problem record.

`deferred_with_test` means you finished building it but forgot to remove the deferral, or the mark is wrong.
If it is built, write the decision in a decision record and then remove the `- deferred:` line.
If it is not built yet, remove the mark.

`depends_on_deferred` is a sign that an item you are about to build relies on an item you are not building.
Decide whether to defer the referring item too, bring the requirement back from deferral, or drop the reference.
Each of these is a specification decision.

## Errors in the declaration

<!-- @kotowari[REQ-core-209:bc5f958e, REQ-core-210:0772cc2a, REQ-core-115:117f839a, TBL-core-008:67ba1ee9, TBL-core-019:d5c9adce] -->

The value of `- deferred:` is checked by the same rules as `- source:`.
The line of the error is that `- deferred:` line.

```console
$ kotowari check --format text      # 値を空にした
docs/ir/greet/i18n.md:4 [error] missing_source deferred
$ kotowari check --format text      # 無い決定の番号を書いた
docs/ir/greet/i18n.md:4 [error] source_invalid docs/decision/records/2026-09-24-greet.md#A9
```

| Kind | Common cause | How to fix |
|---|---|---|
| `missing_source` | The value of `- deferred:` is empty. detail is `deferred` | Write the decision to defer |
| `source_invalid` | The source target has no line with that decision number | Point at a decision that exists in the decision record |
| `duplicate_field` | There are two document-level `- deferred:` lines (the same applies when you write two under a requirement). detail is `deferred` | Merge them into one line. Only the value of the first line is read |
| `unknown_field` | Written after the title of `CONTEXT.md` or `FLAGS.md` | Write it in a topic document |
| `missing_scope` | After the title there is nothing but the `- deferred:` line | Write a sentence stating what the document covers |

Even with an empty value or a wrong source, the requirement stays deferred.
Until you fix the error, the source error appears in place of the missing-test error.

## Fingerprints and guides

<!-- @kotowari[REQ-core-203:1fdc0443] -->

Unlike `- source:`, the `- deferred:` line under a requirement's heading is part of the requirement's fingerprint.
In a document with a document-level declaration, the first such line is prepended to the fingerprint of every requirement in that document.
It is not part of the fingerprints of decision tables or scenarios.

So declaring or removing a deferral raises `guide_stale` on any guide section whose mark lists that requirement.
Removing a deferral means shipping that feature, so this prompts you to review the guide.
How fingerprints work is described in [writing-guides.md](writing-guides.md#fingerprints).

## Related

- Specification: [deferral](../ir/core/deferred.md), [missing-test checks](../ir/core/coverage.md)
- List of findings: [findings.md](findings.md)
- Counts and listings: [status](commands/status.md), [list](commands/list.md), [query](commands/query.md)
- Test marks: [marks.md](marks.md)
