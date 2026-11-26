# In-memory input and already-read results

English | [日本語](library-inputs.ja.md)

This document covers the input groups used when the input needed for checks is passed in memory, how missing input is treated, the reuse of already-read results, and the scope each operation reads. The entry points of the operations and the failure kinds are decided in [Rust API for using a project](library-api.md), and the paths of in-memory input in [Paths of library input](library-paths.md).

## If a required input group is not provided, InputMissing is returned, and an empty group is checked by the ordinary rules

Status: implemented

### Problem

We want to check generated documents and unsaved strings without saving files. In doing so, we do not want missing information needed for the check to be treated as the target not existing or as a successful check. We also do not want input whose found items became empty because parsing failed to be mistaken for success.

On the other hand, kotowari does not look for files from in-memory input, so it does not know which files should be in a group. If the absence of `InputMissing` were mistaken for all input being present, the check would complete without the files that were forgotten having been checked. The reason for this division is not recorded.

### Decision

`ReadModel::build` and `Inspection::build` receive the configuration and the input groups in the following table. For each group, they distinguish whether it was not provided or was provided (including when an empty set was provided).

If a required group is not provided, they do not return a completed result, and return `InputMissing`. A group provided as an empty set is checked by the ordinary rules. A group that does not meet the condition in the table may be left unprovided, and is not used for judgement even if provided.

What `InputMissing` finds is only the groups not provided. It does not find files that the configuration targets but that are missing inside a provided group. Making sure which files go into a group is the caller's job. Duplicates within the same group and content conflicts between groups are decided in [When inputs with the same logical path overlap, they are accepted only when they can be treated as the same file](library-paths.md#when-inputs-with-the-same-logical-path-overlap-they-are-accepted-only-when-they-can-be-treated-as-the-same-file).

