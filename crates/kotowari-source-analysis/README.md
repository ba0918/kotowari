# kotowari-source-analysis

`Analyzer` discovers test markers and named surfaces from `SourceText` using bundled or caller-provided rules.
It returns the original source, discovered facts and parse diagnostics.
It does not acquire files or judge IR coverage.
Pass its core-owned results into `ReadInputs` or `CheckInputs` without conversion.

Run `cargo run --example analysis -p kotowari-source-analysis`.
