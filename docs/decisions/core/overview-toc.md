# The overview table of contents

English | [日本語](overview-toc.ja.md)

Decides where the overview table of contents (the file that declares how the index page bundles the overview pages and in which order it lists them) is placed and how it is read, its form, the check of disagreements with the overview source data, and passing the table of contents to the rendering engine. How the index page is drawn from the table of contents is decided in view's decisions ([Draw the index on one page, in the order and nesting written in the table of contents](../view/rendering.md#draw-the-index-on-one-page-in-the-order-and-nesting-written-in-the-table-of-contents) and [Index and navigation](../view/navigation.md)). The pair of the table of contents is decided in [Guides, overview source data and the table of contents have a side for every language of the language list](translation-pairs.md#guides-overview-source-data-and-the-table-of-contents-have-a-side-for-every-language-of-the-language-list).

## How the index is bundled and ordered is written in the one table of contents file that overview.toc points at

Status: implemented

### Problem

Overview pages increase topic by topic, and the number of pages is set by the size of the product, so it can become very large. Getting lost in the index lowers the value of the overview. Japanese titles look unordered to the reader even when sorted by the characters of the names, and they have no grouping either.

Writing the map of the index (the names of groups, nesting, order, and the pages that belong to each group) in one place is closer to the reader's intent. The table of contents is data, not something people read. The location of the overview source data reads only files with the extension ".md", so making it YAML keeps it from colliding with the source data. Keeping an ordering without a table of contents as well would mean thinking every time about which index it is.

If the reason for the stop when the table of contents is not UTF-8 were dropped, the way it stops would not match the overview source data. The way the table of contents is read is aligned with the way the source data is read.

### Decision

How the index page is bundled (the names of contents groups, nesting, order, and the pages that belong to each group) is written in one YAML file that the configuration's "overview.toc" points at. This is called the table of contents. When a new overview page is made, the table of contents is also updated.

