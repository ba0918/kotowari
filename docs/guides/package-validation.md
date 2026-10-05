# Validating distribution packages

English | [日本語](package-validation.ja.md)

This is the procedure for validating the real archives and independent consumers without publishing the crates.

## Local validation

<!-- @kotowari[REQ-core-309:96174f95, REQ-schema-051:9546b439] -->

Use Cargo 1.98 or later and Python 3.11 or later (which provides safe tar extraction), and run it in a clean working tree with your changes committed.

```sh
python3 scripts/check-packages.py --output "$ABSOLUTE_NEW_SCRATCH_DIRECTORY"
```

The output location is a new absolute path outside the repository. An existing directory is not overwritten.
The helper vendors the locked external dependencies, then builds 9 archives through offline Cargo workspace staging without source replacement.
It then extracts the original archives safely and builds and tests each package from an independent, checksummed directory source.
It also checks Tokio enabled and disabled, calls from a separate crate, the identity of re-exported types, and the compatibility tests of both CLIs.
It never rewrites the archives or the shipped manifests, and uses no path dependencies or patches pointing at the working tree.
The output keeps the commands and exit codes, the manifests, locks and dependency graphs, the archive hashes, and the consumers' run results. PASS in `result.json` means the local validation succeeded, not that anything has been published.

## Future publishing order

<!-- @kotowari[TBL-core-040:4f46993e] -->

Whoever publishes runs this on the same change that has passed validation and independent review, after bumping the version of each product.
Crates that others depend on are published first.

1. `kotowari-markdown-schema` and `kotowari-markdown-view`
2. `kotowari-core` and `kotowari-markdown-schema-io`
3. `kotowari-source-analysis`, `kotowari-mds`, `kotowari-overview`
4. `kotowari`
5. `kotowari-cli`

Independent crates in the same stage can go in any order. This is a procedure for a future publish by a person; the validation helper does not upload anything.
The version of the kotowari family is set in the root `Cargo.toml`, and that of the schema family in `crates/kotowari-markdown-schema/Cargo.toml`. `kotowari-overview` follows the kotowari family's version, and `kotowari-markdown-view` the schema family's.
When the schema family's version changes, also update the dependency declarations coming in from core and overview. This is not a procedure for changing the tags, binary names or distribution archive names of the two existing products.
