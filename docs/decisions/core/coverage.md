# Correspondence between tests and markers

English | [日本語](coverage.ja.md)

Covers the check of whether a test has a marker, how markers in files of a language without a query are counted, and the check of tests when there are no decision documents. The rule that counts, per confirmation row, whether a test points at it is decided in [Check counts, per confirmation row, whether a test points at it](confirmations.md#check-counts-per-confirmation-row-whether-a-test-points-at-it). The form of markers and the rules that tie a marker to a test are decided in [Test markers](test-markers.md).

## A test without a marker gets a test_without_id error

Status: implemented

### Problem

What to do when a test that check found has no marker has to be decided. The reason for deciding to make a test without a marker an error is not recorded. If the name of the test is used as the detail of the error, the detail is empty for a test that has no name, and it cannot be told which test it is.

### Decision

When a test in a language with a query has no marker, check reports a test_without_id error with the name of that test as the detail.

When the name of the test is null (when the query does not capture a name), the detail is the text of the whole first line of the test's node with leading and trailing whitespace removed. Other kinds of findings also use the text of the line as is for the detail. The whole line is used even when the node starts partway through the line ("const t = test(...)") because the "line" of a finding is also decided by line.

### Confirmations

- C-core-cover-1 When a test in a language with a query has no marker, check reports a test_without_id error with the name of that test as the detail, and does not report it once a marker is added
- C-core-cover-2 When a test whose name is null has no marker, the detail of test_without_id is the text of the whole first line of that test with leading and trailing whitespace removed

## Markers in files of a language without a query also count toward satisfying confirmation rows

Status: implemented

### Problem

In test files of a language without a query, check cannot find tests and only picks up the markers in the file ([In files of a language without a query, all markers in the file are picked up](test-markers.md#in-files-of-a-language-without-a-query-all-markers-in-the-file-are-picked-up)). How the markers picked up are counted has to be decided. The handling of markers is not split in two by language.

### Decision

Markers picked up in test files of a language without a query count toward satisfying a confirmation row when counting whether the row has a test, the same as the markers of tests in a language with a query.

No tests are found in files of a language without a query, so no test_without_id error is reported for them.

### Confirmations

- C-core-cover-3 If a marker in a file of a language without a query (".go") points at a confirmation row of an implemented decision, confirmation_without_test is not reported for that row
- C-core-cover-4 test_without_id is not reported for files of a language without a query

## The check of tests is performed even when there is no decision document at all

Status: implemented

### Problem

Whether to check tests when the location of decision documents has no documents at all has to be decided. The reason for that decision is not recorded.

### Decision

When the location of decision documents has no documents at all, the findings of the check of decision documents are zero, and the check of tests is performed. There are no confirmation rows, so confirmation_without_test is not reported, and test_without_id is reported for tests without a marker.

### Confirmations

- C-core-cover-5 When the location of decision documents has no documents and there is one test without a marker, check reports only test_without_id for that test and ends with exit code 1
