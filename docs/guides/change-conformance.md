# Bringing change conformance into hooks and CI

English | [日本語](change-conformance.ja.md)

## Targets and records

<!-- @kotowari[REQ-core-264:e0aaf99d, REQ-core-276:be843c08] -->

List product code, tests, distributed skills, and build, hook and CI configuration in `changes.files`, and name generated files explicitly in `changes.exclude`. Keep change records in `.kotowari/changes/*.yaml`, using two fixed files: `implementation.yaml` for the whole branch and `review.yaml` for the independent review. This is a convention of the adopting project, not a file-name constraint of the core. For the storage format, see [changes](commands/changes.md). The records themselves, the IR, decision records and the configuration in use are excluded from the diff targets.

## Intermediate commits

<!-- @kotowari[REQ-core-258:b4a8ba89] -->

Intermediate commits require neither a change record nor a passing changes run. Do not put changes in pre-commit. If the implementer wants to check their own record before committing, they can optionally run `changes --base HEAD --staged --phase implementation`.

## Final check

<!-- @kotowari[REQ-core-257:546bf22d, REQ-core-259:5a524bd6, REQ-core-275:4f3cea7d, REQ-core-276:be843c08] -->

Fix the comparison base for the whole branch, and write the implementer's record against that base. A separate review checks whether the reasons hold, what the specification and implementation mean, and the delegated scope, and writes its own reviewer record. Re-reconcile first any added tests, changed expected values, or deleted tests that carry a requirement ID, and include changes to product code and helper functions as well.

Commit both roles' records and fixes, pin the full SHA of the target, then run the tests, check, and `changes --base <base> --head <head> --phase review`. Merging requires both check and changes to exit with 0. status complete alone does not guarantee the final reconciliation. If there is a change or an IR fix, recheck all files and the IR of that change, update both roles' records, commit again, and reconcile again. Do not put past entries with dates or hashes, or listings, in the records directory; leave the history to Git. Delete the records of both roles that a change in the meaning of the changes, the IR or a decision, a rebase, a cherry-pick, or a parallel integration has invalidated, then rewrite them through an independent re-reconciliation.

Classify specification gaps separately from severity and action, and record either the choice with the decision behind it and the related IR, a fix to the existing specification with the requirement that addresses it, or an explicit deferral with who takes it over. Do not treat info/record_only alone as handled. Only make the delegated scope more concrete, keeping the approved requirements, as an autonomous addition; send changes or deletions of approved requirements, contradictory choices, and meanings the reasons cannot settle back to a person. After adding IR, run check again, together with an independent reconciliation of the specification and implementation that includes the addition.

## What CI targets

<!-- @kotowari[REQ-core-261:17f7c85b, REQ-core-276:be843c08] -->

This repository's PR workflow fetches the history of the event's base SHA and head SHA, uses their merge-base as the comparison base and the head SHA as the target. The synthesized merge SHA is not a target. It runs check and the review-phase changes with the same engine. Publishing the workflow externally requires approval.

If you bring it into push, use the event's before as the comparison base and after as the target. For a new branch whose before is all zeros, the comparison base cannot be determined, so it stops, and the caller states the comparison base explicitly. The event's comparison base is never chosen from what the records claim about themselves alone.

## Everyday searching

Writing `.kotowari/changes/` in `.ignore` keeps the long machine-oriented records out of ordinary rg searches. To look at the records, read the file by name or use `rg --no-ignore`. This is not a Git ignore, so the records stay tracked, and check/status and changes read the configured records. A completed comparison is verified with the configuration and records of the commit at that time. Past commits in the old format with `state` need the tool version of that time.
