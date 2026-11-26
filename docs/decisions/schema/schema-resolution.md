# Schema resolution

English | [日本語](schema-resolution.ja.md)

Covers how the schema that "$schema" in a document's frontmatter points at is found, fetched and cached. What is written in a schema is decided in [the "The skeleton of the schema language" topic](schema-language.md). Sharing the loading from files and URLs between the CLI and outside applications is decided in [Loading of schemas and documents is placed in a separate crate shared by the CLI and outside applications](library-io.md#loading-of-schemas-and-documents-is-placed-in-a-separate-crate-shared-by-the-cli-and-outside-applications). Which inputs stop is decided in this topic, and how a stop is reported (the exit code and the name of the stop reason) in [the CLI and how results are output topic](cli.md#there-are-five-reasons-for-a-stop).

## A schema is specified by a path relative to the document's location or by a URL

Status: implemented

### Problem

To mechanically fix the format of Markdown written by an LLM, mds checks a document and extracts values with the schema that "$schema" in the document's frontmatter points at. To share the same format across several repositories, a schema needs to be pointed at by URL.

When ".." is stacked in a relative path, cancelling an earlier ".." with a later ".." would silently validate the document against a schema not intended. The reason for deciding the rule that prevents this is not in the records.

### Decision

Two ways of specifying the schema are accepted: a path relative to the document's location, and a URL. The specification is resolved in the following order.

