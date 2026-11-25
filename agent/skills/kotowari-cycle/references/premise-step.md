# Questioning the premise

When fixes keep raising new findings next to the ones they closed, the fixes usually share a premise
that is wrong, and another fix on the same premise moves the problem without removing it. This step
names that premise and tries once with it replaced before the loop is handed to the person.

Cycle runs it itself, as the one judge of the loop; it adds no agent and no seat. Cycle's
**Endings** says which clauses of ending 3 enter it. kotowari-iterate runs it as part of cycle's
loop, the main session runs it on a direct edit (kotowari-using-workflow), and an implementer runs
its own form of it (**In implement**).

## The step

1. **Write the premise ladder.** Read the fixes behind the clause that fired: their commits, the
   findings each addressed, and the premises the fixer reported for a finding still present after
   a fix. Write the premises those fixes shared, one sentence per rung, from the most specific
   (the place the last fix touched) up to the one the others rest on. Each rung says where the
   premise lives: in the implementation, or in a decision document.
2. **Replace the top premise** the findings call into question: state what replaces it and which
   findings point there.
3. **Record the attempt** in the findings file before delegating anything (the kotowari-review
   skill's `references/finding-schema.md`, `premise_attempts`).
4. **Send it by where the premise lives:**
   - the implementation → the fixer, with the visible findings, the ladder, and the replaced
     premise, to fix from the new premise rather than patch the last finding; the loop goes on with
     the diff review after that fix;
   - a decision document, and grounds and measurements can decide it (a sentence that does not
     match the facts, a reading of it) → a consistency phase run, with the ladder and the replaced
     premise as a finding to resolve; the loop then goes on where the clause fired;
   - otherwise (the premise is the intent a person decided, which no grounds or measurement may
     replace) → end as ending 3; the terminal report carries the ladder.

   When the clause fired on a `consistency` finding, the first two both go to a consistency phase
   run, which fixes code as well as the decision documents: a `consistency` finding never goes to the fixer.

## One attempt per run

A run gets one attempt. After it, the next time any clause that enters this step holds, end as
ending 3 without another attempt; the terminal report carries the ladder of every attempt in the
findings file. On a resume or after "run more", read `premise_attempts` first: the run already has
its attempt when the list is not empty. Only a new start of kotowari-iterate on the branch renews
the allowance, since that start is a new request.

## In implement

An implementer whose approach has changed once without progress does not hand back yet. It writes
the premises its attempts shared, as a ladder like the one above, replaces the top one, and tries
once more from it. Still no progress: hand back to cycle with the ladder. When the premise is the
intent a person decided in a decision document, hand back at once; the implementer never replaces
it. A sentence it may correct to the facts, without changing that intent, is not such a premise.
