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

The merge target is the one cycle received (cycle's **Inputs**), and the pull request points at
it. Run this with `sh` in the worktree, with `TARGET` set to the merge target; it is POSIX shell only:

```sh
set -eu
cd "$(git rev-parse --show-toplevel)"
git fetch -q origin "$TARGET"
base=$(git merge-base "origin/$TARGET" HEAD)
list=.kotowari/confirm-paths
git diff --name-only --no-renames "$base" HEAD -- "$list"
git cat-file -e "HEAD:$list" 2>/dev/null || echo "$list"
for rev in "origin/$TARGET" HEAD; do
  git cat-file -e "$rev:$list" 2>/dev/null || continue
  specs=$(git show "$rev:$list")
  while IFS= read -r spec; do
    spec=${spec%"${spec##*[![:space:]]}"}
    spec=${spec#"${spec%%[![:blank:]]*}"}
    [ -z "$spec" ] || git diff --name-only --no-renames "$base" HEAD -- "$spec"
  done <<EOF
$specs
EOF
done
```

It compares the commit where the branch parted from the merge target with the head of the branch,
and matches the changed paths against the pathspecs in `.kotowari/confirm-paths`, one per line,
each read with the blanks around it and a carriage return at its end taken off (a line that is
then empty is skipped), and each on its own so that no line can take a hit away from another. It reads both
the merge target's current list and the branch's; a hit under either is a hit. `--no-renames`
counts a moved file under both its old and its new path. A change to the file itself always
counts, whatever it lists, and a branch without the file prints its name, the same as a hit. Hit:
the command printed a path (a path can print more than once); the printed paths are the paths
that were hit. No hit: it printed nothing and exited 0. A nonzero exit is a failure of the
procedure, whatever it printed: treat it as a hit, record no paths, and report the failure with
its error output, not as paths that were hit.

A project lists in `.kotowari/confirm-paths` the definitions of its checks: CI, hooks, the scripts
that run checks, and the configuration of mutation tests, of static analysis and of tool versions.
The places of the specification and of decision records are not listed. A repository that holds
the source of this skill text lists that source's path too.

Judge the line again each time cycle adds commits to the branch, always over the whole diff: a hit
that a later change takes out of the diff is no longer a hit.

## When a person confirms

Of the runs that converged, a person confirms the merge in these three cases only:

- the line is hit;
- a finding is still open as `human_judgment`, whatever raised it (the consistency phase or a
  review);
- the consistency phase revised an approved requirement after showing by a contradiction or a
  reproduction that it was wrong: a `consistency` finding in the findings file whose claim starts
  with `revised <ID>`. The phase finishes its work all the same; only the request to merge is
  withheld.

In each case cycle creates the pull request, does not ask for it to be merged, leaves it open, and
shows the person where it is and what caused the stop: the paths that were hit (or, when the
procedure of the line failed, that failure with its error output), the open findings, or the
revised requirements. It shows this as soon as the pull request is created, before the watch, and
adds no summary of the change. A concretisation the consistency phase added to the specification
or a decision record after approval is not a reason to stop.

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
3. Confirm whether the merge target requires checks, by branch protection
   (`gh api repos/{owner}/{repo}/branches/<target> --jq '.protection.required_status_checks.contexts'`)
   and by a ruleset
   (`gh api repos/{owner}/{repo}/rules/branches/<target> --jq '[.[] | select(.type == "required_status_checks")]'`):
   it does when either prints a non-empty list, and does not when both succeed with none; a failing
   command leaves it unconfirmed. Every
   run that creates a pull request does this, one that stops at it included. The answer is not
   kept: **Watching** reads the names again itself.
4. When a case of **When a person confirms** holds, stop at the pull request as that section says,
   and go on to **Watching**.
5. Otherwise confirm that the repository allows merging by itself
   (`gh api repos/{owner}/{repo} --jq .allow_auto_merge` is `true`), that step 3 found required
   checks, and that the merge target requires no approving review, by branch protection
   (`gh api graphql -f query='query($o:String!,$r:String!,$b:String!){repository(owner:$o,name:$r){ref(qualifiedName:$b){branchProtectionRule{requiresApprovingReviews}}}}' -F o='{owner}' -F r='{repo}' -f b=refs/heads/<target> --jq '.data.repository.ref | select(. != null) | .branchProtectionRule.requiresApprovingReviews // false'`
   prints `false`), by a ruleset
   (`gh api repos/{owner}/{repo}/rules/branches/<target> --jq '[.[] | select(.type == "pull_request") | .parameters.required_approving_review_count] | add // 0'`
   prints `0`), and by the pull request itself
   (`gh pr view <pr> --json reviewDecision --jq .reviewDecision` prints an empty line; the first
   two read the settings, and branch protection is not shown to an account that may not see it,
   while the host reports `REVIEW_REQUIRED` here whatever requires the review). Only these exact
   outputs confirm: all three are needed, and any other output, an empty one from the first two
   included, is not a confirmation. When one of these does not hold or cannot be confirmed (a
   command fails), stop at the pull request, report that merging by itself is not available and
   why, leave the merge to the person, and go on to **Watching**.
6. Ask the hosting service to merge once the required checks pass:
   `gh pr merge <pr> --auto --<method>`, with the method the repository reports
   (`gh repo view --json viewerDefaultMergeMethod`, lowercased).

Cycle never performs the merge itself and never changes a setting of the repository.

## Withdrawing the request

Withdraw the request (`gh pr merge <pr> --disable-auto`) whenever the hosting service shows one on
the branch's pull request, whether this session asked or an earlier one did
(`gh pr list --head <branch> --state open --json number,autoMergeRequest`; a non-null
`autoMergeRequest` is a request):

- before delegating anything that adds a commit to the branch, and before the push of step 2 of
  **Opening and asking**: a resume at any row of cycle's table, "run more", and a new
  kotowari-iterate request on a reused branch read it from the hosting service first, since the
  session that asked may have ended;
- when the run no longer converges, the line is hit, or a case of **When a person confirms** holds;
- when handing the pull request to the person.

Ask again only after confirming, for the new head, that the run converged, the line is not hit and
no case of **When a person confirms** holds.

## Watching

After creating the pull request, watch its required checks to their result, in a run that stops at
the pull request too. First read their names from the hosting service, on a resume as well, with
the reads of step 3 of **Opening and asking** printing names
(`gh api repos/{owner}/{repo}/branches/<target> --jq '.protection.required_status_checks.contexts[]?'`
and
`gh api repos/{owner}/{repo}/rules/branches/<target> --jq '.[] | select(.type == "required_status_checks") | .parameters.required_status_checks[].context'`):

- required (either printed a name): one that has not reported yet is not a failure. Read
  `gh pr checks <pr> --required --json name --jq '.[].name'` again until it prints every one of
  those names, then watch them (`gh pr checks <pr> --required --watch`);
- not required (both succeeded and printed nothing): there is nothing to watch;
- unconfirmed (a read failed): there are no names to wait for. Watch what
  `gh pr checks <pr> --required --watch` shows, then hand the pull request to the person with the
  failure and its error output.

After the checks passed, while the request to merge stands, wait for the merge: read
`gh pr view <pr> --json state,autoMergeRequest,mergeStateStatus` again until it changes. The
hosting service is read as not merging only when the request is gone while the pull request is
open, or `mergeStateStatus` is a state that waiting does not change: `DIRTY` (a conflict) or
`BEHIND` (the branch is behind its target, when the merge target requires it to be up to date).
Cycle has no time limit of its own for the watch.

A required check that failed becomes a finding in the findings file: perspective `quality`,
severity `critical`, action `fix_and_verify`, the evidence the check's name and the failing lines
of its log (`gh run view <run> --log-failed`), the oracle that check passing, `measured`
`fails_now`. It follows the existing round trips and endings: withdraw the request, run steps 4 and
6 of cycle's **Loop**, then come back to step 1 of **Opening and asking**. When an ending other
than ending 1 is reached, the checks still do not pass: hand the pull request to the person.

The watch ends when:

- cycle sees the pull request merged (`gh pr view <pr> --json state` is `MERGED`): **Cleanup**;
- it hands the pull request to the person after failed checks;
- the checks passed and the hosting service is read as not merging, as above: hand the pull request
  to the person with that reason;
- the pull request was closed without being merged (`state` is `CLOSED`): report it to the person;
- in a run that stopped at the pull request, the checks passed;
- the names could not be read: it hands the pull request to the person, as above;
- the merge target requires no check (the names read none): the watch ends at once.

Then write `ending`: `merged`, or `pr_open` with the cause as its reason (`pr_closed` when the
pull request was closed without being merged, `pushed` when no pull request could be created).

## Resuming

When the branch has a pull request, a resume reads from the hosting service whether it exists,
whether merging by itself was asked for, the state of the checks, and whether it was merged
(`gh pr list --head <branch> --state all --json number,state,autoMergeRequest`, then
`gh pr checks <pr> --required`), and goes on:

| The host shows | Go on at |
|---|---|
| merged | write `ending` `merged` and report; clean up nothing, since this run's watch did not see the merge |
| closed without a merge | write `ending` `pr_closed` and report it to the person |
| open, merging asked for | **Watching** |
| open, not asked for | step 6 of cycle's **Loop**, then step 1 of **Opening and asking**, the pull request already there |

A resume reaches this table only with no visible finding open; one still open goes through
cycle's **Loop** first.

## Cleanup

Only when cycle saw the merge itself, in this run's watch. A run the person merges later is not
cleaned up.

1. Confirm that the findings file lies outside the worktree. If it does not, leave everything and
   report.
2. `git -C <worktree> status --porcelain --untracked-files=all`: when it lists anything other than
   the implementer's working notes (`.agents/scratch/`), leave the worktree and the branch and
   report what is left. Files git ignores, build output for one, are not uncommitted changes: they
   do not keep the worktree.
3. From the main working tree, remove the worktree (`git worktree remove --force <worktree>`; the
   force discards the working notes and the files git ignores) and the local branch
   (`git branch -D <branch>`).

Delete nothing on the hosting service: not the remote branch, not the pull request.
