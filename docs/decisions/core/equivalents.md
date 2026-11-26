# List of equivalents

English | [日本語](equivalents.ja.md)

Covers the location and form of the list of equivalents, how it is matched against mutation outcomes, and the findings on an entry of the list. Equivalence is a person's or an LLM's judgment that putting in a mutation does not change the behavior users can observe (the output and the exit code). The findings and tallies for misses and timeouts are decided in [Checking mutation outcomes](mutants.md), the reading of the results file in [Reading the results of mutation tests](mutants-input.md), the normalization of paths in [Paths in configuration values are normalized before comparing and before output](base-directory.md#paths-in-configuration-values-are-normalized-before-comparing-and-before-output), and the check of configuration key values in [If the configuration file cannot be read as YAML, or its keys and values do not follow the rules, kotowari stops with a configuration error](config.md#if-the-configuration-file-cannot-be-read-as-yaml-or-its-keys-and-values-do-not-follow-the-rules-kotowari-stops-with-a-configuration-error).

## Judgments of equivalence are placed in the YAML list that mutants.equivalents in the configuration points at

Status: implemented

### Problem

Among the misses of mutation testing are some where putting in the mutation does not change the behavior (equivalents). Where to keep that judgment and its reason had to be decided.

If they are placed in cargo-mutants' exclusion settings (.cargo/mutants.toml), the excluded mutations are not measured, and the reasons are comments that nobody checks. If the judgment is placed in a tool's settings, it is lost when the tool is replaced. The list is a mechanical list of tens to a hundred entries, different in nature from decision documents that people read.

That the everyday check stops because the list is missing or broken is a mixing of responsibilities. What is broken is the list file, so that is the path the person fixing it should look at.

### Decision

Misses judged equivalent are held in one file on kotowari's side, as a list of mutations with their classification and reason. Mutation testing is run with no exclusions, and kotowari removes the misses on the list from the findings.

The location of the list is pointed at by the configuration key "mutants.equivalents". The value is the path of the list file (one string, relative to the base directory), and there is no default. The list is a YAML sequence.

| State of the list file | Behavior of "kotowari mutants" |
|---|---|
| The configuration has no "mutants.equivalents" key | Continues with a list of zero entries |
| The target is empty (zero bytes, or only comments) | Continues with a list of zero entries, matching the handling of an empty configuration file |
| The target does not exist or cannot be read | Stops with unreadable file as the reason |
| The target is not UTF-8 | Stops with a file that is not UTF-8 as the reason |
| It cannot be read as YAML, or the top level is not a sequence | Stops with a configuration error as the reason, and the details show the relative path of the list file and a description of the error |

"kotowari check" only looks at the type of the key's value; it does not read the target file and does not look at whether it exists.

### Rejected options

- Place them in the exclusion settings of .cargo/mutants.toml, with comments for the reasons. The excluded mutations are not measured, nobody checks the reasons, and they are lost when the tool is replaced.
- Mix them into decision documents that people read (Markdown). It is a mechanical list of tens to a hundred entries, different in nature from documents that people read.

### Confirmations

- C-core-equiv-1 When the configuration has no "mutants.equivalents" key, or the target is zero bytes or only comments (including blank lines between comments), "kotowari mutants" continues with a list of zero entries
- C-core-equiv-2 If the target of "mutants.equivalents" does not exist, kotowari stops with exit code 2, and the first line of standard error starts with "unreadable file: " and the path of the list file
- C-core-equiv-3 If the top level of the list is not a sequence, kotowari stops with exit code 2, and the first line of standard error starts with "config error: " and the relative path of the list file
- C-core-equiv-4 Even if the target of "mutants.equivalents" does not exist, "kotowari check" does not stop

## An entry of the list identifies a mutation by the file, the description of the change, and the text of the line

Status: implemented

### Problem

Line numbers shift every time the code is fixed. Names without line numbers collide (382 of 804 mutations collided with others). If the text of the line is taken, the match is not lost when the line only moves, and is lost only when that line is rewritten, which falls on the side of judging again.

If the source changes after the results are taken, the file a result points at may not exist, or the line may exceed the file's line count. In that case it is safe to fall on the side of not matching. The miss is reported as an error and a notice is reported on the list side, so nothing is silently skipped.

### Decision

An entry of the list has "file" (the path of the source), "change" (the description of the change), "text" (the original text of the line the mutation goes into), "class" (the classification) and "why" (the reason). The key for the text of the line is not named line because it would be confused with the line number.

A mutation outcome whose result is "missed" and a well-formed entry match when all three of the following hold.

- "file", after normalization, is the same string as the file of the mutation outcome
- "change" is the same string as the description of the change
- "text" is the same string as the text of that line in the current content of the file of the mutation outcome, both with leading and trailing half-width spaces and tabs removed

The only whitespace removed is the two, half-width spaces and tabs; full-width spaces stay as part of the text. This is because it is safe as a way of detecting rewrites. Lines are separated the same way as when kotowari counts the lines of other files ([kotowari counts "\n", "\r\n" and a lone "\r" each as one line ending](ir-document.md#kotowari-counts-n-rn-and-a-lone-r-each-as-one-line-ending)), and the "\r" of a "\r\n" at the end of a line is not included in the text. "file" is normalized so that the match is not lost only through a difference in how it is written, and the normalization is applied only when comparing.

When the same file has several lines with the same text and the same change goes into them, one entry matches the miss on any of those lines.

A matched miss is not made a finding, and is counted not in survived but in equivalent in the counts. Only misses can be removed by the list.

When the file of a mutation outcome does not exist, cannot be read, or is not UTF-8, or the line exceeds that file's line count, kotowari does not stop. That mutation outcome matches no entry, and if it is a miss, mutant_survived is reported at the line of the outcome as it is.

### Rejected options

- Identify by line number. It shifts every time the code is fixed.
- Identify by the file, the function and the description of the change only. Names collide.

### Confirmations

- C-core-equiv-5 A miss that matches an entry of the list gets no mutant_survived, and is counted not in survived but in equivalent in the counts
- C-core-equiv-6 If the line of the text only moves, it still matches, and if the text of that line is rewritten, it no longer matches and mutant_survived is reported
- C-core-equiv-7 An entry with a different "file" does not match, even if the same line has the same text
- C-core-equiv-8 An entry matches even if "file" is written as "./src/a.rs", the source is indented with tabs, or there is a "\r" at the end of the line
- C-core-equiv-9 If there are two lines with the same text and both lines have a miss of the same change, one entry matches both, and equivalent is 2
- C-core-equiv-10 When the line of a miss exceeds the file's line count, or the file is not UTF-8, kotowari does not stop, and since it does not match, mutant_survived is reported

## An entry whose text is gone from the source gets an equivalent_stale notice

Status: implemented

### Problem

That an entry of the list has become stale is to be made known. When mutation testing is run on the diff only, only some of the mutations are in the results. If staleness is judged by whether an entry appears in the results, most of the list is wrongly said to be stale. If it is judged against the source, it works the same whether the results are complete or partial, and it does not depend on the tool.

A person or an LLM who sees the finding searches the list file for the detail string and finds the entry in question.

### Decision

When the file of "file" has no line whose text, with leading and trailing half-width spaces and tabs removed, is the same as the "text" of a well-formed entry, kotowari reports an equivalent_stale notice on that entry. When the file does not exist, cannot be read, or is not UTF-8, it also reports it, as if the text were absent. Whether the mutation appears in the results file is not looked at. It is a notice, so it does not change the exit code.

An equivalent_stale finding has the list file as path, no line (null), and as detail the string joining "file" and "change", as written in the list, with ": ". No normalization is applied.

### Rejected options

- Previous decision: Report a notice for an entry that is on the list but does not appear in the results. In an actual run on the diff only, the results are partial and entries are wrongly said to be stale, so it was decided to judge against the source.

### Confirmations

- C-core-equiv-11 If the line of an entry's "text" is nowhere in the file of "file", an equivalent_stale notice is reported whose path is the list file, whose line is null, and whose detail joins "file" and "change", as written, with ": " (for an entry written as "./src/a.rs", the detail also starts with "./src/a.rs")
- C-core-equiv-12 If the file of "file" is not UTF-8, an equivalent_stale notice is reported on that entry
- C-core-equiv-13 If the text of an entry is in the source, equivalent_stale is not reported, even if that entry matches no miss

## A malformed entry is an equivalent_invalid error and removes no miss

Status: implemented

### Problem

A malformed entry is not to be given the power to silence findings. If the handling were divided by how the form is broken, an opening for silently skipping would be made.

When misses are examined, they are divided into three: "equivalent" (the behavior does not change under the mutation), "untested" (no test looks at it), and "suspected defect" (the original code is wrong). If states that should be fixed could be silenced by the list, the list would become a place for putting things off. Whether to release with misses still present is decided by a person looking at their number and content.

### Decision

When an entry of the list matches any of the following, kotowari reports an equivalent_invalid error on that entry, and that entry is not matched with any mutation outcome.

- It is not a mapping of keys and values
- Any of the keys "file", "change", "text", "class" and "why" is missing
- It has a key other than these five
- A value is not a string
- "why" is empty after removing leading and trailing half-width spaces and tabs
- "class" is not "equivalent". The only classification that can be written in the list is equivalence; "untested" and "suspected defect" cannot remove findings through the list
- "file" is an absolute path or contains a ".." element. Nothing outside the base directory is read

An equivalent_invalid finding has the list file as path, no line (null), and as detail the string joining "file" and "change", as written in the list, with ": ". If "file" or "change" is missing or not a string, that side is an empty string. There are situations in which a malformed entry cannot be normalized.

A malformed entry gets only equivalent_invalid, not equivalent_stale. What should be fixed is the form, and reporting two would be noise.

That two or more entries have the same content is not itself checked. Form errors are reported per entry.

### Confirmations

- C-core-equiv-14 An entry whose "why" is only whitespace, whose "class" is not "equivalent", that has a key other than the five, or whose "file" points outside the base directory gets an equivalent_invalid error whose path is the list file and whose line is null
- C-core-equiv-15 A malformed entry does not remove the misses that fit it (mutant_survived is reported), and does not get equivalent_stale either
- C-core-equiv-16 The detail of equivalent_invalid joins "file" and "change", as written, with ": ", and for an entry without the "file" key the part before ": " is empty
- C-core-equiv-17 If there are two malformed entries with the same content, two equivalent_invalid errors are reported as well
