# The process that writes the overview

English | [日本語](overview-workflow.ja.md)

Decides how the process skills write the overview source data, the unit of an overview, how the table of contents is placed, the step that rebuilds the overview in the brainstorm approval, and that the implementation process does not fix the overview source data. It holds only decisions that a person or an LLM confirms by reading the text of kotowari's skills. The checks of the source data and the behavior of build and serve are decided in [Overview source data](overview-data.md) and [Commands that build and show the overview](overview-commands.md), and that the source data points at decision documents in [The overview's source data points at decision documents](languages-and-guides.md#the-overviews-source-data-points-at-decision-documents).

## The overview source data starts with the opening conclusion, and shows the chain of decisions, the behavior and the current state with parts suited to the subject

Status: implemented

### Problem

A brainstorm settles decisions one question at a time, so each question is understandable to a person. But once decisions number in the tens to over a hundred, a person cannot picture the final shape that combines them all, and can no longer explain afterwards what was built and why it came out that way. As the scale grows, both the connections between decisions and the shape of what is built (how it behaves as seen by the people who use it) become unclear.

When the same subject was laid out side by side in a form that explains the summary and topic groups in prose and a form that uses many diagrams, tables and decorations, the prose form was hard to understand, and the form rich in visual information was entirely different. A conclusion and summary at the start was the right composition. On the other hand, the best form of explanation depends on the subject, so the composition cannot be fixed.

The source data is seen through rendering, and people read only the drawn pages. Making it from scratch every time brings back the cost and the drift of the look. Making a snapshot of the moment at each brainstorm turns into a lie when a later brainstorm replaces it.

The readers of the overview are the people who build the product (the person who brainstormed, the implementer, the reviewer), and the readers of guides are the people who use the product. Even for the same feature they answer different questions, so writing them separately means the same explanation does not have to be maintained in two places. There is no way to judge by machine whether a section is usage or decisions, so the separation is not made a check but a writing guideline.

### Decision

kotowari's skill has a scene for writing the overview source data, and asks for the following way of writing. The source data is written by an LLM, and people read only the drawn pages.

- Right after the title, place a lead part of conclusion and summary.
- Choose and arrange diagram, table and decoration parts to suit the subject. The arrangement of parts is not fixed.
- Show both the connections between decisions and the behavior that results from the decisions. The behavior is only shown briefly; usage procedures (commands, options, configuration examples) are left to the guides, and the overview only mentions the names of features.
- Show the current state, what was decided but not built yet (decisions whose status is "proposed"), and what is open.
- Write by revising the previous source data, not by rewriting from scratch.
- Next to a section's heading, attach guide markers pointing at the decisions that section explains.
- Write the references of parts in "refs" or "ref" fields.

The separation of the overview and guides comes with examples of the boundary. A diagram of the flow of a process is behavior resulting from decisions, so it may go in the overview, while how to write commands and examples of options go in the guides.

### Rejected options

- A form that lists the summary, topic groups, risk and certainty, and items in prose. It was hard to understand when compared.
- Fix the arrangement of parts as conclusion, summary, flow chart, three-column table and so on. The best form depends on the subject.
- Make it temporary data made from scratch at every approval. The cost and the drift of the look come back.
- Make a snapshot of the moment at each brainstorm. It turns into a lie when a later brainstorm replaces it.
- Drop the overview and fold it into the guides. There would be no place to show the connections between decisions, the plans and what is open.
- Drop the guides and fold them into the overview. The overview is HTML drawn locally, cannot be read on GitHub, and does not link outside the page.

### Confirmations

- C-core-ovflow-1 (by reading) kotowari's skill has a scene for writing the overview source data, which says to place a lead of conclusion and summary right after the title, to choose the kinds and arrangement of parts to suit the subject, and to show both the connections between decisions and the behavior that results from the decisions
- C-core-ovflow-2 (by reading) The same scene says to show the current state, decisions whose status is "proposed", and what is open, to write by revising the previous source data, to attach guide markers pointing at decisions next to section headings, and to write references in "refs" or "ref" fields
- C-core-ovflow-3 (by reading) kotowari's skill states, with examples of the boundary, the separation that the overview holds what is decided and why, the plans and what is open, and that usage procedures are left to the guides

## The unit of an overview is something users can call one feature by name, and a person decides it

Status: implemented

### Problem

If the unit of an overview is left to the LLM every time, the way it is cut drifts. Basing the unit on the topics of the specification is reasonable, and making it something users can call one feature by name gives it the same shape as guides. Making the overviews of existing topics all at once means paying in advance the cost of topics nobody touches.

### Decision

Overviews are kept in units based on the topics of the decision documents. The guideline for a unit is something users can call one feature by name (examples: guide markers, adoption, mutation testing). Things that cut across features (examples: the shape of output, configuration) are not made overviews of their own, and are mentioned inside the overviews of the features involved.

When making an overview for the first time, and when a document of a topic that belongs to no overview is changed, the LLM proposes a unit (add it to an existing overview, or make a new one) and a person decides. A decided unit is not changed except when a brainstorm decides to split or merge it. Overviews of topics not touched are not made all at once.

### Rejected options

- Make overviews of all topics when the feature is introduced. It pays in advance the cost of topics nobody touches.

### Confirmations

- C-core-ovflow-4 (by reading) kotowari's skill says that the guideline for an overview's unit is something users can call one feature by name, with cross-cutting things not made units of their own, that when a document of a topic that belongs to no overview is changed the LLM proposes a unit and a person decides, that a decided unit is not changed unless a brainstorm decides so, and that overviews of topics not touched are not made all at once

## The place in the table of contents is decided by the LLM that writes the overview without asking a person, and is not subject to approval

Status: implemented

### Problem

Where to put a new page in the table of contents is not as weighty a decision as the unit of an overview. Proceeding without waiting for a person is the ideal, and if something feels off, the person can fix it when they look. If the index is to be a map of the product, the order itself, such as the order of use or the order of the process, is information. One level is a special case of a hierarchy, so a small product can stay at one level.

### Decision

Where to put a new overview in the table of contents, and whether to split a contents group or make a new one, are decided by the LLM that writes the overview, which updates the table of contents without asking a person. The table of contents is not subject to approval. A contents group is split only after it has grown large. Within the same level, items are ordered in an order meaningful to the reader (the order of use or the order of the process).

### Confirmations

- C-core-ovflow-5 (by reading) kotowari's skill says that the LLM that writes the overview decides the place of a new overview in the table of contents and whether to split a contents group or make a new one without asking a person, that the table of contents is not subject to approval, that a group is split only after it has grown large, and that items within the same level are ordered by the order of use or the order of the process

## The brainstorm approval procedure has a step that rebuilds the overview, and the overview is not subject to approval

Status: implemented

### Problem

Brainstorm decisions are understandable one question at a time, but it often becomes unclear what it all comes to. The overview is one deliverable used both at approval time and when reading back later, and at every brainstorm approval the overviews involved are made or revised. Making it every time during a brainstorm costs money every round. Building it into the approval flow makes approval heavy, and heaviness was one of the reasons the summary (brief) made earlier did not take hold.

cycle and implementation do not fix the overview source data ([cycle and implementation do not fix the overview source data](#cycle-and-implementation-do-not-fix-the-overview-source-data)), so without this step in the brainstorm, nobody makes or fixes the overview, and stale sections remain forever.

### Decision

Only in projects whose configuration has the "overview" key, the brainstorm approval procedure has a step that rebuilds the overview. In the step, the following is done in this order.

1. From the decision documents changed in this brainstorm, determine the overviews involved by the lists of the source data. For a decision document of a topic that belongs to no overview, the LLM proposes whether to add it to an existing overview or make a new one, and a person decides ([The unit of an overview is something users can call one feature by name, and a person decides it](#the-unit-of-an-overview-is-something-users-can-call-one-feature-by-name-and-a-person-decides-it)).
2. Make or revise the source data of the overviews involved, and if a new page was added, update the table of contents too ([The place in the table of contents is decided by the LLM that writes the overview without asking a person, and is not subject to approval](#the-place-in-the-table-of-contents-is-decided-by-the-llm-that-writes-the-overview-without-asking-a-person-and-is-not-subject-to-approval)). guide_stale notices on the overview source data are fixed here.
3. Run "kotowari overview build" and fix the errors.
4. Show the URL of "kotowari overview serve" for reference.

The overview is not subject to approval, and whether to look at it is left to the person. The approval material (the diff of the decision documents) and the reading order do not change. The overview of an existing topic is made for the first time when a brainstorm touches that topic's decision document.

### Rejected options

- Have the overview opened first in the approval scene. Approval becomes heavy.
- Make the overview subject to approval too. Approval becomes heavy.
- Make the overview every round during the brainstorm. It costs money every round.

### Confirmations

- C-core-ovflow-7 (by reading) The approval procedure of the brainstorm skill has a step that, only in projects whose configuration has the "overview" key, determines the overviews involved from the changed decision documents, makes or revises the source data and the table of contents, fixes errors with "kotowari overview build", and shows the URL of "kotowari overview serve" for reference
- C-core-ovflow-8 (by reading) The same step says that for a decision document of a topic that belongs to no overview the LLM proposes a unit and a person decides, that guide_stale on the overview source data is fixed in this step, and that the overview of an existing topic is made for the first time when a brainstorm touches that topic
- C-core-ovflow-9 (by reading) The brainstorm skill says that the overview is not subject to approval, and that the approval material and the reading order do not change

## cycle and implementation do not fix the overview source data

Status: implemented

### Problem

Even when sections of the overview source data become stale, readers can tell the stale parts apart by the markers on the page. The work of cycle is not to be increased. Fixing them is done by the approval step of the next brainstorm that touches that topic ([The brainstorm approval procedure has a step that rebuilds the overview, and the overview is not subject to approval](#the-brainstorm-approval-procedure-has-a-step-that-rebuilds-the-overview-and-the-overview-is-not-subject-to-approval)).

### Decision

The cycle and implementation processes do not fix the overview source data. They do not fix it even to clear guide_stale notices on the overview source data.

### Confirmations

- C-core-ovflow-6 (by reading) The cycle skill says that the overview source data is not changed during the run and that its findings and guide_stale are not delegated, and the cycle and implementation skills have no procedure for fixing the overview source data
