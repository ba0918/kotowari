# Index and navigation

English | [日本語](navigation.ja.md)

Decides the status counts that view adds to the index page, how contents groups are drawn, the means of moving from a document page to other places in the contents, and the means of moving from the outline to a section within a document page. Even when the contents disagree with the documents, view draws them in a fixed way without checking. [How the index page itself is drawn](rendering.md#draw-the-index-on-one-page-in-the-order-and-nesting-written-in-the-table-of-contents) and [the order of title, outline and lead](rendering.md#draw-a-document-page-in-the-order-title-outline-lead-opening-parts-sections) are decided in the decisions of the rendering topic.

## Index cards show a document's status counts, only when they are not 0

Status: implemented

### Problem

The index page is the entry point for finding the page one wants to read, and the place that shows, as a map of the product, the groupings and order of features. Making readers notice pages that need review or that still have open items supports that. Yet the index cards had only the title and the conclusion of the lead, and each page's status (unreviewed sections, open items, planned items) could not be seen from the index.

Status is a supporting aid, so only as much is shown as can be noticed. view already receives whether a section is stale and the contents of the status part, so no render input needs to be added for counting.

### Decision

Besides the title and the conclusion of the lead, an index card shows the number of stale sections of that document, the number of items of the [status part](parts.md#the-kinds-of-parts-are-a-fixed-list-of-eight-held-by-view) whose label is "open" (open), and the number of items whose label is "planned" (planned). Each is shown only when it is not 0. view does the counting.

The text added is taken from the UI strings. A UI string that takes a number contains exactly one "{n}", and view draws it with that replaced by the number ([Strings that take a number have the number put where "{n}" is](languages.md#strings-that-take-a-number-have-the-number-put-where-n-is)).

### Confirmations

- C-view-navigation-1 An index card shows, among the number of stale sections, the number of open items and the number of planned items, only those that are not 0, in UI strings with "{n}" replaced by the number

## A contents group's heading shows the number of all documents under it and the status totals

Status: implemented

### Problem

The number of pages in the index has no upper limit, and contents groups can be nested without a depth limit. If a group's heading has totals, the status inside can be noticed even when the group is collapsed. For that, counting only the direct children is not enough.

### Decision

A contents group's heading shows the number of all documents under it regardless of nesting depth, the total of stale sections of those documents, and the total of open items. The total of planned items is not shown.

The outermost level of the contents is also treated as one contents group, and the index heading shows the same. As with the cards, the text is taken from the UI strings.

### Confirmations

- C-view-navigation-2 A contents group's heading shows the number of all documents under it regardless of nesting depth, the total of stale sections, and the total of open items
- C-view-navigation-3 The index heading (the outermost level of the contents) also shows the number of documents and the totals of stale sections and open items, and does not show the total of planned items

## Contents groups are drawn open and collapse when their heading is selected

Status: implemented

### Problem

The index is one page, nests the contents to show every page, and puts no upper limit on the number of pages. The reasons for making groups collapsible, for starting in the open state and for not remembering the collapsed state have not been kept.

### Decision

On the index page, every contents group in the contents is drawn in the open state, and selecting its heading collapses its contents. The collapsed state is not remembered.

### Confirmations

- C-view-navigation-4 On the index page, every contents group in the contents is drawn as an element that is open and whose contents collapse when its heading is selected

## No page that view returns contains a script

Status: implemented

### Problem

The reason for deciding not to include scripts has not been kept.

### Decision

No page that view returns contains a script.

### Confirmations

- C-view-navigation-5 No page that view returns has a script element

## A document page shows its position in the contents above its title

Status: implemented

### Problem

With contents, pages are connected through groups. No further reason has been kept.

### Decision

On the page of a document whose name is in the contents, the titles from the contents down to the contents group that contains that document are listed above the title, from the outside in. Each title links to the place of that contents group on the index page.

### Confirmations

- C-view-navigation-6 Above the title of the page of a document whose name is in the contents, the titles from the contents' title down to the title of the contents group containing that document are listed from the outside in, and each links to the place of that contents group on the index page

## Links to the other documents of the same contents group are listed after the last section

Status: implemented

### Problem

With contents, pages are connected through groups. Adding a separate declaration that connects related pages increases the effort of maintaining it.

### Decision

On the page of a document whose name is in the contents, links to the other documents that belong to the same contents group as that document are listed after the last section, in the order written in the contents. No link to the document itself is listed.

These links are not put in the outline but placed after the last section ([Document pages get an outline listing the section headings](#document-pages-get-an-outline-listing-the-section-headings)).

### Rejected options

- Build a mechanism that declares related pages in the source data. The contents are enough, and adding declarations adds maintenance.

### Confirmations

- C-view-navigation-7 After the last section of the page of a document whose name is in the contents, links to the other documents belonging to the same contents group are listed in the order written in the contents, and there is no link to the document itself

## Input that disagrees with the contents is drawn in a fixed way, not treated as an error

Status: implemented

### Problem

Checking for disagreement between the contents and the source data is kotowari's side, and view does not check the input it is given ([view does not check its input and uses neither files nor the network](rendering.md#view-does-not-validate-its-input-and-uses-neither-files-nor-the-network)). Still, view is a public crate, so how to draw disagreeing contents passed from something other than kotowari also needs to be decided. Even for disagreeing contents, the way of drawing is fixed to one.

### Decision

view draws the contents as given and does not treat disagreement as an error.

- A name that is in the contents but has no document is skipped. It is not drawn in the index, not counted in the contents group headings' numbers, and not shown among the links to the other documents of the same contents group.
- A document whose name is not in the contents still gets its page returned. It is not shown in the index. Above that page's title, instead of the position in the contents, only a link to the index page is shown.
- A name that appears in the contents two or more times is drawn each time it appears. The position in the contents for that document's page is the place where it first appears when the contents are traversed depth-first in the order written.

### Confirmations

- C-view-navigation-8 A name that is in the contents but has no document appears neither in the index, nor in the contents group headings' numbers, nor in the links to the other documents of the same contents group, and is not an error
- C-view-navigation-9 The page of a document whose name is not in the contents is also returned, is not shown in the index, and above that page's title there is only a link to the index page
- C-view-navigation-10 A name that appears twice in the contents is drawn twice in the index, and the position in the contents for that document's page is the place where it first appears when the contents are traversed depth-first in the order written

## Document pages get an outline listing the section headings

Status: implemented

### Problem

On the page of a long document with many sections, it was unclear where one was reading and how to move to other sections.

### Decision

A document page with one or more sections gets an outline. No threshold on the number of sections is made. A threshold would be a number without grounds and would only add rules.

The outline lists the heading text of the sections ("## " headings) as it is, in section order (the order written in the source data). Using text different from the heading would make it impossible to tell which section an item corresponds to, and reordering would diverge from the top-to-bottom order of the body. Each item links to that section within the page. The link targets do not overlap within one page, and even when two sections have the same heading they are different places.

The following are not put in the outline.

- An item for the opening (the lead and the opening parts that follow the lead). The opening is right below the title and is visible by going back to the top of the page.
- "### " and deeper headings inside a section. The section is the unit that carries whether it is stale, and listing the deeper headings too makes the outline long and blurs its role.
- Links to other pages. The outline is limited to moving within that page.

The index page gets no outline. The index draws the contents itself, and listing them again would repeat the same thing.

The section being read is not highlighted as the page scrolls. That would need a script ([No page that view returns contains a script](#no-page-that-view-returns-contains-a-script)).

### Confirmations

- C-view-navigation-11 The outline of a document page with one or more sections lists the section heading texts in section order, and has no "### " or deeper headings and no item for the opening
- C-view-navigation-12 Each outline item links to that section within the page, and even when two sections have the same heading the link targets are different places
- C-view-navigation-13 A document page without sections and the index page have no outline

## Outline items for stale sections carry the stale-section marker

Status: implemented

### Problem

Finding the sections that need review on a long page is the main use of the outline.

### Decision

Among the outline items, an item for a stale section gets, after the heading text, a marker with the same meaning as the stale-section marker placed near the section heading ([A stale-section marker is drawn near the heading of a stale section](rendering.md#draw-a-stale-section-marker-near-the-heading-of-a-stale-section)). The text is taken from the UI strings.

### Confirmations

- C-view-navigation-14 In the outline, only the items for stale sections get the stale-section marker after the heading text

## On wide screens the outline stays visible to the left of the body; on narrow screens it sits below the title

Status: implemented

### Problem

Even on a long page with many sections, where one is and the means of moving to other sections should always be visible (in the user's words: "if the outline can always be shown, even long pages are easy to read"). On the other hand, fixing it on the left on a narrow screen takes up the body's width.

### Decision

On a wide screen, the outline is placed to the left of the body and stays visible while the body scrolls. On a narrow screen, the outline is placed below the title and above the opening lead ([A document page is drawn in the order title, outline, lead, opening parts, sections](rendering.md#draw-a-document-page-in-the-order-title-outline-lead-opening-parts-sections)) and does not follow scrolling.

What changes with screen width and scrolling cannot be confirmed by unit tests of the string view returns, so a person confirms it by looking at the page drawn at a wide width and at a narrow width.

### Confirmations

- C-view-navigation-15 (by reading) A person opens a document page with many sections in a browser and confirms by looking that at a wide width the outline is to the left of the body and stays visible while the body scrolls, and at a narrow width it is below the title and above the lead and does not follow scrolling
