# Finding shape

One JSON file per branch holds the current state of every finding (a snapshot that is
overwritten, never an append-only log), and the record of the run: the decisions (`path#anchor`) and the merge target
it received, the line, the earlier request it fixes, the reason a plan was written, and its ending. Cost is not recorded. Only cycle writes it; it is never committed, lives
outside the worktree that cleanup removes, and is never deleted by the workflow (the kotowari-cycle
skill's **Resuming and running more**). Reviewers return arrays of
findings in the same shape, without `id`, `status`, `commits`, or `evaluations`; the caller
fills those.

```json
{
  "ids": ["docs/decisions/core/status.md#<anchor of the decision's heading>"],
  "merge_target": "main",
  "base": "<commit the full review diffs from>",
  "last_reviewed_head": "<branch head at the previous review>",
  "first_review_head": "<head cycle's first full review read; null until it runs>",
  "premise_attempts": [
    {
      "fired_at_round": 3,
      "clause": "overlap_after_fix",
      "ladder": [
        {"premise": "the most specific premise the fixes shared", "lives_in": "implementation"},
        {"premise": "the premise the others rest on", "lives_in": "implementation"}
      ],
      "replaced": {"premise": "the premise replaced", "by": "what replaces it", "findings": [5, 7]},
      "sent_to": "fixer"
    }
  ],
  "fixes": [
    {"findings": [5], "commits": ["<hash>"], "reviewed_in": 3}
  ],
  "line": {"hit": true, "paths": ["scripts/gates.sh"]},
  "fix_of": null,
  "plan_reason": null,
  "ending": {"kind": "pr_open", "reason": "the line was hit: scripts/gates.sh"},
  "findings": [
    {
      "id": 7,
      "severity": "critical",
      "action": "fix_and_verify",
      "profile": "Code",
      "perspective": "quality",
      "claim": "one-sentence statement of the problem",
      "evidence": [
        {"path": "src/x.py", "lines": "40-58", "summary": "what was observed"}
      ],
      "oracle": {
        "proposal": "pytest tests/test_x.py::test_rejects_empty",
        "measured": "fails_now",
        "note": "fails because the test does not exist yet; or why not run / why no mechanical oracle"
      },
      "status": {"state": "open", "closed_reason": null},
      "commits": [],
      "evaluations": [{"round": 2, "verdict": "still_present"}]
    }
  ]
}
```

| Key | Values |
|---|---|
| `severity` | `security` / `critical` / `warn` / `info`; the caller changes a finding that states no defect to `warn` |
| `action` | `auto_fix` / `fix_and_verify` / `human_judgment` / `record_only` (reviewer proposal; caller decides) |
| `profile` | `Code` / `Document` / `Skill` |
| `perspective` | `quality` from reviewers; `consistency` on a finding cycle's consistency phase returned |
| `oracle.measured` | `fails_now` / `not_run` (unsafe; reason in note) / `not_applicable` (info, human_judgment) |
| `status.state` | `open` / `closed`; `closed_reason` is `fixed` or `accepted` |
| `commits` | commit hashes the fixer, or for a `consistency` finding the phase, reported for this finding |
| `premise_attempts` | written by cycle only, one entry per attempt of the kotowari-cycle skill's premise step, appended before the attempt's delegation; `[]` until one runs |
| `fixes` | written by cycle only, one entry per fixer return in order: the findings it addressed and its commits, and `reviewed_in`, the round of the review after it (`null` until it runs); facts for resuming and the terminal report |
| `clause` | the ending 3 clause that fired: `still_present_twice` / `cause_returned` / `not_shrinking` / `overlap_after_fix` |
| `lives_in` | `implementation` / `decision_document` |
| `sent_to` | `fixer` / `consistency_phase` / `person` |
| `ids` | written by cycle only, at its start: the decisions it received, one `path#anchor` each, `[]` when it received none; a later request on the same branch, and a kotowari-iterate run on it, adds its decisions; a resume counts them again only when the key is absent |
| `merge_target` | written by cycle only, at its start: the merge target it received, the repository's default branch when none was decided |
| `line` | written by cycle only, each time it judges the line: `hit` (`true` / `false`) and `paths`, the paths that were hit (`[]` when not hit, and when the procedure failed, which is a hit with no paths, reported as a failure); `null` until first judged |
| `fix_of` | written by cycle only: the branch name of the earlier request this request fixes, only when the request says it is a fix of an earlier one; `null` otherwise |
| `plan_reason` | written by cycle only, before its first delegation to implement: the one line of the reason it judged that a plan is to be written; `null` when it judged none is needed, when the branch already had a plan, and until it judges |
| `ending` | written by cycle only, when the run ends: `kind` and `reason`; `null` until then |
| `kind` | `merged` / `pr_open` (stopped at the pull request for the person) / `pr_closed` (the pull request was closed without being merged) / `pushed` (no pull request could be created) / `limit` (round-trip limit) / `no_progress` / `hand_back` (to brainstorm or the person) |
| `evaluations` | one per review or consistency phase run that evaluated this finding, with the round-trip number: `{"round": n, "verdict": "still_present" \| "no_longer_visible"}`; a new finding's first entry is `still_present` with the round that raised it; a full-review match appends `still_present` |

Diff-review return shape: `{"verdicts": [{"id": 7, "verdict": "still_present"}], "new": [ ...findings... ]}`.
