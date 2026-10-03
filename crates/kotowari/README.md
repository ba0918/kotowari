# kotowari

`Project` reads a repository from an explicit absolute start path without changing the current directory.
Its typed operations are `check`, `list`, `query`, `status`, `plan`, `mutants` and `changes`.
Use `read` or `inspect` to retain loaded values; those values do not refresh when files change.
Call the project again to read current files.
Completed findings belong to reports, while execution failures return `Error` with `ErrorKind`.

The default-disabled `tokio` feature provides `AsyncProject` and `AsyncOptions`.
It requires the caller's runtime and defaults to one blocking operation per object.
Clones share that limit. Dropping a waiting future does not interrupt started work or release its slot early.

Run `cargo run --example project -p kotowari --features tokio`.
