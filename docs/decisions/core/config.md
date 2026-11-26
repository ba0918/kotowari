# Configuration

English | [日本語](config.ja.md)

Covers the location of the configuration file, the keys and their default values, how values are read, and the stops when a location is missing and on configuration errors. What the paths in configuration values are read relative to and how they are normalized is decided in [Paths in configuration values and the "path" of the output are relative to the base directory](base-directory.md#paths-in-configuration-values-and-the-path-of-the-output-are-relative-to-the-base-directory), and the stop when what "--config" points at does not exist in [An option not accepted, an unknown command, an extra positional argument, or an option with a wrong value stops with an argument error](cli.md#an-option-not-accepted-an-unknown-command-an-extra-positional-argument-or-an-option-with-a-wrong-value-stops-with-an-argument-error). The meaning of each key's value is decided in the topic that uses that key.

## The configuration file is placed at .kotowari/config.yaml in the base directory, and --config can point at another file

Status: implemented

### Problem

Where to have people write what kotowari reads has to be decided. The user judged that placing it in a ".kotowari/" directory makes it easy to manage, since things can be added inside that directory as dependencies grow, and also lets an LLM read that configuration file. The reason for reading the path of "--config" relative to the current directory is not recorded.

### Decision

A command other than "kotowari plan", when there is no "--config", reads ".kotowari/config.yaml" in the base directory ([The base of relative paths is the first directory with .kotowari/ found by searching upward](base-directory.md#the-base-of-relative-paths-is-the-first-directory-with-kotowari-found-by-searching-upward)) as the configuration file. When there is "--config", it reads that value as a path relative to the current directory and makes that file the configuration file. "kotowari plan" does not read the configuration.

"kotowari.toml" at the repository root is not read as the configuration file.

### Rejected options

- Place the configuration in "kotowari.toml" at the repository root. Being able to add things inside ".kotowari/" as dependencies grow is easier to manage.

### Confirmations

- C-core-config-1 When there is no "--config", ".kotowari/config.yaml" in the base directory is read as the configuration file
- C-core-config-2 The path of "--config" is read as relative to the current directory, and check, list, query, status and overview build all use the configuration in that file
- C-core-config-3 Even if a "kotowari.toml" with values that change the locations is placed at the repository root, it is not read

## When the configuration file is missing or empty, kotowari runs with the default values

Status: implemented

### Problem

kotowari is to be usable from the start even in a project that writes no configuration file. The reason is not recorded.

### Decision

When "--config" is not given and there is no configuration file, kotowari runs with the default values. When the configuration file is empty (zero bytes, or only comments), kotowari runs with the default values even if it is the one "--config" points at.

### Confirmations

- C-core-config-4 When there is no configuration file, check runs with the default values
- C-core-config-5 A zero-byte configuration file and a configuration file with only comments are read as the default values

## The configuration keys are only those in the table, and a key not written takes the default value in the table

Status: implemented

### Problem

Where to look for tests, and the locations of guide and surface files, differ from product to product. The user judged that they should be changeable by configuration. The reason for each key's default value is in the topic that uses that key.

### Decision

The configuration keys are only those in the following table (the key for the location of decision documents is decided in [The location of decision documents is pointed at by decisions.documents with docs/decisions as the default, and the keys for the locations of the old form are removed](#the-location-of-decision-documents-is-pointed-at-by-decisionsdocuments-with-docsdecisions-as-the-default-and-the-keys-for-the-locations-of-the-old-form-are-removed), and the keys that are removed in [The keys for the limits on line count and number of requirements, limits.lines and limits.requirements, are removed](#the-keys-for-the-limits-on-line-count-and-number-of-requirements-limitslines-and-limitsrequirements-are-removed)). A key not written takes the default value in the table.

| Key | Value | Default |
|---|---|---|
| tests.files | A list of globs. Where to look for test files ([Test files are selected with the globs of tests.files](test-discovery.md#test-files-are-selected-with-the-globs-of-testsfiles)) | "src/\*\*/\*.rs", "tests/\*\*/\*.rs" |
| tests.rust.attributes | A list of attribute paths counted in addition to "#[test]" ([In Rust, functions whose attribute path ends in test are always counted, and attributes and macros are added by configuration](test-queries.md#in-rust-functions-whose-attribute-path-ends-in-test-are-always-counted-and-attributes-and-macros-are-added-by-configuration)) | An empty list |
| tests.rust.macros | A list of names of macros that contain tests (same as above) | An empty list |
| tests.rules | A list of paths of YAML files of ast-grep rules. Globs cannot be used ([Queries are added with tests.rules in the configuration, and the bundled queries cannot be removed](query-rules.md#queries-are-added-with-testsrules-in-the-configuration-and-the-bundled-queries-cannot-be-removed)) | An empty list |
| guides.files | A list of globs. The location of guides ([Guides are chosen by the globs of guides.files, and only check and status read them](guides.md#guides-are-chosen-by-the-globs-of-guidesfiles-and-only-check-and-status-read-them)) | An empty list |
| mutants.equivalents | The path of the file of the list of equivalents (one string) ([Judgments of equivalence are placed in the YAML list that mutants.equivalents in the configuration points at](equivalents.md#judgments-of-equivalence-are-placed-in-the-yaml-list-that-mutantsequivalents-in-the-configuration-points-at)) | None (the list of equivalents has zero entries) |
| surface.files | A list of globs. The location of surface files ([Surfaces are taken out of the code with surface.files and surface.rules in the configuration, as nodes matched by ast-grep rules](surface.md#surfaces-are-taken-out-of-the-code-with-surfacefiles-and-surfacerules-in-the-configuration-as-nodes-matched-by-ast-grep-rules)) | An empty list |
| surface.rules | A list of paths of YAML files of ast-grep rules. Globs cannot be used. If it is not an empty list, the surface check is performed | An empty list |
| surface.unspecified | The path of the file of the list of unspecified surfaces (one string) ([A surface not yet made part of the specification is not an error if it is put on the list of unspecified surfaces with a reason](surface-unspecified.md#a-surface-not-yet-made-part-of-the-specification-is-not-an-error-if-it-is-put-on-the-list-of-unspecified-surfaces-with-a-reason)) | None (the list of unspecified surfaces has zero entries) |
| vague_words | A list of vague words (the sections vague words are matched against are in [Keep the glossary as a document for people, and drop the term check](decision-documents.md#keep-the-glossary-as-a-document-for-people-and-drop-the-term-check)) | The four words "適切に", "必要に応じて", "通常は", "など" |
| overview.files | A list of globs. The location of the overview source data ([The overview source data is committed to the repository and maintained, and selected by the globs of overview.files](overview-data.md#the-overview-source-data-is-committed-to-the-repository-and-maintained-and-selected-by-the-globs-of-overviewfiles)). Required when "overview" is written | When there is no "overview", the overview source data is not read |
| overview.toc | The path of the table of contents file (one string) ([How the index is bundled and ordered is written in the one table of contents file that overview.toc points at](overview-toc.md#how-the-index-is-bundled-and-ordered-is-written-in-the-one-table-of-contents-file-that-overviewtoc-points-at)). Required when "overview" is written | When there is no "overview", the table of contents is not read |
| languages | A list of language tags ([The language list is decided by the configuration's languages](translation-pairs.md#the-language-list-is-decided-by-the-configurations-languages)) | None (taken as a list of only "en") |
| labels | Per language tag, a mapping from UI strings keys to strings ([kotowari holds the English UI strings, and other languages are written in the configuration's labels](overview-languages.md#kotowari-holds-the-english-ui-strings-and-other-languages-are-written-in-the-configurations-labels)) | None (kotowari holds the English UI strings) |

The overview is enabled only when the "overview" key is written. The mere existence of the location of the overview source data does not enable it. This is so that it is not silently enabled by default.

### Rejected options

- If the location of the overview source data (".kotowari/overview/") exists, enable it automatically even without the "overview" key. It would be silently enabled by default.

### Confirmations

- C-core-config-6 If no key is written in the configuration file, each key takes the default value in the table
- C-core-config-7 A value written for a key in the table is read as written

## The location of decision documents is pointed at by decisions.documents with docs/decisions as the default, and the keys for the locations of the old form are removed

Status: implemented

### Problem

The specification is held only in decision documents, and check stops reading the IR location and the locations of decision records and ADRs ([Put decision documents in docs/decisions, and move documents in the old form to the frozen tree](decision-documents.md#put-decision-documents-in-docsdecisions-and-move-documents-in-the-old-form-to-the-frozen-tree) and [Switch at once in a new version, and build no mechanism to read the old form](form-switch.md#switch-at-once-in-a-new-version-and-build-no-mechanism-to-read-the-old-form)). If keys that are not read are still accepted, the person who writes them misunderstands them as having an effect. The user has decided not to build a mechanism that reads the old form.

### Decision

The location of decision documents is written in the configuration key "decisions.documents" as the path of a directory (one string). The default is "docs/decisions".

The keys for the locations of the old form, "ir", "decisions.records" and "decisions.adr", are removed. If they are written in the configuration file, kotowari stops with a configuration error, as for keys not in the table.

### Confirmations

- C-core-config-8 When there is no "decisions.documents" in the configuration, check reads the decision documents under "docs/decisions"
- C-core-config-9 If the configuration file has any of "ir", "decisions.records" or "decisions.adr", kotowari stops with a configuration error

## The keys for the limits on line count and number of requirements, limits.lines and limits.requirements, are removed

Status: implemented

### Problem

For decision documents, the notice for the limit on line count (too_many_lines) is not reported. The notice for the limit on the number of requirements (too_many_requirements) loses its meaning because requirements disappear. If the keys that hold the limit values are still accepted, the person who writes them misunderstands them as having an effect.

### Decision

The configuration keys "limits.lines" and "limits.requirements" are removed. If they are written in the configuration file, kotowari stops with a configuration error, as for keys not in the table. The stop when the value is negative or zero also disappears with these keys.

### Confirmations

- C-core-config-27 If the configuration file has "limits.lines" or "limits.requirements", kotowari stops with a configuration error

## If the location of decision documents does not exist, is not a directory, or cannot be read, kotowari stops with unreadable file

Status: implemented

### Problem

If a mistake in writing a location is silently let through as a state with zero documents to read, it would seem to be checked while nothing is checked ([Unreadable input and broken input become a stop or an error finding, and are not silently skipped](cli-environment.md#unreadable-input-and-broken-input-become-a-stop-or-an-error-finding-and-are-not-silently-skipped)). The reason for making a missing location a stop with unreadable file rather than a configuration error is not recorded.

### Decision

"kotowari check" stops with unreadable file as the reason when what the location of decision documents points at does not exist, is not a directory, or cannot be read.

When scanning "tests.files", "guides.files", "surface.files" or "overview.files" meets a directory that cannot be read or a symbolic link whose target does not exist, it also stops for the same reason. Rules that differ by location are hard to remember, so the scanning of every location is the same.

### Confirmations

- C-core-config-10 If what the location of decision documents points at does not exist, is not a directory, or cannot be read, check stops with unreadable file as the reason
- C-core-config-11 If scanning "tests.files", "surface.files" or "overview.files" meets a directory that cannot be read or a symbolic link whose target does not exist, check stops with unreadable file as the reason

## If the configuration file cannot be read as YAML, or its keys and values do not follow the rules, kotowari stops with a configuration error

Status: implemented

### Problem

The user decided that if there is an unknown key, kotowari does not run and returns an error. If configuration errors are silently skipped or silently replaced with default values, the person who wrote them misunderstands the configuration as having an effect ([Unreadable input and broken input become a stop or an error finding, and are not silently skipped](cli-environment.md#unreadable-input-and-broken-input-become-a-stop-or-an-error-finding-and-are-not-silently-skipped)).

A glob element that starts with "/" does not match any path relative to the base directory, and matches nothing. Rejecting it does not break any configuration that worked. Whether a path is absolute is told only by the leading "/" because the target operating systems are Linux and macOS ([The target operating systems are Linux and macOS, and on Windows only the normalization of path separators is promised](cli-environment.md#the-target-operating-systems-are-linux-and-macos-and-on-windows-only-the-normalization-of-path-separators-is-promised)). Looking at Windows drive letters as well would reject relative paths that can be written on Linux and macOS ("a:b") as absolute paths.

An empty string in "vague_words" has no meaning as a word, and accepting it would silently let a broken configuration through. The reason for making an empty string a stop is not recorded.

When kotowari stops on a duplicate key, outputting the YAML library's wording and a multi-line excerpt as they are makes it hard to read which key to fix. The text of the description is not a contract, but it is a clue for fixing.

### Decision

If any of the following applies, kotowari stops with a configuration error (config error) as the reason.

| Configuration error | Note |
|---|---|
| The configuration file cannot be read as YAML | |
| A key not in the table | At whatever level it is written |
| A second occurrence of the same key | The description is the one line "duplicate key: <key>", and neither the library's wording nor a multi-line excerpt is output |
| A key whose value is null (a line with only "tests:") | An empty list is accepted ([A list key accepts only a list, the list written replaces the default, and an empty list is a list with no elements](#a-list-key-accepts-only-a-list-the-list-written-replaces-the-default-and-an-empty-list-is-a-list-with-no-elements)) |
| A value of the wrong type | |
| A value that is an absolute path (one starting with "/") | Including the glob elements of "tests.files", "guides.files", "surface.files" and "overview.files". Whether a path is absolute is told only by the leading "/", and a value that resembles a Windows drive letter, such as "a:b", is not rejected as an absolute path |
| An element of "vague_words" that is an empty string, and a second occurrence of the same word | |
| An element of "tests.files", "guides.files", "surface.files" or "overview.files" that cannot be read as a glob | |
| "overview" is written but "overview.files" or "overview.toc" is missing | |

### Confirmations

- C-core-config-12 A configuration file that cannot be read as YAML, and a key not in the table ("limit:" at the top level, an unknown key inside nesting), stop with a configuration error
- C-core-config-13 When the same key appears twice, standard error is one line that starts with "config error: " and contains "duplicate key: <key>"
- C-core-config-14 A key whose value is null and a value of the wrong type stop with a configuration error
- C-core-config-15 A value or glob element that starts with "/" stops with a configuration error
- C-core-config-16 An element of "tests.files", "guides.files", "surface.files" or "overview.files" that cannot be read as a glob stops with a configuration error
- C-core-config-17 If "vague_words" has the same word twice, kotowari stops with a configuration error
- C-core-config-18 If "overview" is written but "overview.files" or "overview.toc" is missing, kotowari stops with a configuration error
- C-core-config-25 If "vague_words" has an element that is an empty string, kotowari stops with exit code 2 and outputs nothing to standard output
- C-core-config-26 A value that does not start with "/" is not rejected as an absolute path, even if it resembles a Windows drive letter, as "a:b" does

## A list key accepts only a list, the list written replaces the default, and an empty list is a list with no elements

Status: implemented

### Problem

Whether the value written for a list key is added to the default list or replaces it has to be decided. The reason is not recorded.

### Decision

A list key accepts only a list. The list written replaces the default list. When an empty list is written, the key is treated as a list with no elements.

### Confirmations

- C-core-config-19 A list written for a list key replaces the default list without being added to it
- C-core-config-20 Writing an empty list for a list key makes it treated as a list with no elements

## Configuration keys are read in nested form

Status: implemented

### Problem

How the "." separators of the keys in the table are written in the configuration file has to be decided. The reason is not recorded.

### Decision

kotowari reads the keys of the configuration file in nested form. "tests.rust.attributes" is written as "attributes:" under "rust:" under "tests:".

### Confirmations

- C-core-config-21 A key written in nested form is read as a key in the table

## The glob "**" is read recursively, and hidden directories are entered only when named by a path component

Status: implemented

### Problem

How far the glob "**" goes, and how hidden directories (directories whose names start with ".") are handled, has to be decided. The reason for excluding hidden directories is not recorded.

The overview source data is placed in ".kotowari/overview/". ".kotowari/" is the location for tool data that people do not read directly, such as the configuration and caches, and the overview source data belongs with them. If hidden directories stayed excluded, the overview source data could not be read.

To count hidden directories named inside braces as named, kotowari would have to hold its own brace expansion. The default values name them by path components, and no user is inconvenienced.

### Decision

kotowari reads the glob "**" as recursive. Hidden directories are not entered even when a glob names them. The exception is "overview.files": under hidden directories named by a path component (".kotowari" and "overview" in ".kotowari/overview/*.md") is read. A broad "**" alone does not enter hidden directories that are not named. A hidden directory named only inside braces does not count as named by a path component.

Hidden files are read if a glob matches them. Symbolic links to directories are not followed.

### Confirmations

- C-core-config-22 "**" matches recursively into subdirectories, and does not match files under hidden directories
- C-core-config-23 Hidden files that a glob matches are read
- C-core-config-24 Overview source data under hidden directories that "overview.files" names by a path component is read, and hidden directories are not entered with only a broad "**" or when named only inside braces
