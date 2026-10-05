# Writing guides — guide marks and fingerprints

English | [日本語](writing-guides.ja.md)

A guide is a how-to document for users, written by a person or an LLM who has read the IR (this page is one of them).
kotowari does not generate guides.
Instead, each section of a guide carries a mark stating "which IR items it explains" and "the item's fingerprint at the time of writing", and `kotowari check` compares them with the current IR.
Sections that have gone stale because the IR changed are reported as `guide_stale` notices whenever you run it.

## Format

<!-- @kotowari[REQ-core-201:82ef697c, TBL-core-036:96778f03] -->

A guide mark is written inside an HTML comment, so it does not appear in the rendered guide.

```markdown
## 会員の割引

<!-- @kotowari[REQ-001:874e14a4, EX-001:54af105a] -->

会員の注文は1割引きになります。
```

| Part | Form |
|---|---|
| Start | `@kotowari[` |
| Contents | Each entry in the form `ID:fingerprint`, separated by commas |
| Fingerprint | 8 lowercase hexadecimal characters (`0`–`9` and `a`–`f`) |
| End | `]` on the same line as the start |

- Each entry has its surrounding whitespace removed and is split into ID and fingerprint at the first `:`. Whitespace is allowed just inside the brackets and around the `:`.
- If you write two entries with the same ID, each is checked on its own.
- These are different rules from test marks (`@kotowari[REQ-001]`). For test marks, see [Marking tests](marks.md).

## Where marks are read

<!-- @kotowari[REQ-core-200:97b3eb9b, EX-core-366:c019e492, EX-core-374:4cdeb739] -->

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
# 印の書き方

印は @kotowari[REQ-001] の形ではなく、`<!-- @kotowari[REQ-001:874e14a4] -->` の形で書きます。

```markdown
<!-- @kotowari[REQ-001] -->
```

<!-- メモ --> @kotowari[REQ-001:00000000]

<!-- @kotowari[REQ-001 : 874e14a4] -->
````

```console
$ kotowari check | jq -c .guides
{"files":1,"marks":1}
```

## Configuring where guides live

<!-- @kotowari[REQ-core-198:253e79fd, REQ-core-199:55922b15, EX-core-369:248c0ebf] -->

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
| It matches a file in the IR or decision record locations | Does not stop. That file is read as a guide as well |

