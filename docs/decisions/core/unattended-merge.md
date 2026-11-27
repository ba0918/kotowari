# Merging without waiting for a person

English | [日本語](unattended-merge.ja.md)

This document covers how the workflow skills carry one request from the approval of its specification to the merge without waiting for a person: what the cycle takes as input, when a plan is written, by whom, and when it is deleted, the line that decides whether a person confirms the merge, how the merge is made, and what is kept in the findings file about the run. Writing the details the implementing side decided, with what, why and the evidence, in the commit body and the pull request body is decided in the workflow topic's [Details found during implementation are decided by the implementing side](workflow.md#details-found-during-implementation-are-decided-by-the-implementing-side). The step inside the cycle that looks at agreement with the specification is decided in [Consistency phase](consistency-phase.md).

## The brainstorm and the approval of the specification stay with the person, and from then on to the merge nothing waits for a person

Status: implemented

### Problem

In the earlier workflow, a person came in four times during one request: answering the brainstorm, approving the specification, approving the plan, and the final check and merge. The user wants to come closer to a form where, once the first goal is handed over, it runs to completion with LLMs alone. Of the four, the approval of the plan and the final check can be handed to the machine and to LLMs once the line between merges that need a person's confirmation and merges that do not is decided. Removing the approval of the specification as well would remove the scene where a person reads the specification and stops a mistake in it.

Handing something back to a person does not mean the person can then steer it correctly. Most things can be decided by measurement or by the machine's judgment, and only a part truly needs a person's judgment. In a form where a person calls the implementation after the approval, one scene of starting it by hand remains for each request. Making only the cycle run without a person leaves small tasks waiting for a person in the same places as now.

### Decision

The brainstorm and the approval of the specification stay with the person, and from then on the request is carried to the merge without waiting for a person. Only what cannot be decided without a person is handed back to a person. What stays with the person is what to build (the brainstorm and the specification), operations that cannot be undone, and the root of the yardstick (the list that decides the line, and the definitions of the checks). Everything else is decided by measurement and the machine's judgment; what cannot be measured is decided by an LLM with its reason left, and the person can read it afterwards. That something cannot be measured by machine is not a reason to send it to a person.

The point at which the person has approved the specification and committed it, or said it may be committed, is the signal that implementation may start, and the brainstorm goes on to the cycle in the same flow. When the person wants to put it off, they say so.

What is carried to the merge without waiting for a person is a request the cycle runs and a small task kotowari-iterate runs. A change the main session edits directly ends at the commit.

### Rejected options

- Leave the approval of the specification to the machine too, under conditions. The scene where a person reads the specification and stops a mistake in it would be gone.

### Confirmations

- C-core-merge-1 (by reading) The kotowari-brainstorm skill says that when the person has committed the approved specification, or said it may be committed, it goes on to kotowari-cycle in the same flow unless the person says to put it off
- C-core-merge-2 (by reading) The kotowari-iterate skill says that a converged run merges in the same way as the cycle, and the `references/merge.md` of kotowari-cycle says that a change the main session edits directly ends at the commit

## A plan is written only when the work is more than a single implementer can hold, by the implementer, committed on the working branch, and without approval

Status: proposed

### Problem

Even with the approval of the plan removed, if a step that writes a plan always remains, one more thing to write remains between the specification and the implementation. Deciding the procedure can be made the implementer's job.

On the other hand, when a plan is only the implementer's uncommitted working notes, the remaining procedure is not left in git when the implementer's context runs out on a large request. The implementer that takes over has only the git history and the decisions still "proposed" as clues. The request that removed the code reading the old form was more than one implementer could hold, and stopping at a boundary and reporting was added by hand to the delegation text to make up for it. The command that checks the form of a plan was kept, yet no skill said to write in that form.

After the approval of the specification no person comes in along the way, so there is no scene where a person takes over part way. A handover happens when the implementer changes, and its cause is the amount of work. Even when the session running the cycle changes, the implementer works in a separate context, so the amount an implementer can hold does not change.

### Decision

A plan is not handled as a deliverable to approve, and is not placed as a step of the workflow that every request passes either. There is no workflow skill that writes a plan.

A plan is written only when the work is more than a single implementer can hold. The cycle judges the amount before the first delegation, and when it judges that a plan is to be written, it writes the reason as one line in the findings file. The amount is not decided by a numeric limit. The one who writes is the implementer that received the delegation. The delegation that writes the plan and the delegation that implements are not separated. The implementer writes the plan, brings it to a form in which `kotowari plan` reports no error, commits it on the working branch, and goes on to implement. No person reads it, and no person approves it.

