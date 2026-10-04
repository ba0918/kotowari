# kotowari-markdown-schema-io

`SchemaLoader` reads Markdown and its declared schema using explicit absolute start and cache bases.
`load` returns a reusable schema/document pair. `check` accepts a file or directory.
Validated extraction rejects completed validation findings; partial extraction retains values alongside findings.
I/O and invalid schema failures return typed errors.

The default-disabled `tokio` feature exposes `AsyncSchemaLoader` with the same results and bounded blocking-work scheduling.
Clones share the configured limit; cancellation cannot interrupt work already submitted.

Run `cargo run --example loader -p kotowari-markdown-schema-io --features tokio`.
