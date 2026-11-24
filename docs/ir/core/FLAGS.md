# Flag record

English | [日本語](FLAGS.ja.md)

Records the places this IR is known not to specify fully. What is decided but not yet implemented is held by the `deferral` of each `requirement`.

## Flags

### FLAG-core-012: Whether a finding whose failing oracle was replaced counts as shown by running

- kind: ambiguity
- related: REQ-core-368
- source: docs/decision/records/2026-11-24-unattended-merge-consistency.md#A4

REQ-core-368 lets only a mistake that running something shows keep a run from converging. The cycle replaces the oracle of a finding with "all existing checks passing" in two cases: the finding demands a new test or fixture without showing that it meets the evidence conditions, and the finding is a warn. The run the reviewer saw fail was a run of the oracle that was replaced. Neither the requirement nor the decisions it cites say whether such a finding counts as shown by running. The default in force: it counts, so a finding the reviewer saw fail stays among the findings to fix, and one not seen failing is recorded only. The other reading: when the replaced oracle did not meet the evidence conditions, its failing run is not evidence, and the finding is recorded only.
