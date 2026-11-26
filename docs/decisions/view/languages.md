# Page language

English | [日本語](languages.ja.md)

Decides that view indicates the page's language, that view takes the text it writes from the UI strings, the links to the same page in other languages, and how references without a body are drawn. view holds no text of any language inside it and is drawn once per language.

## The page's lang attribute and the text view writes follow the render input's language

Status: implemented

### Problem

On the overview pages, both the UI strings (such as count labels and the stale-section marker) and the lang attribute of the "html" element were fixed to Japanese, making the UI unreadable for users who write in English. On the other hand, kotowari cannot translate text the user wrote. What kotowari owns is only the text view writes itself.

### Decision

view always sets the lang attribute of the "html" element of an HTML page to the language tag of the render input.

All text view writes itself is taken from the UI strings. Text that comes from the documents, the reference table, the contents or the other languages is drawn as it is, without translation.

The text view writes itself is as follows, and a string that takes a number has "{n}" where the number goes ([Strings that take a number have the number put where "{n}" is](#strings-that-take-a-number-have-the-number-put-where-n-is)).

| Text view writes | Where it is used | Takes a number |
|---|---|---|
| Number of documents under a contents group | The contents group headings on the index page | Yes |
| Number of stale sections | The document items and the contents group headings on the index page | Yes |
| Number of items labelled "open" | The document items and the contents group headings on the index page | Yes |
| Number of items labelled "planned" | The document items on the index page | Yes |
| Link to the index page | The page of a document whose name is not in the contents | No |
| Stale-section marker | Near the heading of a stale section | No |
| Outline stale-section marker | The outline items for stale sections | No |
| Superseded marker | References whose state is "superseded" | No |
| Deferred marker | References whose state is "deferred" | No |
| Column headings for before, after and reason | The compare part | No |
| Display of the status labels (the four "decided", "planned", "open", "dropped") | The status part | No |
| Language name | Not used by view (the names of the other languages are decided outside view) | No |

Where and how each string is drawn is decided in the decisions of the topics [the index page](navigation.md), [document pages](rendering.md) and [parts](parts.md).

### Confirmations

- C-view-languages-1 When drawn with the language tag set to "fr", every HTML page has the lang attribute of the "html" element set to "fr"
- C-view-languages-2 When drawn with the value of the stale-section marker in the UI strings changed, that value is drawn near the stale section

## view holds no UI strings itself and receives them from the render input

Status: implemented

### Problem

view is a rendering engine that does not know kotowari. Holding the defaults of the UI strings (the English strings and the rules for when they are omitted) in two places, view and kotowari, lets them disagree.

### Decision

The render input is always given a language tag and a complete set of UI strings. view holds no UI strings of any language inside it. Only kotowari's side holds the English strings and the rules for when they are omitted.

Of the text view writes on a page, view takes nothing that does not come from the documents, the reference table, the contents or the other languages from anywhere other than the UI strings.

### Confirmations

- C-view-languages-3 When a document that uses all 8 kinds of parts and has stale sections and superseded and deferred references is drawn with UI strings whose values are all marked strings, every text on the page that does not come from the documents, the reference table, the contents or the other languages is a value of the UI strings (including ones with a number put in)

## Strings that take a number have the number put where "{n}" is

Status: implemented

### Problem

The order of number and words changes with the language. If view decides whether to put the number before or after the text, some languages end up with an unreadable order.

### Decision

A UI string that takes a number has "{n}" written where the number goes. When view draws it, view replaces "{n}" with that number. Whether there is exactly one "{n}" is checked by kotowari's side when it reads the configuration (decided in kotowari's configuration topic).

### Confirmations

- C-view-languages-4 When the string for the number of documents under a contents group is set to "全{n}件" and three documents are placed under a contents group, "全3件" is drawn in that contents group's heading

## Every page links to the same page in the other languages, without scripts

Status: implemented

### Problem

A project that handles multilingual documents builds the overview per language. Readers of each language need to read the pages in their own language and be able to move to other languages. On the other hand, we do not want to change [the index page's decision not to include scripts in any page](navigation.md#no-page-that-view-returns-contains-a-script).

### Decision

The overview writes the pages of the first language in the current location, and the pages of the other languages under "<language tag>/" with the same structure (writing the pages is kotowari's side, decided in kotowari's overview topic).

On every HTML page, view draws, for each of the other languages, a link to the page with the same name in that language's location. If there are no other languages, no link is drawn. No mechanism to remember the chosen language is built, and [no page contains a script](navigation.md#no-page-that-view-returns-contains-a-script).

### Confirmations

- C-view-languages-5 When the other languages have one entry with location "en/" and there is a document "a", "a.html" has a link to "en/a.html" and "index.html" has a link to "en/index.html"
- C-view-languages-6 Even when there are other languages, no page has a script element
- C-view-languages-7 When the other languages are empty, neither "a.html" nor "index.html" has a link to a page in another language

## A link to another language is labelled with that language's name written in that language

Status: implemented

### Problem

A reader who lands on a page in a language they cannot read cannot move to the page in their own language if they cannot read the link text.

### Decision

The text of a link to a page in another language is the name of the target language, written in that language itself. The name is taken from one key of that language's UI strings. view uses the name that an entry of the other languages carries as the link text, as it is.

### Confirmations

- C-view-languages-8 When the name of an entry of the other languages is "English", the text of the link to that language's page is "English"

## A reference without a body is drawn as its display name only and opens nothing when selected

Status: implemented

### Problem

Decision records are not translated. If a decision record's reference is opened on a page of a language other than the first, text the page's reader cannot read appears. For the reader, nothing opening is better than unreadable text appearing.

### Decision

When an entry of the reference table has no body, view draws that reference with its display name only and makes selecting it open nothing. Which references get no body (the decision-record references on pages of languages other than the first) is decided by kotowari's side when it builds the reference table (decided in kotowari's overview topic).

### Rejected options

- On pages of languages other than the first too, open the decision text of the decision record in the original. It would be text the page's reader cannot read.

### Confirmations

- C-view-languages-9 When the reference table has one entry whose display name is "x A1" and that has no body, and a part that has it in refs is drawn, "x A1" is drawn on the page and there is no element that opens when selected
