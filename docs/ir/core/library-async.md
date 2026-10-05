# Optional Tokio support

English | [日本語](library-async.ja.md)

Covers the entry points for awaiting the high-level operations of kotowari and Markdown schema I/O from Tokio, the number of concurrent executions, and the behaviour when the caller stops waiting.

## Requirements

### REQ-core-318: Asynchronous entry points that await synchronous operations

- kind: ubiquitous
- source: docs/decision/records/2026-10-03-public-crate-api.md#A22, docs/decision/records/2026-10-03-public-crate-api.md#A27, docs/decision/records/2026-10-03-public-crate-api.md#A45
- verification: unit

"kotowari" and "kotowari-markdown-schema-io" provide "AsyncProject" and "AsyncSchemaLoader" respectively, behind a "tokio" feature that is disabled by default. They cover the synchronous high-level operations and the loading operations, hand the synchronous processing to "spawn_blocking", and share the values, findings and ordinary failure classification for the same input. The CLI uses the synchronous API.

### REQ-core-319: Sharing execution slots

- kind: ubiquitous
- source: docs/decision/records/2026-10-03-public-crate-api.md#A23, docs/decision/records/2026-10-03-public-crate-api.md#A30, docs/decision/records/2026-10-03-public-crate-api.md#A31, docs/decision/records/2026-10-03-public-crate-api.md#A45
- verification: unit

"AsyncOptions" sets the default number of concurrent executions to 1, and receives a changed value as "NonZeroUsize". Each asynchronous execution object has its own limit, and cloned objects share execution slots with each other.

### REQ-core-320: Cancellation and the lifetime of execution slots

- kind: ubiquitous
- source: docs/decision/records/2026-10-03-public-crate-api.md#A24, docs/decision/records/2026-10-03-public-crate-api.md#A32
- verification: unit

Processing cancelled while waiting for an execution slot is not submitted. Once submitted, it keeps its execution slot until the processing completes, even if the caller stops waiting. Stopping Git, HTTP, computation or cache writes that have already started is not guaranteed.

### REQ-core-321: Boundaries of runtime and threads

- kind: ubiquitous
- source: docs/decision/records/2026-10-03-public-crate-api.md#A45
- verification: unit

The asynchronous API runs on the user's Tokio runtime and does not create an internal runtime. The absence of a runtime and a task execution failure are returned as typed failures. The inputs passed, the results returned and the public Futures are provided as types that can be moved between threads.

## Examples

```gherkin
@id=EX-core-492 @about=REQ-core-318,REQ-core-321 @source=docs/decision/records/2026-10-03-public-crate-api.md#A22,docs/decision/records/2026-10-03-public-crate-api.md#A45
Scenario: Synchronous and asynchronous calls give the same findings
  Given there are a synchronous entry point and an asynchronous entry point on a Tokio runtime that read the same input
  When the same check is executed
  Then they return the same values and findings
  And the Future and the result of the asynchronous call can be passed to another thread

@id=EX-core-493 @about=REQ-core-319,REQ-core-320 @source=docs/decision/records/2026-10-03-public-crate-api.md#A30,docs/decision/records/2026-10-03-public-crate-api.md#A31,docs/decision/records/2026-10-03-public-crate-api.md#A32
Scenario: Stopping waiting for started processing does not free the execution slot
  Given there are an execution object with the default limit of 1 and its clone
  And the first processing has started and has not completed
  When the caller stops waiting for the first processing and requests other processing from the clone
  Then the other processing is not submitted until the first processing completes

@id=EX-core-494 @about=REQ-core-320 @source=docs/decision/records/2026-10-03-public-crate-api.md#A32
Scenario: Processing that stopped waiting for a slot is not submitted later
  Given processing is waiting for an execution slot
  When the call is cancelled before it obtains a slot
  Then that processing is not submitted even when a slot becomes free

@id=EX-core-495 @about=REQ-core-321 @source=docs/decision/records/2026-10-03-public-crate-api.md#A45
Scenario: Awaiting outside a Tokio runtime
  Given the caller polls the asynchronous API outside a Tokio runtime
  When execution of an operation is requested
  Then the absence of a runtime is returned as a typed failure, and no internal runtime is created
```