- Only `kotowari check` and `kotowari status` read guides. `list` and `query` do not.
- How globs are read, how files are scanned, and stopping on unreadable files work the same as for `tests.files` ([Configuration file](config.md#how-globs-are-read)).

## Fingerprints

<!-- @kotowari[REQ-core-203:1fdc0443, TBL-core-026:382b0b95, EX-core-364:cc1080a1, EX-core-375:6d9bd3bb] -->

A fingerprint is an 8-character lowercase hexadecimal value taken from the body of an item or scenario.
It appears as `fingerprint` in the JSON of `kotowari list` and `kotowari query`.
Copy it from there rather than computing it by hand.

```console
$ kotowari query REQ-001 | jq -r '.items[0].fingerprint'
874e14a4
$ kotowari list | jq -r '.items[] | "\(.id) \(.fingerprint)"'
REQ-001 874e14a4
REQ-002 76709f33
EX-001 54af105a
```

These changes do and do not change the fingerprint:

| Change | Fingerprint |
|---|---|
| Changing the item's body (the `- kind:` and `- verification:` lines, statements, tables) | Changes |
| Changing a scenario's step lines (Given, When, Then, And) | Changes |
| Renaming the item's heading | Does not change |
| Changing the `- source:` line (the source) | Does not change |
| Adding, removing or changing a `- deferred:` line under a requirement heading | Changes |
| Adding, removing or changing a document-level `- deferred:` line (after the title) | Changes for the requirements of that document. Decision tables and scenarios do not change |
| Changing the name on a scenario's `Scenario:` line, or its tag lines (`@source` and so on) | Does not change |
| Changing line endings (`\r\n` versus `\n`) | Does not change |

Sources are added every time the IR is revised, so including them in the fingerprint would raise noisy notices each time.
[Deferral](deferred.md) declarations are included in the fingerprint: removing a deferral means shipping that feature, so the guide should be reviewed.
A document-level declaration sits outside the requirements' bodies, so its first line is prepended to the fingerprint input of each requirement in that document.
The definition of the calculation is in [REQ-core-203](../ir/core/guides.md#REQ-core-203).

## Output

### The `guides` group

<!-- @kotowari[REQ-core-206:e2f3dc5c, EX-core-371:f354c476] -->

A `guides` group appears at the top level of the JSON of `kotowari check` and in `kotowari status`.

| Key | Type | Description |
|---|---|---|
| `files` | number | The number of guides read |
| `marks` | number | The number of entries in well-formed guide marks (2 for `REQ-001:…, EX-001:…`) |

It does not appear in `check --format text`. In `status --format text` it appears as the `guides` line.

```console
$ kotowari check | jq -c .guides
{"files":1,"marks":3}
$ kotowari status --format text | grep -E '^(guides|findings|complete)'
guides files=1 marks=3
findings error=0 notice=0
complete true
```

If a mistyped glob leaves you with zero guides, `files=0` lets you notice.

### `guide_stale` (notice)

<!-- @kotowari[REQ-core-204:fa82249e, EX-core-363:9bff9f93, EX-core-365:542b2c69] -->

For each entry of a well-formed mark, one notice is reported when:

- no item or scenario with that ID exists in the IR, or
- no item with that ID has a fingerprint equal to the one written.

| Field | Value |
|---|---|
| path | The path relative to the guide's base directory |
| line | The line where the mark starts |
| detail | `ID written-fingerprint current-fingerprint` (separated by single spaces). When the ID is not in the IR, the current fingerprint is `-` |

Changing REQ-001's statement from "10%" to "20%" produces:

```console
$ kotowari check --format text
guides/discount.md:5 [notice] guide_stale REQ-001 874e14a4 19e1ff5a
$ echo $?
0
```

Removing REQ-002 from the IR makes the current fingerprint `-`.

```console
$ kotowari check --format text
guides/discount.md:11 [notice] guide_stale REQ-002 76709f33 -
```

`guide_stale` is a notice, so it changes neither the exit code nor `complete` in `status`.
It stays in the counts of `check` and `status` until you fix it.

When two or more items with the same ID exist in the IR, the entry is not stale if it matches either fingerprint.

### `invalid_marker` (error)

<!-- @kotowari[REQ-core-202:907dd989, EX-core-367:68e78a4a, EX-core-373:acd8d951] -->

One finding per mark is reported (exit code 1) when:

- the contents are empty or only separators
- there is no `]` on the same line as the start
- an entry has no `:` (the fingerprint was forgotten)
- the part before `:` is not in ID form (including empty)
- the part after `:` is not 8 lowercase hexadecimal characters (including uppercase)

The detail is the whole line containing the mark.
None of the entries in a malformed mark are checked, including the well-formed ones.

```markdown
## 会員の割引

<!-- @kotowari[REQ-001] -->

## 会員でない注文

<!-- @kotowari[REQ-002:76709F33] -->

## 例

<!-- @kotowari[REQ-001:874e14a4, EX-001] -->
```

```console
$ kotowari check --format text
guides/discount.md:5 [error] invalid_marker <!-- @kotowari[REQ-001] -->
guides/discount.md:9 [error] invalid_marker <!-- @kotowari[REQ-002:76709F33] -->
guides/discount.md:13 [error] invalid_marker <!-- @kotowari[REQ-001:874e14a4, EX-001] -->
$ kotowari check | jq -c .guides
{"files":1,"marks":0}
```

## Steps for writing a guide

<!-- @kotowari[REQ-core-205:7110f061] -->

1. Read the IR and write a section for your readers.
2. List the items and scenarios the section explains. Read their bodies with `kotowari query ID` and confirm that the section really is based on those items. If you do not know an ID, look it up with `kotowari list`.
3. Place one mark just below the section heading, with one entry per ID, copying the fingerprint from `kotowari query ID | jq -r '.items[0].fingerprint'`.
4. Keep a mark to a few IDs. A section that needs more usually explains several things, so split it (kotowari does not check this).
5. Run `kotowari check` and confirm that there are no findings on the guide and that `guides` counts the guides and entries.

If you forget to list in the mark an item the section explains, kotowari cannot tell. Step 2 is the only safeguard.

Sections not tied to IR items, such as `## Related`, do not need a mark.

## Steps for reviewing a guide

<!-- @kotowari[REQ-core-205:7110f061, REQ-core-204:fa82249e] -->

When `guide_stale` appears, do the following for each finding.

1. Read the section the mark belongs to, and read the current body with `kotowari query ID`.
2. If the section no longer agrees with the IR, fix the section first. While you are at it, use `kotowari query` to check whether the section explains any item missing from the mark, and add it if so.
3. Only after reviewing the section, copy the current fingerprint from the detail into the mark's entry.
4. If the current fingerprint is `-`, the item has been removed from the IR. Rewrite or remove what the section says about that item, and either change the entry to the replacing ID or remove the entry.
5. Run `kotowari check` again and confirm that `guide_stale` is gone.

Copying only the fingerprint without reading the section clears the notice while leaving the stale section unfixed ([pitfalls](#the-notice-is-gone-but-the-section-is-still-stale)).

## Common pitfalls

### `marks` is 0 even though you wrote marks

<!-- @kotowari[REQ-core-200:97b3eb9b, EX-core-366:c019e492] -->

The mark is outside an HTML comment, or inside a code block or code span.
Enclose it in `<!--` and `-->`, and keep it outside code.
A mark written after `-->` on the same line is not read either.

### `files` in `guides` is 0

<!-- @kotowari[REQ-core-198:253e79fd, EX-core-369:248c0ebf] -->

The configuration has no `guides.files`, or its globs match no files.
Without `guides.files`, nothing is reported even if guides contain stale marks.

```console
$ kotowari status --format text | grep '^guides'
guides files=0 marks=0
```

### You fixed one entry, but the other entries of the same mark are still not checked

<!-- @kotowari[REQ-core-202:907dd989, EX-core-373:acd8d951] -->

If even one entry in a mark is malformed, the whole mark becomes `invalid_marker`, and its well-formed entries are not checked either.
Fix `invalid_marker` first, then look at `guide_stale`.

### Stopping because guide and test locations overlap

<!-- @kotowari[REQ-core-199:55922b15, EX-core-368:d4ff36cf] -->

`tests.files` is too broad (such as `**/*`) and also matches guides.
Files in languages without a query have every `@kotowari[` picked up, so kotowari stops to prevent guide marks from being read as test marks.
For details, see [Configuration file](config.md#stopping-because-the-guide-and-test-locations-overlap).

### The notice is gone but the section is still stale

You did not review the section before copying the fingerprint.
Copying the fingerprint is a declaration that "this section agrees with the current IR". Do steps 1 and 2 of [the review steps](#steps-for-reviewing-a-guide) first.

## Related

- Specification: [IR for guide marks](../ir/core/guides.md)
- Configuring locations: [Configuration file](config.md)
- Marks written in tests: [Marking tests](marks.md)
- Getting fingerprints: [kotowari list](commands/list.md), [kotowari query](commands/query.md)
- Checking whether everything is in place: [kotowari status](commands/status.md)
- Kinds of findings: [Findings](findings.md)
