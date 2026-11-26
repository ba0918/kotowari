# Validation and value extraction in the library

English | [日本語](library-extraction.ja.md)

Covers the reusable schema and document types, the distinction between validated values and partial extraction, and the relation to extraction in the CLI.

## Requirements

### REQ-schema-068: A schema that cannot bypass validation

- kind: invariant
- source: docs/decision/records/2026-10-03-public-crate-api.md#A21, docs/decision/records/2026-10-03-public-crate-api.md#A42
- verification: unit

"Schema::parse" returns an immutable "Schema" that has passed semantic validation. Validation cannot be bypassed by changing fields from outside or by deserializing directly without validation. A parsed "Schema" and "Document" can be reused for validation and extraction.

### REQ-schema-069: Validated values and partial extraction

- kind: ubiquitous
- source: docs/decision/records/2026-10-03-public-crate-api.md#A20, docs/decision/records/2026-10-03-public-crate-api.md#A43
- verification: unit

"extract_validated" validates, and returns "ValidatedValues" if the document has no violation; if it has a violation, it returns those findings and does not return validated values. "extract_partial" returns a "PartialExtraction" that holds the findings and the JSON values that could be obtained. Values are handled as "serde_json::Value".

### REQ-schema-070: Respecting the schema's open

- kind: ubiquitous
- source: docs/decision/records/2026-10-03-public-crate-api.md#A42
- verification: unit

The default of "ValidationOptions" respects the "open" of the schema. When the user specifies relaxation, the logical OR of it and the schema's value is used for validation.

### REQ-schema-071: Keeping the meaning of CLI extraction

- kind: invariant
- source: docs/decision/records/2026-10-03-public-crate-api.md#A9, docs/decision/records/2026-10-03-public-crate-api.md#A43
- verification: unit

The CLI's "values" and "ast --schema" use the values of partial extraction, and violations in the document do not change their existing exit codes and output. "ast_json" and "extract_typed_partial" provide the existing plain AST JSON and the typed extraction JSON.

## Examples

```gherkin
@id=EX-schema-085 @about=REQ-schema-068,REQ-schema-069 @source=docs/decision/records/2026-10-03-public-crate-api.md#A21,docs/decision/records/2026-10-03-public-crate-api.md#A42,docs/decision/records/2026-10-03-public-crate-api.md#A43
Scenario: Validating and extracting repeatedly from parsed values
  Given a Schema that has passed semantic validation and a conforming Document
  When validation and extract_validated are called with the same parsed values
  Then ValidatedValues is returned without requiring a re-parse

@id=EX-schema-086 @about=REQ-schema-068 @source=docs/decision/records/2026-10-03-public-crate-api.md#A42
Scenario: An invalid schema is not turned into the validated type
  Given a schema YAML that violates semantic validation
  When Schema::parse is called
  Then it returns a failure and does not return a Schema

@id=EX-schema-087 @about=REQ-schema-069 @source=docs/decision/records/2026-10-03-public-crate-api.md#A20,docs/decision/records/2026-10-03-public-crate-api.md#A43
Scenario: Partial extraction is not treated as validated
  Given a document with violations and extractable values
  When extract_validated and extract_partial are called
  Then the former returns the findings and does not return ValidatedValues
  And the latter returns the findings and the values that could be obtained

@id=EX-schema-088 @about=REQ-schema-070 @source=docs/decision/records/2026-10-03-public-crate-api.md#A42
Scenario: The default options do not cancel the schema's relaxation
  Given the schema's open is true
  When validating with the default ValidationOptions
  Then it validates with open as true

@id=EX-schema-089 @about=REQ-schema-071 @source=docs/decision/records/2026-10-03-public-crate-api.md#A9,docs/decision/records/2026-10-03-public-crate-api.md#A43
Scenario: A violation in the document does not stop the existing values command
  Given a readable document with validation violations and extractable values
  When the values command is run
  Then it outputs the values as the existing contract specifies and does not change the exit code because of the document's violations alone
```
