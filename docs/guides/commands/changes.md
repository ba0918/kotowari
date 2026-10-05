# kotowari changes — checking changes against change records

English | [日本語](changes.ja.md)

Enumerates the changes independently from Git and checks that they correspond to the YAML records the caller wrote, and that those records are fresh. Whether the reasons are valid, whether the meaning matches, and whether the review was independent are confirmed in a separate review.

## What is compared, and phases

<!-- @kotowari[REQ-core-263:316e333f, REQ-core-265:5cd15a3c] -->

```sh
kotowari changes --base HEAD --staged --phase implementation
kotowari changes --base <base-sha> --head <head-sha> --phase review --format json
```

base, head or staged, and phase are all required and have no defaults. staged can only be combined with base HEAD and implementation. A commit REV is resolved to the full object ID. For the index and for commits, the configuration, the records and the IR are also read from the same snapshot; unstaged or untracked content is never used to fill gaps.

## Records

<!-- @kotowari[REQ-core-268:26f876d5, REQ-core-270:66112bc2] -->

The settings go under `changes` in [config.md](../config.md), and the caller creates YAML files matched by the `changes.records` glob. kotowari does not write records. For the exact storage format, see [change-record-format.md](../../ir/core/change-record-format.md); for the operating procedure, see [change conformance](../../../agent/skills/kotowari/references/changes.md).

A file holds version: 1 and entries; each entry has id, base, role, files, ir, conclusion, reason, requirements, decisions, handoff and gaps. role is implementer/reviewer, and conclusion is existing/new/deferred. The entry ids, and how files are grouped into entries, need not match between the two roles.

The path, before and after of files record the identifiers of the comparison base and the target. An addition has before:null, and a deletion has after:null. The identifier is the SHA-256 of Git's six-character mode, a NUL, and all bytes of the blob, written as `sha256:` followed by 64 lowercase hexadecimal digits. The path and sha256 of ir are the SHA-256 of all bytes of the IR. Do not repeat a path within the file list or the IR list.

existing has the corresponding requirements and the IR that defines them; new has references to decisions; deferred has references to decisions and handoff. A decision is a repository-relative reference such as `path#A1`. gaps has missing_spec/spec_conflict/premise_conflict, recorded/fixed/deferred, and refs, recording where each gap is handled.

## Pass or fail, and output

<!-- @kotowari[REQ-core-272:af645e8c, REQ-core-273:aeb58924, REQ-core-274:5a370ca0] -->

implementation requires an implementer-side record for each file. review requires records from both roles, and the reviewer must include all related IR of the corresponding implementer. If any target file or IR of an entry is stale, coverage is invalid for all files of that entry. In review, deferred conflicting with the conclusion is an error. Every record matched by the configuration is checked for form and current references; records with a different base are not used for freshness or coverage.

The JSON has six keys: base, target, phase, files, covered and findings. target is a commit ID or index, and files and covered are numbers of files. The notation of findings, and of each entry in text, is shared with check; with no errors the exit code is 0, with errors it is 1, and a stop due to input or execution gives 2. The stop reason for Git read failures is `git error`.

The records use the comparison base of the whole branch as base, and the implementer and an independent reviewer each create theirs separately. `--staged` is an optional self-check before committing; intermediate commits need no records. Only the current comparison is kept in fixed files. History stays in Git, and a past comparison is re-verified from that commit's configuration and records. When a fix to the meaning of the change, IR or decisions, a rebase, a cherry-pick, or merging in a parallel change invalidates the records of both roles, delete them, reconcile the whole entry again independently, rewrite them, and rerun check and changes. A successful check/status alone does not mean the current change has been reconciled.

`state` was removed from version: 1 while it was in development. Old records with `state` are rejected as having an unknown key and are not converted automatically. Re-verify past commits in the old format with the tool version of that time. Past commits that carry the new format can be read from their snapshot by the new version as well.