When the configuration has the "overview" key, "overview.toc" is also required. There is no behavior that orders pages without a table of contents (the configuration error when "overview.toc" is absent is decided in [If the configuration file cannot be read as YAML, or its keys and values do not follow the rules, kotowari stops with a configuration error](config.md#if-the-configuration-file-cannot-be-read-as-yaml-or-its-keys-and-values-do-not-follow-the-rules-kotowari-stops-with-a-configuration-error)).

When the "overview" key is present, "kotowari check", "kotowari status", "kotowari overview build" and "kotowari overview serve" read the table of contents. When the table of contents file does not exist or cannot be read, they stop with an unreadable file as the reason. When it is not UTF-8, they stop, as for the overview source data ([The overview source data is committed to the repository and maintained, and selected by the globs of overview.files](overview-data.md#the-overview-source-data-is-committed-to-the-repository-and-maintained-and-selected-by-the-globs-of-overviewfiles)), with a non-UTF-8 file ("non-UTF-8 file") as the reason.

### Rejected options

- Express the bundling by the directories of the source data. Directory names are alphanumeric, so the names and order of groups would be written somewhere else, and the places to write would split in two.
- Write the group and a number in the frontmatter of each piece of source data. The order would be scattered, and the numbers would tend to shift every time a page is added.
- Make the bundling by machine from the directories of the specification location. There are only three group directories, and almost every page would go into one group.

### Confirmations

- C-core-ovtoc-1 When the "overview" key is present, check reads the table of contents and reports its errors as findings whose path is the table of contents file
- C-core-ovtoc-2 When the table of contents file does not exist, check, status and overview build stop with exit code 2 with an unreadable file as the reason
- C-core-ovtoc-3 When the table of contents file is not UTF-8, check stops with exit code 2
- C-core-ovtoc-16 When the table of contents file is not UTF-8, the first line of standard error is "non-UTF-8 file: " and the path of the table of contents

## If the table of contents file is also in the location of source data, guides or tests, it stops with a configuration error

Status: implemented

### Problem

A mistake that makes the table of contents a ".md" file and pulls it into the location of the overview source data can actually happen. The handling of the overlap and the way the detail is written are aligned with the form of the overlap of the overview source data and guide locations.

If the overlap were decided by the glob strings, then, as when the source data glob is "dir/*" and the table of contents is "dir/toc.yaml", files that are not read as source data would also be made errors. If this way of judging were dropped, the rule that prevents that error would disappear from the decision.

### Decision

When the table of contents file is also in one of the locations of the overview source data, guides or tests ("overview.files", "guides.files", "tests.files"), it stops with a configuration error. Whether it is in a location is decided not by the glob strings but by whether the table of contents file is among the files that the scan of each location actually reads. The scan of the overview source data reads only files whose extension is a lowercase ".md". The detail is the path of the table of contents followed by ": matched by both overview.toc and " and the name of the first of the matched keys in the order "overview.files", "guides.files", "tests.files".

If there is an [overlap of the overview source data with the guide or test locations](overview-data.md#if-the-overview-source-data-overlaps-the-location-of-guides-or-tests-it-stops-with-a-configuration-error) or an [overlap of the guide and test locations](guides.md#when-the-guide-and-test-locations-overlap-it-is-a-stop-with-a-configuration-error), that is checked first.

### Confirmations

- C-core-ovtoc-4 When "overview.files" is ".kotowari/overview/*.md" and "overview.toc" is ".kotowari/overview/toc.md", check stops with exit code 2, and the first line of standard error is "config error: .kotowari/overview/toc.md: matched by both overview.toc and overview.files"
- C-core-ovtoc-5 When the table of contents file is in the location of "guides.files" or "tests.files", it stops with a configuration error whose detail ends with the name of that key
- C-core-ovtoc-6 When the overview source data and guide locations also overlap, the configuration error for the overlap of source data and guides appears before the overlap of the table of contents
- C-core-ovtoc-15 When "overview.files" is "notes/*" and "overview.toc" is "notes/toc.yaml", check does not stop on an overlap

## The table of contents is written as nesting whose outermost level is one contents group

Status: implemented

### Problem

The number of pages has no upper limit, so with bundling at one level only, the list of group headings becomes just as hard to read when the groups increase. One level is a special case of a hierarchy, so a small product can stay at one level. If the outermost level has the same form as a contents group, nothing with a different form for the outside only is needed, and the index gets a heading and a count. A group with an empty heading cannot be read in the index, and an empty name points at no page.

### Decision

The table of contents is written in the following form. The depth of nesting is not limited.

| Part | Form |
|---|---|
| The whole table of contents | One contents group |
| Contents group | Has only "title" (required; a non-empty string), "note" (may be omitted; a one-line description without line breaks) and "items" (required; a sequence of page names and contents groups in order) |
| Page name | The file name of the overview source data without ".md". A non-empty string |

What does not fit the form is a place that is one of: cannot be read as YAML, has an unknown key, lacks a required key, has a value of the wrong type, has an empty "title" or page name, or has a "note" with a line break; an overview_toc_invalid error is reported for each such place. A group whose "items" is an empty sequence is not a form error, and is handled by overview_toc_group_empty of [Each overview page is placed exactly once in the table of contents](#each-overview-page-is-placed-exactly-once-in-the-table-of-contents).

Findings of the table of contents have the table of contents file as path and null as line ([line is counted from 1, and a finding that covers the whole document is null](finding-order.md#line-is-counted-from-1-and-a-finding-that-covers-the-whole-document-is-null)). The detail of overview_toc_invalid is the JSON Pointer of the place that did not fit, written the same way as overview_part_invalid of the parts of the overview source data ([The contents of parts are checked by the kotowari side with view's part schemas](overview-data.md#the-contents-of-parts-are-checked-by-the-kotowari-side-with-views-part-schemas)) (the details of the findings of the table of contents are as in the table of [The detail of a finding is a short string that identifies that finding, decided per kind in a table](findings.md#the-detail-of-a-finding-is-a-short-string-that-identifies-that-finding-decided-per-kind-in-a-table)).

### Rejected options

- Bundle at one level only. When the groups increase, the same hard reading occurs one level up.

### Confirmations

- C-core-ovtoc-7 A missing "title" or "items", an empty "title", a "note" with a line break, an empty page name, a value of the wrong type, a group with an unknown key, and a table of contents that cannot be read as YAML each get an overview_toc_invalid error
- C-core-ovtoc-8 A well-formed table of contents that includes a group with "note" and a nested group gets no overview_toc_invalid

## Each overview page is placed exactly once in the table of contents

Status: implemented

### Problem

Placing one page in several groups makes the counts in the index wrong, and the place to fix is not uniquely determined either. The table of contents is maintained by the LLM that writes the overview, so stopping leaves nothing forgotten, compared with silently dropping a page that is not in the table of contents into "uncategorized". If errors of the table of contents go into the findings of check and status like other source data errors, the flow in which build points to "run kotowari check" can be used as it is.

### Decision

When the table of contents and the overview source data disagree, the following errors are reported.

| Disagreement | Finding |
|---|---|
| Overview source data exists, but its name is nowhere in the table of contents | overview_toc_page_missing |
| A name in the table of contents has no overview source data | overview_toc_page_unknown |
| A name that has overview source data appears twice or more in the table of contents | overview_toc_page_duplicate on the second and later |
| The "items" of a contents group (the outermost included) is an empty sequence | overview_toc_group_empty |

Names are matched only for the source data and the table of contents of the first-language side ([The other-language sides of overview source data are not counted as separate pages](translation-pairs.md#the-other-language-sides-of-overview-source-data-are-not-counted-as-separate-pages)).

When these errors exist, "kotowari overview build" and "kotowari overview serve" write nothing and stop (the reason and detail of the stop: [If the source data or the table of contents has errors, it writes nothing and stops with overview error](overview-commands.md#if-the-source-data-or-the-table-of-contents-has-errors-it-writes-nothing-and-stops-with-overview-error)).

### Rejected options

- Show pages not in the table of contents in an "uncategorized" group. Forgotten fixes become hard to see.

### Confirmations

- C-core-ovtoc-9 When the overview source data is "a.md", "b.md" and "c.md" and the names in the table of contents are "a", "z", "a" in that order, "b" and "c" get overview_toc_page_missing, "z" gets overview_toc_page_unknown, and the second "a" gets an overview_toc_page_duplicate error
- C-core-ovtoc-10 A contents group whose "items" is empty gets an overview_toc_group_empty error, whether it is a nested group or the outermost group
- C-core-ovtoc-11 When the table of contents has errors, overview build and overview serve write nothing and stop with exit code 2

## A malformed table of contents is not matched against the source data

Status: implemented

### Problem

Matching against a malformed table of contents would report almost every page as missing from the table of contents, burying the real cause.

### Decision

When the table of contents has even one overview_toc_invalid error, the checks overview_toc_page_missing, overview_toc_page_unknown, overview_toc_page_duplicate and overview_toc_group_empty are not made on that table of contents.

### Confirmations

- C-core-ovtoc-12 In a table of contents with a group that has an unknown key, only overview_toc_invalid is reported and overview_toc_page_missing is not, even if no source data name is in the table of contents

## The table of contents is passed to the rendering engine in the order and nesting as written

Status: implemented

### Problem

If the index is to be a map of the product, the order itself, such as the order of use or the order of the process, is information. Japanese titles have no means of being sorted by reading, and an automatic order looks unordered.

### Decision

"kotowari overview build" and "kotowari overview serve" pass the "title", "note" and "items" of the table of contents to the rendering engine as its table of contents, in the order and nesting as written. They do not reorder them.

### Rejected options

- Order automatically by the characters of titles or names. With Japanese titles it looks unordered.

### Confirmations

- C-core-ovtoc-13 The order and nesting of the items written in the table of contents and its "note" become the table of contents of the render input as they are
- C-core-ovtoc-14 When the "title" of the table of contents is "kotowari" and "items" is "b", "a" in that order, the heading of the index that overview build writes is "kotowari", and the title of "b" comes before the title of "a"
