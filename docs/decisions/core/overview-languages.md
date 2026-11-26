# Per-language overviews and UI strings

English | [日本語](overview-languages.ja.md)

Decides that overview pages are written for each language of the configuration's language list, the language and UI strings (text that kotowari writes into the overview pages and that is in neither the source data nor the specification) passed to the rendering engine, how UI strings are determined from the configuration's "labels" and the errors in doing so, and where the body of a reference is taken from on another language's page. How the language list and pairs are read is decided in the pair topic that starts with [The language list is decided by the configuration's languages](translation-pairs.md#the-language-list-is-decided-by-the-configurations-languages), and how view draws what it is passed in [Page language](../view/languages.md).

## The overview is written for each language of the language list, from that language's side

Status: implemented

### Problem

Changing only the UI language of the overview does not solve the problem of a reader who goes from an English introduction page to the specification or a guide and runs into Japanese. Nor does it solve the case of users of different nationalities reading the overview in the same project. The readers of each language are to be able to read the pages in their own language, and move to other languages by links alone.

### Decision

"kotowari overview build" and "kotowari overview serve" draw pages, for each language of the language list, from the overview source data and the table of contents of that language's side. The first language's pages are written in the location of the overview pages (".kotowari/cache/overview/"), and the other languages' pages under ".kotowari/cache/overview/<tag>/", with the same names (page names: [overview build draws everything every time, writes only the files whose contents differ, and deletes files no longer produced](overview-commands.md#overview-build-draws-everything-every-time-writes-only-the-files-whose-contents-differ-and-deletes-files-no-longer-produced)). When there is one language, only that language's pages are written in the location of the overview pages, and no language directory is created.

The stale sections of a language's page are found from the guide markers of the source data of that language's side.

### Rejected options

- Keep the overview for a single user, draw it in one configured language, and handle translations of published material separately. The problem of English readers running into Japanese partway remains.

### Confirmations

- C-core-ovlang-1 When the language list is "ja" and "en", overview build writes "a.html" and "en/a.html", and the pages under "en/" are drawn with the text of the English side's source data
- C-core-ovlang-2 With three languages, an index page whose heading is the title of that language side's table of contents is written in each language's location
- C-core-ovlang-3 When the language list is only "ja", no language directory is created, and the index is written to "index.html"

## The rendering engine is passed the page's language, that language's UI strings, and the names and page locations of the other languages

Status: implemented

### Problem

view is a rendering engine that does not know kotowari, so holding defaults of the UI strings in two places, view and kotowari, would make them disagree. Also, even a reader who lands on a page in a language they cannot read can find the link to their own language's page if it is in their own language's name.

### Decision

When drawing a language's page, the rendering engine is passed that language tag, the full set of that language's UI strings, and for each other language its name and the relative path to the location of that language's pages. The name of another language is taken from "language_name" of that language's UI strings (so it is the name written in that language itself).

### Confirmations

- C-core-ovlang-4 The "html" element of each language's page carries the lang attribute of that language's tag
- C-core-ovlang-5 When the language list is "ja" and "en" and "labels.ja.language_name" is "日本語", "a.html" has an "English" link to "en/a.html", and "en/a.html" has a "日本語" link to "../a.html"

## kotowari holds the English UI strings, and other languages are written in the configuration's labels

Status: implemented

### Problem

kotowari cannot translate text that users wrote, and what kotowari owns is only the text that view writes itself. The configuration is written by an LLM, not by people, so there is no reason for kotowari to limit the languages. Filling missing keys with English would mix two languages on one page. Projects that do not use the overview also need UI strings for the language names written in the switcher lines of guide pairs, so their place cannot be under the overview configuration.

### Decision

The UI strings that kotowari holds internally are English only. The English UI strings are kotowari's English text with the values of the keys written in the configuration's "labels.en" overriding it. The UI strings of a language other than English are exactly the value of the configuration's top-level "labels.<tag>". Without a language list, pages are drawn with the English UI strings.

The keys of the UI strings are as in the following table. The "{n}" of a string that takes a number is replaced by the number when drawn.

| Key | Where it is used | Takes a number |
|---|---|---|
| language_name | The name of the language, written in switcher lines and in links to the overview in other languages | No |
| index_link | The link to the index from an overview page whose name is not in the table of contents | No |
| pages | The number of overviews of a contents group in the index | Yes |
| stale_sections | The number of stale sections in the index | Yes |
| open_items | The number of items whose label is open in the index | Yes |
| planned_items | The number of items whose label is planned in the index | Yes |
| stale_mark | The marker near the heading of a stale section | No |
| outline_stale | The marker of a stale section in the outline | No |
| superseded | The marker of a superseded reference | No |
| deferred | The marker of a deferred reference | No |
| compare_before | The heading of the before column of the compare part | No |
| compare_after | The heading of the after column of the compare part | No |
| compare_why | The heading of the reason column of the compare part | No |
| state_decided | The display of the status label decided | No |
| state_planned | The display of the status label planned | No |
| state_open | The display of the status label open | No |
| state_dropped | The display of the status label dropped | No |

### Rejected options

- kotowari holds English and Japanese translations internally and accepts only those two. The reason to limit to languages whose translations can be verified does not hold if an LLM writes them.
- Hold only the language names on the "languages" side, and keep the UI strings under the overview configuration. The place of the UI strings would split in two.

### Confirmations

- C-core-ovlang-6 When only "labels.en.pages" is set to "{n} docs", the number of a contents group in the index is drawn in the form "<number> docs", and the other UI strings stay kotowari's English text
- C-core-ovlang-7 The UI strings of a language other than English are exactly the value written in "labels.<tag>"

## If the UI string configuration is missing something or a number is placed wrongly, it stops with a configuration error

Status: implemented

### Problem

Filling missing keys with English would mix two languages on one page. The order of numbers and words changes with the language, so the place where the number goes must be marked exactly once in the string.

If the stops on "labels" for a language not in the language list and on keys of "labels.en" not in the table were dropped, a typo in a language tag or a key would remain silently, used by no page. The reason for stopping on these two is not on record.

### Decision

It stops with a configuration error in the following cases.

- For a language other than English in the language list, "labels.<tag>" is absent or lacks one of the keys of the table
- "labels.<tag>" of a language other than English, or "labels.en", has a key not in the table
- "labels" has a key for a language tag not in the language list
- The value of a key that takes a number does not contain exactly one "{n}" ("labels.en" included)

### Confirmations

- C-core-ovlang-8 When the language list is only "ja" and "labels.ja" has no "stale_mark" key, it stops with a configuration error
- C-core-ovlang-9 When "labels.<tag>" of a language other than English has a key not in the table, it stops with a configuration error
- C-core-ovlang-10 When the value of a key that takes a number has no "{n}" or has two, it stops with a configuration error
- C-core-ovlang-12 When there is no "languages" key (English only) and there is "labels.ja", it stops with a configuration error
- C-core-ovlang-13 When "labels.en" has a key not in the table, it stops with a configuration error

## On another language's page, the body of a reference is taken from the decision document on that page's language side

Status: proposed

### Problem

References of overview parts point at a decision's heading or a confirmation ID ([The overview's source data points at decision documents](languages-and-guides.md#the-overviews-source-data-points-at-decision-documents)). Decision documents can also be kept as language pairs ([Decision documents can also be kept as multilingual pairs](languages-and-guides.md#decision-documents-can-also-be-kept-as-multilingual-pairs)). The body opened by a reference is of no use unless it is in a language the reader of that page can read. Opening an untranslated body on another language's page gives text that readers of that language cannot read.

### Decision

On a language's page, the body of a reference that points at a decision is taken from the same decision on that page's language side of that decision document. What is written in a reference is decided in [References inside parts are written in refs and ref fields](overview-data.md#references-inside-parts-are-written-in-refs-and-ref-fields), and what is shown in the body in [The reference table holds, for each reference, the body of what it points at](overview-commands.md#the-reference-table-holds-for-each-reference-the-body-of-what-it-points-at).

### Confirmations

- C-core-ovlang-11 With two languages and decision documents in pairs, the body of a reference that points at a decision is taken from the first language's side on the first language's pages, and from the same decision on that language's side on another language's pages
