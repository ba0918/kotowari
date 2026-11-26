# The CLI and how results are output

English | [日本語](cli.ja.md)

This covers the commands the mds CLI `kotowari-mds` accepts, the exit codes, the output formats, the shape of findings, and how a stop is reported when a check cannot be performed. The handling of undeclared headings and lines and `--open` are decided in [the "Closed world and open world" topic](closed-world.md), and the entry point for using the same loading as the CLI in outside applications in [Loading of schemas and documents is placed in a separate crate shared by the CLI and outside applications](library-io.md#loading-of-schemas-and-documents-is-placed-in-a-separate-crate-shared-by-the-cli-and-outside-applications). How to write a schema is decided in the ["The skeleton of the schema language"](schema-language.md) topic, how the structure of a document is read in the ["The skeleton of a document"](document-structure.md) and ["Rules for the lines under a heading"](block-rules.md) topics, the shape of value extraction in the ["Extraction and the plain syntax tree"](extraction.md) topic, and how "$schema" is resolved in the ["Schema resolution"](schema-resolution.md) topic. Which inputs make a check impossible is decided in each of those topics, and this topic decides how the stop is reported in that case.

## The mds CLI is named kotowari-mds and accepts check, ast, values, and --version

Status: implemented

### Problem

mds is a CLI that checks a Markdown document against the YAML schema that the "$schema" in the document's frontmatter points at, and extracts values. Its purpose is to make it possible to fix mechanically the format of Markdown written by an LLM.

If the name of the CLI binary were the short "mds", it would collide with a name someone else has already published on the publishing site for Rust crates, and could not be distributed. The reason for making the accepted commands the four of checking, syntax tree, extraction, and version is not recorded.

### Decision

The name of the CLI binary is `kotowari-mds`. Writing the product's short name as "mds" in prose does not change.

`kotowari-mds` accepts four: `check` for checking, `ast` for the plain syntax tree, `values` for extraction, and `--version` for printing the version. The plain syntax tree that `ast` outputs is decided in [ast outputs the plain syntax tree of a document as JSON, separately from validation](extraction.md#ast-outputs-the-plain-syntax-tree-of-a-document-as-json-separately-from-validation). When `--schema` is given to `ast`, it outputs typed values instead of the plain syntax tree ([ast --schema adds the schema's name as type to the same values as values --format json](#ast---schema-adds-the-schemas-name-as-type-to-the-same-values-as-values---format-json)).

### Rejected options

- Make it a kotowari subcommand, "kotowari mds". The people who use the mds CLI are people who do not use kotowari (kotowari calls mds as a library), and they would be made to install the specification tool along with it. The versions kept separate for mds and kotowari would also become kotowari's version on the CLI.

### Confirmations

- C-schema-cli-1 `kotowari-mds --version` prints the version and ends with exit code 0

## ast --schema adds the schema's name as type to the same values as values --format json

Status: implemented

### Problem

`ast --schema` is built as the extracted values with the name of a type added, and has no way of assembling separate from `values`. If they are decided to be the same, the rules of extraction apply to `ast --schema` as they are. The top-level "name" of a schema is used only for the "type" of `ast --schema`. Unless it is decided what the key is for, it cannot be known.

### Decision

When `--schema` is given to `kotowari-mds ast`, mds assembles, with the schema the document declares, the same values as `kotowari-mds values --format json`, and outputs them as JSON. If the schema declares a top-level "name", its value is placed under the "type" key of the root. When "name" is not declared, "type" is not placed, and the output is the same as `values --format json`.

"name" is accepted as the name of a type of string, and is used only for the "type" of `ast --schema`. It does not change the results of `check` and `values`.

### Confirmations

- C-schema-cli-3 For a schema that declares "name", the output of `ast --schema` is the values of `values --format json` with a root "type" key whose value is that of "name" added, and for a schema that does not declare "name", the two outputs are the same values and have no "type" key
- C-schema-cli-4 Even for a schema that declares "name", the output of `values` has no "type" key

## The exit code is 0 with no findings, 1 with findings, and 2 when the check cannot be performed

Status: implemented

### Problem

"The format is wrong" and "the check could not be performed" are different events for the caller. With the same exit code they cannot be told apart.

### Decision

The exit code is decided by applying the rows in order from the top.

| Order | Condition | Exit code |
|---|---|---|
| 1 | The check could not be performed (one of the [reasons for a stop](#there-are-five-reasons-for-a-stop) applied) | 2 |
| 2 | 1 does not apply, and there is at least one finding | 1 |
| 3 | Neither 1 nor 2 applies | 0 |

### Confirmations

- C-schema-cli-5 `check` of a document that satisfies the whole schema ends with exit code 0 and outputs nothing, a document with findings ends with 1, and a stop ends with 2

## The output format is chosen from text and json with --format

Status: implemented

### Problem

Since the primary user of mds is an LLM, a structured format is the basis. On the other hand, there are also situations where a person types it by hand. The reason why only `ast` accepts only `json` is not recorded.

### Decision

The output format is taken with `--format`, chosen from two: `text` for humans and `json` for machines. `ast` accepts only `json`, and when `--format text` is given, it stops with an argument error ([There are five reasons for a stop](#there-are-five-reasons-for-a-stop)).

### Confirmations

- C-schema-cli-6 `check` and `values` output results in two formats: text for humans, and JSON when `--format json` is given

## The text output of values indents by two spaces per nesting level and numbers array elements from 1

Status: implemented

### Problem

Even for output meant for humans, tests cannot be written unless the format is decided. The current behavior was made the rule as it is.

### Decision

In the `text` output of `values`, each level deeper of nesting is indented by two spaces, and array elements get a number starting from 1 and ".".

### Confirmations

- C-schema-cli-7 In the output of `values --format text`, the line of a nested key is indented two spaces deeper than the line of its parent key, array elements are listed with numbers starting from "1.", and an array inside an element is also numbered from "1." and indented one level deeper

## A finding holds the kind, severity, document path, line number, and detail

Status: implemented

### Problem

A finding must let a machine branch on its kind and let a person trace its location. The minimal set for that is decided.

### Decision

One finding holds the kind, severity, document path, line number, and detail. For a finding that has no line, the line number is omitted (how the line is decided is in [The line of a finding is the start line of the violating node, or for a missing node the start line of the node that contains it](#the-line-of-a-finding-is-the-start-line-of-the-violating-node-or-for-a-missing-node-the-start-line-of-the-node-that-contains-it)). In addition, the node name, the raw line, and the rule kind are added (the three decisions below).

The kinds of findings are classified into the following five by what they find.

| Class | What it finds |
|---|---|
| Title | The title is missing, there are two or more, or it does not match the regular expression |
| Missing | A required node is absent |
| Violation of form | A value does not match the regular expression or the allowed list, the depth of a heading does not match, the form of an item ID does not match, the order of field lines does not match, the header of a table does not match, or the language of a code block does not match |
| Cardinality | Below the lower bound of cardinality, or above the upper bound |
| Closed world | There are undeclared headings and lines |

The individual kinds of findings and the conditions under which each is raised are decided in the ["The skeleton of a document"](document-structure.md), ["Rules for the lines under a heading"](block-rules.md), ["Skeleton of the schema language"](schema-language.md#cardinality-is-declared-in-five-ways), and ["Closed world and open world"](closed-world.md) topics.

### Confirmations

- C-schema-cli-8 A finding of `check --format json` holds the kind, severity, document path, line number, and detail

## The line of a finding is the start line of the violating node, or for a missing node the start line of the node that contains it

Status: implemented

### Problem

If the line a finding points at were decided separately per kind of node, callers would have to read it differently per kind. It is natural for mds itself to hold line numbers, and since the line numbers are already computed when the document is parsed, there is no performance concern either.

### Decision

The line number of a finding is decided by one rule. If there is a violating node, it is the start line of that node. For a missing node, it is the start line of the node that contains it. When the containing node has no line (for example, a missing document title), the line number is omitted. When an element inside a node (for example, a data row of a table) violates, it is the line of that element.

### Rejected options

- Callers judge the line from the structure of extraction and discard mds's findings. The judgment would be split into two places.
- Accept that lines disappear. The lines of findings that users see would change.

### Confirmations

- C-schema-cli-9 The line number of a finding is the start line of the violating node (for example, the heading line of an undeclared section, a field line that does not match the form, or the second field line that exceeds the upper bound), for a missing node the start line of the node that contains it (the heading line of an item that lacks a required field line), and for a violation in a data row of a table that row
- C-schema-cli-10 A finding whose containing node has no line (for example, a missing title and a missing required section) has no line number

## A finding holds the name of the declared node as structure

Status: implemented

### Problem

The wording of a finding's detail is not defined anywhere in the specification. If callers read from the wording of the detail which field line or section a finding is about, it silently breaks when the wording is rephrased. mds holds the name of the declared node at the time it creates the finding.

### Decision

A finding holds, as structure, the name of the node the schema declared. Which findings hold a name is decided in [Findings of section and item rules that select by a list of names hold the first name of that list as the node name](#findings-of-section-and-item-rules-that-select-by-a-list-of-names-hold-the-first-name-of-that-list-as-the-node-name).

### Rejected options

- Fix the wording of the detail in the specification, and have callers read it. This would make English prose meant for people into a machine contract.
- Callers judge from the structure of extraction. The judgment would be split into two places.

### Confirmations

- C-schema-cli-11 Findings of field lines and sections hold the name the schema declared, and findings of titles and tables hold no node name

## Findings of section and item rules that select by a list of names hold the first name of that list as the node name

Status: implemented

### Problem

Only sections and field lines give findings a node name. Once items can be selected by name ([Section and item rules select headings by a list of names or by a regular expression](schema-language.md#section-and-item-rules-select-headings-by-a-list-of-names-or-by-a-regular-expression)), the cardinality findings of items hold which named item is lacking nowhere but in the English prose of the detail. For a rule that selects by a list of names, which name of the list a finding shows also needs to be decided.

### Decision

Findings of section and item rules that select by a list of names ("name") hold the first name of that list as the node name. When "name" is a single string, they hold that name. Findings of rules that select by regular expression ("pattern"), and of item rules with neither "name" nor "pattern", hold no node name. Findings of field lines hold the declared name, and findings of titles and tables hold no node name.

### Rejected options

- Previous decision: the node name is attached only to findings of sections and field lines (the second sentence of the Decision of [A finding holds the name of the declared node as structure](#a-finding-holds-the-name-of-the-declared-node-as-structure)). Since findings of items selected by name hold no structure saying which item, the name is attached to items selected by name too.

### Confirmations

- C-schema-cli-30 In a section with no item matching the item rule "name: [問題, Problem]", the `repeat_min_not_met` finding holds "問題" as the node name (the same in the output of `check --format json`)
- C-schema-cli-31 Findings of a section rule that selects by "pattern" (for example `repeat_max_exceeded` over the upper bound) hold no node name

## A finding holds the text of the line it points at as it is

Status: implemented

### Problem

When callers show a finding to a person, they want to output the text of the line they read as it is (including indentation and trailing whitespace, without reassembling it). The wording of the detail has only the name of the heading, not the text of the line.

### Decision

A finding that has a line number holds the text of the line it points at, as it is, as the raw line. For a finding that has no line number, it is omitted. mds returns "what and where" as structure, and callers assemble the wording.

### Rejected options

- Callers read the lines of the document. The division of roles, in which callers do not read Markdown and leave reading it to mds, would become a formality.

### Confirmations

- C-schema-cli-12 The raw line of a finding that has a line number does not differ by a single character from that line of the document, and a finding that has no line number has no raw line

## Cardinality findings hold the rule kind of the counted node

Status: implemented

### Problem

Findings other than those of sections and field lines have no node name attached. For the findings of the lower and upper bounds of cardinality, there is no clue to which node was counted, either in the kind or in a name. Callers should not be made to read the English prose of the detail.

### Decision

The findings of the lower bound (`repeat_min_not_met`) and upper bound (`repeat_max_exceeded`) of cardinality hold the rule kind of the counted node. The kind attached to findings of undeclared lines is decided in [A finding on an undeclared line carries the kind the line was read as](closed-world.md#a-finding-on-an-undeclared-line-carries-the-kind-the-line-was-read-as). It is not attached to other findings.

### Confirmations

- C-schema-cli-13 For a schema that declares a lower bound of cardinality on the statements of a section or an item, the `repeat_min_not_met` finding of a section or item with no statements holds statement as its rule kind (the same in the output of `check --format json`)

## When a check cannot be performed, it stops without outputting partial results

Status: implemented

### Problem

If inputs that cannot be read, are broken, or do not match the form were silently skipped, documents that were not checked would look as if they passed. Since the schema cannot be resolved, no check result can be output, and outputting partial results would look as if they passed.

The document side can write any file in "$schema". When it points at a file that cannot be read as a schema, the parser's explanation quotes the contents of that file, so passing the explanation through as it is would put fragments of any readable file on standard error. The reason this rule to prevent that was decided is not recorded.

### Decision

Inputs that cannot be read, are broken, or do not match the form are not skipped silently. When the schema cannot be resolved and when the frontmatter is broken, each of `check`, `values`, and `ast --schema` stops without outputting partial results (exit code 2). Which inputs cause a stop is decided in the ["Schema resolution"](schema-resolution.md#a-document-whose-frontmatter-is-broken-or-that-does-not-point-at-a-schema-stops-without-checking) and ["Skeleton of the schema language"](schema-language.md#a-schema-with-an-unknown-key-or-a-value-that-does-not-fit-its-shape-stops-without-checking) topics and in [There are five reasons for a stop](#there-are-five-reasons-for-a-stop) below.

When it stops, it outputs to standard error only one line, the name of the reason for the stop with an explanation, and outputs none of the findings collected until then. When a reason for a stop applies in the middle of checking a directory, the whole check also stops.

Even when the explanation of a stop spans several lines, only the first line is output, and the contents of the file that "$schema" pointed at, from the second line on, are not passed to standard error.

### Confirmations

- C-schema-cli-14 For a document without "$schema", a document whose referenced schema does not exist, and a document with broken frontmatter, each of `check`, `values`, and `ast --schema` stops with exit code 2
- C-schema-cli-15 When it stops, it outputs only one line to standard error and no finding at all to standard output
- C-schema-cli-16 In checking a directory, when one document hits a reason for a stop, the whole check stops with exit code 2, and the findings of the other documents are not output either
- C-schema-cli-29 When it stops because "$schema" points at a multi-line file that cannot be read as a schema, standard error is only one line and does not include the contents of that file from the second line on

## There are five reasons for a stop

Status: implemented

### Problem

How the reasons for a stop are divided and named is decided. The reason for deciding the conditions assigned to each reason is not recorded.

### Decision

The reasons for a stop are the following five. The one line of a stop sets the name of this reason and the explanation side by side.

| Reason | When |
|---|---|
| `schema_not_found` (the schema is not found) | The referenced schema does not exist. Fetching a schema at a URL failed (including when the response exceeds 4MiB and when the whole fetch exceeds 10 seconds. The fetching rules are decided in [A URL schema that cannot be fetched, or whose fetch exceeds a limit, stops](schema-resolution.md#a-url-schema-that-cannot-be-fetched-or-whose-fetch-exceeds-a-limit-stops)) |
| `schema_invalid` (the schema does not match the form) | [Placement paths collide](#a-schema-whose-placement-paths-collide-stops-without-reading-documents), [a value is placed in the root "type" in a schema that declares "name"](#in-a-schema-that-declares-name-placing-a-value-in-the-root-type-stops), [the same name is declared twice](#declaring-the-same-name-in-the-same-place-or-the-same-column-name-in-a-table-header-twice-stops), [a placement path has an empty name](#a-placement-path-with-an-empty-dot-separated-name-stops). A value that is neither "paragraph" nor "line" is written in "reading" ([Paragraph is the default reading mode, and the schema can choose reading by line](schema-language.md#paragraph-is-the-default-reading-mode-and-the-schema-can-choose-reading-by-line)). "select" is written in a table rule without "header", or the value of "select" is not "first" ([A table rule can choose only the first table whose header matches, with "select: first"](block-rules.md#a-table-rule-can-choose-only-the-first-table-whose-header-matches-with-select-first)) |
| `frontmatter_invalid` (the frontmatter is broken) | The frontmatter is broken YAML or is not a YAML mapping. The value of "$schema" is empty or only whitespace. The value of "$schema" is not a string |
| `unreadable_file` (the document cannot be read) | The document's file cannot be read |
| `argument_error` (argument error) | A `--format` value that is not accepted, an unknown flag, [the same option passed twice](#passing-the-same-option-twice-stops-with-an-argument-error), `--format text` given to `ast` |

### Confirmations

- C-schema-cli-17 For an input that hits each condition in the table, it stops with exit code 2 and the name of that reason appears on standard error (`schema_not_found` for a schema whose reference does not exist and for a URL that cannot be fetched, `schema_invalid` for "select" without "header", `frontmatter_invalid` for broken frontmatter and an empty "$schema", `unreadable_file` for a document that cannot be read, and `argument_error` for an unknown flag and `ast --format text`)

## A schema whose placement paths collide stops without reading documents

Status: implemented

### Problem

When placement paths that place extracted values overlap within the same place, values are silently overwritten and lost. This judgment can be made only by comparing the declarations of the schema with each other, so there is no need to look at documents. On the other hand, dot-separated nesting is a legitimate way of writing, and judging by the first name alone would reject it by mistake. Only pairs that overlap as parent and child are real collisions.

### Decision

A schema whose placement paths collide within the same place is a `schema_invalid` stop at the time the schema is read, and the values of documents are not output. Identical paths, and a path that is at a level before the other ("a" and "a.b"), are collisions. Paths that are the same partway and then branch ("a.b" and "a.c") are not collisions.

The judgment is made both inside an element object and outside element objects (for example, directly under a section). Placing extracted values nested along placement paths is decided in [Values are placed where the placement path points, nested by the dot-separated names](extraction.md#values-are-placed-where-the-placement-path-points-nested-by-the-dot-separated-names), and the "value" and "of" keys of an element object in [An extraction is declared as one nested mapping holding path, value, and of](extraction.md#an-extraction-is-declared-as-one-nested-mapping-holding-path-value-and-of). Inside an element object, the inner placement paths, the "value" key of the value of the element, and the "of" keys of the derived values are counted as paths of the same place.

### Confirmations

- C-schema-cli-18 For a schema in which an inner placement path overlaps the "value" or "of" key inside an element object, or a schema that declares the same path, or "a" and "a.b", directly under two sections, `values` stops with `schema_invalid` and does not output the values of documents
- C-schema-cli-19 Placement paths that branch, such as "a.b" and "a.c", do not stop either inside or outside element objects

## In a schema that declares name, placing a value in the root type stops

Status: implemented

### Problem

Because the typed extraction (`ast --schema`) places "name" under the "type" key of the root, placing an extracted value under the same key makes "name" silently disappear. Stopping it in the same form as a collision of placement paths avoids changing the output shape.

### Decision

In a schema that declares "name", a placement path placed at the root of the document's values that is "type" or starts with "type." is a `schema_invalid` stop. The explanation of the stop says that it collides with the key under which the typed extraction places "name". In a schema that does not declare "name", values may be placed in "type".

### Rejected options

- Change the output shape, and separate "name" into a key apart from the extracted values (placing the extracted values one level lower, for example). The side that reads the output of `ast --schema` would break.
- Forbid "type" even in schemas that do not declare "name". Since "type" does not appear in `values`, this would stop even schemas that do not collide.

### Confirmations

- C-schema-cli-20 In a schema that declares "name", making the placement path of the title "type" or "type.title" makes `values` stop with `schema_invalid`, and the same schema without "name" declared does not stop and places the title's value in "type"

## Declaring the same name in the same place, or the same column name in a table header, twice stops

Status: implemented

### Problem

When a section or field line with the same name is declared twice in the same place, validation uses only the first declaration, and extraction uses both declarations. The regular expression of the second declaration silently has no effect, and values that have not passed validation go into extraction. When the same column name is written twice in a table header, the value of the later column silently overwrites the value of the earlier column in extraction, and the value of the earlier column is lost. Making it impossible to write lets the mistake be noticed better than deciding which declaration takes effect.

### Decision

A schema that declares a section with the same name, or a field line with the same name, twice in the same place is a `schema_invalid` stop. A schema that writes the same column name twice in a table header is also a `schema_invalid` stop.

### Rejected options

- Ignore the second and later declarations. Written rules would silently have no effect.
- Make the current behavior of a table, in which a later column overwrites an earlier one, the rule. Values would silently disappear.

### Confirmations

- C-schema-cli-21 For a schema that declares two sections with the same name, a schema that declares two field lines with the same name in the preamble, and a schema that writes the same column name twice in a table header, `check` stops with `schema_invalid`

## A placement path with an empty dot-separated name stops

Status: implemented

### Problem

A placement path that contains an empty name makes a JSON key of the empty string, and has no meaning as a dot-separated name. A slip in writing should be noticed right away.

### Decision

None of the dot-separated names of a placement path may be empty. A schema that writes a placement path containing an empty name (the empty string, "a..b", ".a", "a.") is a `schema_invalid` stop.

### Confirmations

- C-schema-cli-22 A schema that writes the empty string, "a..b", ".a", or "a." as a placement path stops with `schema_invalid`

## Passing the same option twice stops with an argument error

Status: implemented

### Problem

When the same option is passed twice, noticing the slip is better than deciding which value to take. The kotowari CLI also makes the second occurrence of the same option an argument error.

### Decision

When the same option is passed twice to `kotowari-mds`, it is an `argument_error` stop.

### Confirmations

- C-schema-cli-23 Passing `--format` twice to `kotowari-mds check` stops with exit code 2

## The wording of a stop's explanation is not promised; it includes the name of the problem field and the expected type or accepted words

Status: implemented

### Problem

If the wording of a stop's explanation were a promise, tests would break every time the phrasing is touched up. If only the content is decided, tests can check that content.

### Decision

The wording itself of a stop's explanation is not promised. When it stops because the type or word of a value in the schema or the frontmatter is wrong, the explanation includes the name of the field that had the problem and the expected type or the list of accepted words.

### Rejected options

- Make the current wording a promise as it is.

### Confirmations

- C-schema-cli-24 For a document whose "$schema" value is a number, a schema whose "reading" value is "foo", a schema in which the type of the value of "open" or "extract" is wrong, and a schema that writes an "of" word the node does not accept, the explanation of the stop includes the name of that field and the expected type or accepted words ("paragraph" and "line" for "reading")

## A directory check checks only documents with $schema, and does not follow directories starting with . or symbolic links

Status: implemented

### Problem

When checking a directory as the target, which documents are targeted is decided. The reasons for selecting only documents that have "$schema" and for not following directories starting with "." are not recorded.

Following symbolic links creates paths that read the same document twice and paths that go outside the place specified as the target of the check.

### Decision

When the target of `check` is a directory, only the documents under it that have "$schema" in their frontmatter are collected and checked. Documents without "$schema" are out of scope. A document that has "$schema" but whose value is empty or only whitespace is not taken as a declaration, and the whole check stops as broken frontmatter.

Directories whose names start with "." are not followed. ".kotowari/", which holds schemas and the cache, is also not followed because of this.

Symbolic links are not followed. Symbolic links to documents under the directory are not checked either.

### Confirmations

- C-schema-cli-25 `check` of a directory outputs only the findings of documents that have "$schema", and outputs neither findings nor a stop for documents without "$schema"
- C-schema-cli-26 `check` of a directory does not check documents under directories whose names start with "."
- C-schema-cli-28 `check` of a directory does not follow a symbolic link to a document with violations outside the place, and does not output that document's findings

## values and ast --schema assemble values regardless of whether the check passes

Status: implemented

### Problem

The reason is not recorded.

### Decision

`values` and `ast --schema` assemble values from the document according to the schema's extraction rules, regardless of whether the check passes. They do not change the exit code and output because of violations in the document. However, when the schema cannot be resolved, they stop ([When a check cannot be performed, it stops without outputting partial results](#when-a-check-cannot-be-performed-it-stops-without-outputting-partial-results)). `--open` applies only to `check`, and has no effect on `values` and `ast`.

Which entry points support this behavior on the library side is decided in [The CLI's values and ast --schema output values as before even for a document with violations](library-extraction.md#the-clis-values-and-ast---schema-output-values-as-before-even-for-a-document-with-violations).

### Confirmations

- C-schema-cli-27 Even when a readable document has a violation (for example, a second title), `values` outputs the values that can be obtained and ends with exit code 0
