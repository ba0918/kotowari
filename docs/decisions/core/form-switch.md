# Switching from the old form

English | [日本語](form-switch.ja.md)

This document decides how kotowari itself is switched from the two layers of IR and decision records to the form of [decision documents](decision-documents.md). Once the switch is done, the decisions of this topic have served their purpose, so this document is deleted (it remains in the git history).

## Switch at once in a new version, and build no mechanism to read the old form

Status: proposed

### Problem

kotowari's own IR has 70 topics, with 434 requirements, 504 scenarios, and 73 decision tables and properties. There are 94 decision record files. These are to be moved into the form of decision documents.

Two topics were actually rewritten into the prose form, and the effort was measured. Per topic, the agent's work was about 100,000 tokens (a topic with 6 requirements and 14 scenarios became 8 decisions and 22 confirmation rows; a topic with 10 requirements and 14 scenarios became 12 decisions and 19 confirmation rows). For all 70 topics, writing the documents alone comes to around 7 million tokens. In another experiment within this study, a cost estimated from a trial with something light was off by a factor of two, so up to twice that is allowed for.

On the other hand, some topics have no role in the new form (the source check, the record form check, links to amendments, deferral declarations, reconciliation, and so on). Rewriting the topics that disappear would be wasted.

### Decision

kotowari itself switches at once in a new version. There is no period in which check reads both the old form and the new form. Repositories not yet moved keep using an old version of kotowari.

In the switch version, the new check and workflow are built first. Topics that have no role in the new form are moved, without rewriting, to the frozen tree ([Decision documents live in docs/decisions, and old-form documents move to a frozen tree](decision-documents.md#put-decision-documents-in-docsdecisions-and-move-documents-in-the-old-form-to-the-frozen-tree)). Only the remaining topics are rewritten into the prose form. Test markers are reassigned by machine from the mapping table made during the rewrite (which new confirmation ID each old requirement and scenario ID corresponds to).

### Rejected options

- check reads both the old form and the new form for the time being. The user judged that the cost of maintaining compatibility is wasted.
- Switch only check and the workflow, freeze the old topics, and rewrite them when they are touched. Until rewritten, check does not count the requirements of frozen topics, and the guarantee that tests exist stops for that part.
- Rewrite all 70 topics before switching. The topics that disappear in the new form would be deleted right after being rewritten.

### Confirmations

- C-core-switch-1 After the switch, check does not read the IR location or the decision record location, and reports no error even if old-form documents exist
- C-core-switch-2 (by reading) In the switch version, topics that have no role are in the frozen tree without being rewritten, and the remaining topics are in the decision document location
- C-core-switch-3 (by reading) After the switch, no markers pointing at old-form IDs (requirements, scenarios, decision tables, properties) remain in the tests

## ADRs are discontinued, and the existing ones are frozen

Status: proposed

### Problem

Until now, both decision records and ADRs (documents that collect design decisions and their reasons for people who read them later) were kept. A decision document has a problem, a decision and rejected options for each decision, so it has the same role as an ADR. Keeping both writes decisions and reasons in two places.

### Decision

ADRs are discontinued. The existing ADRs are moved to the frozen tree together with the decision records. The `decisions.adr` key in the configuration is removed.

### Rejected options

- Previous decision: keep both decision records and ADRs, and do not delete existing ADRs. Dropped because the place to read decisions and reasons becomes the single place of the decision documents.

### Confirmations

- C-core-switch-6 After the switch, check does not read the ADR location and does not require the `decisions.adr` key in the configuration
- C-core-switch-7 (by reading) In the switch version, the ADRs up to then are in the frozen tree

## When rewriting, nothing missing from the records is invented, and details that cannot be told apart are dropped

Status: proposed

### Problem

When two topics were rewritten, decisions whose rejected options were not in the records were more than half (6 of 8, 8 of 12). There were also 2 to 3 details per topic for which the records could not tell whether the person or the agent decided them.

### Decision

When rewriting, if the rejected options are not in the records, the whole section is not written. If the reason is not in the records, "no reason remains" is written in the Problem section. Neither is invented afterwards.

Details for which who decided cannot be told apart are not put in the decision text, and are left to the code and tests. The dropped details are kept as a list in the mapping table. The mapping table is placed in the frozen tree. This is because every document under the decision document location is read as a topic document.

Of the dropped details, those that concern safety (those whose dropping would make a dangerous behaviour disappear from the decisions) are written as decisions and confirmation rows when a current test or a skill's text checks them.

Places where code or tests point at frozen documents as grounds are swept when moving, and rewritten into the current decision documents or into code comments.

### Rejected options

- Keep details that cannot be told apart in the decision text. There is no worry of them disappearing, but agent details get mixed into the decision documents, and values the person did not decide count as approved.

### Confirmations

- C-core-switch-4 (by reading) For each rewritten topic, there is a mapping table that holds the correspondence between old IDs and new IDs and the list of dropped details
- C-core-switch-5 (by reading) No places remain in the code and tests that point at frozen-tree documents as grounds
- C-core-switch-8 (by reading) The mapping table of a rewritten topic has a list of the safety-related dropped details that were returned to the decisions (the detail, the new ID, and what checks it)

## Two points are placed during the switch where the user looks at the real thing and judges

Status: proposed

### Problem

Whether this form really reduces the effort of changing rules, and whether it is really readable when coming back, cannot be judged until the real thing is used. What has been confirmed is that one light request could be implemented from a prose-form document, and the samples of rewriting two topics. Whether a heavy decision can be implemented from the prose form, whether the flow of overturning a decision works, and whether the flow of deciding visible behaviour with a sample works have not been confirmed even once. Finding out it does not work after rewriting everything comes after most of the cost has been paid.

### Decision

Whether the scenes are solved is judged by the user looking at the real thing at the following two points.

1. The specification of the form and the workflow itself, including this topic, is written in the prose form and put up for approval. The user reads it, and does not approve it if it cannot be read.
2. The check and workflow skills that read the new form are built, only the smallest group (view; 4 topics, 30 requirements) is rewritten, and on top of that one real small change is taken through, starting from changing a rule. Work stops there and the result is shown to the user. What is shown is how many steps changing a rule took, whether the flow of overturning a decision was performed, whether implementation from the prose form was possible, and the list of details dropped in the rewrite.

If the second point is good, the remaining topics are rewritten. If not, it goes back to a brainstorm to fix the form. Work up to the second point is on a branch and does not go into main. The version released stays one.

### Rejected options

- Judge only once, at approval, and proceed with the rest without stopping. The writing side finds out it does not work only after rewriting everything.
- Before building check, take one change through with only hand-rewritten documents and a manual procedure, and judge. Without check, whether counting and checks work is not known at that point.
- First run an experiment comparing the current form and the prose form on several requests that include heavy decisions. Several documents to compare would have to be rewritten first, and the cost is large.

## Open

- Whether, with what was decided, the writing side's scene (the effort of changing one rule) and the reading side's scene (readable when coming back) are solved. The user decides. The first point at the approval of this specification; the second point during the work on the switch version.
- Whether to have, as part of the product, a procedure for moving other repositories that use kotowari to the form of decision documents (whether to make it a command or skill, or only show an example of a move). The user decides. After the second point, having seen the procedure used to move kotowari itself.
