# Evidence conditions

The one statement of when a test, a check, or a fixture counts as evidence. Reviewers and every
agent that edits for cycle get **Conditions** pasted into their prompt; brainstorm reads
the whole file when it decides how a row to confirm is confirmed.

## Conditions

An oracle — a test, a check, or a fixture — counts as evidence only when the conditions that
apply to it hold:

1. the condition it produces has a named operational producer in a supported environment
   (untrusted input arriving at a boundary is one);
2. its subject is the product or a check rather than the oracle itself;
3. the rule it enforces is stated by the specification;
4. every wording, file layout, or internal name it pins is declared there as a contract.

The first two apply to every oracle. The third and fourth apply to every oracle except a declared
test being added or kept: a test whose mark, in the comment block directly before it, carries the
declaration `detail` (`@kotowari[detail]`, or beside row IDs as in
`@kotowari[C-shop-discount-1, detail]`), which says it tests behaviour that has no confirmation row
in the decision documents. Such a test is not removed on the
ground that the specification does not state its rule or declare what it pins; whether it confirms
the rows its mark names is a question of whether the mark is correct, which review still checks,
not of these conditions. A
demand for a new test is held to all four, declared or not.

The specification here is the project's specification, or its public user-facing documentation
when none exists, and the supported environments are those it declares. An oracle that fails a
condition that applies to it is a cost: do not add it, keep it in a change under review, or demand
it.

## A row no oracle can meet

A row to confirm whose only oracle would fail these conditions is not mechanically verifiable:
when it is not code, verify it by a human-run check or by the platform's own checker; when it
is code, drop the row and let the failure join a generic error path a reachable
failure already proves — never resolve it by having the implementer build the fixture.
