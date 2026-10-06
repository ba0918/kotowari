# Evidence conditions

The one statement of when a test, a check, or a fixture counts as evidence. Reviewers and every
agent that edits for cycle get **Conditions** pasted into their prompt; brainstorm and plan read
the whole file when they decide how a requirement is verified.

## Conditions

An oracle — a test, a check, or a fixture — counts as evidence only when all four hold:

1. the condition it produces has a named operational producer in a supported environment
   (untrusted input arriving at a boundary is one);
2. its subject is the product or a check rather than the oracle itself;
3. the rule it enforces is stated by the specification;
4. every wording, file layout, or internal name it pins is declared there as a contract.

The specification here is the project's specification, or its public user-facing documentation
when none exists, and the supported environments are those it declares. An oracle that fails any
condition is a cost: do not add it, keep it in a change under review, or demand it.

## A requirement no oracle can meet

A requirement whose only oracle would fail these conditions is not mechanically verifiable:
when it is not code, verify it by a human-run check or by the platform's own checker; when it
is code, drop the requirement and let the failure join a generic error path a reachable
failure already proves — never resolve it by having the implementer build the fixture.