| Input group | Contents | Required in `ReadInputs` | Required in `CheckInputs` |
|---|---|---|---|
| Specification documents | Paths and contents | Always | Always |
| Test information | The `SourceText` of the target files, whether their language has a query, the found tests and markers, and the findings from parsing | When `tests.files` is not empty | Same as left |
| Guides | Paths and contents | Not required | When `guides.files` is not empty |
| Surface analysis results | The `SourceText` of the target files, whether their language has a query, the found surfaces, and the findings from parsing | Not required | When `surface.rules` is not empty |
| List of unspecified surfaces | The path and contents of the configured list | Not required | When `surface.rules` is not empty and `surface.unspecified` is specified |
| Additional finding groups | Decided in [Named additional finding groups are received, and ordered and counted together with the other findings](#named-additional-finding-groups-are-received-and-ordered-and-counted-together-with-the-other-findings) | Not required | Not required (added only when passed) |

The findings from parsing held by the test information and the surface analysis results are carried over into the check result.

### Confirmations

- C-core-libinputs-1 Calling `ReadModel::build` or `Inspection::build` without providing a required group returns `InputMissing`, not a completed result
- C-core-libinputs-2 Providing a required group as an empty set does not give `InputMissing`, and returns a result checked by the ordinary rules
- C-core-libinputs-3 A group that does not meet the condition in the table does not give `InputMissing` when not provided, and is not included in the check result when provided
- C-core-libinputs-4 Providing test information whose found tests are empty because parsing failed leaves the parse failure finding in the check result
- C-core-libinputs-13 Even if target files are specified by `guides.files` or `surface.files`, providing that group as an empty set does not give `InputMissing` (files missing from a group are not found)

## A check of in-memory input reads no further files, and gives results by the same rules as reading from files

Status: implemented

### Problem

In the use of checking without saving files, it is a problem if files other than those passed are read. Also, even if the internals are rebuilt as a library, the meaning of check results does not change (for the same reason as in [The CLI only maps the library's results onto the existing output and stop reasons](library-api.md#the-cli-only-maps-the-librarys-results-onto-the-existing-output-and-stop-reasons)).

### Decision

When checking from in-memory input, kotowari reads no further files. From the contents of the provided groups alone, it performs the suppression of duplicate findings, the counting and the check of overlapping guide and test locations, by the same rules as when checking by reading from files.

### Confirmations

- C-core-libinputs-5 Test information, guides and surface analysis results passed in memory are used for judgement as they are without reading files, and the marker counts, the guide staleness findings and the findings on surfaces without specification are reported
- C-core-libinputs-6 The same suppression of duplicate findings, counting and check of overlapping guide and test locations as when reading from files apply to input passed in memory too

## Named additional finding groups are received, and ordered and counted together with the other findings

Status: implemented

### Problem

core does not know about the overview. Even so, we want to include the overview findings in the check result. We want to keep the public API rule that only core assembles the check result, and that it is not assembled or rewritten from outside. We do not want to have the ordering and counting of findings in two places.

### Decision

The check input of core can receive named additional finding groups. A group holds a name, the number of files read, the number of markers, and findings. core does not know the meaning of the group; it orders its findings together with the other findings, counts them in counts, the exit code and complete, and includes each group's counts in the result. kotowari's library passes the overview findings in this form as the "overview" group.

### Rejected options

- Make, on kotowari's side, a separate result type that wraps core's result. This conflicts with the rule of re-exporting core's types as they are ([Whether something is public is decided by whether external users rely on it, and kotowari re-exports, as the same types, only the lower types reachable from its entry points](library-crates.md#whether-something-is-public-is-decided-by-whether-external-users-rely-on-it-and-kotowari-re-exports-as-the-same-types-only-the-lower-types-reachable-from-its-entry-points)), and counting would be in two places.

### Confirmations

- C-core-libinputs-7 Passing an additional group to `Inspection::build` orders its findings by the same rules as the other findings and counts them in counts and complete, and the results of `check` and `status` show the group's name, number of files and number of markers
- C-core-libinputs-8 If no additional group is passed, no group appears in the check result

## Already-read results can be reused, and do not change until explicitly read again

Status: implemented

### Problem

We want to support both the simple use of getting a result in one call and the use of performing several operations on the same input.

### Decision

`Project::read` returns `ReadModel` for list and query, and `Project::inspect` returns `Inspection` for check and status. The `list` and `query` of `ReadModel`, and the `check`, `status` and read results of `Inspection`, return results without reading further files. While they are being reused, files are not implicitly read again. To reflect changes to files, read again from `Project`.

When the operations of `Project` are called directly, the needed input is read anew on each call.

### Confirmations

- C-core-libinputs-9 Even if the original files are changed after `ReadModel` and `Inspection` are obtained, their `list`, `query`, `check` and `status` return results from the contents at the time of reading
- C-core-libinputs-10 After the original files are changed, the results read again from `Project` and the results of calling the operations of `Project` directly come from the changed contents

## An operation reads only the input that operation uses

Status: implemented

### Problem

Even with already-read results made reusable, we do not want to require every operation to read all files. This avoids an operation failing because of a problem in a file it did not need until then. The existing behaviour that list and query do not read guides is also kept.

### Decision

The input for list and query (`ReadInputs`) does not require the guides and surfaces that only check and status use. The same holds when building already-read results. `plan` and `mutants` return results without reading the specification documents.

### Confirmations

- C-core-libinputs-11 Even with `guides.files` configured, `ReadModel` can be built without passing guides, and its `list` and `query` return results
- C-core-libinputs-12 Even in a project without the specification document location, `plan` and `mutants` of `Project` return results

## A public entry point is provided that parses the string of a decision document and returns, read-only, the decisions and confirmations obtained

Status: implemented

### Problem

We want to use the result of parsing specification documents from editor integrations and custom tools. Unless what was obtained and the findings can be referred to even for a document with form errors, half-written documents cannot be handled. We do not want to require users who use only parsing to provide inputs meant for checks, such as test information and guides.

The current public entry point `ir::parse` parses an IR document and returns items, references and positions. Because the specification is held only in decision documents ([The specification lives only in decision documents, one per topic, holding the decisions now in force](decision-documents.md#the-specification-lives-only-in-decision-documents-one-per-topic-holding-the-decisions-now-in-force)), an entry point that parses IR documents no longer has anything to be used for.

### Decision

kotowari-core has a public entry point that receives the string of a decision document together with its logical path, and returns, read-only, the decisions and confirmations of that document together with their lines in the original document. Even for a document with form errors, it returns the decisions and confirmations obtained and the findings on the form errors. This entry point requires no input groups other than the string of the document.

`ir::parse`, which parses IR documents, is replaced by this entry point.

### Confirmations

- C-core-libinputs-14 Passing only the string of a decision document to the entry point returns the decisions and confirmations in the document together with their lines in the original document
- C-core-libinputs-15 Even when a decision document with form errors is passed, the decisions and confirmations obtained and the findings on the form errors are returned
