# Queries added by configuration

English | [日本語](query-rules.ja.md)

Covers the form and reading of the queries added with "tests.rules" in the configuration, their relation to the bundled queries, and the handling when a rule file has an error. The bundled queries are decided in [Bundled queries](test-queries.md), and the rules for applying queries to files to find tests in [Finding tests](test-discovery.md).

## Queries are added with tests.rules in the configuration, and the bundled queries cannot be removed

Status: implemented

### Problem

What counts as a test changes with the test library in use. In Rust, attributes can be added with "tests.rust.attributes", but the same thing happens with libraries for TypeScript, Python and Php. With only the bundled queries, the kotowari binary would have to be fixed every time a library is added.

### Decision

kotowari uses the bundled queries regardless of the configuration, and a project can add queries with "tests.rules" in the configuration. The relation is addition only; the configuration cannot remove or replace the bundled queries. The user judged that, as with "tests.rust.attributes", even if a bundled query matches by mistake, the test files can be narrowed with the glob of "tests.files", so the actual harm is small.

The reason regular expressions written in the configuration were rejected was that searching lines with regular expressions produces false detections ([Tests are found by applying queries to the syntax tree, not by searching lines with regular expressions](test-discovery.md#tests-are-found-by-applying-queries-to-the-syntax-tree-not-by-searching-lines-with-regular-expressions)). A rule applied to nodes of the syntax tree does not fall under this.

### Rejected options

- Keep only the bundled queries, with no way to add them in the configuration. The kotowari binary would have to be fixed every time a library in use is added.
- Previous decision: Decide, as a prohibition, that queries cannot be added in the configuration. Being unable to add them causes the problem above, so this was dropped.

### Confirmations

- C-core-qrules-1 A node matched by a rule of "tests.rules" is counted as a test
- C-core-qrules-2 Even if a rule is added for the same language with "tests.rules", the bundled queries keep working as they are

## A rule file is read as an ast-grep rule YAML as it is

Status: implemented

### Problem

Those who add queries are the people using kotowari, so the form of rules is a promise to them. If kotowari made its own form, what matches could not be confirmed without going through kotowari.

### Decision

"tests.rules" lists the paths of ast-grep rule YAML files. The content of a file uses the form of ast-grep rules as it is, with no form of our own. A rule file can be tried as it is with "ast-grep scan -r", and what matches can be confirmed without going through kotowari. The only promise on the kotowari side is that a node matched by a rule is a test, and what goes into "$NAME" is the test's name ([A test's name is taken from the text the query captured](test-discovery.md#a-tests-name-is-taken-from-the-text-the-query-captured)).

One file may list several rules separated by "---". Each rule joins the queries of the language of its "language". "language" is matched case-insensitively, as in ast-grep, and the language aliases of ast-grep (for example "ts", "py") are also accepted. TypeScript and Tsx are separate languages, as in ast-grep, and a rule with "language: typescript" does not match ".tsx" files.

The default of "tests.rules" is an empty list, and the value is a list of [paths relative to the base directory](base-directory.md#paths-in-configuration-values-and-the-path-of-the-output-are-relative-to-the-base-directory). Globs cannot be used. As with "mutants.equivalents" ([Judgments of equivalence are placed in the YAML list that mutants.equivalents in the configuration points at](equivalents.md#judgments-of-equivalence-are-placed-in-the-yaml-list-that-mutantsequivalents-in-the-configuration-points-at)), it was aligned to the form of writing the path itself.

### Rejected options

- Write rules inside the configuration file. Rules would be hard to try without going through kotowari.

### Confirmations

- C-core-qrules-3 The default of "tests.rules" is an empty list, and the value is read as a list of paths
- C-core-qrules-4 All the rules listed separated by "---" take effect, and the language of a rule's "language" (for example "go") becomes a language that has queries
- C-core-qrules-5 A rule's "language" is matched case-insensitively ("PY"), and language aliases (for example "ts") are also accepted
- C-core-qrules-6 A rule with "language: typescript" does not match ".tsx" files

## The files and ignores of a rule are honoured with the same reading as ast-grep

Status: implemented

### Problem

Sometimes a rule should apply to only part of the test files. The form of ast-grep rules can express this with "files" and "ignores".

### Decision

If a rule has "files" or "ignores", their globs are applied to the path of the test file relative to the base directory, and the rule is not applied to a test file that does not match "files" or that matches "ignores". The globs are read the same way as in ast-grep, not aligned with the reading of "tests.files". The result is the same as when tried with "ast-grep scan".

### Confirmations

- C-core-qrules-7 A rule with "files" is applied only to the files that match it, and is not applied to files that match "ignores"

## The fix, message, severity, note and metadata of a rule are not used

Status: implemented

### Problem

ast-grep rules have items that have nothing to do with finding tests (how to fix, a message to convey, a severity, and so on). How kotowari handles them needs to be decided.

### Decision

The "fix", "message", "severity", "note" and "metadata" of a rule are not used for finding tests. A rule whose "severity" is "off" is also applied. To stop a rule, remove it from "tests.rules".

### Confirmations

- C-core-qrules-8 A rule with "severity: off" is also applied, and even if "fix", "message", "note" or "metadata" are present, the tests counted do not change

## If a rule file has an error, it stops as a configuration error

Status: implemented

### Problem

If a rule file cannot be read and is silently skipped, the check passes without counting the tests that were supposed to be added. When it stops, the rule file that needs fixing must be identifiable.

### Decision

If any of the following holds, kotowari stops with a configuration error as the reason. It is handled the same way as the current configuration errors.

- The file at a path in "tests.rules" is missing, the path is not a file, it cannot be read, or it is not UTF-8
- The same path is listed twice
- It cannot be read as YAML, or cannot be read as an ast-grep rule
- A rule's "language" is not among the languages of [the correspondence between extensions and languages](test-discovery.md#the-correspondence-between-extensions-and-languages-is-the-same-as-ast-grep-and-is-case-sensitive)

Overlapping "id" values of rules are not looked at. kotowari does not use the "id" of rules.

The detail on a stop is the path of the rule file relative to the base directory and a description of the error. Because it is the rule file that needs fixing, outputting its path is more useful. The description when "language" is an unknown language is the single line "unknown language: <language>", and neither the library's wording nor a multi-line excerpt is output. The wording of the reason for a stop is decided in [The reason for a stop is shown with fixed English wording](cli-environment.md#the-reason-for-a-stop-is-shown-with-fixed-english-wording).

### Confirmations

- C-core-qrules-9 If a rule file is missing, is a directory, is not UTF-8, cannot be read as YAML, cannot be read as a rule, has an unknown "language", or the same path is listed twice, in every case the exit code is 2 and standard error starts with "config error"
- C-core-qrules-10 When it stops on a rule file, standard error contains the path of that rule file and does not contain ".kotowari/config.yaml"
- C-core-qrules-11 When it stops on an unknown "language", standard error is a single line containing the rule file and "unknown language: <language>"
