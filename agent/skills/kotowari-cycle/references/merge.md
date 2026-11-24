# Merge

What cycle does after ending 1 (converged): judge the line, open the pull request, ask the hosting
service to merge it or stop for the person, and watch the required checks. Cycle runs every command
here itself; nothing here adds a delegate. Only ending 1 comes here: endings 2–4 report to the
person as before, with no pull request and no request to merge. A change the main session edits
directly ends at its commit; nothing here applies to it.

The commands are for GitHub (`gh`). On a host where cycle cannot create a pull request, it pushes
the branch, stops, and shows the cause in the terminal report.

## The line

The line separates a change a person confirms from one that merges without them. It is judged by
git alone, never by an LLM and never by a kotowari command, and has two values only: hit or no hit.

The merge target is the branch the pull request points at; before there is one, the branch decided
for it, else the repository's default branch (`gh repo view --json defaultBranchRef`). Run this in
the worktree, with `TARGET` set to the merge target:

```bash
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
git fetch -q origin "$TARGET"
base=$(git merge-base "origin/$TARGET" HEAD)
if git cat-file -e HEAD:.kotowari/confirm-paths 2>/dev/null; then
  mapfile -t specs < <(git show HEAD:.kotowari/confirm-paths | grep -v '^[[:space:]]*$' || true)
  git diff --name-only --no-renames "$base" HEAD -- .kotowari/confirm-paths "${specs[@]}"
else
  echo .kotowari/confirm-paths
fi
```

It compares the commit where the branch parted from the merge target with the head of the branch,
and matches the changed paths against the pathspecs in `.kotowari/confirm-paths`, one per line,
as written. `--no-renames` counts a moved file under both its old and its new path. The file itself
always counts, listed or not, and a missing file prints its name, the same as a hit. Hit: the
command printed a path or exited nonzero; the printed paths are the paths that were hit. No hit: it
printed nothing and exited 0.

A project lists in `.kotowari/confirm-paths` the definitions of its checks: CI, hooks, the scripts
that run checks, and the configuration of mutation tests, of static analysis and of tool versions.
The places of the specification and of decision records are not listed. A repository that holds
the source of this skill text lists that source's path too.

Judge the line again each time cycle adds commits to the branch, always over the whole diff: a hit
that a later change takes out of the diff is no longer a hit.

## When a person confirms

Of the runs that converged, a person confirms the merge in these three cases only:

- the line is hit;
- a finding is still open as `human_judgment`, whatever raised it (the consistency phase, a review,
  or a finding the person chose to leave);
- the consistency phase revised an approved requirement after showing by a contradiction or a
  reproduction that it was wrong (the phase returns those IDs). The phase finishes its work all the
  same; only the request to merge is withheld.

In each case cycle creates the pull request, does not ask for it to be merged, leaves it open, and
shows the person where it is and what caused the stop: the paths that were hit, the open findings,
or the revised requirements. It adds no summary of the change. A concretisation the consistency
phase added to the specification or a decision record after approval is not a reason to stop.

When the implementer returns that an approved requirement has to be changed or deleted, that is
ending 4: a person comes in and nothing is merged. An operation that cannot be undone (a release,
publishing, a tag, a deletion of something outside the local machine) stays with the person under
the existing stop rules.

## Opening and asking

1. Judge the line.
2. Push the branch (`git push -u origin <branch>`) and, when there is none yet, create the pull
   request against the merge target
   (`gh pr create --base <target> --head <branch> --title <one line> --body-file <file>`). The
   body is the decision lines of cycle's **Terminal report** and no summary of the change.
3. When a case of **When a person confirms** holds, stop at the pull request as that section says,
   and go on to **Watching**.
4. Otherwise confirm both that the repository allows merging by itself
   (`gh api repos/{owner}/{repo} --jq .allow_auto_merge` is `true`) and that the merge target
   requires one or more checks, by branch protection
   (`gh api repos/{owner}/{repo}/branches/<target> --jq '.protection.required_status_checks.contexts'`)
   or by a ruleset
   (`gh api repos/{owner}/{repo}/rules/branches/<target> --jq '[.[] | select(.type == "required_status_checks")]'`).
   When either is absent or cannot be confirmed (a command fails), stop at the pull request, report
   that merging by itself is not available and why, and leave the merge to the person.
5. Ask the hosting service to merge once the required checks pass:
   `gh pr merge <pr> --auto --<method>`, with the method the repository reports
   (`gh repo view --json viewerDefaultMergeMethod`, lowercased).

Cycle never performs the merge itself and never changes a setting of the repository.

## Withdrawing the request

After asking, withdraw it (`gh pr merge <pr> --disable-auto`):

- before delegating anything that adds a commit to the branch;
- when the run no longer converges, the line is hit, or a case of **When a person confirms** holds;
- when handing the pull request to the person.

Ask again only after confirming, for the new head, that the run converged, the line is not hit and
no case of **When a person confirms** holds.

## Watching

After creating the pull request, watch its required checks to their result
(`gh pr checks <pr> --required --watch`), in a run that stops at the pull request too. Cycle has no
time limit of its own for the watch.

A required check that failed becomes a finding in the findings file: perspective `quality`,
severity `critical`, action `fix_and_verify`, the evidence the check's name and the failing lines
of its log (`gh run view <run> --log-failed`), the oracle that check passing, `measured`
`fails_now`. It follows the existing round trips and endings: withdraw the request, run step 4 and
step 6, then come back to step 1 of **Opening and asking**. When an
ending other than ending 1 is reached, the checks still do not pass: hand the pull request to the
person.

The watch ends when:

- cycle sees the pull request merged (`gh pr view <pr> --json state` is `MERGED`);
- it hands the pull request to the person after failed checks;
- the checks passed and the hosting service reports that it does not merge (the merge request is
  gone while the pull request is open, or `mergeStateStatus` is `DIRTY` or `BLOCKED`): hand the
  pull request to the person with that reason;
- in a run that stopped at the pull request, the checks passed.

## Resuming

When the branch has a pull request, a resume reads from the hosting service whether it exists,
whether merging by itself was asked for, the state of the checks, and whether it was merged
(`gh pr list --head <branch> --state all --json number,state,autoMergeRequest`, then
`gh pr checks <pr> --required`), and goes on:

| The host shows | Go on at |
|---|---|
| merged | the watch has ended at the merge |
| closed without a merge | report it to the person |
| open, merging asked for | **Watching** |
| open, not asked for | step 1 of **Opening and asking**, the pull request already there |
