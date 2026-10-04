# kotowari-overview

Check overview data in memory against a kotowari specification that `kotowari-core` has already read, and build the render input for `kotowari-markdown-view`.
`inspect` takes a `ReadModel`, the overview data texts and the table of contents text as `SourceText`, and returns an `Overview` with the findings, the number of files and guide marks read, the reference table, the stale sections and the render input.
The table of contents is checked against its embedded JSON Schema and against the overview data names, and is passed to the view as written.
`Overview::pages` renders the pages when there is no error, and `Overview::into_group` turns the findings and numbers into the `overview` finding group that `kotowari-core` adds to a check.
It reads and writes no files, opens no network connection and reads no environment variables; the `kotowari` library acquires the files and writes the pages.

Run `cargo run --example overview -p kotowari-overview`.
