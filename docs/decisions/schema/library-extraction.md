# Validation and value extraction in the library

English | [日本語](library-extraction.ja.md)

This covers the reusable schema and document types, the distinction between validated values and partial extraction, and the relation to extraction in the CLI. The list of published entry points is decided in [the "Library entry points"](library.md) topic, in [A depending crate can go from reading the reference to extraction with the decided entry points alone](library.md#a-depending-crate-can-go-from-reading-the-reference-to-extraction-with-the-decided-entry-points-alone).

## A schema is returned only after it passes semantic validation, and a parsed schema can be reused

Status: implemented

### Problem

Validating and extracting the same content should not require parsing the schema and the document again every time.

That the entry point that parses a schema returns either a schema or a violation of form was ratified by the user after checking the implementation. The reason for it is not recorded.

If a `Schema` could be assembled from outside, have its fields rewritten, or be deserialized directly without validation, validation and extraction would run with a schema that has not passed semantic validation. This is so that the procedure leaves no way to bypass validation.

### Decision

`Schema::parse` parses the YAML of a schema, and if it violates semantic validation, returns a failure and does not return a `Schema`.

The `Schema` returned by `Schema::parse` and the `Document` returned by `Document::parse` can be passed to validation and extraction any number of times, and are not required to be parsed again each time.

`Schema` is an immutable type. From outside the crate, it cannot be assembled by specifying fields, its fields cannot be rewritten, and it cannot be deserialized directly without going through `Schema::parse`.

### Confirmations

- C-schema-libext-1 For a schema YAML that violates semantic validation, `Schema::parse` returns a failure and does not return a `Schema`
- C-schema-libext-2 `extract_validated` can be called repeatedly with one parsed `Schema` and `Document`, and returns validated values each time
- C-schema-libext-7 (by reading) The compile_fail doctests of `Schema` in `crates/kotowari-markdown-schema/src/schema.rs` check that all three of the following fail to compile: assembling it from outside the crate by specifying fields, rewriting its fields, and deserializing it directly with `serde_saphyr::from_str`

## The normal entry point for value extraction returns values only from a document with no violation

Status: implemented

### Problem

Users should not confuse validated values with values partially obtained from a document with violations.

### Decision

`extract_validated` validates the document, and if there is no violation, returns the validated values `ValidatedValues`. If there is a violation, it returns the findings and does not return `ValidatedValues`.

When the values obtainable from a document with violations are wanted, the separate entry point `extract_partial` is called explicitly. `extract_partial` returns a `PartialExtraction` that holds the findings and the values that could be obtained.

Both hand out values as `serde_json::Value`.

### Confirmations

- C-schema-libext-3 For a document with violations, `extract_validated` returns the findings and does not return `ValidatedValues`, and `extract_partial` returns the same findings and the values that could be obtained

## Validation in the library does not cancel the schema's open

Status: implemented

### Problem

Validation is closed world by default. Since the purpose is to fix the format, silently passing what is not declared would defeat the purpose. A relaxing setting works only in the relaxing direction (decided in [The open world allows only undeclared structures and what is inside them](closed-world.md#the-open-world-allows-only-undeclared-structures-and-what-is-inside-them)). The settings for validation in the library should also not misread the schema's open setting.

### Decision

The default of `ValidationOptions` follows the schema's open. When the user specifies relaxation, validation uses the logical OR of it and the schema's open.

| The schema's open | The relaxing setting of `ValidationOptions` | Validation |
|---|---|---|
| false | false (default) | closed world |
| false | true | open world |
| true | false (default) | open world |
| true | true | open world |

### Confirmations

- C-schema-libext-4 If either the schema's open or the relaxing setting of `ValidationOptions` is true, `validate`, `extract_partial`, and `extract_validated` do not make an undeclared heading an error, and if both are false, they make it an error

## The CLI's values and ast --schema output values as before even for a document with violations

Status: implemented

### Problem

Even when the library's entry points are rebuilt, the existing CLI's contract for commands, arguments, exit codes, and output is not broken. This is so as not to break the input and processing of CLI users.

### Decision

mds's `values` and `ast --schema` use the values of partial extraction. That they output values even for a document with violations, and do not change the exit code and output because of violations in the document, is decided on the CLI side in [values and ast --schema assemble values regardless of whether the check passes](cli.md#values-and-ast---schema-assemble-values-regardless-of-whether-the-check-passes).

For that purpose, the library provides the JSON of the plain syntax tree through `ast_json` and the JSON of the typed extraction through `extract_typed_partial`. The shape of each JSON is decided in [ast outputs the plain syntax tree of a document as JSON, separately from validation](extraction.md#ast-outputs-the-plain-syntax-tree-of-a-document-as-json-separately-from-validation) and [ast --schema adds the schema's name as type to the same values as values --format json](cli.md#ast---schema-adds-the-schemas-name-as-type-to-the-same-values-as-values---format-json).

### Confirmations

- C-schema-libext-5 Even for a document with violations, `extract_typed_partial` returns the JSON of the typed extraction, holding the values that could be obtained and the schema's name, together with the findings
