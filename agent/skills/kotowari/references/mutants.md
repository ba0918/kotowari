Based on the kotowari specification (revised 2026-09-23; the version of kotowari itself is not pinned)

Mutation testing puts one small change (a mutation) into the implementation and sees whether the tests fail. If they do not fail, there is no test constraining that change. A mutation that did not make the tests fail is called a miss. `kotowari check` looks only as far as "do requirements and examples have tests with marks", so whether the tests constrain the implementation is looked at here.

## Using kotowari mutants

Running the mutation tests is kept outside kotowari. What kotowari reads is the results file the tool wrote.

```
kotowari mutants --tool cargo-mutants --format text mutants.out/outcomes.json
```

- `--tool` is required. Omitting it or an unknown value stops with an argument error. The shape of the results differs per tool, so there is no default
- The positional argument is exactly one path of a results file, read relative to the current directory
- `--format` is `json` (default) or `text`. `--config` is the same as check

In this repository, `scripts/mutants.sh` does everything from running to `kotowari mutants` in one command. It creates the results file on the spot every time, so there is no room to have stale results read.

## Reading the output

Findings have the same shape as check (`path`, `line`, `kind`, `severity`, `detail`). Look up the kinds in the table in findings.md. `mutant_survived` is an error, `mutant_timeout` and `equivalent_stale` are notices, and `equivalent_invalid` is an error.

The top level of the JSON is only the three keys `findings`, `counts` and `mutants`. The `files`, `lines` and `tests` of `kotowari check` do not appear. `mutants` is the count of mutations:

| Key | What it counts |
|---|---|
| caught | The tests failed with the mutation in place |
| survived | Misses that matched no entry in the list of equivalents |
| timeout | The tests did not finish in time with the mutation in place |
| unviable | It could not be built with the mutation in place |
| equivalent | Misses that matched one or more entries in the list of equivalents and were removed from the findings |

The sum of the five equals the total number of mutations. With `--format text`, the same tally appears on the last line after the finding lines (it appears even with 0 findings).

```
src/a.rs:3 [error] mutant_survived replace == with != in f
mutants: caught=12 survived=1 timeout=0 unviable=4 equivalent=2
```

The `equivalent` count appears every time. It is an amount that relies on declarations of equivalence, and a number for noticing abnormal growth.

The exit code is the same as check: 1 if there is one or more errors, 0 if none, 2 on a stop. Timeouts alone end with 0.

## The list of equivalents

"Equivalent" is a person's or an LLM's judgement that the mutation changes none of kotowari's standard output (every key and line of JSON and text), standard error (the stop message and details) and exit code. Differences in a function's return value or internal state do not count as observations. Since it cannot be verified by machine, the judgement is left in the list file, with a reason required.

The place is the YAML file that the configuration's `mutants.equivalents` points at. There is no default, and without the key the list behaves as empty. One entry has exactly five keys:

```yaml
- file: src/a.rs
  change: replace == with != in f
  text: "if a == b {"
  class: equivalent
  why: This branch is unreachable. An LLM in a separate context was asked to write a test that fails on it, and could not
```

| Key | Content |
|---|---|
| file | Path of the source (relative to the base directory. Absolute paths and `..` cannot be written) |
| change | The description of the change. Copy the text of the results file as it is |
| text | The current source text of the line the mutation goes into |
| class | Only `equivalent`. "Untested" and "suspected defect" cannot clear a finding through the list |
| why | The reason. An empty one (containing only half-width spaces and tabs) is an error |

How matching works: an entry matches when `file` and `change` are the same strings, and `text` is the same as the text of the source line at the result's line number, both compared after trimming leading and trailing half-width spaces and tabs. Since it is not identified by line number, a line that merely moved does not come off; only rewriting that line makes it come off, and falls back to redoing the judgement. In a file with several lines of the same text, one entry applies to the mutations of any of those lines.

The list can remove only misses. A timeout is a notice and does not change the exit code, so there is no way to remove it.

`equivalent_stale` is raised when the `text` of an entry appears nowhere in `file` any more. It is not judged by whether it appears in the results (in a run over only the diff, only some mutations make it into the results, and judging against the results would wrongly call most of the list stale).

## Investigating misses

Sort each miss into one of the following three.

| Class | Meaning | What to do |
|---|---|---|
| Equivalent | The mutation changes none of kotowari's standard output, standard error and exit code (as defined above) | "The equivalent steps" below |
| Untested | The behaviour changes, but no test looks at it | "The untested steps" below |
| Suspected defect | The original code is wrong | Fix it. If it needs an interpretation of the specification, return to brainstorm |

The untested steps: first confirm whether the requirement is specific enough to distinguish that mutation. If it is vague and cannot distinguish it, do not add a test; return to the IR. Adding a test first would pin the vague requirement to the implementation's current behaviour. If it is specific, add a test with the mark of that requirement or example.

The equivalent steps:

1. Before writing it in the list, first see whether simplifying the code can remove the mutation itself. Deleting an unreachable branch or an unused value keeps that mutation from arising. If the list does not grow, there is less upkeep too
2. If it still remains, before adding an entry to the list, have an LLM in a separate context write a test that fails on that mutation. If it can, adopt that test (it was not equivalent). If it cannot, write that attempt in `why`
3. Add an entry to the list. In `why`, write what was tried and could not make it fail

Do not pass your own "this is equivalent" judgement by yourself. Changes to the list mix into the person's ordinary diff review, so no dedicated work is created for the person, but the judgement shows up where it is visible as a diff.

## When to run it

- After a cycle: only the mutations inside the diff with main. The project's gate runs it and stops the integration if misses remain. In kotowari itself that gate is the pull request CI, split into parallel shards, and no hook runs it, because running it before every push made the work wait for hours
- Before a release: the mutations inside the diff from the product's previous release tag, or everything when there is none. In kotowari itself the release CI runs it before building binaries

Exclusions are not used. The source's `#[mutants::skip]` and the tool's exclusion settings do not measure the excluded mutations, so they are no place for judgements. Judgements go in the list of equivalents.
