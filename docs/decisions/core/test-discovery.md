# Finding tests

English | [日本語](test-discovery.ja.md)

Covers how check selects test files, decides the language from the extension, and finds tests and their names with the queries of that language. The content of the queries bundled per language is decided in [Bundled queries](test-queries.md), and the queries added by configuration in [Queries added by configuration](query-rules.md). How markers are tied to the tests found is decided in [A marker is tied to a test only when it is in the comment block directly before that test](test-markers.md#a-marker-is-tied-to-a-test-only-when-it-is-in-the-comment-block-directly-before-that-test).

## Test files are selected with the globs of tests.files

Status: implemented

### Problem

Where tests are placed differs by product. If kotowari searched only fixed places, it could not be used in projects that place them otherwise.

When the scan hits something it cannot read, silently skipping it lets the check pass without counting the markers of the tests there ([Unreadable input and broken input become a stop or an error finding, and are not silently skipped](cli-environment.md#unreadable-input-and-broken-input-become-a-stop-or-an-error-finding-and-are-not-silently-skipped)). Something that is neither a directory nor a regular file (a socket, a named pipe, a device) can hang when read.

### Decision

Files matched by the globs of "tests.files" in the configuration are read as test files. The user judged that, because where to search for tests depends on the product, it should be changeable by configuration. The default globs are decided in [The configuration keys are only those in the table, and a key not written takes the default value in the table](config.md#the-configuration-keys-are-only-those-in-the-table-and-a-key-not-written-takes-the-default-value-in-the-table).

In the scan, symbolic links to files are read, and symbolic links to directories are not followed. No reason is left for the decision on how symbolic links are handled.

The scan walks the whole base directory and then selects with the globs ([Hidden directories are handled the same way as the reading of globs](config.md#the-glob--is-read-recursively-and-hidden-directories-are-entered-only-when-named-by-a-path-component)). If there is a dangling symbolic link, it stops with an unreadable file as the reason, even if the link is in a place no glob matches. When it hits an unreadable directory, it also stops with an unreadable file as the reason. Something that is neither a directory nor a regular file is not read even if a glob matches it.

### Confirmations

- C-core-discover-1 Files matched by the globs of "tests.files" are read as test files
- C-core-discover-2 In the scan of "tests.files", the file a symbolic link to a file points at is read
- C-core-discover-3 In the scan of "tests.files", what a symbolic link to a directory points at is not read
- C-core-discover-14 If there is a dangling symbolic link in a place matched by the globs of "tests.files", check stops with exit code 2 and outputs nothing to standard output
- C-core-discover-15 Even for a dangling symbolic link in a place no glob matches, check stops with exit code 2 and outputs nothing to standard output
- C-core-discover-16 A socket matched by the globs of "tests.files" is not read and is not included in the number of test files
- C-core-discover-17 If there is an unreadable directory under the globs of "tests.files", check stops with exit code 2 and outputs nothing to standard output

## Tests are found by applying queries to the syntax tree, not by searching lines with regular expressions

Status: implemented

### Problem

Finding test definitions requires judging, per language, what a test is. Applying regular expressions to the lines of a file to search for definition lines is not universal and picks up unintended things. Writing the judgement per language by hand in code grows heavier every time a language is added.

### Decision

A test file is parsed into a syntax tree with tree-sitter, and all the queries of its language (ast-grep rules that find tests) are applied to find tests. With tree-sitter, ranges are taken reliably. All languages, Rust included, use this one mechanism. This is because separating the mechanism per language would make the processing that ties markers to tests two ways, which would be a source of drift in how markers are tied depending on the language.

Test definition lines are not found with regular expressions written in the configuration file. The only regular expressions that come from the configuration are those applied to syntax-tree nodes inside ast-grep rules.

### Rejected options

- Find test definition lines with regular expressions written in the configuration. It is not universal, and unintended detections occur.
- Keep Rust alone in the form that walks the syntax tree by hand, and use ast-grep only for new languages. The processing that ties markers would be two ways.

### Confirmations

- C-core-discover-4 A "#[test]" function in a Rust test file is found as a test with the bundled queries
- C-core-discover-5 (by reading) The code that finds tests does not apply regular expressions to the lines of test files to search for definition lines. The only regular expressions that come from the configuration are those applied to syntax-tree nodes inside ast-grep rules

## Files of a language with no query at all are not parsed into syntax trees

Status: implemented

### Problem

kotowari has the grammars of many languages. If merely adding a grammar changed behaviour, a user who has ".md" in a glob, for example, would suddenly get unparsable_file.

### Decision

A language with a query is a language that has one or more bundled or configured queries. A language that has a grammar but no query at all remains a language without a query, and its files are not parsed into syntax trees. How markers are picked up from files of a language without a query is decided in [In files of a language without a query, all markers in the file are picked up](test-markers.md#in-files-of-a-language-without-a-query-all-markers-in-the-file-are-picked-up).

### Confirmations

- C-core-discover-6 A file whose language is determined but that has no query at all (".go") is not parsed into a syntax tree, and even if it has a syntax error, unparsable_file is not output

## The correspondence between extensions and languages is the same as ast-grep, and is case-sensitive

Status: implemented

### Problem

Queries are written in the form of ast-grep rules ([Queries added by configuration](query-rules.md)), so if the reading of which file is which language drifted from ast-grep, the result would differ from trying it with "ast-grep scan". Narrowing the readable languages would narrow the situations in which kotowari can be used.

### Decision

The readable languages are not narrowed; common languages are widely readable. The user judged that, with the form of judging by rules, the effort of adding a language is small, and the more languages are readable, the wider the situations in which kotowari can be used. All 27 grammars of ast-grep-language are included, giving 28 languages (the TypeScript grammar has two languages, TypeScript and Tsx).

The correspondence between extensions and languages uses the correspondence held by the included ast-grep-language as it is. The same files as with "ast-grep scan" get the same languages. TypeScript and Tsx are treated as separate languages, as in ast-grep.

The extension is the characters after the last "." of the file name, and is case-sensitive. A file with an extension not in the table gets no language and is treated as a language without a query. No reason is left for being case-sensitive.

| Language | Extensions |
|---|---|
| Bash | bash, bats, cgi, command, env, fcgi, ksh, sh, tmux, tool, zsh |
| C | c, h |
| Cpp | cc, hpp, cpp, c++, hh, cxx, cu, ino |
| CSharp | cs |
| Css | css, scss |
| Dart | dart |
| Elixir | ex, exs |
| Go | go |
| Haskell | hs |
| Hcl | hcl, nomad, tf, tfvars, workflow |
| Html | html, htm, xhtml |
| Java | java |
| JavaScript | cjs, js, mjs, jsx |
| Json | json |
| Kotlin | kt, ktm, kts |
| Lua | lua |
| Markdown | markdown, md |
| Nix | nix |
| Php | php |
| Python | py, py3, pyi, bzl, bazel |
| Ruby | rb, rbw, gemspec |
| Rust | rs |
| Scala | scala, sc, sbt |
| Solidity | sol |
| Swift | swift |
| TypeScript | ts, cts, mts |
| Tsx | tsx |
| Yaml | yaml, yml |

### Rejected options

- Narrow the grammars to the five languages the user uses, or to the 16 programming languages. The difference from all 27 is 6.7MB; including all of them removes the need to judge which to choose, and people can use a language just by adding rules in the configuration.

### Confirmations

- C-core-discover-7 Files with the extensions in the table get the languages in the table (".py3" and ".bzl" are Python, ".mts" is TypeScript), and files with spellings differing in case (".PY", ".Rs", ".RS") get no language and are treated as a language without a query

## A test file with a syntax error gets unparsable_file and is skipped

Status: implemented

### Problem

Test files may include ones with syntax errors, half-written ones for example. How to handle a file whose syntax tree cannot be read had to be decided. No reason is left for the decision to skip rather than stop. When languages were added, it was decided to keep the earlier behaviour of Rust regardless of the language.

### Decision

A test file of a language with a query is read with tree-sitter, and if it has even one syntax error, an unparsable_file error is output for the whole file, the file is skipped, and it does not stop. Files of a language without a query are not parsed into syntax trees, so unparsable_file is not output for them ([Files of a language with no query at all are not parsed into syntax trees](#files-of-a-language-with-no-query-at-all-are-not-parsed-into-syntax-trees)).

### Confirmations

- C-core-discover-8 If a file of a language with a query has a syntax error, one unparsable_file is output for that file, and check continues the check without stopping

## A test's name is taken from the text the query captured

Status: implemented

### Problem

Queries are also written by the people using kotowari, so which part of the node a rule matched becomes the test's name has to be decided as a promise on kotowari's side. In TypeScript's "it(\"does x\", ...)", a string including its quotation marks is captured in the position of the name.

### Decision

One syntax-tree node a query matched is one test, and the text of the node that went into the metavariable "$NAME" is that test's name.

If the first and last characters of that text are the same quotation mark (a single quote, a double quote or a backquote), one character is removed from each end. Any other form (for example, Python's triple-quoted strings, or strings with an "f" prefix) becomes the name as it is. The user judged that this rule is enough, because telling string literals apart independently of the language is hard, and names that are strings are about only TS/JS and Pest's "it('...')".

When a query does not capture "$NAME", the test's name is null. "name" in list can take null ([A confirmation row gets the tests that point at it, and a decision gets a fingerprint](list.md#a-confirmation-row-gets-the-tests-that-point-at-it-and-a-decision-gets-a-fingerprint)).

The name of an "it" inside a nested "describe" is only the first argument of the "it" itself, without the name of the outer "describe". The name is information output in list and is not used for tying markers. Joining them would add a rule for the separator character used to join them.

### Confirmations

- C-core-discover-9 If the first and last characters of the text of "$NAME" are the same quotation mark, one character is removed from each end, and other forms (for example, a quotation mark on only one side, or an expression) become the name as they are
- C-core-discover-10 The name of an "it" inside a "describe" does not get the name of the "describe"
- C-core-discover-11 A test found by a query that does not capture "$NAME" has null as "name" in list

## Overlapping matches on the same node count as one, and a test inside a test is counted separately

Status: implemented

### Problem

A bundled query and a query added in the configuration may match the same "it(...)". Counted as they are, one test would become two. On the other hand, another test may be written inside a test.

### Decision

If several queries match the same node, that node is counted as one test. A node matched further inside a test's node is counted as a separate test.

### Confirmations

- C-core-discover-12 Even if a bundled query and a configured query match the same node, it is counted as one test
- C-core-discover-13 A node further matched by a query inside a test's node is counted as a test separate from the outer test
