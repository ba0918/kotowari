# Rust API for using a project

English | [日本語](library-api.ja.md)

Covers the entry points through which an external Rust application calls the seven existing operations, their results and failures, and CLI compatibility. Holding inputs is covered by library-inputs.md, and Tokio support by library-async.md.

## Requirements

### REQ-core-310: Entry points for project operations

- kind: algorithm
- source: docs/decision/records/2026-10-03-public-crate-api.md#A10, docs/decision/records/2026-10-03-public-crate-api.md#A36, docs/decision/records/2026-10-03-public-crate-api.md#A41
- definition: TBL-core-041
- verification: unit

### REQ-core-311: The calling start location is given explicitly

- kind: ubiquitous
- source: docs/decision/records/2026-10-03-public-crate-api.md#A36, docs/decision/records/2026-10-03-public-crate-api.md#A52
- verification: unit

"Project::new" receives, through "ProjectOptions", the calling start location as an absolute path and an optional configuration file to use. It does not change the current directory of the process, and performs the existing configuration search and resolution of relative paths. The CLI obtains the current location and passes it. The contract on input paths follows REQ-core-323.

### REQ-core-312: Findings and execution failures are distinguished

- kind: ubiquitous
- source: docs/decision/records/2026-10-03-public-crate-api.md#A7, docs/decision/records/2026-10-03-public-crate-api.md#A8, docs/decision/records/2026-10-03-public-crate-api.md#A40
- verification: unit

A completed check returns a result holding the findings on the documents, and when it cannot be executed it returns the "Err" of "Result". The kind of a failure can be told apart, and failures implement "std::error::Error" and "Display". Missing input, invalid configuration or input format, an unknown ID in a query, read or Git failures, and internal inability to handle are distinguished. The result provides, read-only, the kind, severity and path of each finding and its line when there is one, and does not display anything.

### REQ-core-313: The CLI contract is kept

- kind: invariant
- source: docs/decision/records/2026-10-03-public-crate-api.md#A9, docs/decision/records/2026-10-03-public-crate-api.md#A14, docs/decision/records/2026-10-03-public-crate-api.md#A33, docs/decision/records/2026-10-03-public-crate-api.md#A40, docs/decision/records/2026-10-03-public-crate-api.md#A43
- verification: unit

The restructuring keeps the meaning of the commands, arguments, exit codes, output, configuration, IR, schemas and check results of both binaries as currently implemented. The CLI maps the library's results onto the existing JSON/text output and stop reasons. Changing the name of the package installed from source does not change the name of the command that is run. The overview, not yet implemented, is added by a separate plan.

## Decision tables

### TBL-core-041: Operations of Project

- source: docs/decision/records/2026-10-03-public-crate-api.md#A10, docs/decision/records/2026-10-03-public-crate-api.md#A36, docs/decision/records/2026-10-03-public-crate-api.md#A41, docs/decision/records/2026-10-03-public-crate-api.md#A55, docs/decision/records/2026-10-04-overview-on-public-api.md#A3, docs/decision/records/2026-10-04-overview-on-public-api.md#A7

| Method | Input and operation | Result |
|---|---|---|
| check | Checks the project according to the configuration. Includes the findings on `overview data` and the "overview" group | Findings and counts |
| list | Lists the items that could be read | A typed list |
| query | Queries the given IDs | A typed result including bodies and reverse references |
| status | Checks the same scope as check | Counts and the completion state |
| plan | Checks the given plan, without requiring the IR to be read | Findings and counts |
| mutants | Checks the given mutation outcomes and equivalence information, without requiring the IR to be read | Findings and mutation counts |
| changes | Checks the Git comparison information and the change records, given the comparison base, the targets and the stage | A typed change conformance result |
| overview_prepare | Checks and renders `overview data`, without writing files | The rendering result before writing. It has an operation to write |
| overview_build | Following overview_prepare, writes its result under ".kotowari/cache/overview/" (REQ-core-293) | The lists of written and removed files, and the number of files not written |

## Examples

```gherkin
@id=EX-core-483 @about=REQ-core-310,REQ-core-311,REQ-core-312 @source=docs/decision/records/2026-10-03-public-crate-api.md#A7,docs/decision/records/2026-10-03-public-crate-api.md#A10,docs/decision/records/2026-10-03-public-crate-api.md#A36,docs/decision/records/2026-10-03-public-crate-api.md#A40
Scenario: Violations in the documents are received as a check result
  Given a project with an explicit calling start location has IR that raises findings
  When check of Project is called
  Then a completed check result is returned in which the findings can be told apart by type
  And it writes no result to standard output and does not change the current directory

@id=EX-core-484 @about=REQ-core-310,REQ-core-312 @source=docs/decision/records/2026-10-03-public-crate-api.md#A36,docs/decision/records/2026-10-03-public-crate-api.md#A40
Scenario: Unreadable input is not turned into a result without findings
  Given a file needed for the check cannot be read
  When check of Project is called
  Then an Err in which the read failure can be told apart is returned

@id=EX-core-485 @about=REQ-core-313 @source=docs/decision/records/2026-10-03-public-crate-api.md#A9,docs/decision/records/2026-10-03-public-crate-api.md#A40
Scenario: The CLI output is independent of changes to internal Rust types
  Given the existing CLI contract tests define inputs and the expected JSON, text and exit codes
  When the restructured CLI is run with the same inputs
  Then the output and exit codes match the existing expectations

@id=EX-core-486 @about=REQ-core-310 @source=docs/decision/records/2026-10-03-public-crate-api.md#A41
Scenario: Checks of plans and mutation outcomes are available without IR
  Given the inputs needed for a plan or mutation outcomes exist, and there is no IR location
  When the corresponding plan or mutants operation is called
  Then it returns a result without requiring the IR to be read
```
