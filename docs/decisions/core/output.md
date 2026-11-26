# Output form

English | [日本語](output.ja.md)

Covers the output form chosen by "--format", the content of check's JSON, and the form of one line of text output. The kinds and detail of findings are decided in [Kinds of findings](findings.md), and the order and line of findings in [Order and lines of findings](finding-order.md). The top level of mutants' JSON is decided in [Counts are always output, and the number excluded as equivalent is shown every time](mutants.md#counts-are-always-output-and-the-number-excluded-as-equivalent-is-shown-every-time), and the top level of plan's JSON in [The output of kotowari plan uses the same finding keys as check, and the JSON has only findings and counts](plan.md#the-output-of-kotowari-plan-uses-the-same-finding-keys-as-check-and-the-json-has-only-findings-and-counts).

## "--format" accepts json and text, with json as the default

Status: implemented

### Problem

kotowari is used first of all by an LLM ([kotowari is a CLI used first of all by an LLM](cli-scope.md#kotowari-is-a-cli-used-first-of-all-by-an-llm)), so a structured form is the basis of the output. Text output for people to read is hard to read unless its content is worked out carefully.

### Decision

The output form is chosen with "--format". There are two values, "json" and "text", and without "--format" it is "json".

With "json", one JSON value is output to standard output. The form with "text" is decided in [--format text outputs one finding per line](#--format-text-outputs-one-finding-per-line).

### Confirmations

- C-core-output-1 For check without "--format", the whole of standard output is one JSON value
- C-core-output-2 For check with "--format json", the whole of standard output is one JSON value, and check with "--format text" outputs no JSON

## The top level of check's JSON shows the findings, the count per kind, and the counts of what was read

Status: implemented

### Problem

What to output at the top level of check's JSON had to be decided. If the glob that looks for test files or guides is written wrongly, what is read becomes zero. Without the counts, it cannot be told whether there are zero findings or nothing was read, and the mistake passes silently. The count of overview source data is handled the same way as guides. For a project that writes no surface rules, nothing about the surface check is to happen.

### Decision

The top level of the JSON of "kotowari check" has the following keys.

| Key | Content |
|---|---|
| findings | The list of findings. The keys of one finding are as in [A finding is output with the five keys kind, severity, path, line and detail](#a-finding-is-output-with-the-five-keys-kind-severity-path-line-and-detail) |
| counts | The number of findings per kind. A kind with no finding in findings is not included |
| tests | The number of test files read, per extension ([The "tests" of check's JSON counts the test files read per extension](#the-tests-of-checks-json-counts-the-test-files-read-per-extension)) |
| guides | Has two keys, "files" (the number of guides read) and "marks" (the number of entries in guide markers). If there are zero guides, both are 0 |
| overview | Has two keys, "files" (the number of overview source data read) and "marks" (the number of entries in guide markers in the overview source data) |
| surface | Has one key, "unspecified" (the number of surfaces removed by the list of unspecified surfaces). When "surface.rules" is an empty list, the key itself is not output |
| decisions | Has in "documents" the number of decision documents read |

### Confirmations

- C-core-output-3 The top level of the JSON of a check that read the location of decision documents has the keys findings, counts, tests, guides, overview and decisions, and also the surface key when "surface.rules" has rules
- C-core-output-4 For any set of findings, the value of each kind in counts equals the number of findings of that kind in findings, and a kind with no finding in findings is not in counts
- C-core-output-5 guides has the number of guides read in "files" and the number of entries in markers in "marks", and if there are zero guides, both are 0

## A finding is output with the five keys kind, severity, path, line and detail

Status: implemented

### Problem

The keys that one finding in the JSON has had to be decided. The reason is not recorded.

### Decision

A finding in the JSON has the following five keys.

| Key | Content |
|---|---|
| kind | The kind of the finding ([Kinds of findings](findings.md)) |
| severity | "error" (error) or "notice" (notice). "warning" is not used |
| path | A path relative to the base directory ([Paths in configuration values and the "path" of the output are relative to the base directory](base-directory.md#paths-in-configuration-values-and-the-path-of-the-output-are-relative-to-the-base-directory)). For a finding on a decision document, the normalized location of decision documents and the document's path relative to the location, joined with "/". The path of a finding on a file that is not a decision document is decided in the topic that decided that finding (for example test files, guides, surface files, the list of unspecified surfaces) |
| line | The line (from 1). null for a finding on a whole document ([line is counted from 1, and a finding that covers the whole document is null](finding-order.md#line-is-counted-from-1-and-a-finding-that-covers-the-whole-document-is-null)) |
| detail | A string decided per kind ([The detail of a finding is a short string that identifies that finding, decided per kind in a table](findings.md#the-detail-of-a-finding-is-a-short-string-that-identifies-that-finding-decided-per-kind-in-a-table)) |

### Confirmations

- C-core-output-6 A finding in the JSON has the five keys kind, severity, path, line and detail, severity is "error" or "notice", and line is a number or null
- C-core-output-7 The path of a finding on a decision document in a directory under the location is the normalized location and the document's path relative to the location, joined with "/" (even if the location is written as "./docs//decisions/./", it is "docs/decisions/sub/a.md")

## --format text outputs one finding per line

Status: implemented

### Problem

The form of one line of text output had to be decided. If a file name contains a newline, one finding splits into two visible lines, and the premise of tools that read by line breaks. Mixing the count numbers into the same output would also break the form of one finding per line.

### Decision

When "--format" is "text", each finding is output on one line, in the form "path:line [error] kind detail" or "path:line [notice] kind detail". The square brackets are output too.

When the line of a finding is null, the line is written as "-".

A newline ("\n" and "\r") in the path and the detail is written as two characters, a backslash and "n" or "r". The JSON output is not rewritten.

check's "--format text" does not output the number of test files in addition to the finding lines. That a line with the number of unspecified surfaces is output last is decided in [The number of surfaces excluded by the list of unspecified surfaces is always output at the end of the output of check](surface.md#the-number-of-surfaces-excluded-by-the-list-of-unspecified-surfaces-is-always-output-at-the-end-of-the-output-of-check).

### Confirmations

- C-core-output-8 check with "--format text" outputs each finding as one line in the form "path:line [error] kind detail" or "path:line [notice] kind detail"
- C-core-output-9 With "--format text", a finding whose line is null is one line with "-" written in the place of the line
- C-core-output-10 With "--format text", a finding whose path or detail contains "\n" or "\r" is output on one line, with it turned into two characters, a backslash and "n" or "r"
- C-core-output-11 check's "--format text" with no findings outputs nothing, even when test files were read

## The "tests" of check's JSON counts the test files read per extension

Status: implemented

### Problem

Test files of a language without a query are not read as syntax trees; only their markers are picked up. That could not be read from the output. If a finding were reported for each file of a language without a query, findings would appear as many as the files and get in the way, even though it is not a problem with the files but with the reach of the tool.

### Decision

The "tests" of check's JSON is an object whose keys are the extensions of the test files read and whose values are objects with the two keys "files" and "query". If there are zero test files, "tests" is an empty object.

| Key | Value |
|---|---|
| Extension | The characters after the last "." of the file name, not including ".". For a name without ".", a name whose only "." is the leading one (".rs"), and a name ending in "." ("foo."), the key is an empty string. Upper and lower case are distinguished. The keys are arranged in byte order |
| files | The number of test files read with that extension. Files for which unparsable_file was reported because of a syntax error are also counted. A file is counted once per path, once even if it matches several globs, and a symbolic link and its target are counted as separate paths |
| query | true if the language determined by the extension is a language with a query (a language with one or more bundled or configured rules; [Files of a language with no query at all are not parsed into syntax trees](test-discovery.md#files-of-a-language-with-no-query-at-all-are-not-parsed-into-syntax-trees)), and false otherwise |

### Confirmations

- C-core-output-12 When the globs of "tests.files" match two ".rs" files and one ".go" file with no query, "tests" has "rs" with files 2 and query true and "go" with files 1 and query false, and no other keys
- C-core-output-13 The keys of "tests" are the characters after the last ".", names without ".", ".rs" and "foo." are gathered under the empty-string key, "RS" and "rs" are separate keys, and the keys are arranged in byte order
- C-core-output-14 The same file matched by two globs is counted once, a symbolic link and its target are counted separately, and a file that cannot be read because of a syntax error is also counted in files
- C-core-output-15 If there are zero test files, "tests" is an empty object
- C-core-output-16 For any set of test files, the sum of files over the extensions in "tests" equals the number of test files read
