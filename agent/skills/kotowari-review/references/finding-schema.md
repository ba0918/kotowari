# Finding shape

One JSON file per branch holds the current state of every finding (a snapshot that is
overwritten, never an append-only log). Only cycle writes it. Reviewers return arrays of
findings in the same shape, without `id`, `status`, `commits`, or `evaluations`; the caller
fills those.

```json
{
  "base": "<commit the full review diffs from>",
  "last_reviewed_head": "<branch head at the previous review>",
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
  ],
  "consistency_phase": {
    "review_head": "<head the cycle's step-3 full review last read; null until it runs>",
    "runs": [
      {
        "step": 2,
        "base": "<commit>",
        "head": "<commit>",
        "findings": [
          {"kind": "gap", "where": "src/x.py:40", "mark": "new", "resolution": "IR item added", "commits": ["<hash>"]}
        ],
        "converged": false
      }
    ]
  }
}
```

| Key | Values |
|---|---|
| `severity` | `security` / `critical` / `warn` / `info`; the caller changes a finding that states no defect to `warn` |
| `action` | `auto_fix` / `fix_and_verify` / `human_judgment` / `record_only` (reviewer proposal; caller decides) |
| `profile` | `Code` / `Document` / `Skill` |
| `perspective` | `quality` (the only perspective) |
| `oracle.measured` | `fails_now` / `not_run` (unsafe; reason in note) / `not_applicable` (info, human_judgment) |
| `status.state` | `open` / `closed`; `closed_reason` is `fixed` or `accepted` |
| `commits` | commit hashes the fixer reported for this finding |
| `evaluations` | one per review that evaluated this finding, with the round-trip number: `{"round": n, "verdict": "still_present" \| "no_longer_visible"}`; a full-review match appends `still_present` |

`consistency_phase` is cycle's record of the consistency phase, kept apart from the review
findings. `runs` holds one entry per run in order: the cycle step it ran in, its range, the
findings it returned (`kind` is `gap`, `disagreement` or `within_ir`; `mark` is `new` or
`repeat`), and `converged`, true when it raised no new finding.

Diff-review return shape: `{"verdicts": [{"id": 7, "verdict": "still_present"}], "new": [ ...findings... ]}`.
