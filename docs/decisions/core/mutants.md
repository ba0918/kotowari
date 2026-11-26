# Checking mutation outcomes

English | [日本語](mutants.ja.md)

Covers how "kotowari mutants" turns misses and timeouts among the mutation outcomes into findings and outputs the counts. How the results file is mapped onto mutation outcomes is decided in [Reading the results of mutation tests](mutants-input.md), and where the list of equivalents lives and how a match is taken in [List of equivalents](equivalents.md). The arguments "--tool" and the results file are decided in [kotowari mutants always takes the tool's name with --tool, and takes exactly one results file](cli.md#kotowari-mutants-always-takes-the-tools-name-with---tool-and-takes-exactly-one-results-file), the wording of a stop in [The reason for a stop is shown with fixed English wording](cli-environment.md#the-reason-for-a-stop-is-shown-with-fixed-english-wording), the keys of a finding in [A finding is output with the five keys kind, severity, path, line and detail](output.md#a-finding-is-output-with-the-five-keys-kind-severity-path-line-and-detail), and the order in [Findings are arranged by path, line, kind and detail, in that order](finding-order.md#findings-are-arranged-by-path-line-kind-and-detail-in-that-order).

## Mutation tests run outside kotowari, and kotowari mutants reads their results file and turns it into findings

Status: implemented

### Problem

check checks by machine only as far as whether a test points at what was decided. It does not look at whether that test constrains the implementation. Even if a test checks nothing, a marker brings the findings to zero. Mutation testing (a technique that puts small changes into the implementation and sees whether the tests fail) measures this constraining power.

If mutation tests were started inside kotowari, kotowari would carry the problems of running time and memory, and kotowari would become Rust-only. On the other hand, reading a results file and outputting findings is behaviour of the product, and can be tested by placing an input file.

check is required to report zero before every push, but mutation tests take more than ten minutes and cannot be run every time. If check read an old results file, it would stop the push while pointing at lines that have shifted.

### Decision

Running mutation tests stays outside kotowari. kotowari reads, as input, the results file that the mutation-testing tool wrote, and turns it into findings.

What reads the results is not check but a separate command, "kotowari mutants". The results file to read is passed as an argument, so which results were read is clear. The output has the same form as the findings of check, and if there is a miss, the exit code is 1.

"kotowari mutants" reads only the configuration file, the results file, the list of equivalents, the files that the mutation outcomes point at, and the files that the "file" of each entry of the list of equivalents points at. It does not read the specification documents or the test files, and does not perform the checks of check. Even if what a place read by check points at does not exist, "kotowari mutants" does not stop.

The criterion "zero misses (excluding those on the list of equivalents)" is not made part of the kotowari specification. The gate on the side that calls "kotowari mutants" enforces it through the exit code.

### Rejected options

- Keep only a procedure and a script, and leave the kotowari binary unchanged. The correspondence between misses and what was decided remains in people's hands.
- kotowari starts the mutation tests. The binary carries the problems of running time and memory.
- Mix it into check, and point at the results file with a configuration key. check is required to report zero every time, but mutation tests cannot be run every time, and reading old results stops the push on shifted lines.
- Previous decision: "zero misses" is an item that a person confirms before a release. Because a person has to keep it in mind, it was changed so that the gate on the calling side enforces it through the exit code.

### Confirmations

- C-core-mutants-1 Even if what a place read by check points at does not exist, "kotowari mutants" reads the results file without stopping
- C-core-mutants-2 Even if there is a test without a marker, "kotowari mutants" outputs no finding

## A miss is an error and a timeout is a notice, and nothing is output for caught mutations and unviable mutations

Status: implemented

### Problem

A miss is a mutation with which all the tests still passed, and points at a place where the tests do not constrain the implementation. Timeouts occur mostly with mutations that cause an infinite loop. It cannot be said for certain that the tests caught it, but the breakage has been observed.

Which decision of the specification a miss is tied to was also a question. A miss has zero failed tests, so there is no path from a test marker to what was decided. Placing markers on the implementation side, or keeping a table mapping functions to what was decided, would make it traceable, but both are new correspondences that people maintain, and drift in them cannot be checked.

### Decision

For a mutation outcome whose outcome is "missed", a mutant_survived error is output. For a mutation outcome whose outcome is "timed out", a mutant_timeout notice is output. For a mutation outcome whose outcome is "caught" or "unviable", nothing is output.

A miss is not tied to any decision of the specification, and is output with only its place and the change description. In mutant_survived and mutant_timeout findings, path is the file of the mutation outcome, line is its line, and detail is the change description (the text as the tool produced it). The order is the same as for check. With the same form as the findings of check, the receiving side needs only one way of reading.

A timeout is not excluded even if it is on the list of equivalents. A notice does not change the exit code, so no way of excluding it is needed (the list can exclude only misses; see [An entry of the list identifies a mutation by the file, the description of the change, and the text of the line](equivalents.md#an-entry-of-the-list-identifies-a-mutation-by-the-file-the-description-of-the-change-and-the-text-of-the-line)).

Even if there are two or more mutation outcomes with the same content, they are not folded, and one finding is output for each.

### Rejected options

- Place markers on the implementation side. Keep a table mapping functions to what was decided. Both are new correspondences that people maintain, and drift in them cannot be checked.
- Previous decision: As an aid for misses, list candidate decisions that seem related, from the markers of the tests that failed for mutations caught within the same function. When actually run, the names of the failed tests were not in the results file but only in the text of the log. That text is the display of the test runner and not a contract, and because it stops at the first executable that fails, the list is only partial, so this was dropped.

### Confirmations

- C-core-mutants-3 A missed mutation outcome gets a mutant_survived error whose path is its file, line is its line, and detail is the change description, and the exit code is 1
- C-core-mutants-4 Results with only caught mutations and unviable mutations get no finding, and the exit code is 0
- C-core-mutants-5 A timed-out mutation outcome gets a mutant_timeout notice even if the list of equivalents has an entry that matches that mutation, and the exit code is 0
- C-core-mutants-6 If there are two misses with the same content, two mutant_survived errors are output as well

## Counts are always output, and the number excluded as equivalent is shown every time

Status: implemented

### Problem

When a miss is put on the list of equivalents, that miss is excluded from the findings. Whether it is really equivalent cannot be decided by machine. If the amount relying on the list is visible every time, an abnormal increase can be noticed. There was also an option to put an upper limit on the number of equivalents, but no grounds can be given for the limit's number.

The brakes on declaring equivalents were made three stages. The first stage is these counts; the second stage (before adding to the list, have an LLM in a different context write a test that kills that mutation) and the third stage (changes to the list are mixed into a person's usual review of the diff) are on the procedure side.

If the mutation outcomes are counted separately per kind of outcome and the total matches the total number of mutations, a missed read shows up in the numbers. A push whose diff has no line that gets mutations produces results with zero mutations, and that must not stop it.

### Decision

"kotowari mutants" always outputs, in its output, the counts of mutation outcomes per kind of outcome.

The top level of the JSON has only the following three keys.

| Level | Key | Content |
|---|---|---|
| Top level | findings | The list of findings. Keys and order are the same as check |
| Top level | counts | The number of findings per kind. A kind with no findings is not present |
| Top level | mutants | An object with the five keys below |
| Inside "mutants" | caught | The number of mutation outcomes whose outcome is "caught" |
| Inside "mutants" | survived | The number of mutation outcomes whose outcome is "missed" and that match no entry of the list of equivalents |
| Inside "mutants" | timeout | The number of mutation outcomes whose outcome is "timed out" |
| Inside "mutants" | unviable | The number of mutation outcomes whose outcome is "unviable" |
| Inside "mutants" | equivalent | The number of mutation outcomes whose outcome is "missed" and that match one or more entries of the list of equivalents |

A miss excluded as equivalent is not counted in survived, and is counted only in equivalent.

When "--format" is "text", the same counts are output as the last line, after the lines of the findings, in the form "mutants: caught=<n> survived=<n> timeout=<n> unviable=<n> equivalent=<n>". It is output even when there are zero findings.

For results with zero mutations, the exit code is 0 and all the counts are 0.

### Rejected options

- Put an upper limit on the number of equivalents. No grounds can be given for the limit's number.

### Confirmations

- C-core-mutants-7 The top-level keys of the JSON are only the three findings, counts and mutants, and mutants has caught, survived, timeout, unviable and equivalent
- C-core-mutants-8 The sum of the five values of mutants equals the number of mutation outcomes, survived equals the number of mutant_survived findings, and timeout equals the number of mutant_timeout findings
- C-core-mutants-9 With "--format text", "mutants: caught=<n> survived=<n> timeout=<n> unviable=<n> equivalent=<n>" is output as the last line after the lines of the findings, and this line is output even when there are zero findings
- C-core-mutants-10 For results with zero mutations, the exit code is 0 and the five values of the counts are all 0

## Finding kinds and detail use no words specific to a mutation-testing tool

Status: implemented

### Problem

kotowari itself is a tool that does not restrict the language, and the mutation-testing tool that produces the results may change. For now there are only Rust and cargo-mutants, but if words and forms specific to a tool leak into the findings, adding a tool means changing the existing rules and findings. This is hard to check by machine, but once written, the implementing side and the review read it.

### Decision

The finding kinds and the form of detail use no words specific to a mutation-testing tool. The finding kinds are general terms of the mutation-testing field, and are the four mutant_survived (a miss), mutant_timeout (a timeout), equivalent_stale (the text of an entry of the list is not in the source) and equivalent_invalid (the form of an entry of the list is wrong). The word of cargo-mutants (missed) is not used.

The change description inside detail is the text as the tool produced it, so it is not subject to this rule. The part that maps the values of the tool's results ([Mutation outcomes are held in a form that does not know the tool, and the results file of cargo-mutants is mapped as in the table](mutants-input.md#mutation-outcomes-are-held-in-a-form-that-does-not-know-the-tool-and-the-results-file-of-cargo-mutants-is-mapped-as-in-the-table)) is not subject to it either.

### Confirmations

- C-core-mutants-11 (by reading) The spellings of the values of the tool's results (CaughtMutant, MissedMutant) do not appear in the code that builds the finding kinds (mutant_survived, mutant_timeout, equivalent_stale, equivalent_invalid) and detail. The part that maps the tool's values is out of scope
