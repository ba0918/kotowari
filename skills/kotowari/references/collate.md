Based on the kotowari specification (revised 2026-09-23; the version of kotowari itself is not pinned)

## Input

Hand the following two to an LLM in a separate session. Do not hand over the places for documents or the records whole (the cost of the collation would then be set by the size of the repository).

- The items in the scope of the collation (requirements, decision tables, properties, scenarios, terms, flags) and the source of each. Each item holds its sources in its `- source:` line, its `@source` tag, or the source column of the glossary
- The decision lines those sources point at (the numbered lines of the decision records under `docs/decision/records/`, and their supplementary lines)

## Scope

The scope of the collation is the items this brainstorm added or changed, and the existing items whose sources are decisions this brainstorm changed. Count them mechanically from the diff since the starting point (the commit the brainstorm's branch forked from); do not list them from memory.

- Items added or changed: match the lines changed in `git diff <start> -- <place for the IR>` against the `path` and `line` of each item in `kotowari list`. Terms of the glossary and flags, which do not appear in `kotowari list`, are picked from the lines changed in the diff
- Items whose sources are changed decisions: pick the numbers of the decisions changed in `git diff <start> -- <place for decision records>`, and select the items whose `sources` in `kotowari list` point at them. Example: `kotowari list | jq -r '.items[] | select(.sources | any(test("2026-01-01-example.md#A3$"))) | .id'`

If the scope has 0 items, do not collate.

## Criterion

For each item, confirm whether the one decision line, or the body of the section, that its source points at supports the content of the item.

- Look at whether each piece of the item's content (the statement of a requirement, the rows of a table, the statement of a property, the Given, When and Then of a scenario, the meaning of a term) follows from what is written in the decision the item cites
- If even one value, condition, response or relation is not written in the source, treat the item as "not supported"
- Even when something written elsewhere in the decision records would support it, if it is not written in the item's source, treat the item as "not supported" and say so in the reason
- A rephrasing of words counts as supported if the meaning is the same

## Return shape

Return only the items that are not supported, as JSON. The shape is fixed:

```json
{"unsupported":[{"id":"the item's ID","source":"the source string","reason":"what the source does not say"}]}
```

If everything is supported, return `{"unsupported":[]}`.

## Limit

Collate at most three times. For the items raised, add a source, add a decision to the record, or fix the item, and collate again. On the second and third rounds, hand over only the items raised in the previous round and the items and decisions fixed since. What still remains on the third round becomes a flag (FLAG) and is returned to the person.
