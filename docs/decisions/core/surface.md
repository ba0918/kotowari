# Checking surfaces

English | [日本語](surface.ja.md)

Covers how surfaces visible to users are taken out of the code with "surface.rules" and "surface.files" in the configuration and "kotowari check" confirms whether the names of the surfaces are written in decision documents, how the number of excluded surfaces is output, which commands read surfaces, and the guidance placed in the skill. The place of the list of unspecified surfaces and the handling of one entry are decided in [List of unspecified surfaces](surface-unspecified.md).

## Surfaces are taken out of the code with surface.files and surface.rules in the configuration, as nodes matched by ast-grep rules

Status: implemented

### Problem

kotowari could not find by machine features that were implemented without being written in the specification. New code without tests is stopped by mutation tests on the diff, and tests without markers by test_without_id. But tests given a marker with the ID of a nearby existing decision, changes that do not go through the workflow, and projects that do not run mutation tests slip through. The root is that kotowari holds no list of the behaviour visible to users, so even when behaviour not in the specification increases, it cannot be counted and compared.

The form of surfaces (CLI subcommands and flags, configuration keys, HTTP routes, and so on) differs per framework. With the same construction as "tests.rules", which finds tests, one way of writing rules is enough to learn, and it does not depend on the language. If kotowari shaped the form of names in a fixed way (supplying "--", for example), it would cause trouble for surfaces other than the CLI, such as configuration keys and HTTP routes. Running it on kotowari's own CLI lets how loose and how strict the judgement is be checked within this specification.

### Decision

Surfaces are taken out with "surface.files" (globs of the files to search) and "surface.rules" (files of ast-grep rules) in the configuration. One syntax-tree node matched by a rule is one surface; the rule's "id" is the kind of the surface, and the text of the node captured with the metavariable "$NAME" is the name of the surface. A rule is applied only to surface files of the same language as its "language".

If the text of a name is enclosed before and after by the same quotation mark, one pair is removed. kotowari does no more shaping than that. To change the form, the person writing the rule shapes it inside the ast-grep rule.

