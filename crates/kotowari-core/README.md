# kotowari-core

Parse and inspect specification text in memory without reading files or running source queries.
`SourceText` accepts relative logical paths, normalizes separators and dot components, and retains the original text.
`ReadModel::build` takes explicit IR, decision and source-analysis groups.
`None` means not supplied; `Some(vec![])` means supplied and empty.
Enabled groups are required, while disabled groups are ignored.
`Inspection::build` adds the check-only groups and provides read-only check/status reports.

Run `cargo run --example memory -p kotowari-core` for an independent memory consumer.
See the [API guide](https://github.com/ba0918/kotowari/blob/main/docs/guides/public-crate-api.md).