| Order | Value of "$schema" | Resolution |
|---|---|---|
| 1 | Absent, empty, only whitespace, or not a string; or the frontmatter is not a mapping | Stop ([A document whose frontmatter is broken or that does not point at a schema stops without checking](#a-document-whose-frontmatter-is-broken-or-that-does-not-point-at-a-schema-stops-without-checking)) |
| 2 | Starts with "http://" or "https://" | Fetch it as a URL and place it in the cache ([A URL schema's fetched content is placed in the cache, and fetched again when the cache is corrupt](#a-url-schemas-fetched-content-is-placed-in-the-cache-and-fetched-again-when-the-cache-is-corrupt), [A URL schema that cannot be fetched, or whose fetch exceeds a limit, stops](#a-url-schema-that-cannot-be-fetched-or-whose-fetch-exceeds-a-limit-stops)) |
| 3 | Anything else | Read it as a path relative to the document's location. Stop if it cannot be read |

A relative path is resolved against the location where the document is placed. The base directory is not used to resolve relative paths. A relative path with stacked ".." is followed as stacked, and an earlier ".." is not cancelled by a later "..".

### Confirmations

- C-schema-resolve-1 A "$schema" value that starts with "http://" or "https://" is read as a URL, and any other value as a relative path
- C-schema-resolve-2 A relative-path schema is resolved against the location where the document is placed
- C-schema-resolve-13 A relative path with stacked ".." (for example "../../wrong.yaml") is resolved from the document's location as stacked, and the stacked ".." are not cancelled

## A document whose frontmatter is broken or that does not point at a schema stops without checking

Status: implemented

### Problem

As long as the schema cannot be resolved, no check result can be given. Giving a partial result, or silently skipping input that cannot be read, makes a document that was not checked look as though it passed.

No reason survives for making it a stop when the value of "$schema" is not a string.

### Decision

When the frontmatter is not a YAML mapping, when the value of "$schema" is empty or only whitespace, and when the value of "$schema" is not a string, the frontmatter is taken as broken, and mds stops without giving a partial result.

It also stops when a document given as a file target has no "$schema", and when a relative-path schema cannot be read. How a document with no "$schema" is treated when a directory is the target is decided in [A directory check checks only documents with $schema, and does not follow directories starting with . or symbolic links](cli.md#a-directory-check-checks-only-documents-with-schema-and-does-not-follow-directories-starting-with--or-symbolic-links).

### Confirmations

- C-schema-resolve-3 For a document whose frontmatter is not a YAML mapping, or whose "$schema" value is empty, only whitespace, null or not a string, it is a frontmatter error, and check, values and ast --schema stop without checking
- C-schema-resolve-4 When a document given as a file target has no "$schema", and when a relative-path schema cannot be read, check, values and ast --schema stop

## A URL schema's fetched content is placed in the cache, and fetched again when the cache is corrupt

Status: implemented

### Problem

Fetching a URL schema every time makes checking depend on the outside.

The place of the cache was aligned with how the base directory is searched for, which had already been decided. Aligning the marker of the base directory between kotowari and mds removes one of the hidden directories that tend to multiply. ".kotowari/cache/" alone does not say what the cache is for, so it is divided into a subdirectory per use. Even when caches are added later, responsibilities do not mix, a single one can be deleted, and .gitignore stays one line.

### Decision

For a URL schema, the fetched content is placed in the cache under its SHA-256 name, and from then on the cache is read. If the cache is corrupt, the schema is fetched again and the cache recovers.

The base directory is the first directory containing ".kotowari/" found by searching upward from the current directory. If none is found, it is the current directory. ".mds/" is not looked at.

The cache is placed in ".kotowari/cache/schemas/" of the base directory. ".kotowari/cache/" is the place for things that can be rebuilt if deleted; no file is placed directly in it, and it is divided into a subdirectory per use.

### Rejected options

- Keep compatibility by also searching for ".mds/". There would be two ways of searching, and the priority when both exist would need to be decided. Since almost no one uses it yet at this stage, no compatibility is kept.
- Place each use directly under ".kotowari/", as in ".kotowari/schema-cache/". Each new cache would add a name directly under it and a line to .gitignore.

### Confirmations

- C-schema-resolve-5 A URL schema is fetched and placed in the cache the first time, and from the second time on the cache is read without fetching
- C-schema-resolve-6 If there is a corrupt cache, the schema is fetched again, the cache is rewritten with the fetched content, and checking continues
- C-schema-resolve-7 If there is a directory above the current directory that contains ".kotowari/", the cache is placed in its ".kotowari/cache/schemas/" and not under the current directory, and if there is none anywhere above, it is placed in ".kotowari/cache/schemas/" of the current directory

## Checking continues with the fetched content even when the cache cannot be written

Status: implemented

### Problem

The cache exists to save fetching, and the fetched content is at hand. Stopping when the cache cannot be written would make checking unusable in a read-only environment.

### Decision

When a URL schema cannot be written to the cache (when the cache directory cannot be created, and when the cache cannot be written), mds makes it neither an error nor a finding, and continues checking with the fetched content.

### Confirmations

- C-schema-resolve-8 Both when the cache directory cannot be created and when the cache file cannot be written, checking continues with the fetched schema and it is not an error

## A URL schema that cannot be fetched, or whose fetch exceeds a limit, stops

Status: implemented

### Problem

Limits are needed so that a broken server cannot eat up memory and an unresponsive server cannot make it hang indefinitely. That a large schema cannot be used becomes behavior users can see. Giving a partial result when a schema cannot be fetched makes a document that was not checked look as though it passed.

### Decision

When a URL schema cannot be fetched, mds stops. It also stops when the response exceeds 4MiB, or when the whole fetch exceeds 10 seconds.

### Confirmations

- C-schema-resolve-9 When there is no cache and the server of a URL schema cannot be connected to, it stops
- C-schema-resolve-10 When the response exceeds 4MiB, or when the whole fetch exceeds 10 seconds, it stops
- C-schema-resolve-11 A URL schema that returns a response of exactly 4MiB within the time limit is used without stopping

## Credentials are masked when a URL is put in the explanation of a stop

Status: implemented

### Problem

Standard error flows into logs and CI output. Putting a URL whose fetch failed there as it is copies the credentials written in "$schema" there. Masking has meaning only at the point where the value is first handled.

### Decision

When putting a URL in the explanation of a stop, mds masks the credentials in the authority of that URL.

### Confirmations

- C-schema-resolve-12 When fetching a URL schema that contains credentials fails, no credentials appear on standard error, and the URL appears with its credentials masked. A URL with no credentials, and a URL with "@" in its path, appear as they are