kotowari bundles no surface rules. Examples of how to write them are placed in the skill ([The guidance on writing and fixing the surface check is placed in a reference of the skill, and only the brainstorm and adoption skills add to the list of unspecified surfaces](#the-guidance-on-writing-and-fixing-the-surface-check-is-placed-in-a-reference-of-the-skill-and-only-the-brainstorm-and-adoption-skills-add-to-the-list-of-unspecified-surfaces)). kotowari's own configuration has rules that take out the strings of subcommands and the strings of flags starting with "--" (whether the place they are compared is a match arm or an if), and checks the surfaces of its own CLI.

### Confirmations

- C-core-surface-1 One node matched by a rule becomes one surface; the rule's "id" is the kind, and the text captured with "$NAME" is the name
- C-core-surface-2 If the captured text has the same quotation mark before and after, one pair is removed, and any other form becomes the name as it is
- C-core-surface-3 A rule is not applied to surface files of a language different from the rule's "language"
- C-core-surface-4 (by reading) kotowari's own configuration has surface rules that take out the strings of subcommands and the strings of flags starting with "--"

## Surface rules and surface files are read with the same construction as tests.rules and tests.files

Status: implemented

### Problem

If the rules and files that take out surfaces were read differently from the rules and files that find tests, users would have to learn two ways of writing. No reason is left for the decision not to add surface rules to the queries that find tests.

If rules whose "severity" is set to "off" were not applied, one line in a rule file would stop the surface check, and the surfaces checked would silently decrease. No reason for the decision to apply rules whose "severity" is "off" is left in the records.

### Decision

The rule files of "surface.rules" are read as ast-grep rule YAML, the same as "tests.rules". Even if a rule's "severity" is "off", the rule is applied. Surface rules are not added to the queries that find tests. The reading of the globs of "surface.files" and the scan are the same as for "tests.files".

### Confirmations

- C-core-surface-5 Surface rules match as ast-grep rules in the same way as "tests.rules", and are not added to the queries that find tests (even if there is a function a surface rule matches, it is not counted as a test)
- C-core-surface-26 A surface rule written with "severity: off" also matches and takes out surfaces, like the other rules

## Only surface files in the languages of the rules are read and parsed into syntax trees, and other surface files are not read

Status: implemented

### Problem

If "surface.files" is written broadly, like "src/**", unrelated documents, files of other languages, and binary files such as images match. If those were read and reported syntax errors, or stopped because they are not UTF-8, the check would stop on files unrelated to the rules. Surfaces are not taken out of files not in a rule's language, so there is no need to read them.

### Decision

Among surface files, only the files whose language, determined from the extension, is the same as the "language" of one of the surface rules are read and parsed into syntax trees with tree-sitter. If there is a syntax error, an unparsable_file error is output for that file, the same as for test files ([A test file with a syntax error gets unparsable_file and is skipped](test-discovery.md#a-test-file-with-a-syntax-error-gets-unparsable_file-and-is-skipped)). If that file cannot be read or is not UTF-8, it stops, the same as for test files.

Other surface files are not read. No syntax error is output for them, and neither being unreadable nor not being UTF-8 is a reason for a stop.

### Confirmations

- C-core-surface-6 If a surface file in a rule's language has a syntax error, an unparsable_file error is output for that file
- C-core-surface-7 A surface file not in a rule's language is not read, and even if it has a syntax error or is not UTF-8, neither an error nor a stop is output
- C-core-surface-8 If a surface file in a rule's language is not UTF-8 or cannot be read, check stops

## When the surface configuration keys are not all present, every command that reads the configuration stops

Status: implemented

### Problem

If a configuration with only rules written and the files to search empty, or with only files to search written and the rules empty, were passed without doing anything, a state in which one believes the check runs but nothing is done would go unnoticed. The same holds when only the key of the list of unspecified surfaces is written and it is silently not read: a mistake in writing the key goes unnoticed.

### Decision

With any of the following configurations, every command that reads the configuration file stops with a configuration error.

| Configuration | Handling |
|---|---|
| "surface.rules" is not empty, and "surface.files" is empty | Stops |
| "surface.files" is not empty, and "surface.rules" is empty | Stops |
| "surface.unspecified" is present, and "surface.rules" is empty | Stops |

When a surface rule file falls under a form that would stop it if it were a file of "tests.rules" ([If a rule file has an error, it stops as a configuration error](query-rules.md#if-a-rule-file-has-an-error-it-stops-as-a-configuration-error)), check and status stop with the same handling. Commands that do not read surface rule files do not stop on errors in those files ([Only check and status read surfaces](#only-check-and-status-read-surfaces)).

### Confirmations

- C-core-surface-9 With a configuration that has only "surface.rules", a configuration that has only "surface.files", or a configuration that has "surface.unspecified" with "surface.rules" empty, check, status, list, query and mutants stop with a configuration error
- C-core-surface-10 When a surface rule file is missing, is not a file, cannot be read as a rule, or the same path is written twice, check and status stop

## If the name of a surface is quoted in "Decision" or "Confirmations" of a decision, that surface is in the specification

Status: implemented

### Problem

How to judge whether a surface is in the specification had to be decided. It should pass with nothing added to the code, as long as the specification side is written by the usual rule of writing (concrete values are enclosed in double quotes). The judgement is only whether the name appears; whether the content is correct is not looked at.

A decision whose status is proposed has been decided but not built yet, and it is written in the specification. The drift of something built but still proposed is outside the scope of this check.

### Decision

When the name of a surface appears, with the same characters, as the content enclosed in a pair of double quotes or a pair of backquotes in "Decision" or "Confirmations" of a decision in any decision document, that surface is in the specification. Tables are looked at cell by cell. A name that is only part of the enclosed content does not count as appearing. It counts whether the decision's status is proposed or implemented.

Names appearing in "Problem", "Rejected options" and "Open" are not counted. The inside of code blocks is not read either ([The inside of a code block in a decision document is read neither as decision form nor as a quotation](ir-document.md#the-inside-of-a-code-block-in-a-decision-document-is-read-neither-as-decision-form-nor-as-a-quotation)).

When decision documents are held as multilingual pairs, which side's quotations are counted is decided in [For pairs of decision documents, counting and output are done on the first language's side only](translation-pairs.md#for-pairs-of-decision-documents-counting-and-output-are-done-on-the-first-languages-side-only).

### Rejected options

- Have markers, the same as for tests, written beside the definitions of surfaces. Which decision decides a surface would be known exactly, but the effort of writing in the code increases, and the hole of attaching a loosely related ID remains.

### Confirmations

- C-core-surface-11 A surface whose name is written as it is, enclosed in double quotes or backquotes, in "Decision" or "Confirmations" of a decision gets no surface_without_spec error, even if the decision's status is proposed
- C-core-surface-12 If a name is only part of the enclosed content (for example "--verbose" inside "--verbose true"), that surface is not counted as in the specification
- C-core-surface-13 A surface whose name is written only in "Problem", "Rejected options" or "Open" is not counted as in the specification, and gets a surface_without_spec error
- C-core-surface-14 Even within "Decision" or "Confirmations" of a decision, a surface whose name is written only inside a code block (including one inside a list item or a quotation) is not counted as in the specification

## A surface not in the specification gets one surface_without_spec error at the place in the code where it is first found

Status: implemented

### Problem

The aim of this check is a gate that does not let features not in the specification through. Making it a notice would weaken its meaning. On the other hand, nothing should happen to projects that write no rules.

The same surface may appear in several places (in kotowari itself too, the same flag appears in two match arms). Counting the same surface many times inflates the findings.

### Decision

A surface that is not in the specification and is not excluded by any entry of the list of unspecified surfaces gets a surface_without_spec error. The place is the surface file where the surface was found and its line, and detail is the kind and name of the surface. Even if surfaces of the same kind and name are in several places, only one is output, at the first place.

If "surface.rules" is an empty list, no surfaces are taken out, and this error is not output.

### Confirmations

- C-core-surface-15 A surface that is not in the specification and not on the list gets a surface_without_spec error whose place is the path and line of the surface file and whose detail is the kind and name, and the exit code is 1
- C-core-surface-16 Even if surfaces of the same kind and name are in two or more places, there is only one surface_without_spec error, at the first place
- C-core-surface-17 If "surface.rules" is empty, no surfaces are taken out, and no surface_without_spec error is output

## The number of surfaces excluded by the list of unspecified surfaces is always output at the end of the output of check

Status: implemented

### Problem

The list of unspecified surfaces exists so that an existing project can adopt the check even if a large number of errors appear from the start. If surfaces that raise no error could be silently increased, the amount relying on the list would become invisible. The amount relying on the list should be visible every time.

### Decision

When "surface.rules" is not empty, "kotowari check" always outputs, in its output, the number of surfaces excluded by the list of unspecified surfaces. In JSON it is output in "unspecified" of the top-level "surface", and when "--format" is "text", "surface: unspecified=<n>" is output as the last line after the lines of the findings. It is output even when there are zero findings and even when the number is 0.

If "surface.rules" is empty, neither the "surface" key of the JSON nor the text line is output (the top-level keys of the JSON are decided in [The top level of check's JSON shows the findings, the count per kind, and the counts of what was read](output.md#the-top-level-of-checks-json-shows-the-findings-the-count-per-kind-and-the-counts-of-what-was-read)).

### Confirmations

- C-core-surface-18 If "surface.rules" is not empty, the number of surfaces excluded by the list is output in "unspecified" of "surface" in the JSON of check, even when there are zero findings and even when the number is 0
- C-core-surface-19 check with "--format" "text" outputs "surface: unspecified=<n>" as the last line after the lines of the findings
- C-core-surface-20 If "surface.rules" is empty, the JSON of check has no "surface" key, and the output with "--format" "text" has no line starting with "surface: "

## Only check and status read surfaces

Status: implemented

### Problem

complete in status is determined by the number of errors of check, so surface errors also enter into it. On the other hand, it is a problem if commands that do not need surfaces stop on errors in lists and rules they do not read. The amount relying on the list should also be visible in status.

### Decision

Only "kotowari check" and "kotowari status" read surface files, surface rule files and the list of unspecified surfaces. status performs the same surface check as check and outputs the surface counts ([status reads the same things as check and runs the same checks, and outputs counts and complete instead of findings](status.md#status-reads-the-same-things-as-check-and-runs-the-same-checks-and-outputs-counts-and-complete-instead-of-findings), [The summary outputs, per group, the counts of test markers, guides, overview, surfaces and findings](status.md#the-summary-outputs-per-group-the-counts-of-test-markers-guides-overview-surfaces-and-findings)).

"kotowari list", "kotowari query", "kotowari plan" and "kotowari mutants" do not read them, and do not stop because of reading them (list and query are decided in [list stops under the same conditions as check, and does not read guides or surfaces](list.md#list-stops-under-the-same-conditions-as-check-and-does-not-read-guides-or-surfaces) and [query stops under the same conditions as check, and does not read guides or surfaces](query.md#query-stops-under-the-same-conditions-as-check-and-does-not-read-guides-or-surfaces)). The output of "kotowari list" does not include surface counts.

### Confirmations

- C-core-surface-21 "kotowari mutants" does not stop even if the file of the list of unspecified surfaces is missing, or surface rule files or surface files are not UTF-8

## The guidance on writing and fixing the surface check is placed in a reference of the skill, and only the brainstorm and adoption skills add to the list of unspecified surfaces

Status: implemented

### Problem

The surface check, both in how to write it and how to fix it, is a different situation from test markers. Surface rules are not bundled, so examples of how to write rules are needed.

If, as the way to fix surface_without_spec, the implementer only adds an entry to the list of unspecified surfaces, a feature not in the specification gets through the gate. Adding an entry to the list is a change of the specification, not something decided within implementation. The work of reducing the list is put on the adoption flow. An entry is removed from the list only when the surface is written into a decision, so the entry of a surface that did not become a decision remains on the list.

### Decision

A document "surface.md" on how to write and fix the surface check is placed in the references of the kotowari skill, with examples of surface rules that take out CLI flags and subcommands. The configuration keys "surface.files", "surface.rules" and "surface.unspecified" are listed in "config.md", and the finding kinds surface_without_spec, surface_unspecified_invalid and surface_unspecified_stale in "findings.md".

As the way to fix surface_without_spec, only kotowari-brainstorm and kotowari-adopt add an entry to the list of unspecified surfaces. For the implementer, only returning a surface not in the specification to the brainstorm is written.

kotowari-adopt puts on the list all the surfaces not in the specification that appear when surface rules are first written, and reduces them topic by topic. When a topic's surface is written into a decision, that entry is removed from the list. The entries of surfaces that did not become decisions and stayed open, and of surfaces a person excluded in the list round, remain on the list.

### Confirmations

- C-core-surface-22 (by reading) The references of the kotowari skill have "surface.md", which contains how to write and fix the surface check and examples of surface rules that take out CLI flags and subcommands
- C-core-surface-23 (by reading) "config.md" lists the keys "surface.files", "surface.rules" and "surface.unspecified", and "findings.md" lists the kinds surface_without_spec, surface_unspecified_invalid and surface_unspecified_stale
- C-core-surface-24 (by reading) The skill says that, as the way to fix surface_without_spec, only kotowari-brainstorm and kotowari-adopt add an entry to the list of unspecified surfaces, and that the implementer returns a surface not in the specification to the brainstorm
- C-core-surface-25 (by reading) kotowari-adopt says to put on the list all the surfaces not in the specification when rules are first written and reduce them topic by topic, to remove that entry from the list when a surface is written into a decision, and to keep on the list the entry of a surface that did not become a decision
