# Writing guides — guide marks and fingerprints

English | [日本語](writing-guides.ja.md)

A guide is a how-to document for users, written by a person or an LLM who has read the decision documents (this page is one of them).
kotowari does not generate guides.
Instead, each section of a guide carries a mark stating "which decisions it explains" and "each decision's fingerprint at the time of writing", and `kotowari check` compares them with the current decision documents.
Sections that have gone stale because a decision changed are reported as `guide_stale` notices whenever you run it.

## Format

<!-- @kotowari[docs/decisions/core/guides.md#one-entry-of-a-guide-marker-joins-the-target-of-a-decision-and-a-fingerprint-with-a-colon:7ba923d7] -->

A guide mark is written inside an HTML comment, so it does not appear in the rendered guide.

```markdown
## Member discount

<!-- @kotowari[docs/decisions/shop/discount.md#members-get-10-off-their-orders:e3adc927] -->

A member's order is 10% off.
```

| Part | Form |
|---|---|
| Start | `@kotowari[` |
| Contents | Each entry in the form `target:fingerprint`, separated by commas |
| Target | The path of the decision document and the anchor of the decision's heading, joined with `#` (`<path>#<anchor>`). The path is the first language's side, relative to the base directory |
| Fingerprint | 8 lowercase hexadecimal characters (`0`–`9` and `a`–`f`) |
| End | `]` on the same line as the start |

- The anchor is made by the same rule as a link to a heading inside a decision document ([Writing decision documents](decision-documents.md#changing-a-decision)). You do not need to build it: `kotowari list` prints it as the `id` of each decision.
- Whitespace is allowed just inside the brackets and around the `:`.
- If you write two entries with the same target, each is checked on its own.
- These are different rules from test marks (`@kotowari[C-shop-discount-1]`). For test marks, see [Marking tests](marks.md).

## Where marks are read

<!-- @kotowari[docs/decisions/core/guides.md#guide-markers-are-read-only-from-inside-html-comments:6ad1c700] -->

Guides are read as CommonMark.
Only an `@kotowari[` between the `<!--` and the `-->` of an HTML comment is read as a guide mark.

| Location | Read? |
|---|---|
| Inside an HTML comment | Yes. Every mark in one comment, and on one line, is picked up |
| Body text outside comments | No |
| Inside a code block (fenced or indented) | No |
| Inside a code span | No |
| After `-->` on the same line | No |

So if you write an example of a mark in a code block or code span, it is not read as a mark.
All the examples on this page are inside code blocks too.

In the following guide, only the last entry is read as a mark.

````markdown
# How to write marks

Write a mark as `<!-- @kotowari[docs/decisions/shop/discount.md#members-get-10-off-their-orders:e3adc927] -->`, not as @kotowari[docs/decisions/shop/discount.md#members-get-10-off-their-orders:e3adc927].

```markdown
<!-- @kotowari[docs/decisions/shop/discount.md#members-get-10-off-their-orders:00000000] -->
```

<!-- note --> @kotowari[docs/decisions/shop/discount.md#members-get-10-off-their-orders:00000000]

<!-- @kotowari[ docs/decisions/shop/discount.md#members-get-10-off-their-orders : e3adc927 ] -->
````

## Configuring where guides live

<!-- @kotowari[docs/decisions/core/guides.md#guides-are-chosen-by-the-globs-of-guidesfiles-and-only-check-and-status-read-them:05ee20e8, docs/decisions/core/guides.md#when-the-guide-and-test-locations-overlap-it-is-a-stop-with-a-configuration-error:4b7987f7] -->

List the locations of guides as globs under `guides.files` in the configuration file.

```yaml
guides:
  files:
    - "guides/**/*.md"
```

| Situation | Behavior |
|---|---|
| `guides.files` is missing or an empty list | No guides are read, and no guide checks run |
| A glob matches nothing | Not an error (`files` in `guides` becomes 0) |
| It matches the same file as `tests.files` | Stops with a configuration error (exit code 2) |
| It matches a file in the location of decision documents | Does not stop. That file is read as a guide as well |

- Only `kotowari check` and `kotowari status` read guides. `list` and `query` do not.
- How globs are read, how files are scanned, and stopping on unreadable files work the same as for `tests.files` ([Configuration file](config.md#how-globs-are-read)).

## Fingerprints

<!-- @kotowari[docs/decisions/core/guides.md#a-stale-section-is-found-by-comparing-the-fingerprint-written-in-the-marker-with-the-current-fingerprint-of-the-decision:c21ccd0f, docs/decisions/core/languages-and-guides.md#a-guide-is-stale-when-the-decision-or-confirmations-change:de5a25a2, docs/decisions/core/list.md#a-confirmation-row-gets-the-tests-that-point-at-it-and-a-decision-gets-a-fingerprint:64ab7c66] -->

A fingerprint is an 8-character lowercase hexadecimal value taken from the Decision and Confirmations sections of a decision.
`kotowari list` prints one entry of kind `decision` per decision, whose `id` is the target and whose `fingerprint` is the value to copy; `kotowari query '<target>'` prints the same for one decision.
Copy them from there rather than computing them by hand.

```console
$ kotowari list | jq -r '.items[] | select(.kind == "decision") | "\(.id) \(.fingerprint)"'
docs/decisions/shop/discount.md#members-get-10-off-their-orders e3adc927
docs/decisions/shop/discount.md#orders-of-non-members-are-not-discounted 55ce4da7
$ kotowari query 'docs/decisions/shop/discount.md#members-get-10-off-their-orders' | jq -r '.items[0].fingerprint'
e3adc927
```

These changes do and do not change the fingerprint:

| Change | Fingerprint |
|---|---|
| Changing the Decision section (sentences, tables) | Changes |
| Changing the Confirmations section (adding, removing or rewording a row) | Changes |
| Changing the state line (proposed to implemented) | Does not change |
| Changing the Problem or the Rejected options | Does not change |
| Renaming the decision's heading | The target changes, so marks pointing at the old target become `guide_stale` with `-` |
| Changing line endings (`\r\n` versus `\n`) | Does not change |

The fingerprint follows only what the guide explains: the rule and the results that show it is kept.
Turning a decision to implemented, or adding to its reasons, does not ask for the guide to be reviewed.

## Output

### The `guides` group

<!-- @kotowari[docs/decisions/core/output.md#the-top-level-of-checks-json-shows-the-findings-the-count-per-kind-and-the-counts-of-what-was-read:bcec81c7, docs/decisions/core/guides.md#one-entry-of-a-guide-marker-joins-the-target-of-a-decision-and-a-fingerprint-with-a-colon:7ba923d7] -->

A `guides` group appears at the top level of the JSON of `kotowari check` and in `kotowari status`.

| Key | Type | Description |
|---|---|---|
| `files` | number | The number of guides read |
| `marks` | number | The number of entries in well-formed guide marks (2 for a mark with two targets) |

It does not appear in `check --format text`. In `status --format text` it appears as the `guides` line.

For a guide with three sections, the last of which marks both decisions:

```console
$ kotowari check | jq -c .guides
{"files":1,"marks":4}
$ kotowari status --format text | grep -E '^(guides|findings|complete)'
guides files=1 marks=4
findings error=0 notice=0
complete true
```

If a mistyped glob leaves you with zero guides, `files=0` lets you notice.

### `guide_stale` (notice)

<!-- @kotowari[docs/decisions/core/guides.md#a-stale-marker-is-a-guide_stale-notice-and-does-not-change-the-exit-code-or-complete:7b8dc3ce] -->

For each entry of a well-formed mark, one notice is reported when:

- the target names no decision (the document or heading does not exist, or the heading is the title or a section), or
- the current fingerprint of the decision differs from the one written.

| Field | Value |
|---|---|
| path | The path relative to the guide's base directory |
| line | The line where the mark starts |
| detail | `target written-fingerprint current-fingerprint` (separated by single spaces). When the target names no decision, the current fingerprint is `-` |

Changing the member decision from "10% off" to "20% off" produces:

```console
$ kotowari check --format text
guides/discount.md:5 [notice] guide_stale docs/decisions/shop/discount.md#members-get-10-off-their-orders e3adc927 6db223da
guides/discount.md:17 [notice] guide_stale docs/decisions/shop/discount.md#members-get-10-off-their-orders e3adc927 6db223da
$ echo $?
0
```

Renaming the heading of the guest decision makes the current fingerprint `-`.

```console
$ kotowari check --format text
guides/discount.md:11 [notice] guide_stale docs/decisions/shop/discount.md#orders-of-non-members-are-not-discounted 55ce4da7 -
```

`guide_stale` is a notice, so it changes neither the exit code nor `complete` in `status`.
It stays in the counts of `check` and `status` until you fix it.

### `invalid_marker` (error)

<!-- @kotowari[docs/decisions/core/guides.md#one-entry-of-a-guide-marker-joins-the-target-of-a-decision-and-a-fingerprint-with-a-colon:7ba923d7] -->

One finding per mark is reported (exit code 1) when:

- the contents are empty or only separators
- there is no `]` on the same line as the start
- an entry has no `:` (the fingerprint was forgotten)
- the part before `:` is not in target form (no path, no `#anchor`, or a confirmation ID)
- the part after `:` is not 8 lowercase hexadecimal characters (including uppercase)

The detail is the whole line containing the mark.
None of the entries in a malformed mark are checked, including the well-formed ones.

```markdown
## Member discount

<!-- @kotowari[docs/decisions/shop/discount.md#members-get-10-off-their-orders] -->

## Orders from guests

<!-- @kotowari[docs/decisions/shop/discount.md#orders-of-non-members-are-not-discounted:55CE4DA7] -->

## Example

<!-- @kotowari[C-shop-discount-1:e3adc927, docs/decisions/shop/discount.md#orders-of-non-members-are-not-discounted:55ce4da7] -->
```

```console
$ kotowari check --format text
guides/bad.md:5 [error] invalid_marker <!-- @kotowari[docs/decisions/shop/discount.md#members-get-10-off-their-orders] -->
guides/bad.md:9 [error] invalid_marker <!-- @kotowari[docs/decisions/shop/discount.md#orders-of-non-members-are-not-discounted:55CE4DA7] -->
guides/bad.md:13 [error] invalid_marker <!-- @kotowari[C-shop-discount-1:e3adc927, docs/decisions/shop/discount.md#orders-of-non-members-are-not-discounted:55ce4da7] -->
```

## Steps for writing a guide

<!-- @kotowari[docs/decisions/core/guides.md#the-occasions-for-writing-and-reviewing-guides-are-placed-in-the-kotowari-skill:cfbcc10a] -->

1. Read the decision documents and write a section for your readers.
2. List the decisions the section explains. Read each with `kotowari query '<target>'` and confirm that the section really is based on it. If you do not know a target, look it up with `kotowari list`.
3. Place one mark just below the section heading, with one entry per decision, copying the fingerprint from `kotowari query '<target>' | jq -r '.items[0].fingerprint'`.
4. Keep a mark to a few decisions. A section that needs more usually explains several things, so split it (kotowari does not check this).
5. Run `kotowari check` and confirm that there are no findings on the guide and that `guides` counts the guides and entries.

If you forget to list in the mark a decision the section explains, kotowari cannot tell. Step 2 is the only safeguard.

Sections not tied to decisions, such as `## Related`, do not need a mark.

## Steps for reviewing a guide

<!-- @kotowari[docs/decisions/core/guides.md#the-occasions-for-writing-and-reviewing-guides-are-placed-in-the-kotowari-skill:cfbcc10a, docs/decisions/core/guides.md#a-stale-marker-is-a-guide_stale-notice-and-does-not-change-the-exit-code-or-complete:7b8dc3ce] -->

When `guide_stale` appears, do the following for each finding.

1. Read the section the mark belongs to, and read the current decision with `kotowari query '<target>'`.
2. If the section no longer agrees with the decision, fix the section first. While you are at it, use `kotowari list` and `kotowari query` to check whether the section explains any decision missing from the mark, and add it if so.
3. Only after reviewing the section, copy the current fingerprint from the detail into the mark's entry.
4. If the current fingerprint is `-`, the target names no decision now. Find the decision that holds the content (a renamed heading, or a decision in another topic), rewrite or remove what the section says, and either point the entry at that decision or remove the entry.
5. Run `kotowari check` again and confirm that `guide_stale` is gone.

Copying only the fingerprint without reading the section clears the notice while leaving the stale section unfixed ([pitfalls](#the-notice-is-gone-but-the-section-is-still-stale)).

## Common pitfalls

### `marks` is 0 even though you wrote marks

<!-- @kotowari[docs/decisions/core/guides.md#guide-markers-are-read-only-from-inside-html-comments:6ad1c700] -->

The mark is outside an HTML comment, or inside a code block or code span.
Enclose it in `<!--` and `-->`, and keep it outside code.
A mark written after `-->` on the same line is not read either.

### `files` in `guides` is 0

<!-- @kotowari[docs/decisions/core/guides.md#guides-are-chosen-by-the-globs-of-guidesfiles-and-only-check-and-status-read-them:05ee20e8] -->

The configuration has no `guides.files`, or its globs match no files.
Without `guides.files`, nothing is reported even if guides contain stale marks.

```console
$ kotowari status --format text | grep '^guides'
guides files=0 marks=0
```

### You fixed one entry, but the other entries of the same mark are still not checked

<!-- @kotowari[docs/decisions/core/guides.md#one-entry-of-a-guide-marker-joins-the-target-of-a-decision-and-a-fingerprint-with-a-colon:7ba923d7] -->

If even one entry in a mark is malformed, the whole mark becomes `invalid_marker`, and its well-formed entries are not checked either.
Fix `invalid_marker` first, then look at `guide_stale`.

### Stopping because guide and test locations overlap

<!-- @kotowari[docs/decisions/core/guides.md#when-the-guide-and-test-locations-overlap-it-is-a-stop-with-a-configuration-error:4b7987f7] -->

`tests.files` is too broad (such as `**/*`) and also matches guides.
Files in languages without a query have every `@kotowari[` picked up, so kotowari stops to prevent guide marks from being read as test marks.
For details, see [Configuration file](config.md#stopping-because-the-guide-and-test-locations-overlap).

### The notice is gone but the section is still stale

You did not review the section before copying the fingerprint.
Copying the fingerprint is a declaration that "this section agrees with the current decision". Do steps 1 and 2 of [the review steps](#steps-for-reviewing-a-guide) first.

## Related

- Specification: [guides](../decisions/core/guides.md), [when a guide is stale](../decisions/core/languages-and-guides.md)
- Configuring locations: [Configuration file](config.md)
- Marks written in tests: [Marking tests](marks.md)
- Getting fingerprints: [kotowari list](commands/list.md), [kotowari query](commands/query.md)
- Checking whether everything is in place: [kotowari status](commands/status.md)
- Kinds of findings: [Findings](findings.md)
