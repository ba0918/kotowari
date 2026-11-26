# Bundled queries

English | [日本語](test-queries.ja.md)

Covers the queries kotowari bundles per language, and what is counted as a test in each language. How queries are applied to files and how test names are taken is decided in [Finding tests](test-discovery.md), and the form of adding queries by configuration in [Queries added by configuration](query-rules.md). How markers are tied to the tests found is decided in [A marker is tied to a test only when it is in the comment block directly before that test](test-markers.md#a-marker-is-tied-to-a-test-only-when-it-is-in-the-comment-block-directly-before-that-test), and how the line of a finding is counted in [line is counted from 1, and a finding that covers the whole document is null](finding-order.md#line-is-counted-from-1-and-a-finding-that-covers-the-whole-document-is-null).

## Queries are bundled only for the four families of languages the user uses

Status: implemented

### Problem

kotowari has the grammars of many languages, but the default of what is counted as a test has to be decided per language. The policy "only what is certain becomes the default" applies to defaults, and deciding what is certain in each language needs a brainstorm, a specification and tests.

### Decision

kotowari bundles queries for Rust, TypeScript, Tsx, JavaScript, Python and Php. They were narrowed to the languages the user uses. For other languages, people add rules in the configuration ([Queries are added with tests.rules in the configuration, and the bundled queries cannot be removed](query-rules.md#queries-are-added-with-testsrules-in-the-configuration-and-the-bundled-queries-cannot-be-removed)). Adding defaults is decided later, per language.

TypeScript and Tsx are separate languages, so queries with the same content are bundled for both.

### Confirmations

- C-core-queries-1 Files with the extensions of Rust, TypeScript, Tsx, JavaScript, Python and Php become languages with a query without adding rules in the configuration, and ".go" files do not
- C-core-queries-2 "it(...)" in a ".tsx" file is counted as a test with the bundled queries
- C-core-queries-3 "it(...)" in ".js", ".jsx", ".mjs" and ".cjs" files is counted as a test with the bundled queries

## In Rust, functions whose attribute path ends in test are always counted, and attributes and macros are added by configuration

Status: implemented

### Problem

What is counted as a test in Rust changes with the libraries in use, so it cannot be stated definitely, and the proviso "in this project" is always attached. The user judged that only "#[test]" is certain. kotowari itself also has tests with the "kani::proof" attribute and the "proptest" macro.

### Decision

The bundled query takes functions with "#[test]" as the certain default, and other attributes and macros are added per project to "tests.rust.attributes" and "tests.rust.macros" in the configuration. Both are kept without changing the form of the configuration, so as not to break existing configurations.

| Target | How it is counted |
|---|---|
| A function whose attribute path has "test" as its last element ("#[test]", "#[ test ]", "#[core::prelude::v1::test]", "#[tokio::test]") | Fixed in the bundled query, and always counted regardless of the configuration |
| A function with an attribute of "tests.rust.attributes" | Counted if the path, with "#[", "]" and the arguments removed from the attribute, matches exactly ("kani::proof" also matches "#[kani::proof(unwind = 3)]") |
| A macro of "tests.rust.macros" | Written in the configuration with the name without "!". If the last element of the macro's name matches ("proptest::proptest!" also matches "proptest"), its content is read again as Rust items, and each top-level function in it is counted. The name is the function's name |
| When the functions and macros of the rows above are inside the body of another function | Counted regardless of where they are placed |

Markers of functions inside a macro are tied by the same rules as outside the macro ([A marker is tied to a test only when it is in the comment block directly before that test](test-markers.md#a-marker-is-tied-to-a-test-only-when-it-is-in-the-comment-block-directly-before-that-test)).

### Confirmations

- C-core-queries-4 Functions whose attribute path has "test" as its last element ("#[test]", "#[ test ]", "#[tokio::test]") are counted as tests even without configuration, and functions with only unrelated attributes are not counted
- C-core-queries-5 An attribute of "tests.rust.attributes" is counted if the path, with "#[", "]" and the arguments removed, matches exactly, and functions without that attribute are not counted
- C-core-queries-6 A function with "#[test]" inside the body of a function is also counted as a test
- C-core-queries-7 A macro of "tests.rust.macros" is matched by the last element of its name, and each function in its content is counted with the function's name. The content of a macro not in the configuration, and functions further inside a function inside a macro, are not counted

## In TypeScript and JavaScript, calls of it and test are counted

Status: implemented

### Problem

TypeScript, Tsx and JavaScript have many libraries for writing tests. A form that is common and certain in every library has to be the default.

### Decision

TypeScript, Tsx and JavaScript share the following counting. The user judged that "it" and "test" are common and certain in the main libraries, and that skipped tests are also test definitions.

| Target | How it is counted |
|---|---|
| A call whose callee is "it" or "test" ("it(...)", "test(...)") | Counted. The name is the first argument |
| A call whose callee is "it" or "test" followed by one or more "." and name ("it.only(...)", "it.skip(...)", "it.todo(...)", "test.concurrent(...)", "it.only.each(...)") | Counted. The name is the first argument. Except the inner call of the next row |
| Of the forms above, a form that further calls the result of a call whose last element is "each" ("it.each(table)(name, fn)", "test.concurrent.each(table)(name, fn)") | Only the outer call is counted. The name is the first argument of the outer call. Counting the inner "it.each(table)" would count one test as two |
| "describe(...)" and its "." forms | Not counted. The calls inside are counted as in the rows above |
| A call whose callee is not "it" or "test" ("regex.test(s)") | Not counted |

How the quotation marks of a name are removed, and not attaching the name of "describe", are decided in [A test's name is taken from the text the query captured](test-discovery.md#a-tests-name-is-taken-from-the-text-the-query-captured).

### Confirmations

- C-core-queries-8 Calls of "it(...)", "test(...)" and the forms followed by "." and a name ("it.only", "it.skip", "it.todo", "test.concurrent") are counted, and the name is the first argument
- C-core-queries-9 "it.each(table)(name, fn)" and "it.only.each(table)(name, fn)" are counted as only one, the outer call
- C-core-queries-10 "describe(...)" and calls whose callee is not "it" or "test" ("regex.test(s)") are not counted

## In Python, functions and methods whose names start with test are counted

Status: implemented

### Problem

Python tests are written in two ways: top-level functions of a file whose names start with "test", and methods inside a class whose names start with "test". A default that picks up both of the default discovery methods of the main test libraries is needed.

### Decision

| Target | How it is counted |
|---|---|
| A top-level function of the file, whose name starts with "test" | Counted. The name is the function's name |
| A method inside a class, whose name starts with "test" | Counted. The name is the method's name |
| The functions and methods above with decorators | Counted |
| A function nested inside a function | Not counted |

### Confirmations

- C-core-queries-11 Top-level functions of the file and methods inside a class, whose names start with "test", are counted, and functions nested inside a function are not counted
- C-core-queries-12 Functions and methods with decorators whose names start with "test" are also counted

## In Php, methods and calls of test are counted

Status: implemented

### Problem

Php tests are written as one of: methods starting with "test", the "Test" attribute, "@test" in a docblock, or calls of Pest functions. "@test" in docblocks was removed in newer versions of the main test library, but much code has it written.

### Decision

| Target | How it is counted |
|---|---|
| A method whose name starts with "test" | Counted. The name is the method's name |
| A method with an attribute whose path's last element is "Test" ("#[Test]", "#[\PHPUnit\Framework\Attributes\Test]") | Counted. The name is the method's name |
| A method with "@test" in the preceding comment starting with "/**" | Counted. The name is the method's name |
| A call of a function whose callee is "test" or "it" (Pest's "test('...', fn)") | Counted. The name is the first argument. If a test file calls "test('...', fn)", it is almost certainly Pest |

### Confirmations

- C-core-queries-13 Methods whose names start with "test", and methods with an attribute whose path's last element is "Test" ("#[Test]", "#[\PHPUnit\Framework\Attributes\Test]"), are counted, and methods with only other attributes are not counted
- C-core-queries-14 Methods with "@test" in the preceding comment starting with "/**" are counted, and methods with a "/*" comment or only "@testdox" are not counted
- C-core-queries-15 Calls of functions whose callee is "test" or "it" are counted, and the name is the first argument. Calls of "describe" are not counted
