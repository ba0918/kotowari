# Test-side findings

English | [日本語](test-side-findings.ja.md)

Covers the test-side findings that can be left out of the exit-code judgement of check, so as not to stop commits in which the tests are not yet complete. It decides the behaviour of the option, which findings are test-side findings, and the guidance on building it into hooks. How the option is accepted is decided in [The options each command accepts are fixed, and options and positional arguments may be written in any order](cli.md#the-options-each-command-accepts-are-fixed-and-options-and-positional-arguments-may-be-written-in-any-order), and the list of exit codes in [The exit code is 0 when there is no error, 1 when there is an error, and 2 when kotowari stops](cli.md#the-exit-code-is-0-when-there-is-no-error-1-when-there-is-an-error-and-2-when-kotowari-stops).

## With --allow-test-findings, check does not count test-side findings toward the exit code

Status: implemented

### Problem

The commit that approves the specification comes before the implementation, and at that point what was newly decided always gets the error "there is no test". Commits partway through writing tests first also get errors in test files. In another project that uses kotowari, putting check into the pre-commit hook stopped the commit approving the specification several times.

kotowari itself avoided this by hard-coding the kinds of test-side findings in the hook configuration. That knowledge was not in the kotowari binary, did not reach other projects, and went stale as finding kinds increased. Which findings do not stop a commit is held on the kotowari side. If the place of stopping moves to push, not delivering things without tests to the shared branch is still kept.

Now, if a decision not built yet is written in the "proposed" status, check does not stop even if only the specification is committed first ([The implementing side changes the status, and decisions not built yet stay "The implementing side changes the status, and decisions not built yet stay "proposed""](confirmations.md#the-implementing-side-changes-the-status-and-decisions-not-built-yet-stay-proposed)). Even so, commits in which the tests of an implemented decision are not yet complete still occur, so this option is kept ([When taking in existing code, behaviour the person has ratified is written as "When taking in existing code, behaviour the person has ratified is written as "implemented""](workflow.md#when-taking-in-existing-code-behaviour-the-person-has-ratified-is-written-as-implemented)).

### Decision

Only when `--allow-test-findings` is given to check are test-side findings (their range is in [Test-side findings are limited to errors caused by test files, and their range is set in a table](#test-side-findings-are-limited-to-errors-caused-by-test-files-and-their-range-is-set-in-a-table)) not counted in the exit-code judgement.

Even with the option, test-side findings are not removed from the output, and their severity stays "error". The kinds, severity and order of findings in the output are the same as without the option; only the exit-code judgement changes. This is because if the meaning of the same finding changed with how check is invoked, it would disagree with status and with the reading side, and if they became invisible, the amount left unfinished would not be known. The name is allow because the findings stay output, so allowing fits the content better than ignoring (skip, ignore).

Only check accepts this option. This is because what decides whether to stop is only the exit code of check used as the gate. complete in status is not changed.

### Rejected options

- Previous decision: Narrow down by listing, in the pre-commit hook configuration, the kinds of findings that do not stop. Because whole kinds were excluded, even unreadable surface files (product code) and form errors of guide markers were let through. The knowledge is not in the kotowari binary and does not reach other projects.
- Add a classification field to the JSON of findings. The narrowing would end up being written on the hook side anyway.
- Write it in the configuration file. There is one configuration, so pre-push and CI would loosen too.
- Downgrade test-side findings to notices. The meaning of the same finding would change with how check is invoked.
- Remove test-side findings from the output. The amount left unfinished would not be known.
- Pass only that commit with an option that skips the hooks. It builds the habit of skipping the gate.
- Put what was newly decided into the specification, for the time being, in a form not checked by tests. It would mean writing a lie in the specification.

### Confirmations

- C-core-testside-1 When there is no error besides test-side findings, check with `--allow-test-findings` ends with exit code 0, and check without it ends with exit code 1
- C-core-testside-2 With or without `--allow-test-findings`, the kinds, severity and order of findings in the output of check are the same, and the severity of test-side findings stays "error"
- C-core-testside-3 Giving `--allow-test-findings` to status stops it

## Test-side findings are limited to errors caused by test files, and their range is set in a table

Status: implemented

### Problem

If test-side findings were decided by kind alone, errors of the same kind from places that are not test files (surface files, guide markers) would also pass through. If the cause is limited to test files, commits that put the specification first and commits partway through writing tests first pass, and everything else stops.

When a file that is both a test file and a surface file cannot be read, it cannot be read as either side, so it is the same as the product code being broken.

With only the general sentence "findings caused by test files", readers would split on which kinds are included, and the range to check by tests would not be fixed either. With a table, drift is visible as a diff in review.

### Decision

Test-side findings are errors caused by test files, and their range is set by the following table. Errors of kinds not in the table are not test-side findings.

| Kind | Condition for being a test-side finding |
|---|---|
| confirmation_without_test | Always |
| test_without_id | Always |
| invalid_marker | When it occurs in a test file |
| unresolved_reference | When it occurs in a test file |
| unparsable_file | When it occurs in a test file and that file is not a surface file |

Errors from guide markers and surface files are not included in test-side findings. When a kind of finding that occurs in test files is added in the future, a row is also added to this table in the brainstorm that decides that kind.

### Rejected options

- Set it with only a general sentence, without a table. Readers would split on which kinds are included, and the range to check by tests would not be fixed either.

### Confirmations

- C-core-testside-4 confirmation_without_test and test_without_id are test-side findings in whatever file they occur
- C-core-testside-5 If there is no error besides invalid_marker, unresolved_reference and unparsable_file occurring in test files, check with `--allow-test-findings` ends with exit code 0
- C-core-testside-6 An error of a kind not in the table stops check with `--allow-test-findings`, even if it comes from a test file
- C-core-testside-7 A form error of a guide marker stops check with `--allow-test-findings`
- C-core-testside-8 unparsable_file for a file that is both a test file and a surface file, and unparsable_file for a file that is only a surface file, stop check with `--allow-test-findings`

## How to build the hooks is guided in the skill and in the guide of check

Status: implemented

### Problem

Even with the option, if other projects do not know how to build the hooks, the same accident happens.

### Decision

The kotowari skill, the adoption guidance of kotowari-adopt, and the "Putting it into hooks" section of the guide of check describe the setup in which the pre-commit hook runs check with `--allow-test-findings`, and the pre-push hook and CI run it without.

### Confirmations

- C-core-testside-9 (by reading) All three places, "agent/skills/kotowari/references/findings.md", "agent/skills/kotowari-adopt/" and "docs/guides/commands/check.md", say that the pre-commit hook gives `--allow-test-findings` to check, and that the pre-push hook and CI do not
