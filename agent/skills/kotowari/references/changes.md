Based on the kotowari specification (revised 2026-10-01; the version of kotowari itself is not pinned)

# Change conformance

This reference governs caller-written change records, snapshot checks and their review. `check` validates record shape and active references; `changes` validates coverage and freshness. Neither proves the reason supports the choice, meaning matches the IR, authority stays delegated, or the reviewer is independent.

## Inputs and responsibility

The caller fixes the full comparison-base commit ID, target commit ID (or index for a commit in progress), phase, worktree and approved scope. Derive these from the branch or CI event, never from a record's self-report. The implementer and a review separate from implementation each author their own YAML records. The caller stages and commits them. `kotowari` writes only stdout/stderr and never creates records or gap dispositions. Review findings JSON remains internal; it does not replace either role's common YAML record.

Prioritise additions, expectation changes and deletions in tests carrying requirement IDs. Confirm their expectations against the requirements. Keep changes to product code and helpers in scope even if test expectations did not change.

## Records

Set `changes.files` to product code, tests, distributed skills, build configuration, hooks and CI. Declare generated-file exclusions in `changes.exclude`; `changes.records` is `docs/changes/**/*.yaml` in the adoption example. All globs are relative to the Git root; files and records must be explicit and nonempty. Omission leaves check/status unchanged and makes changes stop with a configuration error.

A record is UTF-8 YAML with exactly version: 1 and entries. Every entry has all twelve keys: id, base, role, state, files, ir, conclusion, reason, requirements, decisions, handoff, gaps. IDs are unique across all read files and match `[A-Za-z0-9][A-Za-z0-9._-]*`. base is a full lowercase Git object ID; role is implementer/reviewer; state active/archived; reason is nonblank. files is nonempty. The other lists may be empty when the conclusion permits; handoff is null or a decision reference. Unknown keys, duplicate YAML keys, wrong types and other versions are errors, not migrations.

files contains path, before and after. The identities are null or `sha256:` plus 64 lowercase hexadecimal characters; both cannot be null. Hash Git's six-character mode, one NUL byte and all blob bytes, in that order. Mode-only changes therefore need reconciliation. ir contains path and sha256, hashing all bytes of the IR file alone. Paths are normal relative paths with no empty component, absolute prefix, `.`, `..` or backslash. No path repeats within either list.

existing needs requirements and all their definition IR files, plus the reason it stays inside them. new needs decisions describing the choice and reason; a behavior or constraint change is also reflected in related IR. deferred needs decisions and a non-null handoff with explicit deferral. Decision references use repository-relative `path#decision-number`, like `docs/decision/records/topic.md#A1`, and must exist for active entries.

Each gap has category (missing_spec, spec_conflict, premise_conflict), disposition (recorded, fixed, deferred), and nonempty refs to decision records. recorded points at the adopted choice and reason and related IR; fixed points at the correction decision and existing requirements and their definition IR; deferred points at explicit deferral and handoff. Severity and action are separate from this classification: info/record_only alone never closes a specification gap. Mixed recorded and fixed gaps are allowed. Any deferred gap makes conclusion deferred; otherwise any recorded gap makes it new; otherwise existing. fixed still needs its requirements and IR when the conclusion is new.

## Updating judgments

The role making a new judgment records the choice, grounds and deciding role in a decision record. Reflect behavior or constraint changes in IR. Do not hand back merely because a record is missing: reconstruct a justified choice within the delegation and update its record autonomously.

This changes the former unconditional hand-back of IR additions: implementer and fixer may add concrete IR only within the delegated scope, with evidence, while keeping approved requirements unchanged. They may not change or delete approved requirements, contradict an existing choice, choose consequential meaning without grounds, or use recording authority to expand scope. Those cases return to the person; irreversible, dangerous and externally visible actions also return. Cycle orchestrates and delegates recording; it never implements or decides meaning itself. After an IR addition, rerun check and review the specification and implementation including the addition.

## Intermediate commit

After formatting and restaging, author active implementer entries whose base is HEAD's full ID and whose file identities match the actual index. Read configuration, files, records, related IR and decision records from that index. Stage the records too; a record only in the working tree cannot cover the commit.

Run check separately, then:

```sh
kotowari changes --base HEAD --staged --phase implementation
```

Read the findings, update justified records and any permitted concrete IR, restage, and rerun. Intermediate commits require implementer coverage; they do not require final independent review. deferred can pass implementation when its structure and references are complete.

## Final independent review

Fix the branch-wide base and candidate head. Recreate implementer records for that base. A review separate from implementation checks grounds, meaning and delegated scope, then authors reviewer records for the same changed bytes and all related IR. Grouping and IDs may differ by role. A reviewer record must include all IR of corresponding implementer entries; additional IR is allowed. Neither a self-declared reviewer role nor passing the machine check proves that review ran.

Run the whole branch comparison, never just HEAD's parent:

```sh
kotowari check --format json
kotowari changes --base <full-base-id> --head <full-head-id> --phase review --format json
```

Both must exit 0 before integration. Keep out-of-scope findings in the report without expanding the fix; distinguish verified implementation scope from an integration gate that remains blocked. A status complete value does not prove final change conformance. Deferred entries cannot pass review; formally adopting a deferral requires the permission in decision records and IR, then reconciliation as existing or new.

When code or IR changes, reconcile the entire entry containing it: all its files and related IR, both roles as required, then commit, pin the new head and rerun tests, check and changes. Old records with another base do not cover the new comparison. Archive completed comparisons and retain their records; archived entries still need correct shape, but current reference existence and freshness are not checked. Record files, IR, decisions and the selected config are excluded from change enumeration, so adding final records does not create a reconciliation cycle.

## CI and local adoption

Pre-commit runs changes against the formatted index with base HEAD and phase implementation, alongside check. A pull_request workflow fetches base and head history, derives the comparison base using their merge-base, and checks the event's head SHA. Do not use the synthetic merge SHA. Run the same check and changes engine as locally.

For a push example, use the event's before and after SHAs. A before consisting only of zeros is a new branch with no event comparison base: stop and require an explicit base rather than inventing one. A read or history failure also stops. Configure these gates locally before publishing a workflow; publication needs the person's authority.

## Output and stops

JSON has exactly base, target, phase, files, covered and findings. target is the resolved commit ID or index. Counts are per changed file; freshness is per entry. Findings use check's shape and text layout, with error severity and null line. change_uncovered names the changed path and required role. Other change findings name the record path and the entry ID and affected path; invalid files use `file:`, invalid entries use `entry N:` with zero-based position. Sort by path, kind, detail. Exit 0 means no errors, 1 means errors, 2 means input/execution stopped; Git reads stop with `git error`.
