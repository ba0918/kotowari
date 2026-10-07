# Finding shape

One JSON file per branch holds the current state of every finding (a snapshot that is
overwritten, never an append-only log). Only cycle writes it. Reviewers return arrays of
findings in the same shape, without `id`, `status`, `commits`, or `evaluations`; the caller
fills those.

```json
{
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
| `lives_in` | `implementation` / `ir_or_record` |
| `sent_to` | `fixer` / `consistency_phase` / `person` |
| `evaluations` | one per review or consistency phase run that evaluated this finding, with the round-trip number: `{"round": n, "verdict": "still_present" \| "no_longer_visible"}`; a new finding's first entry is `still_present` with the round that raised it; a full-review match appends `still_present` |

Diff-review return shape: `{"verdicts": [{"id": 7, "verdict": "still_present"}], "new": [ ...findings... ]}`.
