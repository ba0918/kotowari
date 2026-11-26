# Test markers

English | [日本語](test-markers.ja.md)

Covers the form of the markers written in tests and the rules that tie markers to tests. How tests are found is decided in [Finding tests](test-discovery.md). The error for a test without a marker, and how markers in a language without a query are counted, are decided in [Correspondence between tests and markers](coverage.md). The error when a marker points at an ID that is in no document is decided in [Check counts, per confirmation row, whether a test points at it](confirmations.md#check-counts-per-confirmation-row-whether-a-test-points-at-it).

## A marker is IDs listed with commas, and may be anywhere in a line

Status: implemented

### Problem

kotowari itself is written in Rust, but which language the users' tests are written in is unknown. The correspondence between tests and the specification should be found by a method that does not depend on the language. Also, if the marker's name is too short, it collides with the notations of other tools.

### Decision

A test marker is written in the following form.

| Part | Form |
|---|---|
| Start | @kotowari[ |
| Content | IDs listed separated by commas. Whitespace may be placed before and after a comma |
| End | ] |

A marker may be at any position in a line, and kotowari does not look at comment symbols. If there are several markers in one line, all are picked up. The name was made "@kotowari" rather than "@spec" because the latter may collide with others.

### Confirmations

- C-core-markers-1 From a marker with whitespace before and after the commas, check reads the IDs one by one and records the line where the marker is
- C-core-markers-2 If one line has two markers, check picks up both, and the test is tied to the IDs of both
- C-core-markers-3 Markers at any position in a line are picked up, whatever the kind of comment symbol

## A malformed marker gets an invalid_marker error, pointing at the marker's line

Status: implemented

### Problem

There are writing mistakes such as a marker whose content is empty or that is not closed. No reason is left for deciding it this way.

### Decision

When a marker's content is empty or only separators (commas and whitespace), or when there is no closing bracket on the same line (including markers that span lines), check outputs an invalid_marker error. detail is the text of the line where that marker is, as it is (including indentation and trailing whitespace, not rebuilt).

The "line" of invalid_marker and unresolved_reference output from a marker is the line where the marker is (for a marker that spans lines, the line where "@kotowari[" is).

In a language with a query, the form of markers not tied to a test ([A marker is tied to a test only when it is in the comment block directly before that test](#a-marker-is-tied-to-a-test-only-when-it-is-in-the-comment-block-directly-before-that-test)) is not looked at.

### Confirmations

- C-core-markers-4 For a marker in the comment directly before a test whose content is empty or only separators, check outputs an invalid_marker error. It is output even for the second marker on the same line
- C-core-markers-5 For a marker that spans lines with its closing bracket on the next line, check outputs an invalid_marker error with the line of "@kotowari[" as "line"
- C-core-markers-6 The detail of invalid_marker is the text of the line where the marker is, as it is, including indentation
- C-core-markers-7 The "line" of invalid_marker and unresolved_reference output from a marker is the line where the marker is. The same holds for a marker on the second line of a multi-line comment, or inside a macro

## A marker is tied to a test only when it is in the comment block directly before that test

Status: implemented

### Problem

If markers in a comment at the start of a function body were also to be tied, the start of the body cannot be determined independently of the language (there is the style of placing "{" on the line after the function, and the habit of Python comments coming before the body's node). Of the 1124 markers in kotowari's own tests, 1 was at the start of a body, and that was only a descriptive comment containing the characters of a marker. A test whose marker is not tied gets the error for a test without a marker, so a marker never disappears silently.

When there is another test inside a test, both the rule that ties and the rule that ignores apply. When two tests start on one line, tying to both would produce two findings from one marker. Applying a separate rule only to functions inside macros would make two rules for tying.

### Decision

In a language with a query, only markers in the comment block directly before a test ([The preceding comment block consists of comment lines up to a blank line and the lines that may be interposed per language](#the-preceding-comment-block-consists-of-comment-lines-up-to-a-blank-line-and-the-lines-that-may-be-interposed-per-language)) are tied to that test.

| Position of the marker | Handling |
|---|---|
| The comment block directly before a test | Tied to that test. When two or more tests have the same first line, it is tied only to the test that starts first on that line, and the other tests are tests without markers |
| Inside a test's node (including a comment at the start of a function body). Except the comment block directly before another test inside the node, whose markers are tied to the inner test | Ignored; neither invalid_marker nor unresolved_reference is output |
| Not in the comment block directly before any test | Ignored; neither invalid_marker nor unresolved_reference is output |
| A function inside a macro of "tests.rust.macros" | The same rules as above are applied |

### Rejected options

- Previous decision: Markers in a comment at the start of a function body are also tied, together with the markers in the preceding comment. The start of the body cannot be determined independently of the language, so this was dropped.

### Confirmations

- C-core-markers-8 A marker in a comment at the start of a function body is not tied to the test, and if there is no marker before it, test_without_id is output for that test. It is not added to the preceding markers either
- C-core-markers-9 A marker that is not in the comment block directly before any test, and a marker in the middle or at the start of a body, output neither invalid_marker nor unresolved_reference, even if malformed or pointing at an ID that does not exist
- C-core-markers-10 A marker directly before another test inside a test is tied to the inner test
- C-core-markers-11 When two tests start on one line, the preceding marker is tied only to the first test, and test_without_id is output for the other test
- C-core-markers-12 The rule of the preceding comment block also applies to functions inside macros of "tests.rust.macros"

## The preceding comment block consists of comment lines up to a blank line and the lines that may be interposed per language

Status: implemented

### Problem

Where comments go in the syntax tree differs per language. Measured in four languages, comments were tree-sitter extra nodes in all of them. If it is decided by lines, the result is the same wherever comments go in the syntax tree, as in Python.

In languages where annotations and attributes go inside the test's node, such as Java, C#, PHP and Kotlin, the table of lines that may be interposed is not needed. Those that go outside are Rust and Python. Without allowing multiple lines, a marker would not be tied when "@pytest.mark.parametrize(...)" is written in the common way. If the block were cut by a whitespace-only line in the middle of a multi-line attribute or decorator, it would go against the intent of allowing multiple lines.

The range of a "comment-only line" had not been decided. The range of surrounding whitespace disagreed with the whitespace of the detail of a test without a name (Unicode whitespace). The promise that markers are written in comments should be kept.

### Decision

The preceding comment block is the block of comment-only lines and of the lines in the following table that may be interposed, continuing upward from directly before the first line of the test's node until a blank line comes.

A comment-only line is a line in which all the characters, after removing surrounding whitespace (Unicode whitespace, including the full-width space and NBSP), are characters of comments (tree-sitter extra nodes). It includes the lines in the middle of a multi-line comment, and does not include comment lines on the same line as code.

Comment-only lines and lines that may be interposed may mix without blank lines. For multi-line attributes and decorators, all their lines are lines that may be interposed. A whitespace-only line in the middle of a multi-line comment, attribute or decorator does not cut the block.

Of the lines of the block, markers are read only from the characters of comments. Markers in the body of an attribute or decorator (including the string of "#[doc = \"...\"]") are not read, and no finding is output for them.

| Language | Lines that may be interposed |
|---|---|
| Rust | Lines of attributes ("#[...]") |
| Python | Lines of decorators ("@...") |
| Other languages | None |

A language for which rules are only added in the configuration forms the block with comment lines only.

### Confirmations

- C-core-markers-13 A marker in a comment that continues directly before a test without a blank line is tied to that test (including a method inside a class), and if there is a blank line between the marker and the test, it is not tied
- C-core-markers-14 Even if Rust attributes or Python decorators (including multi-line ones) and comment lines are interposed between the marker and the test without blank lines, the marker is tied to the test
- C-core-markers-15 A whitespace-only line in the middle of a multi-line comment, attribute or decorator does not cut the block
- C-core-markers-16 If a code line is interposed in a language with no lines that may be interposed, or if a comment line on the same line as code is interposed, the block is cut and the marker is not tied
- C-core-markers-17 A comment line that starts after a full-width space also goes into the block as a comment-only line
- C-core-markers-18 "@kotowari[" in a string in the body of an attribute is not read, and even with an ID that does not exist, unresolved_reference is not output

## In files of a language without a query, all markers in the file are picked up

Status: implemented

### Problem

In files of a language without a query, neither tests nor comments are distinguished ([Files of a language with no query at all are not parsed into syntax trees](test-discovery.md#files-of-a-language-with-no-query-at-all-are-not-parsed-into-syntax-trees)). How markers in such files are picked up had to be decided. No reason is left for deciding it this way.

### Decision

In a test file of a language without a query, all markers in the text of the file are picked up, whether in a comment or not. A malformed marker gets invalid_marker wherever it is in the file. How the picked-up markers are counted is decided in [Markers in files of a language without a query also count toward satisfying confirmation rows](coverage.md#markers-in-files-of-a-language-without-a-query-also-count-toward-satisfying-confirmation-rows).

### Confirmations

- C-core-markers-19 In a file of a language without a query, a marker on a line that is not a comment is also picked up, and "line" is that line
- C-core-markers-20 A malformed marker in a file of a language without a query gets an invalid_marker error

## Even a marker that points only at IDs in no document makes its test count as one with a marker

Status: implemented

### Problem

When an ID a marker points at is not in the specification, how to count the test that marker is tied to had to be decided. No reason is left for deciding it this way.

### Decision

Even if a marker points only at IDs that are in no document, the test that marker is tied to is counted as a test with a marker, and test_without_id is not output. Pointing at an ID that does not exist becomes the unresolved_reference error decided in [Check counts, per confirmation row, whether a test points at it](confirmations.md#check-counts-per-confirmation-row-whether-a-test-points-at-it).

### Confirmations

- C-core-markers-21 A test tied to a marker that points only at IDs in no document does not get test_without_id

## A marker that points at a line checked by reading is not an error

Status: implemented

### Problem

A confirmation row with "(by reading)" written at its head is a line not checked by tests. How to handle a marker that points at the ID of that line had to be decided. No reason is left for deciding it this way.

### Decision

Even if a marker points at the ID of a confirmation row with "(by reading)" written at its head, check does not make it an error. That line still does not require a test.

### Confirmations

- C-core-markers-22 For a marker that points at the ID of a confirmation row with "(by reading)" written at its head, check outputs no error