A plan is one per branch, placed at `docs/plans/<branch name>.md`. Its form is as decided in [A plan is written with fixed plan-wide sections and steps that have eight fields](plan.md#a-plan-is-written-with-fixed-plan-wide-sections-and-steps-that-have-eight-fields). The kotowari-implement skill points at the material that describes the form of a plan.

When an implementer finds part way that it cannot hold the work, it commits at a boundary where the tests pass and then hands back to the cycle. When it started without a plan, before handing back it writes the remaining procedure in a plan of the same place and form and commits it. The cycle delegates the rest again. When the branch already has a plan at the time the cycle starts the run, the cycle makes no judgment and passes it to the implementer.

For a small task kotowari-iterate runs, no plan is written.

### Rejected options

- Previous decision: a plan is not placed as a step of the workflow, and only when the implementer judges it necessary for a large request does it write one as uncommitted working notes. Dropped because the remaining procedure is not left in git when the implementer's context runs out.
- After the approval, write a plan for every request and run it with the cycle. The step that writes a plan would remain between the specification and the implementation.
- Leave the approval of the plan with the person. The reason it was not taken is not recorded.
- Put in one delegation that only writes the plan, and have another implementer start the implementation from the plan. The cost of reading the code is paid twice.
- The implementer judges by itself whether to write one from the start. The reason for writing before the first delegation is not left in the findings file, and nobody notices when one is written for a small amount.
- Write one also when the session running the cycle is likely to change. The amount an implementer can hold does not change, and a plan would be written even for a request with a small amount.
- Reduce the sections and fields of a plan now. The form would be decided before it is used.

### Confirmations

- C-core-merge-54 (by reading) `agent/skills/kotowari-plan/` does not exist, and the kotowari-brainstorm and kotowari-cycle skills have neither an approval of a plan nor a procedure that shows a plan to a person
- C-core-merge-55 (by reading) The kotowari-cycle skill says that before the first delegation it judges whether the work is more than a single implementer can hold, has a plan written inside the delegation of the implementation only when it judges so, and writes that reason as one line in the findings file, and it states no numeric limit that decides the amount
- C-core-merge-56 (by reading) The kotowari-implement skill says that when it is handed the writing of a plan it writes it at `docs/plans/<branch name>.md`, brings it to a form in which `kotowari plan` reports no error, and commits it before going on to implement, and it points at the material that describes the form of a plan
- C-core-merge-57 (by reading) The kotowari-implement skill says that when it finds part way that it cannot hold the work, it commits at a boundary where the tests pass, and when it started without a plan it writes the remaining procedure at `docs/plans/<branch name>.md` in a form in which `kotowari plan` reports no error and commits it, before handing back to the cycle, and the kotowari-cycle skill says that when it is handed back so it delegates the rest again
- C-core-merge-58 (by reading) The kotowari-cycle skill says that when the branch already has a plan at the time it starts the run it makes no judgment and passes it to the implementer, and the kotowari-iterate skill has no procedure that writes a plan

## A plan does not bind the implementer, and the cycle does not read what is in a plan

Status: proposed

### Problem

A step of a plan has fields for the files that may be changed and for the steps it presupposes. If these bound the implementer, every disagreement would produce a round trip of handing back to someone. If the cycle read the plan and divided the delegations by it, the plan would become the cycle's input, and the step that writes a plan would effectively come back. A plan can hold a section for the test command, so a document nobody approved could become the source that decides the check command. In a form where finished steps are written into the plan, the writer's own report becomes the progress.

### Decision

The order of implementation and the files to touch are decided by the implementer, even when there is a plan. When it finds that a way different from the plan is needed, the implementer rewrites the plan, commits it, writes the reason in the commit body, and goes on. It hands back neither to a person nor to the cycle. The "## Stop conditions" section and the "Stop and hand back if" field of a plan add no condition for stopping. Whether to stop is decided by the same rules as when there is no plan.

The cycle only passes the path of the plan to the implementer and does not read what is in it. Even when there is a plan, the delegation hands over all the remaining work at once.

No progress is written in a plan. The implementer that takes over decides what remains from "Done when" and "Shown by" of each step of the plan, the git history, and the states of the decisions. No step number is written in a commit message.

The "## Test command" section of a plan is not a source that decides the test command ([The test command is decided by the project's instructions or the language's standard tool, and the person is asked when neither settles it](#the-test-command-is-decided-by-the-projects-instructions-or-the-languages-standard-tool-and-the-person-is-asked-when-neither-settles-it)).

Even for a request with no plan, the implementer may write uncommitted working notes when it judges them necessary. No person reads them.

### Rejected options

- When a way different from the plan is needed, hand back to the cycle, and the cycle delegates the rewriting. One more round trip at every disagreement.
- The cycle divides the steps of the plan and delegates them. The plan would become the cycle's input, and the step that writes a plan would effectively come back.
- Delete finished steps from the plan as they are done. At the last step the whole plan would be deleted, and forgetting a deletion leaves a disagreement.
- Write the step number in the commit body. Numbers of a plan that disappears would remain in the history.
- Decide the test command in the order of the plan, the project's instructions, the standard tool. A document nobody approved would become the source that decides the check command.

### Confirmations

- C-core-merge-59 (by reading) The kotowari-implement skill says that the order and the files to touch are decided by the implementer even when there is a plan, and that when it finds a way different from the plan is needed it rewrites the plan, commits it, writes the reason in the commit body and goes on, handing back neither to a person nor to the cycle, and that the "## Stop conditions" section and the "Stop and hand back if" field of a plan add no condition for stopping
- C-core-merge-60 (by reading) The kotowari-cycle skill says that it only passes the path of the plan to the implementer without reading what is in it, and that even when there is a plan it delegates all the remaining work at once
- C-core-merge-61 (by reading) The kotowari-implement skill says that no progress is written in a plan, that no step number is written in a commit message, and that on a resume it decides what remains from "Done when" and "Shown by" of the steps of the plan, the git history, and the states of the decisions
- C-core-merge-62 (by reading) The kotowari-implement skill says that the "## Test command" section of a plan is not a source that decides the test command, and that working notes are written only when it judges them necessary, are not committed, and are not read by a person

## A plan is not reviewed, and when the run converges the cycle adds a commit that deletes it

Status: proposed

### Problem

A plan committed on the working branch enters the diff from the commit where the branch parted from the merge target. Left as it is, round trips of review and fixing are paid for a document that disappears, and it enters the merge target too. In a form where the implementer that finished the last step deletes it, there is no plan when a run that ended without converging is resumed. The cycle itself neither implements nor fixes, but delegating an operation that only deletes one file adds one delegation.

### Decision

The reviewers and the consistency phase are handed their range with `docs/plans/` left out of it (the range of the consistency phase is decided in [The consistency phase reads the code of the diff and its surroundings, and the decisions in range](consistency-phase.md#the-consistency-phase-reads-the-code-of-the-diff-and-its-surroundings-and-the-decisions-in-range)). No finding is raised on a plan.

When the run converges, before the branch is pushed, the cycle itself adds a commit that deletes the plan. The cycle confirms with git that the commit touches only files in `docs/plans/` and that the head of the branch has no file in `docs/plans/`. The review and the consistency phase are not run again for this commit. The line is judged again as for any other commit ([The line is judged by git alone, by whether the diff touched a path in .kotowari/confirm-paths](#the-line-is-judged-by-git-alone-by-whether-the-diff-touched-a-path-in-kotowariconfirm-paths)).

In a run that ended without converging, the plan is not deleted and stays for the resume.

### Rejected options

- The implementer that finished the last step deletes it. There is no plan when a run that ended without converging is resumed.
- Review the plan as a document. The cost of review and fixing is paid for a document that disappears.

### Confirmations

- C-core-merge-63 (by reading) The kotowari-cycle skill says that the reviewers and the consistency phase are handed their range with `docs/plans/` left out of it and are made to raise no finding on a plan
- C-core-merge-64 (by reading) The `references/merge.md` of kotowari-cycle says that in a converged run, before the branch is pushed, the cycle adds a commit that deletes the plan, confirms with git that the commit touches only files in `docs/plans/` and that the head of the branch has no file in `docs/plans/`, and does not run the review and the consistency phase again for this commit
- C-core-merge-65 (by reading) The kotowari-cycle skill says that in a run that ended without converging the plan is not deleted

## The cycle receives the place of the decision documents, the list of decisions to build this time, and the merge target

Status: implemented

### Problem

The plan carried four roles: the input of the cycle, the clue for resuming, the test command, and the order and the files to touch. After the plan step is removed, unless a destination is decided for each, the specification is silent. The check that the specification is approved, which the plan workflow did, also disappeared with that workflow. A run started by handing over decisions directly runs even on a specification that has not been approved.

When the range of the review starts at where the branch parted from its parent and the pull request starts at where it parted from the merge target, the two differ, and on a branch cut from a branch not yet in the merge target, the parent's commits can be merged without having been reviewed.

### Decision

The cycle receives from the brainstorm the place of the decision documents, the list of decisions to build this time, and the merge target. When the merge target has not been decided, it is the repository's default branch. The first full review, the first run of the consistency phase, and the judgment of the line read from the commit where the branch parted from the merge target, and the pull request points at that merge target.

When it starts with the place of the decision documents received, the cycle confirms in the working directory that the place of the decision documents holds no uncommitted change, and that `kotowari check` reports no error on a decision document the branch changed. When either does not hold, it says the specification is not approved and stops.

### Rejected options

- When the range of the review and the range of the pull request differ, stop at the pull request. One more rule would be added, and stacked branches would always be merged by a person.

### Confirmations

- C-core-merge-5 (by reading) The kotowari-cycle skill says that it receives the place of the decision documents, the list of decisions to build this time, and the merge target, and that the merge target is the repository's default branch when it has not been decided
- C-core-merge-6 (by reading) The kotowari-cycle skill says that the first full review, the first run of the consistency phase and the line read from the commit where the branch parted from the merge target, and that the pull request points at the merge target
- C-core-merge-7 (by reading) The kotowari-cycle skill says that when it starts with the place of the decision documents received, it confirms that the place holds no uncommitted change and that `kotowari check` reports no error on a decision document the branch changed, and that when either does not hold it says the specification is not approved and stops

## The decisions received are written in the findings file, and a resume reads them from there

Status: implemented

### Problem

In a form where each resume counts the received decisions again from the branch's diff, for a request whose specification is already in the merge target, the branch's diff holds no specification and the count comes to zero. The resumed run could converge without implementing anything.

### Decision

The cycle writes the decisions it received in the findings file of each branch. A resume reads them from the file, and only when the file has no record does it count them again from the part of the branch's diff that lies in the place of the decision documents. The cycle makes no new file for resuming.

On a resume during implementation, the implementer decides what remains, using as clues the git history and those of the received decisions whose state is still "proposed". When a plan or the implementer's working notes exist, it reads them too.

### Rejected options

- When the count comes to zero, ask the person. A person would come in once at every resume.
- Have the implementer always write working notes, and make them the cycle's input and the clue for resuming. Not approving them aside, the step that writes a plan would effectively come back.

### Confirmations

- C-core-merge-8 (by reading) The kotowari-cycle skill says that the decisions received are written in the findings file, that a resume reads them from there, that only when there is no record does it count them again from the part of the branch's diff in the place of the decision documents, and that no new file is made for resuming
- C-core-merge-9 (by reading) The kotowari-cycle skill says that on a resume during implementation the implementer decides what remains from the git history and the decisions still proposed, and the kotowari-implement skill says that on a resume it also reads the working notes when there are any

## In small tasks and in the description of the findings file's shape too, what is received is handled as the list of decisions

Status: implemented

### Problem

The cycle writes the decisions it received in the findings file ([The decisions received are written in the findings file, and a resume reads them from there](#the-decisions-received-are-written-in-the-findings-file-and-a-resume-reads-them-from-there)). kotowari-iterate, which adds to the same file, and the kotowari-review material that describes the file's shape have to write the same thing too. The reason for aligning them is the same as the cycle's decision, and no other reason remains.

### Decision

What kotowari-iterate adds to the findings file on a later request to the same branch, and what is received as described by the material on the findings file's shape, are the list of decisions, as in the cycle.

### Confirmations

- C-core-merge-10 (by reading) The kotowari-iterate skill and the `references/finding-schema.md` of kotowari-review say that what is received and written in the findings file is the list of decisions

## A branch name is not reused after its request ends, and a start or a resume reads the state on the hosting service before going on

Status: implemented

### Problem

Looking up a pull request by the branch name alone, when a branch of the same name is reused, reads the pull request of the earlier request as this one's. Telling them apart by the head commit of the pull request does not work in a repository that merges with merge commits (the head of the earlier request becomes an ancestor of the merge target, and so also an ancestor of a new branch cut from it; this was confirmed in this repository). Not reusing names is simpler than adding a rule to tell them apart. Since findings files are not deleted, reading the file of a run that ended in a merge as a resume would carry over the earlier run's findings and ending.

Guessing the merge target makes the line and the range of the review read from a different branch. On the other hand, the cycle runs even on a host where pull requests cannot be created, so stopping at the start because the hosting service cannot be read would mean nothing could start there. The request to merge is read again before the push, so the checks skipped at the start can be made there.

Unless the start of a new request, a later request to the same branch, and a resume are kept apart, a later request is folded into a resume. When a brainstorm resumed after a hand-back changes the specification and passes it to the cycle, the check of the approval is skipped, and if the first review has been done, none of the new decisions reaches the implementer. A later request's review reads from where the branch parted from the merge target so that commits that were not reviewed are not merged. The reason for reading the state from the hosting service on a resume after the pull request was created is not recorded.

### Decision

A branch name is not reused after its request has ended. At a start that received a new request, when there is a merged or closed pull request under that branch name, the cycle stops and asks the person for a new branch name. When the head of the branch's open pull request is not on the local branch, it stops and tells the person, at any start and at a resume. The findings file of a run that ended in a merge is not read as a resume by a run started later on a branch of the same name.

When the merge target has not been decided and the repository's default branch cannot be read either, the cycle asks the person and does not guess. When the hosting service cannot be read at the start, it goes on without the two checks above, makes them before the push, and writes the checks it could not make in the final report.

A resume is a start that brings no request. A start that brings a request to a branch that already has a findings file is not a resume but a later request to the same branch: it confirms that the specification is approved, adds the received decisions, begins again from implementation, and reads its consistency phase run and its full review from where the branch parted from the merge target. The start of the cycle goes in this order: read the branch's pull requests, confirm the specification, create or rewrite the findings file.

A resume of a branch that has a pull request reads from the hosting service whether the pull request exists, whether the merge was asked for, the state of the checks, and whether it was merged.

### Rejected options

- Previous decision: tell the earlier request's pull request apart by the head commit of the pull request. Dropped because it does not work in a repository that merges with merge commits.
- Write the pull request number in the findings file and tell them apart by it. When the file is lost, they cannot be told apart.

### Confirmations

- C-core-merge-34 (by reading) The kotowari-cycle skill says that a branch name is not reused after its request has ended, that at a start that received a new request it stops and asks for a new branch name when there is a merged or closed pull request, and that when the head of the open pull request is not on the local branch it stops and tells, at any start and at a resume
- C-core-merge-35 (by reading) The kotowari-cycle skill says that a findings file whose ending is merged is not read as a resume by a run started on a branch of the same name
- C-core-merge-36 (by reading) The kotowari-cycle skill says that when the merge target has not been decided and the default branch cannot be read either it asks the person and does not guess, and that when the hosting service cannot be read at the start it goes on without the two checks, makes them before the push, and writes the checks it could not make in the final report
- C-core-merge-37 (by reading) The kotowari-cycle skill says that a resume is a start that brings no request, that a start that brings a request to a branch with a findings file is a later request to the same branch, which confirms the approval, adds the decisions, begins again from implementation, and reads the consistency phase and the full review from where the branch parted from the merge target, and that the start goes in the order: read the pull requests, confirm the specification, create or rewrite the findings file
- C-core-merge-38 (by reading) The `references/merge.md` of kotowari-cycle says that a resume of a branch that has a pull request reads from the hosting service whether the pull request exists, whether the merge was asked for, the state of the checks, and whether it was merged

## The test command is decided by the project's instructions or the language's standard tool, and the person is asked when neither settles it

Status: implemented

### Problem

The plan carried the test command. Also, when the implementer returned that a check command could not succeed as written, it used to be sent back to the plan, but now that the cycle is where it returns, there was no rule for receiving it.

### Decision

The test command is decided by the project's instructions or by the standard tool of that language. When neither settles it, the person is asked. When the implementer returns that a check command cannot succeed as written, the cycle asks the person and delegates again with the answer.

### Rejected options

- When a check command does not succeed, end by sending it back to the brainstorm. A mere mistake in a command would mean redoing everything from the brainstorm.

### Confirmations

- C-core-merge-11 (by reading) The kotowari-cycle skill says that when the test command is not settled by the project's instructions or the language's standard tool, and when the implementer returns that a check command cannot succeed as written, it asks the person and delegates again with the answer

## The brainstorm commits the specification on a new branch, and after approval checks it out in a working directory before the cycle starts

Status: implemented

### Problem

The same branch cannot be checked out in two working trees. If the cycle runs as is in the main working tree, the findings file ends up inside the place that the cleanup removes.

Staged files follow a branch switch. If the main working tree is put back before the commit, the specification is carried back to the original branch, and the cycle is handed a new branch with nothing in it.

### Decision

The brainstorm, in the main working tree, cuts a new branch from the head of the merge target and commits the specification on it. After the approval, it puts the main working tree back on the original branch and checks the new branch out in a working directory of its own before the cycle starts. When the main working tree cannot be switched, it stops and tells the person. A brainstorm resumed after the cycle handed the work back writes in that working directory.

When the person does not commit it themselves but says it may be committed, the brainstorm commits it on the new branch. While an approved file is not committed on the new branch, it does not put the main working tree back: it tells the person which files are not committed and stops.

### Rejected options

- Make the working directory at the start of the brainstorm and write the specification there. The branch name would have to be decided at the point the brainstorm begins.

### Confirmations

- C-core-merge-12 (by reading) The kotowari-brainstorm skill says that it cuts a new branch from the head of the merge target in the main working tree and commits the specification, that after the approval it puts the main working tree back and checks the new branch out in a working directory, that when it cannot switch it stops and tells the person, and that a resume after a hand-back writes in that working directory
- C-core-merge-33 (by reading) The kotowari-brainstorm skill says that when the person said it may be committed, the brainstorm commits on the new branch, and that while an approved file is not committed on the new branch it does not put the main working tree back, tells the person which files, and stops

## A person confirms the merge for three kinds: operations that cannot be undone, changes that change the specification, and changes that change the checks or the line

Status: implemented

### Problem

Whether confirmation is needed is not all or nothing; it depends on the kind of change. A change to the definitions of the checks moves the very standard of what passes, so it cannot be measured by the same checks.

### Decision

A person confirms the following three kinds. Anything else is merged when all of the machine's checks pass.

| Kind | What covers it |
|---|---|
| Operations that cannot be undone (release, publishing, tags, deleting operations) | The existing stop conditions, by the operation itself rather than the nature of the diff |
| Changes that change the specification | The stop conditions of the workflow, not paths |
| Changes to the definitions of the checks and to the line itself | The list of paths ([The line is judged by git alone, by whether the diff touched a path in .kotowari/confirm-paths](#the-line-is-judged-by-git-alone-by-whether-the-diff-touched-a-path-in-kotowariconfirm-paths)) |

### Rejected options

- At first a person confirms everything, and kinds are removed one by one by looking at the records. Until the first one passes without a person, a person comes in as often as now.
- Decide whether a person's confirmation is needed by the paths of the changed files alone. A safe change and a dangerous change in the same file cannot be told apart, and whether it left the specification is not visible.

## The line is judged by git alone, by whether the diff touched a path in .kotowari/confirm-paths

Status: implemented

### Problem

What is needed to keep the decided line is only whether the diff touched the decided paths, and git can answer that. Classifying changes by kind and three levels of handling were complexity the line did not need. If the list is written in the skill, other projects cannot use it without rewriting the skill. Writing it in kotowari's own configuration file does not fit the decision to add nothing to kotowari itself.

Putting the place of the specification in the list would stop every request that started from a brainstorm. Applying kotowari's candidate list to 32 past merged requests, 8 were hit. With a single base point, there is no stretch that passes without being checked. A line with trailing blanks matches nothing (measured), and that is hard to see in a diff.

A change that deletes a line of the list touches the list file, so making the file itself always a hit means the line cannot be slipped past. Reading only the branch's list leaves lines added to the merge target's list after the branch parted without effect. Matching the lines together lets a single added line that excludes paths take away the hits of the other lines. Making a failure of the procedure a hit points the same way as treating a missing list file the same as a hit, and fits the rule that the result has only two values. A failure has no paths that were hit, so what is shown as the cause is the failure itself. The reason for counting a moved file under both its old path and its new path is not recorded. If the line is checked only once, fixes the cycle added after asking for the merge would be merged without being checked.

### Decision

The line between a change a person confirms and a change merged without confirmation is judged by git alone. No new command is made, no kotowari command is used, and no LLM judges it. The result has only two values: hit or no hit.

The list of paths is placed in a per-project file `.kotowari/confirm-paths`, one per line, written in git's path notation. Each line is used as a pathspec after the blanks around it and a carriage return at its end are taken off. The skill holds only the procedure. In a project without this file, merging by itself is not used, and the cycle stops at the pull request.

Two things go in the list: the definitions of the checks (CI, hooks, the scripts that run checks, the configuration of mutation tests, the configuration of static analysis and of tool versions), and the line itself (the list file and the skill that writes the procedure of the line). The place of the decision documents does not go in. Workflow skills other than the skill that writes the procedure of the line are handled the same as ordinary code.

For every request, the judgment of the line checks the whole diff from the commit where the branch parted from the merge target to the head of the branch. Each time the cycle adds commits to the branch, it judges again over the whole diff.

The pathspecs are read from both the list at the head of the merge target and the list at the head of the branch, and a hit under either is a hit. Each line is matched on its own, so that a line that excludes paths does not take away the hits of other lines. A change to the list file itself is a hit, whatever the list says. A moved file counts under both its old path and its new path. When a command of the procedure fails, the result is a hit with no paths that were hit, and it is reported as a failure, with its error output.

### Rejected options

- Make a new kotowari command that reads the diff and returns the handling.
- Leave the judgment of the line to an LLM that read the skill. The judgment would waver even for the same diff, and whether the line was kept could not be checked afterwards.
- Leave the judgment of the line to settings on the hosting service side. It would not work in a project that does not use that service.
- Put all workflow skills in the list. Every request that fixes a skill would stop once.
- Remove the definitions of the checks from the list, and build now a mechanism that measures the checks with broken samples. To be decided after looking in the records at how many times requests that change the definitions of the checks stopped.
- Make a line that matches nothing a failure of the procedure. Paths that do not exist yet could no longer be written in the list in advance.
- Make the base point of the diff checked by the line the commit of the specification's approval. After the place of the specification was removed from the list there is no reason for it, and changes before the approval commit would escape the line.
- Write the list of paths as a section inside the project's instruction document. A procedure to extract the list from the document would be needed, and other edits of the document would also hit the line.

### Confirmations

- C-core-merge-13 (by reading) The `references/merge.md` of kotowari-cycle says that the line is judged by git alone, using neither kotowari commands nor an LLM's judgment, and that the result has only two values, hit or no hit
- C-core-merge-14 (by reading) `references/merge.md` says that the paths are matched against the pathspecs written one per line in `.kotowari/confirm-paths`, that each line is read with the blanks around it and a carriage return at its end taken off, and that when the file does not exist the result is the same as a hit
- C-core-merge-15 (by reading) `references/merge.md` says that what a project writes in this file is the definitions of its checks (CI, hooks, the scripts that run checks, the configuration of mutation tests, the configuration of static analysis and of tool versions), and that the place of the decision documents is not written, and a change to the list file itself is a hit
- C-core-merge-16 (by reading) `references/merge.md` says that the line is judged on the diff from the commit where the branch parted from the merge target to the head of the branch
- C-core-merge-39 (by reading) `references/merge.md` says that each time the cycle adds commits to the branch, the line is judged again over the whole diff
- C-core-merge-40 (by reading) `references/merge.md` says that both the merge target's list and the branch's list are read and a hit under either is a hit, and that each line is matched on its own so that no line can take away another line's hit
- C-core-merge-41 (by reading) `references/merge.md` says that a change to the list file itself is a hit whatever it says, and that a moved file counts under both its old path and its new path
- C-core-merge-42 (by reading) `references/merge.md` says that when a command of the procedure exits with other than 0, the result is a hit whatever it printed, no paths that were hit are recorded, and it is reported as a failure with its error output

## A run that does not hit the line creates a pull request and asks the hosting service to merge it once the required checks pass

Status: implemented

### Problem

Merging locally and pushing is refused by the protection of a merge target that requires checks. A form where a person presses the button leaves one wait. The repository's settings are part of the standard for passing, and belong to the root of the yardstick. Making the setting for merging by itself the only condition would let a project with no required checks at all be merged without anything being checked.

When the merge target requires an approving review, the request stays while the person keeps waiting without being told their approval is needed. Branch protection rules come back as absent to an account that cannot see them, so two reads that only read the settings both come out with the value "not required" even for a merge target that requires an approving review. The pull request's own reviewDecision returns the result the hosting service judged, whatever setting requires it. The REST query of branch protection that returns the number of approving reviews returned 1 even for a merge target that does not require an approving review (measured). A pull request whose target was changed has its line and reviews read against one branch and is merged into another. The reason for stopping when the conditions cannot be confirmed is not recorded.

### Decision

In a run that does not need a person's confirmation, the cycle creates the pull request and asks the hosting service to merge it by itself once the required checks pass. What actually merges is the hosting service, and the protection of the merge target is the last gate. The cycle does not perform the merge itself.

It asks only when it has confirmed that the repository's setting for merging by itself is enabled, that the merge target has one or more required checks, that the merge target does not require an approving review, and that the pull request points at the merge target the cycle received. When any of these is missing or cannot be confirmed (including when a command that reads it fails), it stops at the pull request, reports that merging by itself is not available and why, and the person merges.

Whether the merge target requires checks is read in two ways: branch protection and rulesets. When either prints a non-empty list and neither fails, checks are required; when both succeed and print nothing, they are not required; when even one fails, it is treated as not confirmed, whatever the other printed. That the merge target requires no approving review is treated as confirmed only when three things all hold: the read of branch protection (GraphQL's requiresApprovingReviews) is false, the read of rulesets is 0, and the pull request's own reviewDecision is empty. Any other output, an empty output included, is treated as not confirmed.

On a resume, when the open pull request points at a branch other than the merge target received, or where it points cannot be read, the cycle withdraws the request to merge that remains and hands the pull request to the person with the reason.

The repository setting that makes merging by itself available is enabled by a person, and the cycle does not change the repository's settings. On a host where pull requests cannot be created, it stops after pushing the branch.

### Rejected options

- When the setting for merging by itself is enabled, leave it to that without looking at whether there are required checks. A project with no required checks at all could be merged without anything being checked.

### Confirmations

- C-core-merge-17 (by reading) `references/merge.md` says that the cycle creates the pull request and asks the hosting service to merge it by itself once the required checks pass, and that the cycle does not perform the merge itself
- C-core-merge-18 (by reading) `references/merge.md` says that it asks only when the repository allows merging by itself and the merge target requires one or more checks, and that otherwise it stops at the pull request, reports that merging by itself is not available, and the person merges
- C-core-merge-19 (by reading) `references/merge.md` says that the cycle does not change the repository's settings, and that on a host where pull requests cannot be created it stops after pushing the branch
- C-core-merge-43 (by reading) `references/merge.md` says that, as a condition for asking, that the merge target requires no approving review is treated as confirmed only when the three hold together: branch protection's requiresApprovingReviews is false, the ruleset is 0, and the pull request's reviewDecision is empty, and that any other output, empty included, is treated as not confirmed
- C-core-merge-44 (by reading) `references/merge.md` says that, as a condition for asking, it confirms that the pull request points at the merge target the cycle received, and that on a resume, when it points at another branch or cannot be read, it withdraws the request and hands it to the person with the reason
- C-core-merge-45 (by reading) `references/merge.md` says that when either of the two reads of required checks fails, whether checks are required is treated as not confirmed, and that when any condition for asking does not hold or cannot be confirmed it stops at the pull request, reports why it is not available, and the person merges

## Only a converged run goes on to the merge, and the request to merge is withdrawn before commits are added to the branch and when its conditions break

Status: implemented

### Problem

A run that has not converged still has mistakes that running something shows left open. Listing only the cases where a person confirms reads as if such runs were merged without a person.

If fixes are added to the branch while the request to merge is alive, the branch can be merged as soon as the checks pass, before the judgment of the line and the review of those fixes are done. Fixes after a failed check easily touch the definitions of the checks. Deciding whether to withdraw from the run's memory, "whether this run asked", lets delegation go on with the request still standing on a resume after the run that asked ended in the middle of its watch, or on a new request where kotowari-iterate reuses the branch. Local commits do no harm until they are pushed, so it may go on even when the request cannot be read at the start, but if it cannot be read before the push, commits that were not checked can enter a pull request that has a request to merge. On a host where pull requests cannot be created, a request never stands.

### Decision

Only a run in which the cycle reached the converged ending goes on to the merge. A run that ended at the round-trip limit, by no progress, or by a hand-back reports to the person, creates no pull request, and does not ask for the merge.

When the hosting service shows a request to merge on the branch's pull request, the cycle withdraws it at the following times, whether or not this run asked.

- Before a delegation that adds commits to the branch, and before the push. From whichever row a resume enters, when running more, for a later request to the same branch, and when kotowari-iterate reuses the branch to start a new request, it first reads from the hosting service
- When the run no longer converges, when the line is hit, and when a case where a person confirms holds
- When it hands the pull request to the person

It asks again only after confirming, for the new head, that the run converged, that the line is not hit, and that no case where a person confirms holds.

When it cannot read before the push whether a request remains, it does not push: it stops and tells the person, with the error output. Only on a host where pull requests cannot be created does it go on to the push.

### Confirmations

- C-core-merge-47 (by reading) `references/merge.md` says that only a run of the converged ending comes there, and that a run of another ending creates no pull request, asks for no merge, and reports to the person, and the endings of the kotowari-cycle skill say that endings other than convergence ask for no merge
- C-core-merge-48 (by reading) `references/merge.md` says that whether to withdraw the request is decided by whether the hosting service shows a request, and that before a delegation that adds commits to the branch and before the push it reads from the hosting service, also for a resume, running more, a later request to the same branch, and a kotowari-iterate request that reuses the branch
- C-core-merge-49 (by reading) `references/merge.md` says that it also withdraws the request when the run no longer converges, when the line is hit, when a case where a person confirms holds, and when it hands the pull request to the person, and that it asks again only after confirming for the new head that it converged, that the line is not hit, and that no case where a person confirms holds
- C-core-merge-50 (by reading) `references/merge.md` says that when the request cannot be read before the push it does not push but stops and tells the person, and that only on a host where pull requests cannot be created does it go on to the push

## After creating the pull request, the required checks are watched to their result, and a failed check is fixed as a finding

Status: implemented

### Problem

Ending when the pull request is created leaves a request whose checks failed stopped until a person notices. Right after creating the pull request, or right after the checks pass, the state the tool returns may be misread. The tool's behaviour was not measured at the time of the decision, so it was written in a way that does not misread under either behaviour. Without an ending for a run that could not confirm whether checks are required, when no check appears at all, it becomes a wait with no means to go on.

### Decision

After creating the pull request, the cycle watches the required checks to their result. A failed required check is put in the findings file as one finding, and follows the existing fixing round trips and way of stopping (when the same finding remains twice in a row, the premise is questioned, and if it still makes no progress, the run ends). When the checks still do not pass, the pull request is handed to the person.

When the merge target requires checks, required checks that have not reported yet are not read as a failure: the cycle reads again until they appear. After the checks pass, while the request to merge remains, it waits for the merge. It reads the hosting service as not merging only when the request is gone, or when the state is one that waiting does not change (a conflict, the branch behind the merge target).

When even one read of the names of the required checks fails, there are taken to be no names to wait for: the cycle watches the checks that are visible, and then hands the pull request to the person with the failure and its error output.

### Rejected options

- Measure the tool's behaviour before deciding. One more request would be needed before enabling merging by itself.

### Confirmations

- C-core-merge-20 (by reading) `references/merge.md` says that after creating the pull request it watches the required checks to their result, that a failed required check is put in the findings file as a finding and follows the existing round trips and endings, and that when the checks still do not pass it hands the pull request to the person
- C-core-merge-21 (by reading) `references/merge.md` says that required checks not reported yet are not read as a failure and are read again until they appear, that after the checks pass it waits for the merge while the request remains, and that it reads the hosting service as not merging only when the request is gone, or on a conflict or the branch being behind
- C-core-merge-46 (by reading) `references/merge.md` says that when a read of the names of the required checks fails, it watches the visible checks and then hands the pull request to the person with the failure and the error output

## When a person confirms, the pull request is left open without asking for the merge, and only its location and the cause of the stop are shown

Status: implemented

### Problem

Paths cannot tell the concretisation of a detail from a change to the specification. That distinction is already held by the rules of the consistency phase. With one way of stopping, there is one place for the person to look. Limiting the origin of findings awaiting a person's judgment to the consistency phase would merge even when a review left something awaiting judgment. Adding a summary of the change at the stop could lead to a form where the person reads the summary, approves, and lets it through without reading the diff itself.

### Decision

Of the runs that go on to the merge, a person confirms the merge when the line is hit, and when a finding awaiting a person's judgment (human_judgment) remains open, whatever its origin (the consistency phase, a review). In that case, the cycle creates the pull request, does not ask for it to be merged by itself, leaves it open, and shows the person where the pull request is and the cause of the stop (the paths that were hit, or the open findings). It adds no summary of the change. The person reads the diff and, if it is good, merges it themselves.

When the implementer returns that intents the person decided themselves conflict, a person comes in and nothing is merged. Operations that cannot be undone (a release, publishing, a tag, deleting something outside the local machine) stay with the person under the existing stop rules.

Details the implementing side or the consistency phase decided after the approval, and sentences of decisions corrected to match the facts without changing the decided direction, are not reasons to stop.

### Confirmations

- C-core-merge-22 (by reading) `references/merge.md` says that when the line is hit, and when a human_judgment finding remains open whatever its origin (the consistency phase, a review), a person confirms, and the cycle creates the pull request, does not ask for it to be merged by itself, leaves it open, shows its location and the cause of the stop, and adds no summary of the change
- C-core-merge-23 (by reading) `references/merge.md` says that when the implementer returns a conflict of intent a person comes in and nothing is merged, and that operations that cannot be undone stay with the person under the existing stop rules
- C-core-merge-24 (by reading) `references/merge.md` says that details the implementing side or the consistency phase decided, and sentences of decisions corrected to match the facts without changing the direction, are not reasons to stop

## Of the review findings, only three kinds keep a run from converging

Status: implemented

### Problem

Even when a person reads it, they cannot necessarily steer the quality of a design correctly. Making it a reason to stop runs against the principle that only what cannot be decided without a person is handed back to a person.

Unless findings whose way of checking was replaced with "all existing checks pass" are counted, mistakes the reviewer ran and saw fail could be merged without a person, unfixed. That verification the diff added does not meet the evidence conditions is settled by reading it against the conditions, but because its way of checking is "existing checks still pass after the deletion", it is not something that fails now. Under the rule of showing by running, it would always end up record-only, and tests that are not evidence would remain.

Handing back to a person a security finding that can only be fixed leaves the person nothing to choose. Applying the same rule as other findings (fix only what was seen failing) could let security findings that were not run be merged without a person, unfixed. The reason for limiting what goes to the fixer to findings that propose a fix is not recorded. Unless a security finding that asks for a person's judgment is counted, while open, toward a person's confirmation, it is merged without waiting for the person's judgment.

### Decision

Of the review findings, only the following three kinds keep a run from converging.

- A mistake that running something shows. A finding whose way of checking was replaced with "all existing checks pass" also counts as this, if the reviewer ran its own way of checking before the replacement and saw it fail
- A security finding whose proposed action is a fix
- A proposal to delete verification the diff added because it does not meet the evidence conditions. It is fixed even without having been seen failing

A security finding does not stop the run to wait for a person. It goes to the fixer whether or not it was seen failing, and when it stops making progress without being fixed, it reaches the person through the existing ending for no progress. What was fixed is read afterwards as one line of judgment.

Only security findings whose proposed action is a fix go to the fixer. A security finding that asks for a person's judgment stays open as a finding awaiting a person's judgment, and counts as a case where a person confirms the merge. A security finding that proposes only recording stays recorded only.

### Rejected options

- Assign one seat for re-examination only when the design is split. It would have to be decided in a form where the machine can say that it is split, and there are no grounds yet that it works.
- Put requests that include design judgments on the side a person confirms. The reason it was not taken is not recorded.
- Leave proposals to delete verification that does not meet the evidence conditions as record-only. Tests that are not evidence would remain.
- Hand the evidence conditions to the implementer from the start. What is still added would remain record-only.
- Apply to security findings the same rule as other findings. Findings that were not run could be merged without a person, unfixed.
- For security findings, always stop and hand them to a person, as now. For findings that can only be fixed, the person has nothing to choose.

### Confirmations

- C-core-merge-25 (by reading) The kotowari-cycle skill says that the review findings that keep a run from converging are three kinds: a mistake that running something shows, a security finding that proposes a fix, and a proposal to delete verification the diff added because it does not meet the evidence conditions
- C-core-merge-26 (by reading) The kotowari-cycle skill says that a finding whose way of checking was replaced counts as shown by running if the reviewer saw it fail before the replacement, and that a deletion proposal is fixed even without having been seen failing
- C-core-merge-27 (by reading) The kotowari-cycle skill says that a security finding does not stop the run, goes to the fixer whether or not it was seen failing, reaches the person through the ending for no progress when it cannot be fixed, and leaves what was fixed as one line of judgment
- C-core-merge-51 (by reading) The kotowari-cycle skill says that what goes to the fixer is the security finding that proposes a fix, that one proposing human_judgment stays open and `references/merge.md` counts it as a reason for a person's confirmation, and that one proposing record_only stays recorded only

## Deleting tests, suppressing checks, changing dependencies, and choices of readability and structure do not wait for a person

Status: implemented

### Problem

If a deleted test is a poor test, deleting it is an improvement. Even when a person reads it, they cannot necessarily judge correctly whether it is good, and mutation tests measure more accurately whether protection decreased. A change that deleted the only test of a requirement was failed by the existing checks (measured at the time of the decision). A change of dependency can be checked afterwards if its reason and purpose remain, and until a release it can be removed and reverted. Sending preferences of readability or structure to a person runs against the principle that only what cannot be decided without a person is handed back to a person. If the implementer's judgments appear only in the final report, they cannot be read again after the session is closed.

### Decision

A change that deletes existing tests, a change that adds a suppression of a check, and a change that adds or changes a dependency are not sent to a person for that reason. Choices of readability and structure are decided by the implementer, and are neither a reason to stop nor a reason to add a review seat.

Each judgment the implementer made by itself, an added dependency included, is made one line holding what was decided, why, and the evidence. The same lines as in the pull request body ([Details found during implementation are decided by the implementing side](workflow.md#details-found-during-implementation-are-decided-by-the-implementing-side)) also appear in the cycle's final report.

### Rejected options

- Put a change that deletes existing tests on the side a person confirms. If the deleted test is a poor test it is an improvement, and even when a person reads it they cannot necessarily judge.
- A person confirms when a new dependency is added. It can be checked afterwards if its reason and purpose remain, and until a release it can be removed and reverted.

### Confirmations

- C-core-merge-28 (by reading) The kotowari-cycle skill says that a change that deletes tests, a change that adds a suppression of a check, and a change that adds or changes a dependency do not stop the run for that reason, and that choices of readability and structure are decided by the implementer and are neither a reason to stop nor a reason to add a review seat
- C-core-merge-29 (by reading) The kotowari-cycle skill says that each judgment the implementer made by itself, an added dependency included, is made one line of what, why and the evidence, and the same lines as in the pull request body are put in the final report

## For a run, no new record is made; the result of the line, what it is a fix of, and the ending are added to the findings file

Status: implemented

### Problem

To build a loop that later reads the records and improves the workflow, the minimum items needed to read where paying reduces rework have to be kept. The number of round trips between review and fixing can already be counted from the per-round verdicts and fix records in the findings file the cycle writes for each request. The findings file is placed outside git, so there is also no concern that it shows up in a search and is mistaken for the current rules. How many records have to be kept to be readable is not known yet.

### Decision

The cycle makes no new record file about the run. It adds the following to the findings file it writes for each branch.

- The result of the judgment of the line, and the paths that were hit
- Which request this request is a fix of. It is pointed at by the branch name of the earlier request. It is written only when the request text says it is a fix of an earlier request (for a small task, kotowari-iterate writes it), and is left empty when unknown
- Which ending the run ended with, and its reason. Sending back to an earlier step and stopping to call a person are included
- When it judged that a plan is to be written, the one line of that reason

Cost is not recorded. The findings file is not committed, and the workflow does not delete it.

### Rejected options

- Commit the rework records to the repository as a small file per request, and add a setting that excludes them from search and an instruction not to read them as the current rules. The cycle was already writing almost the same content outside git. The same content would be written in two places, and a procedure to extract it and the exclusion setting would have to be built.
- Write which request it is a fix of by checking with git whether the diff touches lines the earlier request changed. A procedure to check it would have to be built, and requests that merely happened to touch the same lines would be counted as fixes.

### Confirmations

- C-core-merge-30 (by reading) The kotowari-cycle skill says that it makes no new record file, writes in the findings file the result of the line and the paths that were hit, the branch name of the request it is a fix of (only when the request says so), and the ending with its reason, and does not record cost
- C-core-merge-31 (by reading) The kotowari-cycle skill says that the findings file is not committed and the workflow does not delete it

## After the merge, only the local working directory and branch are cleaned up, and nothing on the hosting service is deleted

Status: implemented

### Problem

What to clean up after the merge had to be decided. Deleting something outside the local machine is an operation that cannot be undone, and belongs to what stays with the person.

Since it is decided that the findings file is not deleted, placing it inside the working directory could let the cleanup remove it along with the directory. Cleaning up without having seen the merge loses work.

### Decision

After the merge, the cycle cleans up only the local working directory and the local branch. It does not delete the branch or the pull request on the hosting service side, and leaves them to the repository's settings.

It cleans up only when the cycle saw the merge itself during this run's watch. A run the person merged later is not cleaned up. The findings file is placed outside the working directory the cleanup removes, and before the cleanup the cycle confirms that it is there. If it is not outside, nothing is deleted and this is reported. When the working directory holds uncommitted changes other than the implementer's working notes, neither the working directory nor the branch is deleted, and what remains is reported.

### Confirmations

- C-core-merge-32 (by reading) `references/merge.md` says that what is cleaned up is only the local working directory and the local branch, and that nothing on the hosting service side is deleted
- C-core-merge-52 (by reading) `references/merge.md` says that it cleans up only when the cycle saw the merge during this run's watch, and that a run the person merged later is not cleaned up
- C-core-merge-53 (by reading) The kotowari-cycle skill says that the findings file is placed outside the working directory, and `references/merge.md` says that before the cleanup it confirms that the file is outside, that if not it deletes nothing and reports, and that when there are uncommitted changes other than the implementer's working notes it leaves the working directory and the branch and reports

## Open

- Whether sharing the plan alone is enough for a handover between implementers (whether the implementer that takes over redoes steps already finished). The user decides. After the first request that wrote a plan has ended.
