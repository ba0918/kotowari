Based on the kotowari specification (revised 2026-10-05; the version of kotowari itself is not pinned)

A guide is a user-facing document on how to use the product, written by a person or an LLM who has read the IR. It is not generated from the IR. Its readers are the people who use the product, so it says how to use it: commands, configuration, how to read the output, procedures and examples. It does not say why something was decided or what is planned or undecided; that belongs in an overview, which is read by the people who build the product (overview.md). Each section of a guide carries guide marks naming the IR items it explains, together with each item's fingerprint at the time the section was written. `kotowari check` compares those fingerprints with the IR as it is now, so the sections that may have fallen behind can be found at any time, not only right after the IR changed. kotowari never writes to a guide; the fingerprints are copied in by hand.

## Where guides are

Guides are the files matching the globs in `guides.files` of the configuration (config.md). Without the key, or with an empty list, no guide is read and nothing here applies. The globs are read, walked and excluded the same way as `tests.files`. A file matched by both `guides.files` and `tests.files` stops check and status with a config error whose detail is that file followed by `: matched by both guides.files and tests.files`: change the globs so that they do not overlap. A guide glob may match IR documents or decision records; those files are then read both ways.

Only `kotowari check` and `kotowari status` read guides. `kotowari list` and `kotowari query` do not.

## The form of a guide mark

Write a guide mark inside an HTML comment, so that it does not show in the rendered guide:

```markdown
<!-- @kotowari[REQ-core-165:3fa9c2e1, EX-core-259:77b0d1aa] -->
```

| Part | Form |
|---|---|
| Start | @kotowari[ |
| Content | Entries of the form `ID:fingerprint`, separated by commas. Spaces around an entry, around the ID and around the fingerprint are allowed |
| Fingerprint | 8 lowercase hexadecimal characters (`0` to `9` and `a` to `f`) |
| End | `]` on the same line as the start |

The guide is read as CommonMark. Only `@kotowari[` between `<!--` and `-->` is read as a guide mark. Inside a code block (fenced or indented) or a code span, and after the `-->` on the same line, it is not read, so examples of marks can be shown in code. Every guide mark in one comment and on one line is read.

A guide mark whose content is empty or only commas, that has no `]` on its line, or that has an entry without `:`, an entry whose ID is not of the ID form, or an entry whose fingerprint is not 8 lowercase hexadecimal characters is an invalid_marker error. None of its entries is then compared, not even the well-formed ones.

## The fingerprint

The fingerprint of an item or a scenario is the `fingerprint` of its entry in `kotowari list` and `kotowari query`. Take it from there; do not compute it by hand. It is taken from the body of the item without the `- source:` line, and from the step lines of a scenario, so renaming a heading, adding a source, renaming a scenario and changing its tags do not change it. Any other change to the body does, including a `- deferred:` line under a requirement. For a requirement in a topic document with a document-level `- deferred:` line (after the title), the first such line is put in front of the body, so declaring or removing it changes the fingerprints of that document's requirements, but not of its decision tables and scenarios.

## Scene: writing a guide

1. Write the section for the reader, from the IR.
2. List the items and scenarios the section explains. Confirm each with `kotowari query ID` (its body says whether the section really rests on it); search with `kotowari list` when an ID is not known yet.
3. Put one guide mark for the section, next to its heading, with one entry per ID and the fingerprint from `kotowari query ID | jq -r '.items[0].fingerprint'`.
4. Keep a guide mark to a few IDs. A section that needs many more is usually explaining several things; split it so that each guide mark stays small. This is a guideline for writing, and kotowari does not check it.
5. Run `kotowari check` and confirm that the guide has no findings and that `guides` in its JSON counts the guide and its entries.

Whether a section forgot to mark an item it explains cannot be checked by kotowari. Step 2 is the only place it is caught.

## Scene: reviewing a guide

`kotowari check` raises guide_stale, a notice, on the line where a guide mark starts, once per entry whose fingerprint no longer matches. Its detail is `ID written current`: the ID, the fingerprint written in the entry, and the current fingerprint (`-` when the ID is no longer in the IR). A notice does not change the exit code or `complete` of status, so stale guides stay listed until someone reviews them.

For each guide_stale:

1. Read the section the guide mark belongs to, and read `kotowari query ID` for the current body.
2. If the section no longer matches the IR, fix the section first. While there, check again with `kotowari query` whether the section explains items it does not mark, and add them.
3. Only after the section has been reviewed, copy the current fingerprint from the detail into the entry. Copying it without reviewing the section hides the stale section instead of fixing it.
4. When the current fingerprint is `-`, the item is gone from the IR: rewrite or remove what the section says about it, and change the entry to the ID that replaced it, or remove the entry.
5. Run `kotowari check` again and confirm that the guide_stale is gone.
